#![allow(non_camel_case_types)]
#![no_std]

use embedded_hal::blocking::i2c::{WriteIter, WriteRead};

pub const ID: u8 = 0x29;

mod generic;
pub use generic::*;

pub mod pac;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrCode {
    //ApiNoError,
    /// warning invalid calibration data may be in used \a  VL6180x_InitData() \a VL6180x_GetOffsetCalibrationData \a VL6180x_SetOffsetCalibrationData
    CalibrationWarning,
    /// warning parameter passed was clipped to min before to be applied
    MinCliped,
    /// Correct operation is not guaranteed typically using extended ranging on vl6180x
    NotGuaranteed,

    /// Unqualified error
    ApiError,
    /// parameter passed is invalid or out of range
    //	InvalidParams = -2,
    /// function is not supported in current mode or configuration
    NotSupported,
    /// device report a ranging error interrupt status
    RangeError,
    /// aborted due to time out
    TimeOut,
    I2c(I2cError),
}

impl From<I2cError> for ErrCode {
    fn from(e: I2cError) -> Self {
        match e {
            _ => ErrCode::I2c(e),
        }
    }
}

pub enum Pin {
    P0,
    P1,
}

/// Filtered result data structure  range data is to be used
struct RangeFilterResult {
    /// Filtered ranging value
    range_mm: u16,
    /// raw range value (scaled)
    raw_range_mm: u16,
    /// current filter error code
    filter_error: u32,
}

/// "small" unsigned data type used in filter
///
/// if data space saving is not a concern it can be change to platform native unsigned int
type FilterType1 = u32;

/// @def FILTER_NBOF_SAMPLES
/// sample history len used for wrap around filtering
const FILTER_NBOF_SAMPLES: usize = 10;

/// Wrap around filter internal data
struct FilterData {
    /// current measurement index
    pub measurement_index: u32,
    /// Number of measurements done since last time buffer has been flushed
    pub measurements_since_last_flush: u32,
    /// filtered/corrected  distance history
    pub last_true_range: [u16; FILTER_NBOF_SAMPLES],
    /// Return rate history
    pub last_return_rates: [u32; FILTER_NBOF_SAMPLES],

    std_filtered_reads: u16,
    default_zero_val: FilterType1,
    default_vavg_val: FilterType1,
    no_delay_zero_val: FilterType1,
    no_delay_vavg_val: FilterType1,
    previous_vavg_diff: FilterType1,
    filtering_on_going_consecutive_states: u32,
    /// current filter error code
    pub filter_error: u32,
}

//#[cfg(feature = "have_dmax_ranging")]
type DMaxFix = i32;
#[cfg(feature = "have_dmax_ranging")]
struct DMaxData {
    /// internal algo tuning (*1000)
    amb_tuning_window_factor_k: u32,
    /// intermediate dmax computation value caching @a #SYSRANGE_CROSSTALK_COMPENSATION_RATE and private reg 0x02A
    ret_signal_at_400mm: DMaxFix,
    // int32_t RegB8;              /*!< register 0xB8 cached to speed reduce i2c traffic for dmax computation */
    // place all word data below to optimize struct packing */
    // int32_t minSignalNeeded;     /*!< optimized computation intermediate base on register cached value */
    /// cached and optimized computation intermediate from  @a #SYSRANGE_MAX_AMBIENT_LEVEL_MULT
    snr_limit_k: i32,
    /// Max value for snr limit
    clip_snr_limit: u16,
    // place all byte data below to optimize packing */
    // uint8_t MaxConvTime; */        /*!< cached max convergence time @a #SYSRANGE_MAX_CONVERGENCE_TIME*/
}

struct RangeIgnoreData {
    valid_height: u16,
    ignore_threshold: u16,
    enabled: bool,
}

/// Per VL6180x device St private data structure \n
/// End user should never access any of these field directly

/// These must never access directly but only via VL6180xDev/SetData(dev, field) macro
struct VL6180xDevData {
    /// backed up NVM value
    part2_part_amb_nvm: u32,
    /// Cached XTlak Compensation Rate
    xtalk_comp_rate_kcps: u32,

    /// Ece Factor M numerator
    ece_factor_m: u16,
    /// Ece Factor D denominator
    ece_factor_d: u16,

    range_ignore: RangeIgnoreData,

    #[cfg(feature = "have_als_data")]
    /// cached als Integration period avoid slow read from device at each measure
    integration_period: u16,
    #[cfg(feature = "have_als_data")]
    /// cached Als gain avoid slow read from device at each measure
    als_gain_code: u16,
    #[cfg(feature = "have_als_data")]
    /// cached Als scaler avoid slow read from device at each measure
    als_scaler: u16,

    #[cfg(feature = "have_upscale_data")]
    /// up-scaling factor
    upscale_factor: u8,

    #[cfg(feature = "have_wrap_around_data")]
    /// Filter on/off
    wrap_around_filter_active: bool,
    #[cfg(feature = "have_wrap_around_data")]
    /// Filter internal data state history ...
    filter_data: FilterData,

    #[cfg(feature = "cached_reg")]
    /// Set if valid data got fetched use to control when to fill up register cache
    cache_filled: u8,
    #[cfg(feature = "cached_reg")]
    /// Cache register storage
    cached_regs: [u8; VL6180x_CACHED_REG_CNT],

    #[cfg(feature = "have_dmax_ranging")]
    dmax_data: DMaxData,
    #[cfg(feature = "have_dmax_ranging")]
    dmax_enable: bool,

    /// backed up NVM value
    part2_part_offset_nvm: i8,
}

/// Range and any optional measurement data.
pub struct RangeData {
    /// range distance in mm
    range_mm: i32,
    /// signal rate (MCPS)\n these is a 9.7 fix point value, which is effectively a measure of target reflectance
    signal_rate_mcps: i32,
    /// Error status of the current measurement. \n see @a ::RangeError_u @a VL6180x_GetRangeStatusErrString()
    error_status: pac::result_range_status::ERROR_CODE_R,

    #[cfg(feature = "have_rate_data")]
    rate: RateData,

    #[cfg(feature = "have_dmax_ranging")]
    /// DMax  when applicable
    dmax: u32,

    #[cfg(feature = "have_wrap_around_data")]
    /// Filter result main range_mm is updated
    FilteredData: RangeFilterResult,
}

