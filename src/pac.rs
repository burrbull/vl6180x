#[doc = "0 - Device model identification number. 0xB4 = VL6180X\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_model_id](identification_model_id) module"]
pub const IDENTIFICATION_MODEL_ID: identification_model_id::IDENTIFICATION_MODEL_ID =
    identification_model_id::IDENTIFICATION_MODEL_ID;
#[doc = "Device model identification number. 0xB4 = VL6180X"]
pub mod identification_model_id;
#[doc = "0x01 - Revision identifier of the Device for major change\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_model_rev_major](identification_model_rev_major) module"]
pub const IDENTIFICATION_MODEL_REV_MAJOR:
    identification_model_rev_major::IDENTIFICATION_MODEL_REV_MAJOR =
    identification_model_rev_major::IDENTIFICATION_MODEL_REV_MAJOR;
#[doc = "Revision identifier of the Device for major change"]
pub mod identification_model_rev_major;
#[doc = "0x02 - Revision identifier of the Device for minor change. IDENTIFICATION_MODEL_REV_MINOR = 3 for latest ROM revision\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_model_rev_minor](identification_model_rev_minor) module"]
pub const IDENTIFICATION_MODEL_REV_MINOR:
    identification_model_rev_minor::IDENTIFICATION_MODEL_REV_MINOR =
    identification_model_rev_minor::IDENTIFICATION_MODEL_REV_MINOR;
#[doc = "Revision identifier of the Device for minor change. IDENTIFICATION_MODEL_REV_MINOR = 3 for latest ROM revision"]
pub mod identification_model_rev_minor;
#[doc = "0x03 - Revision identifier of the Module Package for major change. Used to store NVM content version. Contact ST for current information\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_module_rev_major](identification_module_rev_major) module"]
pub const IDENTIFICATION_MODULE_REV_MAJOR:
    identification_module_rev_major::IDENTIFICATION_MODULE_REV_MAJOR =
    identification_module_rev_major::IDENTIFICATION_MODULE_REV_MAJOR;
#[doc = "Revision identifier of the Module Package for major change. Used to store NVM content version. Contact ST for current information"]
pub mod identification_module_rev_major;
#[doc = "0x04 - Revision identifier of the Module Package for minor change. Used to store NVM content version. Contact ST for current information\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_module_rev_minor](identification_module_rev_minor) module"]
pub const IDENTIFICATION_MODULE_REV_MINOR:
    identification_module_rev_minor::IDENTIFICATION_MODULE_REV_MINOR =
    identification_module_rev_minor::IDENTIFICATION_MODULE_REV_MINOR;
#[doc = "Revision identifier of the Module Package for minor change. Used to store NVM content version. Contact ST for current information"]
pub mod identification_module_rev_minor;
#[doc = "0x06 - Part of the register set that can be used to uniquely identify a module\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_date_hi](identification_date_hi) module"]
pub const IDENTIFICATION_DATE_HI: identification_date_hi::IDENTIFICATION_DATE_HI =
    identification_date_hi::IDENTIFICATION_DATE_HI;
#[doc = "Part of the register set that can be used to uniquely identify a module"]
pub mod identification_date_hi;
#[doc = "0x07 - Part of the register set that can be used to uniquely identify a module\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_date_lo](identification_date_lo) module"]
pub const IDENTIFICATION_DATE_LO: identification_date_lo::IDENTIFICATION_DATE_LO =
    identification_date_lo::IDENTIFICATION_DATE_LO;
#[doc = "Part of the register set that can be used to uniquely identify a module"]
pub mod identification_date_lo;
#[doc = "0x08 - Time since midnight (in seconds) = register_value * 2\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [identification_time](identification_time) module"]
pub const IDENTIFICATION_TIME: identification_time::IDENTIFICATION_TIME =
    identification_time::IDENTIFICATION_TIME;
#[doc = "Time since midnight (in seconds) = register_value * 2"]
pub mod identification_time;
#[doc = "0x10 - Configures polarity and select which function gpio 0 serves\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_mode_gpio0](system_mode_gpio0) module"]
pub const SYSTEM_MODE_GPIO0: system_mode_gpio0::SYSTEM_MODE_GPIO0 =
    system_mode_gpio0::SYSTEM_MODE_GPIO0;
