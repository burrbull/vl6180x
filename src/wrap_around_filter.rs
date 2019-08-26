#define PRESERVE_DEVICE_ERROR_CODE		/* If uncommented, device error code will be preserved on top of wraparound error code, but this may lead to some error code instability like overflow error <==> RangingFilteringOnGoing error oscillations */
#define SENSITIVE_FILTERING_ON_GOING	/* If uncommented, filter will go back to RangingFilteringOnGoing if it must go through the std dev testing */

#define FILTER_STDDEV_SAMPLES           6
#define MIN_FILTER_STDDEV_SAMPLES       3
#define MIN_FILTER_STDDEV_SAMPLES_AFTER_FLUSH_OR_BYPASS 5
#define STDDEV_BASE_VALUE               150

#define FILTER_INVALID_DISTANCE     65535

#define _FilterData(field) VL6180xDevDataGet(dev, FilterData.field)
/*
 * One time init
 */
int _filter_Init(&mut self)
{
    int i;
    _FilterData(MeasurementIndex) = 0;

    _FilterData(Default_ZeroVal) = 0;
    _FilterData(Default_VAVGVal) = 0;
    _FilterData(NoDelay_ZeroVal) = 0;
    _FilterData(NoDelay_VAVGVal) = 0;
    _FilterData(Previous_VAVGDiff) = 0;

    _FilterData(StdFilteredReads) = 0;
    _FilterData(FilteringOnGoingConsecutiveStates) = 0;

    for (i = 0; i < FILTER_NBOF_SAMPLES; i++) {
        _FilterData(LastTrueRange)[i] = FILTER_INVALID_DISTANCE;
        _FilterData(LastReturnRates)[i] = 0;
    }
    _FilterData(MeasurementsSinceLastFlush)=0;
    return 0;
}


static uint32_t _filter_StdDevDamper(uint32_t AmbientRate,
                                    uint32_t SignalRate,
                                    const uint32_t StdDevLimitLowLight,
                                    const uint32_t StdDevLimitLowLightSNR,
                                    const uint32_t StdDevLimitHighLight,
                                    const uint32_t StdDevLimitHighLightSNR)
{
    uint32_t newStdDev;
    uint16_t SNR;

    if (AmbientRate > 0)
        SNR = (uint16_t) ((100 * SignalRate) / AmbientRate);
    else
        SNR = 9999;

    if (SNR >= StdDevLimitLowLightSNR) {
        newStdDev = StdDevLimitLowLight;
    } else {
        if (SNR <= StdDevLimitHighLightSNR)
            newStdDev = StdDevLimitHighLight;
        else {
            newStdDev = (uint32_t)(StdDevLimitHighLight -
                                    (SNR - StdDevLimitHighLightSNR) *
                                    (StdDevLimitHighLight - StdDevLimitLowLight) /
                                    (StdDevLimitLowLightSNR - StdDevLimitHighLightSNR));
        }
    }

    return newStdDev;
}


/*
 * Return <0 on error
 */