pub struct RateData {
    /// Return Ambient rate in KCount per sec related to \a RESULT_RANGE_RETURN_AMB_COUNT
    amb_rate: u32,
    /// Return rate in KCount per sec  related to \a RESULT_RANGE_RETURN_SIGNAL_COUNT
    rate: u32,
    /// Return Convergence time \a RESULT_RANGE_RETURN_CONV_TIME
    conv_time: u32,
    /// Reference convergence time \a RESULT_RANGE_REFERENCE_CONV_TIME
    ref_conv_time: u32,
}

/// use where fix point 9.7 bit values are expected
///
/// given a floating point value f it's .7 bit point is (int)(f*(1<<7))*/
type FixPoint97 = u16;

/// Light measurement (Lux)
type Lux = u32;

/// Als measurement error
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlsErr {
    I2c(I2cError),
    Code(pac::result_als_status::ERROR_CODE_A),
}

impl From<I2cError> for AlsErr {
    fn from(e: I2cError) -> Self {
        match e {
            _ => AlsErr::I2c(e),
        }
    }
}

/// Als measurement error
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RangeErr {
    I2c(I2cError),
    DataNotReady,
}

impl From<I2cError> for RangeErr {
    fn from(e: I2cError) -> Self {
        match e {
            _ => RangeErr::I2c(e),
        }
    }
}

impl From<pac::result_als_status::ERROR_CODE_A> for AlsErr {
    fn from(e: pac::result_als_status::ERROR_CODE_A) -> Self {
        match e {
            _ => AlsErr::Code(e),
        }
    }
}

const LUXRES_FIX_PREC: u8 = 8;
const GAIN_FIX_PREC: u8 = 8;  /* ! if not sme as LUX_PREC then :( adjust GetLux */
const AN_GAIN_MULT: u16 = 1 << GAIN_FIX_PREC;

/// default value ECE factor Molecular
const DEF_ECE_FACTOR_M: u16 = 85;
/// default value ECE factor Denominator
const DEF_ECE_FACTOR_D: u16 = 100;
#[cfg(feature = "have_als_data")]
/// default value ALS integration time
const DEF_INT_PEFRIOD: u16 = 100;
#[cfg(feature = "have_als_data")]
/// default value ALS gain
const DEF_ALS_GAIN: u16 = 1;
#[cfg(feature = "have_als_data")]
/// default value ALS scaler
const DEF_ALS_SCALER: u16 = 1;
#[cfg(feature = "have_dmax_ranging")]
/// default value for DMAX Enable
const DEF_DMAX_ENABLE: bool = true;

#[cfg(feature = "single_device_driver")]
impl VL6180xDevData {
    /// the unique driver data  When single device driver is active
    pub fn single_default() -> Self {
        Self {
            ece_factor_m: DEF_ECE_FACTOR_M,
            ece_factor_d: DEF_ECE_FACTOR_D,
            #[cfg(feature = "have_upscale_data")]
            upscale_factor: DEF_UPSCALE,
            #[cfg(feature = "have_als_data")]
            integration_period: DEF_INT_PEFRIOD,
            #[cfg(feature = "have_als_data")]
            als_gain_code: DEF_ALS_GAIN,
            #[cfg(feature = "have_als_data")]
            als_scaler: DEF_ALS_SCALER,
            #[cfg(feature = "have_dmax_ranging")]
            dmax_enable: DEF_DMAX_ENABLE,
        }
    }
}

#[cfg(any(feature = "upscale_support_1", feature = "upscale_support_m1"))]
const DEF_UPSCALE: u8 = 1;
#[cfg(any(feature = "upscale_support_2", feature = "upscale_support_m2"))]
const DEF_UPSCALE: u8 = 2;
#[cfg(any(feature = "upscale_support_3", feature = "upscale_support_m3"))]
const DEF_UPSCALE: u8 = 3;

macro_rules! Fix7_2_KCPs {
    ($x:ident) => {
        (($x as u32)*1000)>>7
    }
}

macro_rules! conv9to7 {
    ($x:ident) => {
        $x
    }
}

pub struct VL6180x<I2C>
where
    I2C: WriteRead + WriteIter,
{
    params: VL6180xDevData,
    dev: I2cDevice<I2C>,
}

