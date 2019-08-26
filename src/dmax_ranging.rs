

#define _DMaxData(field) VL6180xDevDataGet(dev, DMaxData.field)
/*
 * Convert fix point  x.7 to KCpount per sec
 */

#ifndef VL6180x_PLATFORM_PROVIDE_SQRT

/*
 * 32 bit integer square root with not so bad precision (integer result) and is quite fast
 * see http://en.wikipedia.org/wiki/Methods_of_computing_square_roots
 */
uint32_t VL6180x_SqrtUint32(uint32_t num)
{
    uint32_t res = 0;
    uint32_t bit = 1 << 30; /* The second-to-top bit is set: 1 << 30 for 32 bits */

    /* "bit" starts at the highest power of four <= the argument. */
    while (bit > num)
        bit >>= 2;

    while (bit != 0) {
        if (num >= res + bit) {
            num -= res + bit;
            res = (res >> 1) + bit;
        } else
            res >>= 1;
        bit >>= 2;
    }
    return res;
}
#endif


/* DMax one time init */
void _DMax_OneTimeInit(&mut self)
{
    self.params.dmax_data.ambTuningWindowFactor_K = DEF_AMBIENT_TUNING;
}


static uint32_t _DMax_RawValueAtRateKCps(&mut self, int32_t rate)
{
    int32_t DMaxSq;
    uint32_t RawDMax;

    let DMaxData {amb_tuning_window_factor_k, ret_signal_at_400mm, snr_limit_k, .. } = self.params.dmax_data;

    /* 12 to 18 bits Kcps */
    if (rate > 0) {
        DMaxSq = 400 * 400 * 1000 / rate - (400 * 400 / 330);
        /* K of (1/RtnAmb -1/330 )=> 30bit- (12-18)bit  => 12-18 bits*/
        if (DMaxSq <= 0) {
            RawDMax = 0;
        } else {
            /* value can be more 32 bit so base on raneg apply
             * ret_signal_at_400mm before or after division to presevr accuracy */
            if (DMaxSq < (2 << 12)) {
                DMaxSq = DMaxSq * ret_signal_at_400mm /
                            (snr_limit_k + amb_tuning_window_factor_k);
                /* max 12 + 12 to 18 -10 => 12-26 bit */
            } else {
                DMaxSq = DMaxSq / (snr_limit_k + amb_tuning_window_factor_k) * ret_signal_at_400mm;
                /* 12 to 18 -10 + 12 to 18 *=> 12-26 bit */
            }
            RawDMax = VL6180x_SqrtUint32(DMaxSq);
        }
    } else {
        RawDMax = 0x7FFFFFFF; /* bigest possibmle 32bit signed value */
    }
    return RawDMax;
}

/*
 * fetch static data from register to avoid re-read
 * precompute all intermediate constant and cliipings
 *
 * to be re-used/call on  changes of :
 *  0x2A
 *  SYSRANGE_MAX_AMBIENT_LEVEL_MULT
 *  Dev Data XtalkComRate_KCPs
 *  SYSRANGE_MAX_CONVERGENCE_TIME
 *  SYSRANGE_RANGE_CHECK_ENABLES    mask RANGE_CHECK_RANGE_ENABLE_MASK
 *  range 0xb8-0xbb (0xbb)
 */
fn _dmax_init_data(&mut self, i2c: &mut I2C) -> Option<ErrCode>
{
    uint8_t u8;
    uint16_t u16;
    uint32_t u32;
    uint32_t Reg2A_KCps;
    uint32_t RegB8;
    uint8_t  MaxConvTime;
    uint32_t XTalkCompRate_KCps;
    uint32_t RangeIgnoreThreshold;
    int32_t minSignalNeeded;
    uint8_t SysRangeCheckEn;
    uint8_t snrLimit;
    static const int ROMABLE_DATA MaxConvTimeAdjust = -4;

    warning = None;

    let mut b8 = self.read_bits(i2c, 0x02A);

    if (b8 == 0) {
        warning = Some(ErrCode::CalibrationWarning);
        b8 = 40; /* use a default average value */
    }
    Reg2A_KCps = Fix7_2_KCPs(u8); /* convert to KCPs */

    let SysRangeCheckEn = self.read(i2c, pac::SYSRANGE_RANGE_CHECK_ENABLES).bits();

    let MaxConvTime = self.read(i2c, pac::SYSRANGE_MAX_CONVERGENCE_TIME).bits();

    let RegB8 = self.read_bits(i2c, 0x0B8);

    let snrLimit = self.read(i2c, pac::SYSRANGE_MAX_AMBIENT_LEVEL_MULT).bits();
    _DMaxData(snrLimit_K) = 16_i32 * 1000 / snrLimit;
    XTalkCompRate_KCps =   self.dev.XTalkCompRate_KCps);

    if (Reg2A_KCps >= XTalkCompRate_KCps) {
        _DMaxData(retSignalAt400mm) = Reg2A_KCps;
    } else{
        _DMaxData(retSignalAt400mm) = 0;
        /* Reg2A_K - XTalkCompRate_KCp <0 is invalid */
    }

    /* if xtalk range check is off omit it in snr clipping */
    if (SysRangeCheckEn&RANGE_CHECK_RANGE_ENABLE_MASK) {
        let b16 = self.read(i2c, pac::SYSRANGE_RANGE_IGNORE_THRESHOLD).bits();
        RangeIgnoreThreshold = Fix7_2_KCPs(b16);
    } else{
        RangeIgnoreThreshold  = 0;
    }

    minSignalNeeded = (RegB8 * 256) / ((int32_t)MaxConvTime + (int32_t)MaxConvTimeAdjust);
    /* KCps 8+8 bit -(1 to 6 bit) => 15-10 bit */
    /* minSignalNeeded = max ( minSignalNeeded,  RangeIgnoreThreshold - XTalkCompRate_KCps) */
    if (minSignalNeeded  <= (int32_t)RangeIgnoreThreshold - (int32_t)XTalkCompRate_KCps)
        minSignalNeeded  =  RangeIgnoreThreshold - XTalkCompRate_KCps;

    b32 = (minSignalNeeded*(uint32_t)snrLimit) / 16;
    _DMaxData(ClipSnrLimit) = _DMax_RawValueAtRateKCps(dev, b32);
    /* clip to dmax to min signal snr limit rate*/

    warning
}

static int _DMax_Compute(&mut self, VL6180x_RangeData_t *pRange)
{
    uint32_t rtnAmbRate;
    int32_t DMax;
    int scaling;
    uint16_t HwLimitAtScale;
    static const int ROMABLE_DATA rtnAmbLowLimit_KCps = 330 * 1000;

    rtnAmbRate = pRange->rtnAmbRate;
    if (rtnAmbRate  < rtnAmbLowLimit_KCps) {
        DMax = _DMax_RawValueAtRateKCps(dev, rtnAmbRate);
        scaling = _GetUpscale(dev);
        HwLimitAtScale = UpperLimitLookUP[scaling - 1];

        if (DMax > _DMaxData(ClipSnrLimit)) {
            DMax = _DMaxData(ClipSnrLimit);
        }
        if (DMax > HwLimitAtScale) {
            DMax = HwLimitAtScale;
        }
        pRange->DMax = DMax;
    } else {
        pRange->DMax = 0;
    }
    return 0;
}

#undef _DMaxData
#undef Fix7_2_KCPs