#[doc = "Configures polarity and select which function gpio 0 serves"]
pub mod system_mode_gpio0;
#[doc = "0x11 - Configures polarity and select which function gpio 0 serves\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_mode_gpio1](system_mode_gpio1) module"]
pub const SYSTEM_MODE_GPIO1: system_mode_gpio1::SYSTEM_MODE_GPIO1 =
    system_mode_gpio1::SYSTEM_MODE_GPIO1;
#[doc = "Configures polarity and select which function gpio 0 serves"]
pub mod system_mode_gpio1;
#[doc = "0x12 - SYSTEM_HISTORY_CTRL\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_history_ctrl](system_history_ctrl) module"]
pub const SYSTEM_HISTORY_CTRL: system_history_ctrl::SYSTEM_HISTORY_CTRL =
    system_history_ctrl::SYSTEM_HISTORY_CTRL;
#[doc = "SYSTEM_HISTORY_CTRL"]
pub mod system_history_ctrl;
#[doc = "0x14 - SYSTEM_INTERRUPT_CONFIG_GPIO\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_interrupt_config_gpio](system_interrupt_config_gpio) module"]
pub const SYSTEM_INTERRUPT_CONFIG_GPIO: system_interrupt_config_gpio::SYSTEM_INTERRUPT_CONFIG_GPIO =
    system_interrupt_config_gpio::SYSTEM_INTERRUPT_CONFIG_GPIO;
#[doc = "SYSTEM_INTERRUPT_CONFIG_GPIO"]
pub mod system_interrupt_config_gpio;
#[doc = "0x15 - SYSTEM_INTERRUPT_CLEAR\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_interrupt_clear](system_interrupt_clear) module"]
pub const SYSTEM_INTERRUPT_CLEAR: system_interrupt_clear::SYSTEM_INTERRUPT_CLEAR =
    system_interrupt_clear::SYSTEM_INTERRUPT_CLEAR;
#[doc = "SYSTEM_INTERRUPT_CLEAR"]
pub mod system_interrupt_clear;
#[doc = "0x16 - Fresh out of reset bit, default of 1, user can set this to 0 after initial boot and can therefore use this to check for a reset condition\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_fresh_out_of_reset](system_fresh_out_of_reset) module"]
pub const SYSTEM_FRESH_OUT_OF_RESET: system_fresh_out_of_reset::SYSTEM_FRESH_OUT_OF_RESET =
    system_fresh_out_of_reset::SYSTEM_FRESH_OUT_OF_RESET;
#[doc = "Fresh out of reset bit, default of 1, user can set this to 0 after initial boot and can therefore use this to check for a reset condition"]
pub mod system_fresh_out_of_reset;
#[doc = "0x17 - Flag set over I2C to indicate that data is being updated\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_grouped_parameter_hold](system_grouped_parameter_hold) module"]
pub const SYSTEM_GROUPED_PARAMETER_HOLD:
    system_grouped_parameter_hold::SYSTEM_GROUPED_PARAMETER_HOLD =
    system_grouped_parameter_hold::SYSTEM_GROUPED_PARAMETER_HOLD;
#[doc = "Flag set over I2C to indicate that data is being updated"]
pub mod system_grouped_parameter_hold;
#[doc = "0x18 - SYSRANGE_START\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_start](sysrange_start) module"]
pub const SYSRANGE_START: sysrange_start::SYSRANGE_START = sysrange_start::SYSRANGE_START;
#[doc = "SYSRANGE_START"]
pub mod sysrange_start;
#[doc = "0x19 - High Threshold value for ranging comparison. Range 0-255mm\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_thresh_high](sysrange_thresh_high) module"]
pub const SYSRANGE_THRESH_HIGH: sysrange_thresh_high::SYSRANGE_THRESH_HIGH =
    sysrange_thresh_high::SYSRANGE_THRESH_HIGH;
#[doc = "High Threshold value for ranging comparison. Range 0-255mm"]
pub mod sysrange_thresh_high;
#[doc = "0x1a - Low Threshold value for ranging comparison. Range 0-255mm\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_thresh_low](sysrange_thresh_low) module"]
pub const SYSRANGE_THRESH_LOW: sysrange_thresh_low::SYSRANGE_THRESH_LOW =
    sysrange_thresh_low::SYSRANGE_THRESH_LOW;