static int32_t _filter_Start(&mut self,
                                uint16_t m_trueRange_mm,
                                uint16_t m_rawRange_mm,
                                uint32_t m_rtnSignalRate,
                                uint32_t m_rtnAmbientRate,
                                uint16_t errorCode)
{
    int status;
    uint16_t m_newTrueRange_mm = 0;
    #if VL6180x_HAVE_MULTI_READ
    uint8_t MultiReadBuf[8];
    #endif
    uint16_t i;
    uint16_t bypassFilter = 0;
    uint16_t resetVAVGData = 1;

    uint16_t filterErrorCode = NoError;
    uint16_t filterErrorCodeOnRangingErrorCode = NoError;

    uint16_t registerValue;

    uint32_t register32BitsValue1;
    uint32_t register32BitsValue2;

    uint16_t ValidDistance = 0;
    uint16_t SuspicuousRangingZone = 0;

    uint16_t WrapAroundFlag = 0;
    uint16_t NoWrapAroundFlag = 0;
    uint16_t NoWrapAroundHighConfidenceFlag = 0;

    uint16_t FlushFilter = 0;
    uint32_t RateChange = 0;

    uint16_t StdDevSamplesMinNeeded = 0;
    uint16_t StdDevSamples = 0;
    uint32_t StdDevDistanceSum = 0;
    uint32_t StdDevDistanceMean = 0;
    uint32_t StdDevDistance = 0;
    uint32_t StdDevRateSum = 0;
    uint32_t StdDevRateMean = 0;
    uint32_t StdDevRate = 0;
    uint32_t StdDevLimitWithTargetMove = 0;

    uint32_t VAVGDiff;
    uint32_t IdealVAVGDiff;
    uint32_t MinVAVGDiff;
    uint32_t MaxVAVGDiff;

    /* Filter Parameters */
    static const uint16_t ROMABLE_DATA WrapAroundLowRawRangeLimit = 60;
    static const uint32_t ROMABLE_DATA WrapAroundLowReturnRateLimit_ROM = 800;
    /* Shall be adapted depending on crossTalk */
    static const uint16_t ROMABLE_DATA WrapAroundLowRawRangeLimit2 = 165;
    static const uint32_t ROMABLE_DATA WrapAroundLowReturnRateLimit2_ROM = 180;
    /* Shall be adapted depending on crossTalk and device sensitivity*/
    static const uint32_t ROMABLE_DATA WrapAroundLowRawRangeLimit2SuspicuousAddedSignalRate = 150;


    static const uint32_t ROMABLE_DATA WrapAroundLowReturnRateFilterLimit_ROM = 850;
    /* Shall be adapted depending on crossTalk and device sensitivity*/
    static const uint16_t ROMABLE_DATA WrapAroundHighRawRangeFilterLimit = 350;
    static const uint32_t ROMABLE_DATA WrapAroundHighReturnRateFilterLimit_ROM = 1400;
    /* Shall be adapted depending on crossTalk and device sensitivity*/

    static const uint32_t ROMABLE_DATA WrapAroundMaximumAmbientRateFilterLimit = 15000;

    /*  Temporal filter data and flush values */
    static const uint32_t ROMABLE_DATA MinReturnRateFilterFlush = 75;
    static const uint32_t ROMABLE_DATA MaxReturnRateChangeFilterFlush = 50;

    /* STDDEV values and damper values */
    static const uint32_t ROMABLE_DATA StdDevLimitLowLight = STDDEV_BASE_VALUE;
    static const uint32_t ROMABLE_DATA StdDevLimitLowLightSNR = 30; /* 0.3 */
    static const uint32_t ROMABLE_DATA StdDevLimitHighLight = STDDEV_BASE_VALUE*6;
    static const uint32_t ROMABLE_DATA StdDevLimitHighLightSNR = 5; /* 0.05 */

    static const uint32_t ROMABLE_DATA StdDevHighConfidenceSNRLimit = 8;
    static const uint32_t ROMABLE_DATA StdDevNoWrapDetectedMultiplier = 4;

    static const uint32_t ROMABLE_DATA StdDevMovingTargetStdDevLimit = 90000;

    static const uint32_t ROMABLE_DATA StdDevMovingTargetReturnRateLimit = 3500;
    static const uint32_t ROMABLE_DATA StdDevMovingTargetStdDevForReturnRateLimit = STDDEV_BASE_VALUE*25;

    static const uint32_t ROMABLE_DATA MAX_VAVGDiff_ROM = 1800;
    static const uint32_t ROMABLE_DATA SuspicuousMAX_VAVGDiffRatio = 2;

    /* WrapAroundDetection variables */
    static const uint16_t ROMABLE_DATA WrapAroundNoDelayCheckPeriod = 2;
    static const uint16_t ROMABLE_DATA StdFilteredReadsIncrement = 2;
    static const uint16_t ROMABLE_DATA StdFilteredReadsDecrement = 1;
    static const uint16_t ROMABLE_DATA StdMaxFilteredReads = 4;

    uint32_t SignalRateDMax;
    uint32_t WrapAroundLowReturnRateLimit;
    uint32_t WrapAroundLowReturnRateLimit2;
    uint32_t WrapAroundLowReturnRateFilterLimit;
    uint32_t WrapAroundHighReturnRateFilterLimit;

    uint32_t MAX_VAVGDiff = 1800;

    uint8_t u8;//, u8_2;
    uint32_t XTalkCompRate_KCps;
    uint32_t StdDevLimit = 300;
    uint32_t MaxOrInvalidDistance =   255*_GetUpscale(dev);
    /* #define MaxOrInvalidDistance  (uint16_t) (255 * 3) */

    /* Check if distance is Valid or not */
    switch (errorCode) {
    case Raw_Ranging_Algo_Underflow:
    case Ranging_Algo_Underflow:
        filterErrorCodeOnRangingErrorCode = RangingFiltered; /* If we have to go through filter, mean we have here a wraparound case */
        ValidDistance = 0;
        break;
    case Raw_Ranging_Algo_Overflow:
    case Ranging_Algo_Overflow:
        filterErrorCodeOnRangingErrorCode = RangingFiltered; /* If we have to go through filter, mean we have here a wraparound case */
        //m_trueRange_mm = MaxOrInvalidDistance;
        m_trueRange_mm = 200*_GetUpscale(dev);
        ValidDistance = 1;
        break;
    default:
        if (m_rawRange_mm >= MaxOrInvalidDistance) {
            ValidDistance = 0;
            bypassFilter = 1; /* Bypass the filter in this case as produced distance is not usable (and also the VAVGVal and ZeroVal values) */
        } else {
            ValidDistance = 1;
        }
        break;
    }
    m_newTrueRange_mm = m_trueRange_mm;

    XTalkCompRate_KCps = VL6180xDevDataGet(dev, XTalkCompRate_KCps);

    /* Update signal rate limits depending on crosstalk */
    SignalRateDMax = (uint32_t)_GetDMaxDataRetSignalAt400mm(dev) ;
    WrapAroundLowReturnRateLimit = WrapAroundLowReturnRateLimit_ROM  + XTalkCompRate_KCps;
    WrapAroundLowReturnRateLimit2 = ((WrapAroundLowReturnRateLimit2_ROM *
                                    SignalRateDMax) / 312) +
                                    XTalkCompRate_KCps;
    WrapAroundLowReturnRateFilterLimit = ((WrapAroundLowReturnRateFilterLimit_ROM *
                                    SignalRateDMax) / 312) + XTalkCompRate_KCps;
    WrapAroundHighReturnRateFilterLimit = ((WrapAroundHighReturnRateFilterLimit_ROM *
                                    SignalRateDMax) / 312) + XTalkCompRate_KCps;


    /* Checks on low range data */
    if ((m_rawRange_mm < WrapAroundLowRawRangeLimit) && (m_rtnSignalRate < WrapAroundLowReturnRateLimit)) {
        filterErrorCode = RangingFiltered; /* On this condition, wraparound case is ensured */
        bypassFilter = 1;
    }
    if ((m_rawRange_mm < WrapAroundLowRawRangeLimit2) && (m_rtnSignalRate < WrapAroundLowReturnRateLimit2)) {
        filterErrorCode = RangingFiltered; /* On this condition, wraparound case is ensured */
        bypassFilter = 1;
    }
    if ((m_rawRange_mm < WrapAroundLowRawRangeLimit2) && (m_rtnSignalRate < (WrapAroundLowReturnRateLimit2 + WrapAroundLowRawRangeLimit2SuspicuousAddedSignalRate))) {
        SuspicuousRangingZone = 1; /* On this area, we are in an highly suspicuous wraparound ares, filter parameter will be stengthen */
    }


    /* Checks on Ambient rate level */
    if (m_rtnAmbientRate > WrapAroundMaximumAmbientRateFilterLimit) {
        /* Too high ambient rate */
        FlushFilter = 1;
        bypassFilter = 1;
    }

    /*  Checks on Filter flush */
    if (m_rtnSignalRate < MinReturnRateFilterFlush) {
        /* Completely lost target, so flush the filter */
        FlushFilter = 1;
        bypassFilter = 1;
    }
    if (_FilterData(LastReturnRates)[0] != 0) {
        if (m_rtnSignalRate > _FilterData(LastReturnRates)[0])
            RateChange = (100 *
                        (m_rtnSignalRate - _FilterData(LastReturnRates)[0])) /
                        _FilterData(LastReturnRates)[0];
        else
            RateChange = (100 *
                        (_FilterData(LastReturnRates)[0] - m_rtnSignalRate)) /
                        _FilterData(LastReturnRates)[0];
    } else
        RateChange = 0;
    if (RateChange > MaxReturnRateChangeFilterFlush) {
        FlushFilter = 1;
    }
    /* TODO optimize filter  using circular buffer */
    if (FlushFilter == 1) {
        _FilterData(MeasurementIndex) = 0;
        for (i = 0; i < FILTER_NBOF_SAMPLES; i++) {
            _FilterData(LastTrueRange)[i] = FILTER_INVALID_DISTANCE;
            _FilterData(LastReturnRates)[i] = 0;
        }
        _FilterData(MeasurementsSinceLastFlush)=0;
    } else {
        for (i = (uint16_t) (FILTER_NBOF_SAMPLES - 1); i > 0; i--) {
            _FilterData(LastTrueRange)[i] = _FilterData(LastTrueRange)[i - 1];
            _FilterData(LastReturnRates)[i] = _FilterData(LastReturnRates)[i - 1];
        }
    }

    if (ValidDistance == 1)
        _FilterData(LastTrueRange)[0] = m_trueRange_mm;
    else
        _FilterData(LastTrueRange)[0] = FILTER_INVALID_DISTANCE;
    _FilterData(LastReturnRates)[0] = m_rtnSignalRate;
    _FilterData(MeasurementsSinceLastFlush)++;

    /* Check if we need to go through the filter or not */
    if (!(((m_rawRange_mm < WrapAroundHighRawRangeFilterLimit) &&
        (m_rtnSignalRate < WrapAroundLowReturnRateFilterLimit)) ||
        ((m_rawRange_mm >= WrapAroundHighRawRangeFilterLimit) &&
        (m_rtnSignalRate < WrapAroundHighReturnRateFilterLimit))))
        bypassFilter = 1;
    else {
        /* if some wraparound filtering due to some ranging error code has been detected, update the filter status and bypass the filter */
        if(filterErrorCodeOnRangingErrorCode!=NoError){
#ifndef PRESERVE_DEVICE_ERROR_CODE
            filterErrorCode = filterErrorCodeOnRangingErrorCode;
#else
            if((errorCode==Raw_Ranging_Algo_Underflow) || (errorCode==Ranging_Algo_Underflow)) {
                /* Preserves the error codes except for Raw_Ranging_Algo_Underflow and Ranging_Algo_Underflow */
                filterErrorCode = filterErrorCodeOnRangingErrorCode;
            }
#endif
            bypassFilter = 1;
            resetVAVGData = 0;
        }
    }

    /* Check which kind of measurement has been made */
    registerValue = self.read_bits(i2c, 0x01AC);

    /* Read data for filtering */
#if VL6180x_HAVE_MULTI_READ
    status = VL6180x_RdMulti(dev, 0x10C, MultiReadBuf, 8); /* read only 8 lsb bits */
    if (status) {
        VL6180x_ErrLog("0x10C multi rd fail");
        goto done_err;
    }
    register32BitsValue1 = ((uint32_t) MultiReadBuf[0] << 24)
            + ((uint32_t) MultiReadBuf[1] << 16)
            + ((uint32_t) MultiReadBuf[2] << 8)
            + ((uint32_t) MultiReadBuf[3] << 0);
    register32BitsValue2 = ((uint32_t) MultiReadBuf[4] << 24)
            + ((uint32_t) MultiReadBuf[5] << 16)
            + ((uint32_t) MultiReadBuf[6] << 8)
            + ((uint32_t) MultiReadBuf[7] << 0);
#else
    let register32BitsValue1 = self.read_bits(i2c, 0x10C); /* read 32 bits, lower 17 bits are the one useful */
    let register32BitsValue2 = self.read_bits(i2c, 0x0110); /* read 32 bits, lower 17 bits are the one useful */
#endif


    if ((FlushFilter == 1) || ((bypassFilter == 1) && (resetVAVGData == 1))) {
        if (registerValue != 0x3E) {
            status = self.write_byte(0x1AC, 0x3E);
            if (status) {
                VL6180x_ErrLog("0x01AC bypass wr fail");
                goto done_err;
            }
            //status = self.write_byte(0x0F2, 0x01);
            //if (status) {
            //	VL6180x_ErrLog("0x0F2 bypass wr fail");
            //	goto done_err;
            //}
        }
        /* Set both Default and NoDelay To same value */
        _FilterData(Default_ZeroVal) = register32BitsValue1;
        _FilterData(Default_VAVGVal) = register32BitsValue2;
        _FilterData(NoDelay_ZeroVal) = register32BitsValue1;
        _FilterData(NoDelay_VAVGVal) = register32BitsValue2;

        _FilterData(MeasurementIndex) = 0;
    } else {
        if (registerValue == 0x3E) {
            _FilterData(Default_ZeroVal) = register32BitsValue1;
            _FilterData(Default_VAVGVal) = register32BitsValue2;
        } else {
            _FilterData(NoDelay_ZeroVal) = register32BitsValue1;
            _FilterData(NoDelay_VAVGVal) = register32BitsValue2;
        }

        if (_FilterData(MeasurementIndex) % WrapAroundNoDelayCheckPeriod == 0) {
            u8 = 0x3C;
            //u8_2 = 0x05;
        } else {
            u8 = 0x3E;
            //u8_2 = 0x01;
        }
        status = self.write_byte(0x01AC, u8);
        if (status) {
            VL6180x_ErrLog("0x01AC wr fail");
            goto done_err;
        }
        //status = self.write_byte(0x0F2, u8_2);
        //if (status) {
        //	VL6180x_ErrLog("0x0F2  wr fail");
        //	goto done_err;
        //}
        _FilterData(MeasurementIndex)++;
    }

    if (bypassFilter == 1) {
        /* Do not go through the filter */

        /* Update filter error code */
        _FilterData(filterError) = filterErrorCode;

        /* Update reported range */
        if(filterErrorCode==RangingFiltered)
            m_newTrueRange_mm = MaxOrInvalidDistance; /* Set to invalid distance */

        return m_newTrueRange_mm;
    }

    /* Computes current VAVGDiff */
    if (_FilterData(Default_VAVGVal) > _FilterData(NoDelay_VAVGVal))
        VAVGDiff = _FilterData(Default_VAVGVal) - _FilterData(NoDelay_VAVGVal);
    else
        VAVGDiff = 0;
    _FilterData(Previous_VAVGDiff) = VAVGDiff;

    if(SuspicuousRangingZone==0)
        MAX_VAVGDiff = MAX_VAVGDiff_ROM;
    else
        /* In suspicuous area, strengthen the filter */
        MAX_VAVGDiff = MAX_VAVGDiff_ROM / SuspicuousMAX_VAVGDiffRatio;

    /* Check the VAVGDiff */
    if (_FilterData(Default_ZeroVal) > _FilterData(NoDelay_ZeroVal))
        IdealVAVGDiff = _FilterData(Default_ZeroVal) - _FilterData(NoDelay_ZeroVal);
    else
        IdealVAVGDiff = _FilterData(NoDelay_ZeroVal) - _FilterData(Default_ZeroVal);
    if (IdealVAVGDiff > MAX_VAVGDiff)
        MinVAVGDiff = IdealVAVGDiff - MAX_VAVGDiff;
    else
        MinVAVGDiff = 0;
    MaxVAVGDiff = IdealVAVGDiff + MAX_VAVGDiff;
    if (VAVGDiff < MinVAVGDiff || VAVGDiff > MaxVAVGDiff) {
        WrapAroundFlag = 1;
        filterErrorCode = RangingFiltered;
    } else {
        /* Go through filtering check */

        if(_FilterData(MeasurementIndex)<=1)
            /* On measurement after a bypass, uses an increase number of samples */
            StdDevSamplesMinNeeded = MIN_FILTER_STDDEV_SAMPLES_AFTER_FLUSH_OR_BYPASS;
        else
            StdDevSamplesMinNeeded = MIN_FILTER_STDDEV_SAMPLES;

        /* StdDevLimit Damper on SNR */
        StdDevLimit = _filter_StdDevDamper(m_rtnAmbientRate, m_rtnSignalRate, StdDevLimitLowLight, StdDevLimitLowLightSNR, StdDevLimitHighLight, StdDevLimitHighLightSNR);

        /* Standard deviations computations */
        StdDevSamples = 0;
        StdDevDistanceSum = 0;
        StdDevDistanceMean = 0;
        StdDevDistance = 0;
        StdDevRateSum = 0;
        StdDevRateMean = 0;
        StdDevRate = 0;
        for (i = 0; (i < FILTER_NBOF_SAMPLES) && (StdDevSamples < FILTER_STDDEV_SAMPLES); i++) {
            if (_FilterData(LastTrueRange)[i] != FILTER_INVALID_DISTANCE) {
                StdDevSamples = (uint16_t) (StdDevSamples + 1);
                StdDevDistanceSum = (uint32_t) (StdDevDistanceSum + _FilterData(LastTrueRange)[i]);
                StdDevRateSum = (uint32_t) (StdDevRateSum + _FilterData(LastReturnRates)[i]);
            }
        }
        if (StdDevSamples > 0) {
            StdDevDistanceMean = (uint32_t) (StdDevDistanceSum / StdDevSamples);
            StdDevRateMean = (uint32_t) (StdDevRateSum / StdDevSamples);
        }
        /* TODO optimize shorten Std dev in aisngle loop computation using sum of x2 - (sum of x)2 */
        StdDevSamples = 0;
        StdDevDistanceSum = 0;
        StdDevRateSum = 0;
        for (i = 0; (i < FILTER_NBOF_SAMPLES) && (StdDevSamples < FILTER_STDDEV_SAMPLES); i++) {
            if (_FilterData(LastTrueRange)[i] != FILTER_INVALID_DISTANCE) {
                StdDevSamples = (uint16_t) (StdDevSamples + 1);
                StdDevDistanceSum = (uint32_t) (StdDevDistanceSum +
                                    (int)(_FilterData(LastTrueRange)[i] -
                                            StdDevDistanceMean) *
                                            (int) (_FilterData(LastTrueRange)[i] -
                                                    StdDevDistanceMean));
                StdDevRateSum = (uint32_t) (StdDevRateSum +
                                    (int) (_FilterData(LastReturnRates)[i] -
                                            StdDevRateMean) *
                                            (int) (_FilterData(LastReturnRates)[i] -
                                                    StdDevRateMean));
            }
        }
        if (StdDevSamples >= StdDevSamplesMinNeeded) {
            StdDevDistance = (uint16_t) (StdDevDistanceSum / StdDevSamples);
            StdDevRate = (uint16_t) (StdDevRateSum / StdDevSamples);
        } else {
            StdDevDistance = 0;
            StdDevRate = 0;
        }

        /* Check Return rate standard deviation */
        if (StdDevRate < StdDevMovingTargetStdDevLimit) {
            if (StdDevSamples < StdDevSamplesMinNeeded) {
                //m_newTrueRange_mm = MaxOrInvalidDistance;
                filterErrorCode = RangingFiltered;
            } else {
                /* Check distance standard deviation */
                if (StdDevRate < StdDevMovingTargetReturnRateLimit)
                    StdDevLimitWithTargetMove = StdDevLimit +
                        (((StdDevMovingTargetStdDevForReturnRateLimit -
                            StdDevLimit) * StdDevRate) /
                            StdDevMovingTargetReturnRateLimit);
                else
                    StdDevLimitWithTargetMove = StdDevMovingTargetStdDevForReturnRateLimit;

                if(_FilterData(filterError)==NoError){
                    /* No wrapAround detected yet, so relax constraints on the std dev */
                    StdDevLimitWithTargetMove = StdDevLimitWithTargetMove * StdDevNoWrapDetectedMultiplier;
                }

                if (((StdDevDistance * StdDevHighConfidenceSNRLimit) < StdDevLimit) && (StdDevSamples>=FILTER_STDDEV_SAMPLES)) {
                    NoWrapAroundHighConfidenceFlag = 1;
                } else {
                    if (StdDevDistance < StdDevLimitWithTargetMove) {
                            NoWrapAroundFlag = 1;
                        } else {
                        WrapAroundFlag = 1;
                        filterErrorCode = RangingFiltered;
                    }
                }
            }
        } else {
            /* Target moving too fast */
            WrapAroundFlag = 1;
            filterErrorCode = RangingFiltered;
        }
    }

    if (ValidDistance == 0) {
        /* In case of invalid distance */
        if (_FilterData(StdFilteredReads) > 0)
            _FilterData(StdFilteredReads) = (uint16_t) (_FilterData(StdFilteredReads) - 1);
    } else {
        if (WrapAroundFlag == 1) {
            _FilterData(StdFilteredReads) = (uint16_t) (_FilterData(StdFilteredReads) +
                                            StdFilteredReadsIncrement);
            if (_FilterData(StdFilteredReads) > StdMaxFilteredReads)
                _FilterData(StdFilteredReads) = StdMaxFilteredReads;
        } else {
            if (NoWrapAroundFlag == 1) {
                if (_FilterData(StdFilteredReads) > 0) {
                    filterErrorCode = RangingFiltered;
                    if (_FilterData(StdFilteredReads) > StdFilteredReadsDecrement)
                        _FilterData(StdFilteredReads) = (uint16_t) (_FilterData(StdFilteredReads) -
                                                        StdFilteredReadsDecrement);
                    else
                        _FilterData(StdFilteredReads) = 0;
                }
            } else {
                if (NoWrapAroundHighConfidenceFlag == 1) {
                    _FilterData(StdFilteredReads) = 0;
                }
            }
        }
    }

    /* If we detect a change from no Error to RangingFilteringOnGoing, then it means that
     * the filter detected a change in te scene, so discard all previous measurements.
     */
    if((_FilterData(filterError) == NoError) && (filterErrorCode!=NoError)) {
        for (i = 1; i < FILTER_NBOF_SAMPLES; i++) {
            _FilterData(LastTrueRange)[i] = FILTER_INVALID_DISTANCE;
            _FilterData(LastReturnRates)[i] = 0;
        }
    }

    /* Update filter error code */
    _FilterData(filterError) = filterErrorCode;

    /* Update reported range */
    if(filterErrorCode==RangingFiltered)
        m_newTrueRange_mm = MaxOrInvalidDistance; /* Set to invalid distance */

    return m_newTrueRange_mm;
done_err:
    return -1;

#undef MaxOrInvalidDistance
}