impl<I2C> VL6180x<I2C>
where
    I2C: WriteRead + WriteIter,
{
    pub fn range_start_continuous_mode(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSRANGE_START, |w| {
            w.startstop().set_bit().mode().continuous()
        })
    }

    pub fn range_start_single_shot(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSRANGE_START, |w| {
            w.startstop().set_bit().mode().single_shot()
        })
    }

    fn get_upscale(&self) -> u8 {
        if cfg!(feature = "upscale_support_1") {
            1
        } else if cfg!(feature = "upscale_support_2") {
            2
        } else if cfg!(feature = "upscale_support_3") {
            3
        } else {
            #[cfg(feature = "have_upscale_data")] {
                self.params.upscale_factor
            }
            #[cfg(not(feature = "have_upscale_data"))] {
                unreachable!()
            }
        }
    }

    fn set_upscale(&mut self, scaling: u8) {
        #[cfg(feature = "have_upscale_data")]
        {
            self.params.upscale_factor = scaling
        }
    }

    pub fn upscale_get_scaling(&mut self) -> u8 {
        self.get_upscale()
    }

    pub fn range_set_inter_meas_period(
        &mut self,
        i2c: &mut I2C,
        mut inter_meas_time_msec: u32,
    ) -> Result<(), ErrCode> {
        assert!(inter_meas_time_msec <= 2550);

        /* doc in not 100% clear and confusing about the limit practically all value are OK but 0
         * that can hang device in continuous mode */
        if inter_meas_time_msec < 10 {
            inter_meas_time_msec = 10;
        }
        let set_time = (inter_meas_time_msec / 10) as u8;
        self.dev
            .write(i2c, pac::SYSRANGE_INTERMEASUREMENT_PERIOD, |w| unsafe {
                w.bits(set_time)
            })?;

        if (set_time as u32) != inter_meas_time_msec / 10 {
            Err(ErrCode::MinCliped) /* on success change status to clip if it did */
        } else {
            Ok(())
        }
    }

    pub fn set_gpiox_polarity(
        &mut self,
        i2c: &mut I2C,
        pin: Pin,
        active_high: pac::system_mode_gpio0::POLARITY_A,
    ) -> Result<(), I2cError> {
        match pin {
            Pin::P0 => self.dev.modify(i2c, pac::SYSTEM_MODE_GPIO0, |_, w| {
                w.polarity().variant(active_high)
            }),
            Pin::P1 => self.dev.modify(i2c, pac::SYSTEM_MODE_GPIO1, |_, w| {
                w.polarity().variant(active_high)
            }),
        }
    }

    pub fn set_gpiox_functionality(
        &mut self,
        i2c: &mut I2C,
        pin: Pin,
        functionality: pac::system_mode_gpio0::SELECT_A,
    ) -> Result<(), I2cError> {
        match pin {
            Pin::P0 => self.dev.modify(i2c, pac::SYSTEM_MODE_GPIO0, |_, w| {
                w.select().variant(functionality)
            }),
            Pin::P1 => self.dev.modify(i2c, pac::SYSTEM_MODE_GPIO1, |_, w| {
                w.select().variant(functionality)
            }),
        }
    }

    pub fn setup_gpiox(
        &mut self,
        i2c: &mut I2C,
        pin: Pin,
        int_function: pac::system_mode_gpio0::SELECT_A,
        active_high: pac::system_mode_gpio0::POLARITY_A,
    ) -> Result<(), I2cError> {
        match pin {
            Pin::P0 => self.dev.write(i2c, pac::SYSTEM_MODE_GPIO0, |w| {
                w.select()
                    .variant(int_function)
                    .polarity()
                    .variant(active_high)
            }),
            Pin::P1 => self.dev.write(i2c, pac::SYSTEM_MODE_GPIO1, |w| {
                w.select()
                    .variant(int_function)
                    .polarity()
                    .variant(active_high)
            }),
        }
    }

    pub fn disable_gpiox_out(&mut self, i2c: &mut I2C, pin: Pin) -> Result<(), I2cError> {
        self.set_gpiox_functionality(i2c, pin, pac::system_mode_gpio0::SELECT_A::OFF)
    }

    pub fn setup_gpio1(
        &mut self,
        i2c: &mut I2C,
        int_function: pac::system_mode_gpio0::SELECT_A,
        active_high: pac::system_mode_gpio0::POLARITY_A,
    ) -> Result<(), I2cError> {
        self.setup_gpiox(i2c, Pin::P1, int_function, active_high)
    }

    pub fn range_set_ece_factor(
        &mut self,
        i2c: &mut I2C,
        factor_m: u16,
        factor_d: u16,
    ) -> Result<(), I2cError> {
        /* D cannot be 0 M must be <=D and >= 0 */
        assert!(
            factor_m <= factor_d && factor_d > 0,
            "invalid factor {}/{}",
            factor_m,
            factor_d
        );
        self.params.ece_factor_m = factor_m;
        self.params.ece_factor_d = factor_d;
        /* read and re-apply max conv time to get new ece factor set */
        let b8: u8 = self
            .dev
            .read(i2c, pac::SYSRANGE_MAX_CONVERGENCE_TIME)?
            .bits();
        self.range_set_max_convergence_time(i2c, b8)
            .expect("fail to apply time after ece m/d change");
        Ok(())
    }

    pub fn range_set_max_convergence_time(
        &mut self,
        i2c: &mut I2C,
        max_con_time_msec: u8,
    ) -> Result<(), I2cError> {
        self.dev
            .write(i2c, pac::SYSRANGE_MAX_CONVERGENCE_TIME, |w| unsafe {
                w.bits(max_con_time_msec)
            })?;
        self.range_set_early_convergence_eestimate_threshold(i2c)
    }
    pub fn range_set_early_convergence_eestimate_threshold(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<(), I2cError> {
        const C_MICRO_SEC_PER_MILLI_SEC: u32 = 1000;
        const C_ECE_SAMPLE_TIME_US: u32 = 500;
        let ece_factor_m = self.params.ece_factor_m as u32;
        let ece_factor_d = self.params.ece_factor_d as u32;

        let max_conv_ms = self
            .dev
            .read(i2c, pac::SYSRANGE_MAX_CONVERGENCE_TIME)?
            .bits() as u32;
        let ave_time = self.get_ave_total_time(i2c)?;
        let converg_time_us = max_conv_ms * C_MICRO_SEC_PER_MILLI_SEC - (ave_time as u32);
        let mut fine_thresh: u32 = self.dev.read_bits(i2c, 0xB8)?;
        fine_thresh *= 256;
        let ece_thresh =
            ece_factor_m * C_ECE_SAMPLE_TIME_US * fine_thresh / (converg_time_us * ece_factor_d);

        self.dev
            .write(i2c, pac::SYSRANGE_EARLY_CONVERGENCE_ESTIMATE, unsafe {
                |w| w.bits(ece_thresh as u16)
            })
    }

    fn get_ave_total_time(&mut self, i2c: &mut I2C) -> Result<(u32), I2cError> {
        let c_fw_overhead_us = 24_u32;
        let c_vcp_setup_time_us = 70_u32;
        let c_pll2_startup_delay_us = 200_u32;
        let c_meas_mask = 0x07_u8;

        let b8: u8 = self.dev.read_bits(i2c, 0x109)?;
        let samples = (b8 & c_meas_mask) as u32;

        let sample_period = self
            .dev
            .read(i2c, pac::READOUT_AVERAGING_SAMPLE_PERIOD)?
            .bits() as u32;
        let single_time_us = c_fw_overhead_us + c_vcp_setup_time_us + (sample_period * 10);
        let total_ave_time_us = (samples + 1) * single_time_us + c_pll2_startup_delay_us;

        Ok(total_ave_time_us)
    }

    pub fn range_static_init(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        unsafe {
            /* REGISTER_TUNING_SR03_270514_CustomerView.txt */
            self.dev.write_bits(i2c, 0x0207, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x0208, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x0096, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x0097, 0xfd_u8)?;
            self.dev.write_bits(i2c, 0x00e3, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x00e4, 0x03_u8)?;
            self.dev.write_bits(i2c, 0x00e5, 0x02_u8)?;
            self.dev.write_bits(i2c, 0x00e6, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x00e7, 0x03_u8)?;
            self.dev.write_bits(i2c, 0x00f5, 0x02_u8)?;
            self.dev.write_bits(i2c, 0x00d9, 0x05_u8)?;
            self.dev.write_bits(i2c, 0x00db, 0xce_u8)?;
            self.dev.write_bits(i2c, 0x00dc, 0x03_u8)?;
            self.dev.write_bits(i2c, 0x00dd, 0xf8_u8)?;
            self.dev.write_bits(i2c, 0x009f, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x00a3, 0x3c_u8)?;
            self.dev.write_bits(i2c, 0x00b7, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x00bb, 0x3c_u8)?; /*Recommended value = 0x28 in case very fast ranging frequency (~ 100 Hz)*/
            self.dev.write_bits(i2c, 0x00b2, 0x09_u8)?;
            self.dev.write_bits(i2c, 0x00ca, 0x09_u8)?;
            self.dev.write_bits(i2c, 0x0198, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x01b0, 0x17_u8)?;
            self.dev.write_bits(i2c, 0x01ad, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x00ff, 0x05_u8)?;
            self.dev.write_bits(i2c, 0x0100, 0x05_u8)?;
            self.dev.write_bits(i2c, 0x0199, 0x05_u8)?;
            self.dev.write_bits(i2c, 0x01a6, 0x1b_u8)?;
            self.dev.write_bits(i2c, 0x01ac, 0x3e_u8)?;
            self.dev.write_bits(i2c, 0x01a7, 0x1f_u8)?;
            self.dev.write_bits(i2c, 0x0030, 0x00_u8)?;

            /* Recommended : Public registers - See data sheet for more detail */
            self.dev.write_bits(i2c, 0x0011, 0x10_u8)?; /* Enables polling for New Sample ready when measurement completes */
            self.dev.write_bits(i2c, 0x010a, 0x30_u8)?; /* Set the averaging sample period (compromise between lower noise and increased execution time) */
            self.dev.write_bits(i2c, 0x003f, 0x46_u8)?; /* Sets the light and dark gain (upper nibble). Dark gain should not be changed.*/
            self.dev.write_bits(i2c, 0x0031, 0xFF_u8)?; /* sets the # of range measurements after which auto calibration of system is performed */
            self.dev.write_bits(i2c, 0x0040, 0x63_u8)?; /* Set ALS integration time to 100ms */
            self.dev.write_bits(i2c, 0x002e, 0x01_u8)?; /* perform a single temperature calibration of the ranging sensor */

            /* Optional: Public registers - See data sheet for more detail */
            self.dev.write_bits(i2c, 0x001b, 0x09_u8)?; /* Set default ranging inter-measurement period to 100ms */
            self.dev.write_bits(i2c, 0x003e, 0x31_u8)?; /* Set default ALS inter-measurement period to 500ms */
            self.dev.write_bits(i2c, 0x0014, 0x24_u8)?; /* Configures interrupt on New sample ready */
        }

        self.range_set_max_convergence_time(i2c, 50) /*  Calculate ece value on initialization (use max conv) */
    }

    #[cfg(not(feature = "upscale_support_1"))]
    fn _upscale_init_patch0(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        let cal_value = self.params.part2_part_amb_nvm;
        unsafe { self.dev.write_bits(i2c, 0xDA, cal_value) }
    }

    #[cfg(not(feature = "upscale_support_1"))]
    /// only include up-scaling register setting when up-scale support is configured in
    pub fn upscale_reg_init(&mut self, i2c: &mut I2C) -> Result<i8, I2cError> {
        unsafe {
            /*  apply REGISTER_TUNING_ER02_100614_CustomerView.txt */
            self.dev.write_bits(i2c, 0x0207, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x0208, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x0096, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x0097, 0x54_u8)?;
            self.dev.write_bits(i2c, 0x00e3, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x00e4, 0x04_u8)?;
            self.dev.write_bits(i2c, 0x00e5, 0x02_u8)?;
            self.dev.write_bits(i2c, 0x00e6, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x00e7, 0x03_u8)?;
            self.dev.write_bits(i2c, 0x00f5, 0x02_u8)?;
            self.dev.write_bits(i2c, 0x00d9, 0x05_u8)?;

            self._upscale_init_patch0(i2c)?;

            self.dev.write_bits(i2c, 0x009f, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x00a3, 0x28_u8)?;
            self.dev.write_bits(i2c, 0x00b7, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x00bb, 0x28_u8)?;
            self.dev.write_bits(i2c, 0x00b2, 0x09_u8)?;
            self.dev.write_bits(i2c, 0x00ca, 0x09_u8)?;
            self.dev.write_bits(i2c, 0x0198, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x01b0, 0x17_u8)?;
            self.dev.write_bits(i2c, 0x01ad, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x00ff, 0x05_u8)?;
            self.dev.write_bits(i2c, 0x0100, 0x05_u8)?;
            self.dev.write_bits(i2c, 0x0199, 0x05_u8)?;
            self.dev.write_bits(i2c, 0x01a6, 0x1b_u8)?;
            self.dev.write_bits(i2c, 0x01ac, 0x3e_u8)?;
            self.dev.write_bits(i2c, 0x01a7, 0x1f_u8)?;
            self.dev.write_bits(i2c, 0x0030, 0x00_u8)?;
            self.dev.write_bits(i2c, 0x0011, 0x10_u8)?;
            self.dev.write_bits(i2c, 0x010a, 0x30_u8)?;
            self.dev.write_bits(i2c, 0x003f, 0x46_u8)?;
            self.dev.write_bits(i2c, 0x0031, 0xFF_u8)?;
            self.dev.write_bits(i2c, 0x0040, 0x63_u8)?;
            self.dev.write_bits(i2c, 0x002e, 0x01_u8)?;
            self.dev.write_bits(i2c, 0x002c, 0xff_u8)?;
            self.dev.write_bits(i2c, 0x001b, 0x09_u8)?;
            self.dev.write_bits(i2c, 0x003e, 0x31_u8)?;
            self.dev.write_bits(i2c, 0x0014, 0x24_u8)?;

            self.range_set_max_convergence_time(
                i2c,
                if cfg!(feature = "extended_range") {
                    63
                } else {
                    50
                },
            )?;
            Ok(0)
        }
    }
    #[cfg(feature = "upscale_support_1")]
    pub fn UpscaleRegInit(&mut self, _i2c: &mut I2C) -> Result<i8, I2cError> {
        Ok(-1)
    }

    #[cfg(feature="have_dmax_ranging")]
    fn _GetDMaxDataRetSignalAt400mm(&self) -> DMaxFix {
        self.params.dmax_data.ret_signal_at_400mm
    }
    #[cfg(not(feature="have_dmax_ranging"))]
    fn _GetDMaxDataRetSignalAt400mm(&self) -> DMaxFix {
        375 /* Use a default high value */
    }

    pub fn clear_interrupt(&mut self, i2c: &mut I2C, int_clear: u8) -> Result<(), I2cError> {
        assert!(int_clear <= 7);
        self.dev
            .write(i2c, pac::SYSTEM_INTERRUPT_CLEAR, |w| unsafe {
                w.bits(int_clear)
            })
    }

    pub fn als_clear_interrupt(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSTEM_INTERRUPT_CLEAR, |w|
            w.als().set_bit()
        )
    }

    pub fn range_clear_interrupt(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSTEM_INTERRUPT_CLEAR, |w|
            w.range().set_bit()
        )
    }

    pub fn clear_error_interrupt(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSTEM_INTERRUPT_CLEAR, |w|
            w.error().set_bit()
        )
    }

    pub fn clear_all_interrupt(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSTEM_INTERRUPT_CLEAR, |w|
            w.als().set_bit()
            .range().set_bit()
            .error().set_bit()
        )
    }

    pub fn range_set_ece_state(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2cError> {
        self.dev
            .modify(i2c, pac::SYSRANGE_RANGE_CHECK_ENABLES, |_, w| {
                w.early_convergence().bit(enable)
            })
    }

    pub fn range_set_raw_thresholds(
        &mut self,
        i2c: &mut I2C,
        low: u8,
        high: u8,
    ) -> Result<(), I2cError> {
        /* TODO we can optimize here grouping high/low in a word but that's cpu endianness dependent */
        self.dev
            .write(i2c, pac::SYSRANGE_THRESH_HIGH, |w| unsafe { w.bits(high) })?;
        self.dev
            .write(i2c, pac::SYSRANGE_THRESH_LOW, |w| unsafe { w.bits(low) })
    }

    pub fn range_set_thresholds(
        &mut self,
        i2c: &mut I2C,
        low: u16,
        high: u16,
        use_safe_param_hold: bool,
    ) -> Result<(), I2cError> {
        let scale = self.get_upscale() as u16;
        assert!(low <= scale * 255 && high <= scale * 255);

        if use_safe_param_hold {
            self.set_group_param_hold(i2c, true)?;
        }
        self.range_set_raw_thresholds(i2c, (low / scale) as u8, (high / scale) as u8)
            .expect("VL6180x_RangeSetRawThresholds fail");
        if use_safe_param_hold {
            /* tryt to unset param hold vene if previous fail */
            self.set_group_param_hold(i2c, false)?;
        }
        Ok(())
    }

    pub fn range_get_thresholds(&mut self, i2c: &mut I2C) -> Result<(u16, u16), I2cError> {
        let scale = self.get_upscale() as u16;

        let raw_high = self.dev.read(i2c, pac::SYSRANGE_THRESH_HIGH)?.bits() as u16;
        let high = raw_high * scale;
        let raw_low = self.dev.read(i2c, pac::SYSRANGE_THRESH_LOW)?.bits() as u16;
        let low = raw_low * scale;
        Ok((low, high))
    }

    pub fn set_group_param_hold(&mut self, i2c: &mut I2C, hold: bool) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSTEM_GROUPED_PARAMETER_HOLD, |w| {
                w.value().bit(hold)
            })
    }

    pub fn range_wait_device_ready(
        &mut self,
        i2c: &mut I2C,
        max_loop: usize,
    ) -> Result<(), ErrCode> {
        assert!(max_loop > 0);
        let mut flag = false;
        for _ in 0..max_loop {
            flag = self
                .dev
                .read(i2c, pac::RESULT_RANGE_STATUS)?
                .device_ready()
                .bit_is_set()
        }
        if !flag {
            return Err(ErrCode::TimeOut);
        }
        Ok(())
    }

    pub fn range_get_device_ready(&mut self, i2c: &mut I2C) -> Result<bool, I2cError> {
        let r = self.dev.read(i2c, pac::RESULT_RANGE_STATUS)?;
        Ok(r.device_ready().bit_is_set())
    }

    pub fn set_i2c_address(&mut self, i2c: &mut I2C, new_address: u8) -> Result<(), I2cError> {
        self.dev
            .write(i2c, pac::I2C_SLAVE_DEVICE_ADDRESS, |w| unsafe {
                w.bits(new_address >> 1)
            })?;
        self.dev.set_i2c_address(new_address >> 1);
        Ok(())
    }

    fn range_ignore_update_device(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        let enabled = self.params.range_ignore.enabled;
        if enabled {
            // if to be nabled program first range value and threshold
            let mut range = self.params.range_ignore.valid_height;
            range /= self.get_upscale() as u16;
            if range > 255 {
                range = 255;
            }
            let range = range as u8;

            self.dev
                .write(i2c, pac::SYSRANGE_RANGE_IGNORE_VALID_HEIGHT, |w| unsafe {
                    w.bits(range)
                })?;

            let threshold = self.params.range_ignore.ignore_threshold;
            self.dev
                .write(i2c, pac::SYSRANGE_RANGE_IGNORE_THRESHOLD, |w| unsafe {
                    w.bits(threshold)
                })?;
        }
        self.dev
            .modify(i2c, pac::SYSRANGE_RANGE_CHECK_ENABLES, |_, w| {
                w.range_ignore().bit(enabled)
            })?;
        self.dmax_init_data(i2c)
    }

    pub fn range_ignore_set_enable(
        &mut self,
        i2c: &mut I2C,
        enable_state: bool,
    ) -> Result<(), I2cError> {
        if enable_state != self.params.range_ignore.enabled {
            self.params.range_ignore.enabled = enable_state;
            self.range_ignore_update_device(i2c)?;
        }
        Ok(())
    }

    pub fn range_ignore_configure(
        &mut self,
        i2c: &mut I2C,
        valid_height_mm: u16,
        ignore_threshold: u16,
    ) -> Result<(), I2cError> {
        let enabled = self.params.range_ignore.enabled;
        self.params.range_ignore.valid_height = valid_height_mm;
        self.params.range_ignore.ignore_threshold = ignore_threshold;
        if enabled {
            self.range_ignore_update_device(i2c)
        } else {
            Ok(())
        }
    }

    #[cfg(not(feature = "have_dmax_ranging"))]
    #[inline(always)]
    fn dmax_init_data(&mut self, _i2c: &mut I2C) -> Result<(), I2cError> {
        #[cfg(feature = "have_dmax_ranging")]
        self._dmax_init_data(_i2c).expect("Failed to init data")?;
        Ok(())
    }


    pub fn range_config_interrupt(&mut self, i2c: &mut I2C, config_gpio_int: pac::system_interrupt_config_gpio::RANGE_INT_MODE_A) -> Result<(), I2cError> {
        self.dev.modify(i2c, pac::SYSTEM_INTERRUPT_CONFIG_GPIO,
            |_,w| w.range_int_mode().variant(config_gpio_int)
        )
    }

    pub fn get_offset_calibration_data(&self) -> i8 {
        self.params.part2_part_offset_nvm
    }

    pub fn wait_device_booted(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        while self.dev.read(i2c, pac::SYSTEM_FRESH_OUT_OF_RESET)?.value().bit_is_clear() {}
        Ok(())
    }

    pub fn set_xtalk_compensation_rate(&mut self, i2c: &mut I2C, rate: FixPoint97) -> Result<(), I2cError> {
        self.dev.write(i2c, pac::SYSRANGE_CROSSTALK_COMPENSATION_RATE, |w| unsafe { w.bits(rate) })?;
        let xtalk_comp_rate_kcps = Fix7_2_KCPs!(rate);
        self.params.xtalk_comp_rate_kcps = xtalk_comp_rate_kcps;
        /* update dmax whenever xtalk rate changes */
        self.dmax_init_data(i2c)
    }

    pub fn get_interrupt_status(&mut self, i2c: &mut I2C) -> Result<pac::result_interrupt_status_gpio::R, I2cError> {
        self.dev.read(i2c, pac::RESULT_INTERRUPT_STATUS_GPIO)
    }
    pub fn range_get_interrupt_status(&mut self, i2c: &mut I2C) -> Result<(pac::result_interrupt_status_gpio::RANGE_R, pac::result_interrupt_status_gpio::ERROR_R), I2cError> {
    /* FIXME we are grouping "error" with over status the user must check implicitly for it
     * not just new sample or over status , that will nevr show up in case of error*/
        let r = self.get_cached(i2c, pac::RESULT_INTERRUPT_STATUS_GPIO)?;
        let range = r.range();
        let error = r.error();
        Ok((range, error))
    }

    pub fn range_get_result(&mut self, i2c: &mut I2C) -> Result<i32, I2cError> {
        let raw_range: u8 = self.get_cached(i2c, pac::RESULT_RANGE_VAL)?.bits();
        let upscale = self.get_upscale() as i32;
        Ok(upscale * (raw_range as i32))
    }

    #[cfg(feature="have_rate_data")]
    fn get_rate_result(&mut self, i2c: &mut I2C) -> Result<RateData, I2cError> {
        const RTN_SIGNAL_COUNT_MAX: u32 = 0x7FFF_FFFF;
        const DLL_PERIODS: u32 = 6;

        let mut signal_count: u32 = self.get_cached(i2c, pac::RESULT_RANGE_RETURN_SIGNAL_COUNT)?.bits();
        if signal_count > RTN_SIGNAL_COUNT_MAX {
            signal_count = 0;
        }

        let ambient_count: u32 = self.get_cached(i2c, pac::RESULT_RANGE_RETURN_AMB_COUNT)?.bits();

        let conv_time: u32 = self.get_cached(i2c, pac::RESULT_RANGE_RETURN_CONV_TIME)?.bits();

        let ref_conv_time = self.get_cached(i2c, pac::RESULT_RANGE_REFERENCE_CONV_TIME)?.bits();

        let mut calc_conv_time = ref_conv_time;
        if conv_time > ref_conv_time {
            calc_conv_time = conv_time;
        }
        if calc_conv_time == 0 {
            calc_conv_time = 63000;
        }

        Ok(RateData {
            amb_rate: (ambient_count * DLL_PERIODS * 1000) / calc_conv_time,
            rate: (signal_count * 1000) / calc_conv_time,
            conv_time,
            ref_conv_time,
        })
    }

    #[cfg(not(feature="cached_reg"))]
    fn get_cached<U, REG>(&mut self, i2c: &mut I2C, reg: REG) -> Result<R<U, REG>, I2cError>
    where
        REG: Readable + I2cAddress + Size<Type = U>,
        U: Copy + FromU8Array,
        U::U8Array: Default + AsMut<[u8]>
    {
        self.dev.read(i2c, reg)
    }

    pub fn filter_set_state(&mut self, _state: bool) -> Result<(), ErrCode> {
        #[cfg(feature="wrap_around_filter_support")] {
            self.params.WrapAroundFilterActive = _state;
            Ok(())
        }
        #[cfg(not(feature="wrap_around_filter_support"))] {
            Err(ErrCode::NotSupported)
        }
    }

    fn is_wrap_arround_active(&mut self) -> bool {
        #[cfg(feature="wrap_around_filter_support")] {
            self.params.WrapAroundFilterActive
        }
        #[cfg(not(feature="wrap_around_filter_support"))] {
            false
        }
    }

    pub fn filter_get_state(&mut self) -> bool {
        #[cfg(feature="wrap_around_filter_support")] {
            self.params.WrapAroundFilterActive
        }
        #[cfg(not(feature="wrap_around_filter_support"))] {
            false
        }
    }

    pub fn dmax_set_state(&mut self, _state: bool) -> Result<(), ErrCode> {
        #[cfg(feature="have_dmax_ranging")] {
            self.params.dmax_enable = _state;
            Ok(())
        }
        #[cfg(not(feature="have_dmax_ranging"))] {
            Err(ErrCode::NotSupported)
        }
    }

    pub fn dmax_get_state(&mut self) -> bool {
        #[cfg(feature="have_dmax_ranging")] {
            self.params.dmax_enable
        }
        #[cfg(not(feature="have_dmax_ranging"))] {
            false
        }
    }

    fn is_dmax_active(&mut self) -> bool {
        #[cfg(feature="have_dmax_ranging")] {
            self.params.dmax_enable
        }
        #[cfg(not(feature="have_dmax_ranging"))] {
            false
        }
    }

    pub fn set_offset_calibration_data(&mut self, i2c: &mut I2C, mut offset: i8) -> Result<(), I2cError> {
        self.params.part2_part_offset_nvm = offset;
        offset /= self.get_upscale() as i8;

        self.dev.write(i2c, pac::SYSRANGE_PART_TO_PART_RANGE_OFFSET, |w| unsafe { w.bits(offset as u8) })
    }
    
    pub fn get_upper_limit(&mut self) -> u16 {
        let scaling = self.get_upscale() as usize;
        /* FIXME we do assume here _GetUpscale is valid if  user call us prior to init we may overflow the LUT  mem area */
        UPPER_LIMIT_LOOK_UP[scaling - 1]
    }

    
    pub fn init_data(&mut self, i2c: &mut I2C) -> Result<(), ErrCode> {
        self.params.ece_factor_m = DEF_ECE_FACTOR_M;
        self.params.ece_factor_d = DEF_ECE_FACTOR_D;

        self.params.range_ignore.enabled = false;

        #[cfg(feature="have_upscale_data")] {
            self.params.upscale_factor = DEF_UPSCALE;
        }

        #[cfg(feature="have_als_data")] {
            self.params.IntegrationPeriod = DEF_INT_PEFRIOD;
            self.params.als_gain_code = DEF_ALS_GAIN;
            self.params.als_scaler = DEF_ALS_SCALER;
        }

        #[cfg(feature="have_wrap_around_data")] {
            self.params.wrap_around_filter_active = cfg!(feature="wrap_around_filter_support");
            self.params.dmax_enable = DEF_DMAX_ENABLE;
        }

        #[cfg(feature="have_dmax_ranging")]
        _DMax_OneTimeInit(dev);

        /* backup offset initial value from nvm these must be done prior any over call that use offset */
        let offset = self.dev.read(i2c, pac::SYSRANGE_PART_TO_PART_RANGE_OFFSET)?.bits() as i8;
        self.params.part2_part_offset_nvm = offset;

        let mut cal_value = self.dev.read(i2c, pac::SYSRANGE_RANGE_IGNORE_THRESHOLD)?.bits() as u32;
        if (cal_value&0xFFFF0000) == 0 {
            cal_value = 0x00CE03F8;
        }
        self.params.part2_part_amb_nvm = cal_value;

        let b16 = self.dev.read(i2c, pac::SYSRANGE_CROSSTALK_COMPENSATION_RATE)?.bits();

        self.params.xtalk_comp_rate_kcps = Fix7_2_KCPs!(b16);

        self.dmax_init_data(i2c).expect("DMax init failure");

        /* Read or wait for fresh out of reset  */
        let fresh_out_reset = self.dev.read(i2c, pac::SYSTEM_FRESH_OUT_OF_RESET)?.value().bit_is_set();
        if !fresh_out_reset/* || dmax_status*/ {
            Err(ErrCode::CalibrationWarning)
        } else {
            Ok(())
        }
    }

    pub fn prepare(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        self.static_init(i2c)?;

        /* set range InterruptMode to new sample */
        self.range_config_interrupt(i2c, pac::system_interrupt_config_gpio::RANGE_INT_MODE_A::NEW_SAMPLE_READY)?;

        /* set default threshold */
        self.range_set_raw_thresholds(i2c, 10, 200).expect("VL6180x_RangeSetRawThresholds fail");

        #[cfg(feature="als_support")] {
            self.als_set_integration_period(dev, 100)?;
            self.als_set_inter_measurement_period(dev, 200)?;
            self.als_set_analogue_gain(i2c,  0)?;
            self.als_set_thresholds(i2c, 0, 0xFF)?;
            /* set Als InterruptMode to new sample */
            self.als_config_interrupt(i2c, pac::system_interrupt_config_gpio::ALS_INT_MODE_A::NEW_SAMPLE_READY).expect("VL6180x_AlsConfigInterrupt fail");
        }
        #[cfg(feature="wrap_around_filter_support")] {
            self.filter_init();
        }
            /* make sure to reset any left previous condition that can hangs first poll */
        self.clear_all_interrupt(i2c)
    }

    pub fn static_init(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        /* TODO doc When using configurable scaling but using 1x as start condition
         * load tunning upscale  or not ??? */
        if (self.get_upscale() == 1) && !cfg!(feature = "have_upscale_data") {
            self.range_static_init(i2c).expect("StaticInit fail");
        } else {
            match self.upscale_static_init(i2c) {
                Err(ErrCode::I2c(_)) => panic!("StaticInit fail"),
                Err(_) => panic!("StaticInit"),
                _ => {},
            }
        }

        #[cfg(feature="refresh_cached_data_after_init")]
        #[cfg(feature="have_als_data")] {
        /* update cached value after tuning applied */
            self.params.AlsScaler = self.dev.read(i2c, pac::FW_ALS_RESULT_SCALER, &data).expect("StaticInit fail").bits();

            let data = self.dev.read(i2c, pac::SYSALS_ANALOGUE_GAIN).expect("StaticInit fail").bits();
            self.als_set_analogue_gain(i2c, data);
        }
        Ok(())
    }

    pub fn upscale_static_init(&mut self, i2c: &mut I2C) -> Result<(), ErrCode> {
        /* todo make these a fail macro in case only 1x is suppoted */
        self.upscale_reg_init(i2c).expect("regInit fail");
        #[cfg(feature="extended_range")] {
            self.range_set_ece_state(i2c, false).expect("VL6180x_RangeSetEceState fail");
        }
        /*  must write the scaler at least once to the device to ensure the scaler is in a known state. */
        let status = self.upscale_set_scaling(i2c, self.get_upscale());
        
        self.dev.write(i2c, pac::SYSTEM_FRESH_OUT_OF_RESET, |w| w.value().clear_bit())?; /* change fresh out of set status to 0 */
        status
    }

    pub fn upscale_set_scaling(&mut self, i2c: &mut I2C, scaling: u8) -> Result<(), ErrCode> {
        #[cfg(feature="have_upscale_data")]
        assert!(scaling >= 1 && scaling <= 3);

        #[cfg(not(feature="have_upscale_data"))]
        assert_eq!(scaling, VL6180x_UPSCALE_SUPPORT);

        let scaler = SCALER_LOOK_UP[scaling as usize - 1];
        const RANGE_SCALER: u16 = 0x096;
        unsafe { self.dev.write_bits(i2c, RANGE_SCALER, scaler)?; }
        self.set_upscale(scaling);

        /* Apply scaling on  part-2-part offset */
        let offset = (self.params.part2_part_offset_nvm as u8) / scaling;
        self.dev.write(i2c, pac::SYSRANGE_PART_TO_PART_RANGE_OFFSET, unsafe { |w| w.bits(offset) })?;

        /* Apply scaling on CrossTalkValidHeight */
        self.dev.write(i2c, pac::SYSRANGE_CROSSTALK_VALID_HEIGHT,
            unsafe { |w| w.bits(DEF_CROSS_TALK_VALID_HEIGHT_VALUE / scaling) })?;
        /* Apply scaling on RangeIgnore ValidHeight if enabled */
        if self.params.range_ignore.enabled {
            let mut valid_height = self.params.range_ignore.valid_height;
            valid_height /= self.get_upscale() as u16;
            if valid_height > 255 {
                valid_height = 255;
            }

            self.dev.write(i2c, pac::SYSRANGE_RANGE_IGNORE_VALID_HEIGHT,
                        unsafe { |w| w.bits(valid_height as u8)} )?;
        }

        if !cfg!(feature="extended_range") {
            self.range_set_ece_state(i2c, scaling == 1)?; /* enable ece only at 1x scaling */

            if scaling != 1 {
                return Err(ErrCode::NotGuaranteed) ;
            }
        }
        Ok(())
    }

    pub fn range_get_measurement_if_ready(&mut self, i2c: &mut I2C) -> Result<RangeData, RangeErr> {
        let (range, error) = self.range_get_interrupt_status(i2c).expect("fail to get interrupt status");
        if range.is_new_sample_ready() || !error.is_no() {
            let range_data = self.range_get_measurement(i2c)?;
            /*  clear range interrupt source */
            self.range_clear_interrupt(i2c).expect("VL6180x_RangeClearInterrupt fail");
            Ok(range_data)
        } else {
            Err(RangeErr::DataNotReady)
        }
    }

    #[cfg(not(feature="cached_reg"))]
    fn cached_regs_fetch(&mut self, i2c: &mut I2C) -> Result<(), I2cError> {
        Ok(())
    }

    pub fn range_get_measurement(&mut self, i2c: &mut I2C) -> Result<RangeData, I2cError> {
        self.cached_regs_fetch(i2c).expect("Cache register read fail");

        let range_mm = self.range_get_result(i2c).expect("VL6180x_GetRangeResult fail");

        let raw_rate: u16 = self.get_cached(i2c, pac::RESULT_RANGE_RETURN_RATE)?.bits();
        let signal_rate_mcps = conv9to7!(raw_rate) as i32;
        let error_status = self.get_cached(i2c, pac::RESULT_RANGE_STATUS)?.error_code();

        #[cfg(feature="have_rate_data")] {
            let rate = self.get_rate_result(i2c)?;
        }

        #[cfg(feature="wrap_around_filter_support")] {
            /* if enabled run filter */
            if self.is_wrap_arround_active() {
                _filter_GetResult(dev, pRangeData)?;
                /* patch the range status and measure if it is filtered */
                if(pRangeData.FilteredData.filterError != NoError) {
                    pRangeData.errorStatus = pRangeData.FilteredData.filterError;
                    pRangeData.range_mm = pRangeData.FilteredData.range_mm;
                }
            }
        }

        #[cfg(feature="have_dmax_ranging")] {
            if self.is_dmax_active() {
                _DMax_Compute(dev, pRangeData);
            }
        }
        Ok(RangeData {
            range_mm,
            signal_rate_mcps,
            error_status,
            #[cfg(feature="have_rate_data")]
            rate,
        })
    }