#[doc = "Low Threshold value for ranging comparison. Range 0-255mm"]
pub mod sysrange_thresh_low;
#[doc = "0x1b - Time delay between measurements in Ranging continuous mode. Range 0-254 (0 = 10ms). Step size = 10ms\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_intermeasurement_period](sysrange_intermeasurement_period) module"]
pub const SYSRANGE_INTERMEASUREMENT_PERIOD:
    sysrange_intermeasurement_period::SYSRANGE_INTERMEASUREMENT_PERIOD =
    sysrange_intermeasurement_period::SYSRANGE_INTERMEASUREMENT_PERIOD;
#[doc = "Time delay between measurements in Ranging continuous mode. Range 0-254 (0 = 10ms). Step size = 10ms"]
pub mod sysrange_intermeasurement_period;
#[doc = "0x1c - Maximum time to run measurement in Ranging modes. 1 code = 1 ms. Measurement aborted when limit reached to aid power reduction\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_max_convergence_time](sysrange_max_convergence_time) module"]
pub const SYSRANGE_MAX_CONVERGENCE_TIME:
    sysrange_max_convergence_time::SYSRANGE_MAX_CONVERGENCE_TIME =
    sysrange_max_convergence_time::SYSRANGE_MAX_CONVERGENCE_TIME;
#[doc = "Maximum time to run measurement in Ranging modes. 1 code = 1 ms. Measurement aborted when limit reached to aid power reduction"]
pub mod sysrange_max_convergence_time;
#[doc = "0x1e - User-controlled crosstalk compensation in Mcps (9.7 format)\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_crosstalk_compensation_rate](sysrange_crosstalk_compensation_rate) module"]
pub const SYSRANGE_CROSSTALK_COMPENSATION_RATE:
    sysrange_crosstalk_compensation_rate::SYSRANGE_CROSSTALK_COMPENSATION_RATE =
    sysrange_crosstalk_compensation_rate::SYSRANGE_CROSSTALK_COMPENSATION_RATE;
#[doc = "User-controlled crosstalk compensation in Mcps (9.7 format)"]
pub mod sysrange_crosstalk_compensation_rate;
#[doc = "0x21 - Minimum range value in mm to qualify for cross-talk compensation\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_crosstalk_valid_height](sysrange_crosstalk_valid_height) module"]
pub const SYSRANGE_CROSSTALK_VALID_HEIGHT:
    sysrange_crosstalk_valid_height::SYSRANGE_CROSSTALK_VALID_HEIGHT =
    sysrange_crosstalk_valid_height::SYSRANGE_CROSSTALK_VALID_HEIGHT;
#[doc = "Minimum range value in mm to qualify for cross-talk compensation"]
pub mod sysrange_crosstalk_valid_height;
#[doc = "0x22 - FW carries out an estimate of convergence rate 0.5ms into each new range measurement. If convergence rate is below user input value, the operation aborts to save power\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_early_convergence_estimate](sysrange_early_convergence_estimate) module"]
pub const SYSRANGE_EARLY_CONVERGENCE_ESTIMATE:
    sysrange_early_convergence_estimate::SYSRANGE_EARLY_CONVERGENCE_ESTIMATE =
    sysrange_early_convergence_estimate::SYSRANGE_EARLY_CONVERGENCE_ESTIMATE;
#[doc = "FW carries out an estimate of convergence rate 0.5ms into each new range measurement. If convergence rate is below user input value, the operation aborts to save power"]
pub mod sysrange_early_convergence_estimate;
#[doc = "0x24 - 2s complement format\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_part_to_part_range_offset](sysrange_part_to_part_range_offset) module"]
pub const SYSRANGE_PART_TO_PART_RANGE_OFFSET:
    sysrange_part_to_part_range_offset::SYSRANGE_PART_TO_PART_RANGE_OFFSET =
    sysrange_part_to_part_range_offset::SYSRANGE_PART_TO_PART_RANGE_OFFSET;
#[doc = "2s complement format"]
pub mod sysrange_part_to_part_range_offset;
#[doc = "0x25 - Range below which ignore threshold is applied. Aim is to ignore the cover glass i.e. low signal rate at near distance. Should not be applied to low reflectance target at far distance. Range in mm\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_range_ignore_valid_height](sysrange_range_ignore_valid_height) module"]
pub const SYSRANGE_RANGE_IGNORE_VALID_HEIGHT:
    sysrange_range_ignore_valid_height::SYSRANGE_RANGE_IGNORE_VALID_HEIGHT =
    sysrange_range_ignore_valid_height::SYSRANGE_RANGE_IGNORE_VALID_HEIGHT;
