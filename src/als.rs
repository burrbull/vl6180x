
pub fn als_get_lux(&mut self, i2c) -> Result<Lux, I2cError>
{
    int status;
    uint16_t RawAls;
    uint32_t luxValue = 0;
    uint32_t IntPeriod;
    uint32_t AlsAnGain;
    uint32_t GainFix;
    uint32_t AlsScaler;

    #if LUXRES_FIX_PREC !=  GAIN_FIX_PREC
    #error "LUXRES_FIX_PREC != GAIN_FIX_PREC  review these code to be correct"
    #endif
    const LUX_RESX_INT_IME = (0.56 * (DEF_INT_PEFRIOD as f32) * ((1 << LUXRES_FIX_PREC) as f32)) as u32;

    let raw_als: u16 = self.read(i2c, pac::RESULT_ALS_VAL)?.bits();
    /* wer are yet here at no fix point */
    IntPeriod = VL6180xDevDataGet(dev, IntegrationPeriod);
    AlsScaler = VL6180xDevDataGet(dev, AlsScaler);
    IntPeriod+=1; /* what stored is real time  ms -1 and it can be 0 for or 0 or 1ms */
    luxValue = (raw_als as u32) * LUX_RESX_INT_IME; /* max # 16+8bits + 6bit (0.56*100)  */
    luxValue /= IntPeriod;                         /* max # 16+8bits + 6bit 16+8+1 to 9 bit */
    /* between  29 - 21 bit */
    AlsAnGain = VL6180xDevDataGet(dev, AlsGainCode);
    GainFix = AlsGainLookUp[AlsAnGain];
    Ok(luxValue / (AlsScaler * GainFix))
}

pub als_get_measurement(&mut self, i2c) -> Result<Lux, AlsErr> {
    let lux = self.als_get_lux(i2c)?.bits() as Lux;
    let err_status = self.read(i2c, pac::RESULT_ALS_STATUS)?.error_code().variant()
    match err_status {
        pac::result_als_status::ERROR_CODE_A::NO => { Ok(lux) },
        _ => err_status.into()
    }
}


int VL6180x_AlsPollMeasurement(&mut self, VL6180x_AlsData_t *pAlsData)
{
    int status;
    int ClrStatus;
    uint8_t IntStatus;

    LOG_FUNCTION_START("%p", pAlsData);
    #if VL6180X_SAFE_POLLING_ENTER
    /* if device get stopped with left interrupt uncleared , it is required to clear them now or poll for new condition will never occur*/
    status = VL6180x_AlsClearInterrupt(dev);
    if (status) {
        VL6180x_ErrLog("VL6180x_AlsClearInterrupt fail");
        goto over;
    }
    #endif

    status = VL6180x_AlsSetSystemMode(dev, MODE_START_STOP | MODE_SINGLESHOT);
    if (status) {
        VL6180x_ErrLog("VL6180x_AlsSetSystemMode fail");
        goto over;
    }

    /* poll for new sample ready */
    while (1) {
        status = VL6180x_AlsGetInterruptStatus(dev, &IntStatus);
        if (status) {
            break;
        }
        if (IntStatus == RES_INT_STAT_GPIO_NEW_SAMPLE_READY) {
            break; /* break on new data (status is 0)  */
        }

        VL6180x_PollDelay(dev);
    };

    if (!status) {
        status = VL6180x_AlsGetMeasurement(dev, pAlsData);
    }

    ClrStatus = VL6180x_AlsClearInterrupt(dev);
    if (ClrStatus) {
        VL6180x_ErrLog("VL6180x_AlsClearInterrupt fail");
        if (!status) {
            status = ClrStatus; /* leave previous if already on error */
        }
    }
over:
    LOG_FUNCTION_END(status);

    return status;
}

int VL6180x_AlsGetInterruptStatus(&mut self, uint8_t *pIntStatus)
{
    int status;
    uint8_t IntStatus;
    LOG_FUNCTION_START("%p", pIntStatus);

    status = self.read(RESULT_INTERRUPT_STATUS_GPIO, &IntStatus).bits();
    *pIntStatus = (IntStatus >> 3) & 0x07;

    LOG_FUNCTION_END_FMT(status, "%d", (int)*pIntStatus);
    return status;
}