/*
    pub fn range_poll_measurement(&mut self, i2c: &mut I2C) -> Result<RangeData, I2cError> {
        /* start single range measurement */

        #[cfg(feature="safe_polling_enter")] {
            /* if device get stopped with left interrupt uncleared , it is required to clear them now or poll for new condition will never occur*/
            self.range_clear_interrupt(i2c).expect("VL6180x_RangeClearInterrupt fail");
        }
        /* //![single_shot_snipet] */
        self.dev.write(i2c, pac::SYSRANGE_START, |w| w
            .startstop() .set_bit()
            .mode().single_shot()
        )?;


        /* poll for new sample ready */
        loop {
            let (range, error) = self.range_get_interrupt_status(i2c)?;
            
            if range.is_new_sample_ready() || !error.is_no() {
                break;
            }

            VL6180x_PollDelay(dev);
        }
        /* //![single_shot_snipet] */

        let range_data = self.range_get_measurement(i2c);

        /*  clear range interrupt source */
        self.range_clear_interrupt(i2c).expect("VL6180x_RangeClearInterrupt fail");
        range_data
    }*/

}

/// ScalerLookUP scaling factor-1 to register #RANGE_SCALER lookup
const SCALER_LOOK_UP: [u16; 3] = [253, 127, 84]; // lookup table for scaling->scalar 1x2x 3x