#[doc = "Range below which ignore threshold is applied. Aim is to ignore the cover glass i.e. low signal rate at near distance. Should not be applied to low reflectance target at far distance. Range in mm"]
pub mod sysrange_range_ignore_valid_height;
#[doc = "0x26 - User configurable min threshold signal return rate. Used to filter out ranging due to cover glass when there is no target above the device. Mcps 9.7 format\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_range_ignore_threshold](sysrange_range_ignore_threshold) module"]
pub const SYSRANGE_RANGE_IGNORE_THRESHOLD:
    sysrange_range_ignore_threshold::SYSRANGE_RANGE_IGNORE_THRESHOLD =
    sysrange_range_ignore_threshold::SYSRANGE_RANGE_IGNORE_THRESHOLD;
#[doc = "User configurable min threshold signal return rate. Used to filter out ranging due to cover glass when there is no target above the device. Mcps 9.7 format"]
pub mod sysrange_range_ignore_threshold;
#[doc = "0x2c - User input value to multiply return_signal_count for AMB:signal ratio check. If (amb counts * 6) > return_signal_count * mult then abandon measurement due to high ambient (4.4 format)\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_max_ambient_level_mult](sysrange_max_ambient_level_mult) module"]
pub const SYSRANGE_MAX_AMBIENT_LEVEL_MULT:
    sysrange_max_ambient_level_mult::SYSRANGE_MAX_AMBIENT_LEVEL_MULT =
    sysrange_max_ambient_level_mult::SYSRANGE_MAX_AMBIENT_LEVEL_MULT;
#[doc = "User input value to multiply return_signal_count for AMB:signal ratio check. If (amb counts * 6) > return_signal_count * mult then abandon measurement due to high ambient (4.4 format)"]
pub mod sysrange_max_ambient_level_mult;
#[doc = "0x2d - SYSRANGE_RANGE_CHECK_ENABLES\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_range_check_enables](sysrange_range_check_enables) module"]
pub const SYSRANGE_RANGE_CHECK_ENABLES: sysrange_range_check_enables::SYSRANGE_RANGE_CHECK_ENABLES =
    sysrange_range_check_enables::SYSRANGE_RANGE_CHECK_ENABLES;
#[doc = "SYSRANGE_RANGE_CHECK_ENABLES"]
pub mod sysrange_range_check_enables;
#[doc = "0x2e - SYSRANGE_VHV_RECALIBRATE\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_vhv_recalibrate](sysrange_vhv_recalibrate) module"]
pub const SYSRANGE_VHV_RECALIBRATE: sysrange_vhv_recalibrate::SYSRANGE_VHV_RECALIBRATE =
    sysrange_vhv_recalibrate::SYSRANGE_VHV_RECALIBRATE;
#[doc = "SYSRANGE_VHV_RECALIBRATE"]
pub mod sysrange_vhv_recalibrate;
#[doc = "0x31 - User entered repeat rate of auto VHV task (0 = off, 255 = after every 255 measurements)\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrange_vhv_repeate_rate](sysrange_vhv_repeate_rate) module"]
pub const SYSRANGE_VHV_REPEATE_RATE: sysrange_vhv_repeate_rate::SYSRANGE_VHV_REPEATE_RATE =
    sysrange_vhv_repeate_rate::SYSRANGE_VHV_REPEATE_RATE;
#[doc = "User entered repeat rate of auto VHV task (0 = off, 255 = after every 255 measurements)"]
pub mod sysrange_vhv_repeate_rate;
#[doc = "0x38 - SYSALS_START\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysals_start](sysals_start) module"]
pub const SYSALS_START: sysals_start::SYSALS_START = sysals_start::SYSALS_START;
#[doc = "SYSALS_START"]
pub mod sysals_start;
#[doc = "0x3a - High Threshold value for ALS comparison. Range 0-65535 codes\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysals_thresh_high](sysals_thresh_high) module"]
pub const SYSALS_THRESH_HIGH: sysals_thresh_high::SYSALS_THRESH_HIGH =
    sysals_thresh_high::SYSALS_THRESH_HIGH;