int VL6180x_AlsWaitDeviceReady(&mut self, int MaxLoop)
{
    int status;
    int  n;
    uint8_t u8;
    LOG_FUNCTION_START("%d", (int)MaxLoop);
    if (MaxLoop < 1) {
        status = INVALID_PARAMS;
    } else {
        for (n = 0; n < MaxLoop; n++) {
            status = self.read(RESULT_ALS_STATUS, &u8).bits();
            if (status)
                break;
            u8 = u8 & ALS_DEVICE_READY_MASK;
            if (u8)
                break;

        }
        if (!status && !u8) {
            status = TIME_OUT;
        }
    }
    LOG_FUNCTION_END(status);
    return status;
}

int VL6180x_AlsSetSystemMode(&mut self, uint8_t mode)
{
    int status;
    LOG_FUNCTION_START("%d", (int)mode);
    /* FIXME if we are called back to back real fast we are not checking
     * if previous mode "set" got absorbed => bit 0 must be 0 so that wr 1 work */
    if (mode <= 3) {
        status = self.write_byte(SYSALS_START, mode);
    } else {
        status = INVALID_PARAMS;
    }
    LOG_FUNCTION_END(status);
    return status;
}

    pub fn als_config_interrupt(&mut self, i2c: &mut I2C, config_gpio_int: pac::system_interrupt_config_gpio::RANGE_INT_MODE_A) -> Result<(), I2cError> {
        self.dev.modify(i2c, pac::SYSTEM_INTERRUPT_CONFIG_GPIO,
            |_,w| w.als_int_mode().variant(config_gpio_int)
        )
    }

int VL6180x_AlsSetThresholds(&mut self, uint8_t low, uint8_t high)
{
    int status;

    LOG_FUNCTION_START("%d %d", (int)low, (int)high);

    status = self.write_byte(SYSALS_THRESH_LOW, low);
    if (!status) {
        status = self.write_byte(SYSALS_THRESH_HIGH, high);
    }

    LOG_FUNCTION_END(status) ;
    return status;
}


int VL6180x_AlsSetAnalogueGain(&mut self, uint8_t gain)
{
    int status;
    uint8_t GainTotal;

    LOG_FUNCTION_START("%d", (int)gain);
    gain &= ~0x40;
    if (gain > 7) {
        gain = 7;
    }
    GainTotal = gain | 0x40;

    status = self.write_byte(SYSALS_ANALOGUE_GAIN, GainTotal);
    if (!status) {
        VL6180xDevDataSet(dev, AlsGainCode, gain);
    }

    LOG_FUNCTION_END_FMT(status, "%d %d", (int)gain, (int)GainTotal);
    return status;
}

int VL6180x_AlsSetInterMeasurementPeriod(&mut self,  uint16_t intermeasurement_period_ms)
{
    int status;

    LOG_FUNCTION_START("%d", (int)intermeasurement_period_ms);
    /* clipping: range is 0-2550ms */
    if (intermeasurement_period_ms >= 255 * 10)
        intermeasurement_period_ms = 255 * 10;
    status = self.write_byte(SYSALS_INTERMEASUREMENT_PERIOD, (uint8_t)(intermeasurement_period_ms / 10));

    LOG_FUNCTION_END_FMT(status, "%d", (int)intermeasurement_period_ms);
    return status;
}


int VL6180x_AlsSetIntegrationPeriod(&mut self, uint16_t period_ms)
{
    let integration_period: u16;

    if period_ms >= 1 {
        integration_period = period_ms - 1;
    } else {
        integration_period = period_ms;
    }

    if integration_period > 464 {
        integration_period = 464;
    } else if integration_period == 255 {
        integration_period += 1; /* can't write 255 since this causes the device to lock out.*/
    }

    status = VL6180x_WrWord(dev, SYSALS_INTEGRATION_PERIOD, integration_period);
    self.integration_period = integration_period;
}
