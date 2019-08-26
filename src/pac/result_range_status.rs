#[allow(missing_docs)]
pub struct RESULT_RANGE_STATUS;
impl crate::Size for RESULT_RANGE_STATUS {
    type Type = u8;
}
impl crate::I2cAddress for RESULT_RANGE_STATUS {
    const ADDRESS: u16 = 0x4d;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_STATUS {}
#[doc = "Reader of register result_range_status"]
pub type R = crate::R<u8, RESULT_RANGE_STATUS>;
#[doc = "Specific error codes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_CODE_A {
    #[doc = "0: No error"]
    NO,
    #[doc = "1: VCSEL Continuity Test"]
    VCSEL_CONTINUITY_TEST,
    #[doc = "2: VCSEL Watchdog Test"]
    VCSEL_WATCHDOG_TEST,
    #[doc = "3: VCSEL Watchdog"]
    VCSEL_WATCHDOG,
    #[doc = "4: PLL1 Lock"]
    PLL1_LOCK,
    #[doc = "5: PLL2 Lock"]
    PLL2_LOCK,
    #[doc = "6: Early Convergence Estimate"]
    EARLY_CONVERGENCE_ESTIMATE,
    #[doc = "7: Max Convergence"]
    MAX_CONVERGENCE,
    #[doc = "8: No Target Ignore"]
    NO_TARGET_IGNORE,
    #[doc = "11: Max Signal To Noise Ratio"]
    MAX_SNR,
    #[doc = "12: Raw Ranging Algo Underflow"]
    RAW_RANGING_ALGO_UNDERFLOW,
    #[doc = "13: Raw Ranging Algo Overflow"]
    RAW_RANGING_ALGO_OVERFLOW,
    #[doc = "14: Ranging Algo Underflow"]
    RANGING_ALGO_UNDERFLOW,
    #[doc = "15: Ranging Algo Overflow"]
    RANGING_ALGO_OVERFLOW,
}
impl From<ERROR_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERROR_CODE_A) -> Self {
        match variant {
            ERROR_CODE_A::NO => 0,
            ERROR_CODE_A::VCSEL_CONTINUITY_TEST => 1,
            ERROR_CODE_A::VCSEL_WATCHDOG_TEST => 2,
            ERROR_CODE_A::VCSEL_WATCHDOG => 3,
            ERROR_CODE_A::PLL1_LOCK => 4,
            ERROR_CODE_A::PLL2_LOCK => 5,
            ERROR_CODE_A::EARLY_CONVERGENCE_ESTIMATE => 6,
            ERROR_CODE_A::MAX_CONVERGENCE => 7,
            ERROR_CODE_A::NO_TARGET_IGNORE => 8,
            ERROR_CODE_A::MAX_SNR => 11,
            ERROR_CODE_A::RAW_RANGING_ALGO_UNDERFLOW => 12,
            ERROR_CODE_A::RAW_RANGING_ALGO_OVERFLOW => 13,
            ERROR_CODE_A::RANGING_ALGO_UNDERFLOW => 14,
            ERROR_CODE_A::RANGING_ALGO_OVERFLOW => 15,
        }
    }
}
#[doc = "Reader of field `error_code`"]
pub type ERROR_CODE_R = crate::R<u8, ERROR_CODE_A>;
impl ERROR_CODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERROR_CODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ERROR_CODE_A::NO),
            1 => Val(ERROR_CODE_A::VCSEL_CONTINUITY_TEST),
            2 => Val(ERROR_CODE_A::VCSEL_WATCHDOG_TEST),
            3 => Val(ERROR_CODE_A::VCSEL_WATCHDOG),
            4 => Val(ERROR_CODE_A::PLL1_LOCK),
            5 => Val(ERROR_CODE_A::PLL2_LOCK),
            6 => Val(ERROR_CODE_A::EARLY_CONVERGENCE_ESTIMATE),
            7 => Val(ERROR_CODE_A::MAX_CONVERGENCE),
            8 => Val(ERROR_CODE_A::NO_TARGET_IGNORE),
            11 => Val(ERROR_CODE_A::MAX_SNR),
            12 => Val(ERROR_CODE_A::RAW_RANGING_ALGO_UNDERFLOW),
            13 => Val(ERROR_CODE_A::RAW_RANGING_ALGO_OVERFLOW),
            14 => Val(ERROR_CODE_A::RANGING_ALGO_UNDERFLOW),
            15 => Val(ERROR_CODE_A::RANGING_ALGO_OVERFLOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ERROR_CODE_A::NO
    }
    #[doc = "Checks if the value of the field is `VCSEL_CONTINUITY_TEST`"]
    #[inline(always)]
    pub fn is_vcsel_continuity_test(&self) -> bool {
        *self == ERROR_CODE_A::VCSEL_CONTINUITY_TEST
    }
    #[doc = "Checks if the value of the field is `VCSEL_WATCHDOG_TEST`"]
    #[inline(always)]
    pub fn is_vcsel_watchdog_test(&self) -> bool {
        *self == ERROR_CODE_A::VCSEL_WATCHDOG_TEST
    }
    #[doc = "Checks if the value of the field is `VCSEL_WATCHDOG`"]
    #[inline(always)]
    pub fn is_vcsel_watchdog(&self) -> bool {
        *self == ERROR_CODE_A::VCSEL_WATCHDOG
    }
    #[doc = "Checks if the value of the field is `PLL1_LOCK`"]
    #[inline(always)]
    pub fn is_pll1_lock(&self) -> bool {
        *self == ERROR_CODE_A::PLL1_LOCK
    }
    #[doc = "Checks if the value of the field is `PLL2_LOCK`"]
    #[inline(always)]
    pub fn is_pll2_lock(&self) -> bool {
        *self == ERROR_CODE_A::PLL2_LOCK
    }
    #[doc = "Checks if the value of the field is `EARLY_CONVERGENCE_ESTIMATE`"]
    #[inline(always)]
    pub fn is_early_convergence_estimate(&self) -> bool {
        *self == ERROR_CODE_A::EARLY_CONVERGENCE_ESTIMATE
    }
    #[doc = "Checks if the value of the field is `MAX_CONVERGENCE`"]
    #[inline(always)]
    pub fn is_max_convergence(&self) -> bool {
        *self == ERROR_CODE_A::MAX_CONVERGENCE
    }
    #[doc = "Checks if the value of the field is `NO_TARGET_IGNORE`"]
    #[inline(always)]
    pub fn is_no_target_ignore(&self) -> bool {
        *self == ERROR_CODE_A::NO_TARGET_IGNORE
    }
    #[doc = "Checks if the value of the field is `MAX_SNR`"]
    #[inline(always)]
    pub fn is_max_snr(&self) -> bool {
        *self == ERROR_CODE_A::MAX_SNR
    }
    #[doc = "Checks if the value of the field is `RAW_RANGING_ALGO_UNDERFLOW`"]
    #[inline(always)]
    pub fn is_raw_ranging_algo_underflow(&self) -> bool {
        *self == ERROR_CODE_A::RAW_RANGING_ALGO_UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `RAW_RANGING_ALGO_OVERFLOW`"]
    #[inline(always)]
    pub fn is_raw_ranging_algo_overflow(&self) -> bool {
        *self == ERROR_CODE_A::RAW_RANGING_ALGO_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `RANGING_ALGO_UNDERFLOW`"]
    #[inline(always)]
    pub fn is_ranging_algo_underflow(&self) -> bool {
        *self == ERROR_CODE_A::RANGING_ALGO_UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `RANGING_ALGO_OVERFLOW`"]
    #[inline(always)]
    pub fn is_ranging_algo_overflow(&self) -> bool {
        *self == ERROR_CODE_A::RANGING_ALGO_OVERFLOW
    }
}
#[doc = "Reader of field `device_ready`"]
pub type DEVICE_READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 4:7 - Specific error codes"]
    #[inline(always)]
    pub fn error_code(&self) -> ERROR_CODE_R {
        ERROR_CODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Device Ready. When set to 1, indicates the device mode and configuration can be changed and a new start command will be accepted. When 0, indicates the device is busy"]
    #[inline(always)]
    pub fn device_ready(&self) -> DEVICE_READY_R {
        DEVICE_READY_R::new((self.bits & 0x01) != 0)
    }
}