#[doc = "High Threshold value for ALS comparison. Range 0-65535 codes"]
pub mod sysals_thresh_high;
#[doc = "0x3c - Low Threshold value for ALS comparison. Range 0-65535 codes\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysals_thresh_low](sysals_thresh_low) module"]
pub const SYSALS_THRESH_LOW: sysals_thresh_low::SYSALS_THRESH_LOW =
    sysals_thresh_low::SYSALS_THRESH_LOW;
#[doc = "Low Threshold value for ALS comparison. Range 0-65535 codes"]
pub mod sysals_thresh_low;
#[doc = "0x3e - Time delay between measurements in ALS continuous mode. Range 0-254 (0 = 10ms). Step size = 10ms\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysals_intermeasurement_period](sysals_intermeasurement_period) module"]
pub const SYSALS_INTERMEASUREMENT_PERIOD:
    sysals_intermeasurement_period::SYSALS_INTERMEASUREMENT_PERIOD =
    sysals_intermeasurement_period::SYSALS_INTERMEASUREMENT_PERIOD;
#[doc = "Time delay between measurements in ALS continuous mode. Range 0-254 (0 = 10ms). Step size = 10ms"]
pub mod sysals_intermeasurement_period;
#[doc = "0x3f - SYSALS_ANALOGUE_GAIN\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysals_analogue_gain](sysals_analogue_gain) module"]
pub const SYSALS_ANALOGUE_GAIN: sysals_analogue_gain::SYSALS_ANALOGUE_GAIN =
    sysals_analogue_gain::SYSALS_ANALOGUE_GAIN;
#[doc = "SYSALS_ANALOGUE_GAIN"]
pub mod sysals_analogue_gain;
#[doc = "0x40 - Integration period for ALS mode. 1 code = 1 ms (0 = 1 ms). Recommended setting is 100 ms (0x63)\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysals_integration_period](sysals_integration_period) module"]
pub const SYSALS_INTEGRATION_PERIOD: sysals_integration_period::SYSALS_INTEGRATION_PERIOD =
    sysals_integration_period::SYSALS_INTEGRATION_PERIOD;
#[doc = "Integration period for ALS mode. 1 code = 1 ms (0 = 1 ms). Recommended setting is 100 ms (0x63)"]
pub mod sysals_integration_period;
#[doc = "0x4d - RESULT_RANGE_STATUS\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_status](result_range_status) module"]
pub const RESULT_RANGE_STATUS: result_range_status::RESULT_RANGE_STATUS =
    result_range_status::RESULT_RANGE_STATUS;
#[doc = "RESULT_RANGE_STATUS"]
pub mod result_range_status;
#[doc = "0x4e - RESULT_ALS_STATUS\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_als_status](result_als_status) module"]
pub const RESULT_ALS_STATUS: result_als_status::RESULT_ALS_STATUS =
    result_als_status::RESULT_ALS_STATUS;
#[doc = "RESULT_ALS_STATUS"]
pub mod result_als_status;
#[doc = "0x4f - RESULT_INTERRUPT_STATUS_GPIO\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_interrupt_status_gpio](result_interrupt_status_gpio) module"]
pub const RESULT_INTERRUPT_STATUS_GPIO: result_interrupt_status_gpio::RESULT_INTERRUPT_STATUS_GPIO =
    result_interrupt_status_gpio::RESULT_INTERRUPT_STATUS_GPIO;
#[doc = "RESULT_INTERRUPT_STATUS_GPIO"]
pub mod result_interrupt_status_gpio;
#[doc = "0x50 - 16 Bit ALS count output value. Lux value depends on Gain and integration settings and calibrated lux/count setting\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_als_val](result_als_val) module"]
pub const RESULT_ALS_VAL: result_als_val::RESULT_ALS_VAL = result_als_val::RESULT_ALS_VAL;
#[doc = "16 Bit ALS count output value. Lux value depends on Gain and integration settings and calibrated lux/count setting"]
pub mod result_als_val;
#[doc = "0x52 - Range/ALS result value\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_history_buffer](result_history_buffer) module"]
pub const RESULT_HISTORY_BUFFER: result_history_buffer::RESULT_HISTORY_BUFFER =
    result_history_buffer::RESULT_HISTORY_BUFFER;