static int _filter_GetResult(&mut self, VL6180x_RangeData_t *pRangeData)
{
    uint32_t m_rawRange_mm = 0;
    int32_t  FilteredRange;
    const uint8_t scaler = _GetUpscale(dev);
    uint8_t u8;
    int status;

    do {
        status = VL6180x_GetCachedByte(dev, RESULT_RANGE_RAW, &u8);
        if (status) {
            VL6180x_ErrLog("RESULT_RANGE_RAW rd fail");
            break;
        }
        m_rawRange_mm = u8;

        FilteredRange = _filter_Start(dev, pRangeData->range_mm, (m_rawRange_mm * scaler), pRangeData->rtnRate, pRangeData->rtnAmbRate, pRangeData->errorStatus);
        if (FilteredRange < 0) {
            status = -1;
            break;
        }
        pRangeData->FilteredData.range_mm = FilteredRange;
        pRangeData->FilteredData.rawRange_mm = m_rawRange_mm * scaler;
        pRangeData->FilteredData.filterError= _FilterData(filterError);
    } while (0);
    return status;
}

#undef _FilterData
#ifdef PRESERVE_DEVICE_ERROR_CODE
#undef PRESERVE_DEVICE_ERROR_CODE
#endif
#ifdef SENSITIVE_FILTERING_ON_GOING
#undef SENSITIVE_FILTERING_ON_GOING
#endif
#undef FILTER_STDDEV_SAMPLES
#undef MIN_FILTER_STDDEV_SAMPLES
#undef MIN_FILTER_STDDEV_SAMPLES_AFTER_FLUSH_OR_BYPASS
#undef STDDEV_BASE_VALUE
#undef FILTER_INVALID_DISTANCE

