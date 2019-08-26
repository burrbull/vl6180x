#[allow(missing_docs)]
pub struct RESULT_ALS_STATUS;
impl crate::Size for RESULT_ALS_STATUS {
    type Type = u8;
}
impl crate::I2cAddress for RESULT_ALS_STATUS {
    const ADDRESS: u16 = 0x4e;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_ALS_STATUS {}
#[doc = "Reader of register result_als_status"]
pub type R = crate::R<u8, RESULT_ALS_STATUS>;
#[doc = "Specific error and debug codes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_CODE_A {
    #[doc = "0: No error"]
    NO,
    #[doc = "1: Overflow error"]
    OVERFLOW,
    #[doc = "2: Underflow error"]
    UNDERFLOW,
}
impl From<ERROR_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERROR_CODE_A) -> Self {
        match variant {
            ERROR_CODE_A::NO => 0,
            ERROR_CODE_A::OVERFLOW => 1,
            ERROR_CODE_A::UNDERFLOW => 2,
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
            1 => Val(ERROR_CODE_A::OVERFLOW),
            2 => Val(ERROR_CODE_A::UNDERFLOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ERROR_CODE_A::NO
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == ERROR_CODE_A::OVERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == ERROR_CODE_A::UNDERFLOW
    }
}
#[doc = "Reader of field `device_ready`"]
pub type DEVICE_READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 4:7 - Specific error and debug codes"]
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