#[doc = "Range/ALS result value"]
pub mod result_history_buffer;
#[doc = "0x62 - Final range result value presented to the user for use. Unit is in mm\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_val](result_range_val) module"]
pub const RESULT_RANGE_VAL: result_range_val::RESULT_RANGE_VAL = result_range_val::RESULT_RANGE_VAL;
#[doc = "Final range result value presented to the user for use. Unit is in mm"]
pub mod result_range_val;
#[doc = "0x64 - Raw Range result value with offset applied (no cross-talk compensation applied). Unit is in mm\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_raw](result_range_raw) module"]
pub const RESULT_RANGE_RAW: result_range_raw::RESULT_RANGE_RAW = result_range_raw::RESULT_RANGE_RAW;
#[doc = "Raw Range result value with offset applied (no cross-talk compensation applied). Unit is in mm"]
pub mod result_range_raw;
#[doc = "0x66 - Sensor count rate of signal returns correlated to IR emitter. Computed from RETURN_SIGNAL_COUNT / RETURN_CONV_TIME. Mcps 9.7 format\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_return_rate](result_range_return_rate) module"]
pub const RESULT_RANGE_RETURN_RATE: result_range_return_rate::RESULT_RANGE_RETURN_RATE =
    result_range_return_rate::RESULT_RANGE_RETURN_RATE;
#[doc = "Sensor count rate of signal returns correlated to IR emitter. Computed from RETURN_SIGNAL_COUNT / RETURN_CONV_TIME. Mcps 9.7 format"]
pub mod result_range_return_rate;
#[doc = "0x68 - Sensor count rate of reference signal returns. Computed from REFERENCE_SIGNAL_COUNT / RETURN_CONV_TIME. Mcps 9.7 format\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_reference_rate](result_range_reference_rate) module"]
pub const RESULT_RANGE_REFERENCE_RATE: result_range_reference_rate::RESULT_RANGE_REFERENCE_RATE =
    result_range_reference_rate::RESULT_RANGE_REFERENCE_RATE;
#[doc = "Sensor count rate of reference signal returns. Computed from REFERENCE_SIGNAL_COUNT / RETURN_CONV_TIME. Mcps 9.7 format"]
pub mod result_range_reference_rate;
#[doc = "0x6c - Sensor count output value attributed to signal correlated to IR emitter on the Return array\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_return_signal_count](result_range_return_signal_count) module"]
pub const RESULT_RANGE_RETURN_SIGNAL_COUNT:
    result_range_return_signal_count::RESULT_RANGE_RETURN_SIGNAL_COUNT =
    result_range_return_signal_count::RESULT_RANGE_RETURN_SIGNAL_COUNT;
#[doc = "Sensor count output value attributed to signal correlated to IR emitter on the Return array"]
pub mod result_range_return_signal_count;
#[doc = "0x70 - Sensor count output value attributed to signal correlated to IR emitter on the Reference array\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_reference_signal_count](result_range_reference_signal_count) module"]
pub const RESULT_RANGE_REFERENCE_SIGNAL_COUNT:
    result_range_reference_signal_count::RESULT_RANGE_REFERENCE_SIGNAL_COUNT =
    result_range_reference_signal_count::RESULT_RANGE_REFERENCE_SIGNAL_COUNT;
#[doc = "Sensor count output value attributed to signal correlated to IR emitter on the Reference array"]
pub mod result_range_reference_signal_count;
#[doc = "0x74 - Sensor count output value attributed to uncorrelated ambient signal on the Return array. Must be multiplied by 6 if used to calculate the ambient to signal threshold\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_return_amb_count](result_range_return_amb_count) module"]
pub const RESULT_RANGE_RETURN_AMB_COUNT:
    result_range_return_amb_count::RESULT_RANGE_RETURN_AMB_COUNT =
    result_range_return_amb_count::RESULT_RANGE_RETURN_AMB_COUNT;
#[doc = "Sensor count output value attributed to uncorrelated ambient signal on the Return array. Must be multiplied by 6 if used to calculate the ambient to signal threshold"]
pub mod result_range_return_amb_count;
#[doc = "0x78 - Sensor count output value attributed to uncorrelated ambient signal on the Reference array\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_reference_amb_count](result_range_reference_amb_count) module"]
pub const RESULT_RANGE_REFERENCE_AMB_COUNT:
    result_range_reference_amb_count::RESULT_RANGE_REFERENCE_AMB_COUNT =
    result_range_reference_amb_count::RESULT_RANGE_REFERENCE_AMB_COUNT;