/// scaling factor to Upper limit look up
const UPPER_LIMIT_LOOK_UP: [u16; 3] = [185, 370, 580]; // lookup table for scaling->limit  1x2x3x

/// Als Code gain to fix point gain lookup
const ALS_GAIN_LOOK_UP: [u16; 8] = [
    (20. * AN_GAIN_MULT as f32) as u16,
    (10. * AN_GAIN_MULT as f32) as u16,
    (5.  * AN_GAIN_MULT as f32) as u16,
    (2.5  * AN_GAIN_MULT as f32) as u16,
    (1.67 * AN_GAIN_MULT as f32) as u16,
    (1.25 * AN_GAIN_MULT as f32) as u16,
    (1.  * AN_GAIN_MULT as f32) as u16,
    (40. * AN_GAIN_MULT as f32) as u16,
];

const DEF_CROSS_TALK_VALID_HEIGHT_VALUE: u8 = 20;


#[cfg(feature="als_support")]
mod als;

#[cfg(feature="cached_reg")]
mod cached_reg;

#[cfg(feature="wrap_around_filter_support")]
mod wrap_around_filter;

#[cfg(feature="have_dmax_ranging")]
mod dmax_ranging;

/*
#define VL6180x_9to7Conv(x) (x)

/* TODO when set all "cached" value with "default init" are updated after init from register read back */
#define REFRESH_CACHED_DATA_AFTER_INIT  1


/** default ambient tuning factor %x1000 */
#define DEF_AMBIENT_TUNING  80



static int32_t _GetAveTotalTime(&mut self);



















*/