#[doc = "Sensor count output value attributed to uncorrelated ambient signal on the Reference array"]
pub mod result_range_reference_amb_count;
#[doc = "0x7c - Sensor count output value attributed to signal on the Return array\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_return_conv_time](result_range_return_conv_time) module"]
pub const RESULT_RANGE_RETURN_CONV_TIME:
    result_range_return_conv_time::RESULT_RANGE_RETURN_CONV_TIME =
    result_range_return_conv_time::RESULT_RANGE_RETURN_CONV_TIME;
#[doc = "Sensor count output value attributed to signal on the Return array"]
pub mod result_range_return_conv_time;
#[doc = "0x80 - Sensor count output value attributed to signal on the Reference array\n\nThis register you can [`read`](crate::generic::I2cDevice::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result_range_reference_conv_time](result_range_reference_conv_time) module"]
pub const RESULT_RANGE_REFERENCE_CONV_TIME:
    result_range_reference_conv_time::RESULT_RANGE_REFERENCE_CONV_TIME =
    result_range_reference_conv_time::RESULT_RANGE_REFERENCE_CONV_TIME;
#[doc = "Sensor count output value attributed to signal on the Reference array"]
pub mod result_range_reference_conv_time;
#[doc = "0x010a - The internal readout averaging sample period can be adjusted from 0 to 255. Increasing the sampling period decreases noise but also reduces the effective max convergence time and increases power consumption\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [readout_averaging_sample_period](readout_averaging_sample_period) module"]
pub const READOUT_AVERAGING_SAMPLE_PERIOD:
    readout_averaging_sample_period::READOUT_AVERAGING_SAMPLE_PERIOD =
    readout_averaging_sample_period::READOUT_AVERAGING_SAMPLE_PERIOD;
#[doc = "The internal readout averaging sample period can be adjusted from 0 to 255. Increasing the sampling period decreases noise but also reduces the effective max convergence time and increases power consumption"]
pub mod readout_averaging_sample_period;
#[doc = "0x0119 - FW must set bit once initial boot has been completed\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [firmware_bootup](firmware_bootup) module"]
pub const FIRMWARE_BOOTUP: firmware_bootup::FIRMWARE_BOOTUP = firmware_bootup::FIRMWARE_BOOTUP;
#[doc = "FW must set bit once initial boot has been completed"]
pub mod firmware_bootup;
#[doc = "0x0120 - Analogue gain 1 to 16x\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [firmware_result_scaler](firmware_result_scaler) module"]
pub const FIRMWARE_RESULT_SCALER: firmware_result_scaler::FIRMWARE_RESULT_SCALER =
    firmware_result_scaler::FIRMWARE_RESULT_SCALER;
#[doc = "Analogue gain 1 to 16x"]
pub mod firmware_result_scaler;
#[doc = "0x0212 - User programmable I 2 C address (7-bit). Device address can be re-designated after power-up\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_slave_device_address](i2c_slave_device_address) module"]
pub const I2C_SLAVE_DEVICE_ADDRESS: i2c_slave_device_address::I2C_SLAVE_DEVICE_ADDRESS =
    i2c_slave_device_address::I2C_SLAVE_DEVICE_ADDRESS;
#[doc = "User programmable I 2 C address (7-bit). Device address can be re-designated after power-up"]
pub mod i2c_slave_device_address;
#[doc = "0x02a3 - Write 0x1 to this register to select ALS+Range interleaved mode. Use SYSALS_START and SYSALS_INTERMEASUREMENT_PERIOD to control this mode. A range measurement is automatically performed immediately after each ALS measurement\n\nThis register you can [`read`](crate::generic::I2cDevice::read), [`reset`](crate::generic::I2cDevice::reset), [`write`](crate::generic::I2cDevice::write), [`write_with_zero`](crate::generic::I2cDevice::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interleaved_mode_enable](interleaved_mode_enable) module"]
pub const INTERLEAVED_MODE_ENABLE: interleaved_mode_enable::INTERLEAVED_MODE_ENABLE =
    interleaved_mode_enable::INTERLEAVED_MODE_ENABLE;
#[doc = "Write 0x1 to this register to select ALS+Range interleaved mode. Use SYSALS_START and SYSALS_INTERMEASUREMENT_PERIOD to control this mode. A range measurement is automatically performed immediately after each ALS measurement"]
pub mod interleaved_mode_enable;
