#[allow(missing_docs)]
pub struct RESULT_INTERRUPT_STATUS_GPIO;
impl crate::Size for RESULT_INTERRUPT_STATUS_GPIO {
    type Type = u8;
}
impl crate::I2cAddress for RESULT_INTERRUPT_STATUS_GPIO {
    const ADDRESS: u16 = 0x4f;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_INTERRUPT_STATUS_GPIO {}
#[doc = "Reader of register result_interrupt_status_gpio"]
pub type R = crate::R<u8, RESULT_INTERRUPT_STATUS_GPIO>;
#[doc = "Interrupt bits for Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: No error reported"]
    NO,
    #[doc = "1: Laser Safety Error"]
    LASER_SAFETY,
    #[doc = "2: PLL error (either PLL1 or PLL2)"]
    PLL,
}
impl From<ERROR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        match variant {
            ERROR_A::NO => 0,
            ERROR_A::LASER_SAFETY => 1,
            ERROR_A::PLL => 2,
        }
    }
}
#[doc = "Reader of field `error`"]
pub type ERROR_R = crate::R<u8, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERROR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ERROR_A::NO),
            1 => Val(ERROR_A::LASER_SAFETY),
            2 => Val(ERROR_A::PLL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ERROR_A::NO
    }
    #[doc = "Checks if the value of the field is `LASER_SAFETY`"]
    #[inline(always)]
    pub fn is_laser_safety(&self) -> bool {
        *self == ERROR_A::LASER_SAFETY
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == ERROR_A::PLL
    }
}
#[doc = "Interrupt bits for ALS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALS_A {
    #[doc = "0: No threshold events reported"]
    NO,
    #[doc = "1: Level Low threshold event"]
    LEVEL_LOW,
    #[doc = "2: Level High threshold event"]
    LEVEL_HIGH,
    #[doc = "3: Out Of Window threshold event"]
    OUT_OF_WINDOW,
    #[doc = "4: New Sample Ready threshold event"]
    NEW_SAMPLE_READY,
}
impl From<ALS_A> for u8 {
    #[inline(always)]
    fn from(variant: ALS_A) -> Self {
        match variant {
            ALS_A::NO => 0,
            ALS_A::LEVEL_LOW => 1,
            ALS_A::LEVEL_HIGH => 2,
            ALS_A::OUT_OF_WINDOW => 3,
            ALS_A::NEW_SAMPLE_READY => 4,
        }
    }
}
#[doc = "Reader of field `als`"]
pub type ALS_R = crate::R<u8, ALS_A>;
impl ALS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ALS_A::NO),
            1 => Val(ALS_A::LEVEL_LOW),
            2 => Val(ALS_A::LEVEL_HIGH),
            3 => Val(ALS_A::OUT_OF_WINDOW),
            4 => Val(ALS_A::NEW_SAMPLE_READY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ALS_A::NO
    }
    #[doc = "Checks if the value of the field is `LEVEL_LOW`"]
    #[inline(always)]
    pub fn is_level_low(&self) -> bool {
        *self == ALS_A::LEVEL_LOW
    }
    #[doc = "Checks if the value of the field is `LEVEL_HIGH`"]
    #[inline(always)]
    pub fn is_level_high(&self) -> bool {
        *self == ALS_A::LEVEL_HIGH
    }
    #[doc = "Checks if the value of the field is `OUT_OF_WINDOW`"]
    #[inline(always)]
    pub fn is_out_of_window(&self) -> bool {
        *self == ALS_A::OUT_OF_WINDOW
    }
    #[doc = "Checks if the value of the field is `NEW_SAMPLE_READY`"]
    #[inline(always)]
    pub fn is_new_sample_ready(&self) -> bool {
        *self == ALS_A::NEW_SAMPLE_READY
    }
}
#[doc = "Interrupt bits for Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGE_A {
    #[doc = "0: No threshold events reported"]
    NO,
    #[doc = "1: Level Low threshold event"]
    LEVEL_LOW,
    #[doc = "2: Level High threshold event"]
    LEVEL_HIGH,
    #[doc = "3: Out Of Window threshold event"]
    OUT_OF_WINDOW,
    #[doc = "4: New Sample Ready threshold event"]
    NEW_SAMPLE_READY,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        match variant {
            RANGE_A::NO => 0,
            RANGE_A::LEVEL_LOW => 1,
            RANGE_A::LEVEL_HIGH => 2,
            RANGE_A::OUT_OF_WINDOW => 3,
            RANGE_A::NEW_SAMPLE_READY => 4,
        }
    }
}
#[doc = "Reader of field `range`"]
pub type RANGE_R = crate::R<u8, RANGE_A>;
impl RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RANGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RANGE_A::NO),
            1 => Val(RANGE_A::LEVEL_LOW),
            2 => Val(RANGE_A::LEVEL_HIGH),
            3 => Val(RANGE_A::OUT_OF_WINDOW),
            4 => Val(RANGE_A::NEW_SAMPLE_READY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == RANGE_A::NO
    }
    #[doc = "Checks if the value of the field is `LEVEL_LOW`"]
    #[inline(always)]
    pub fn is_level_low(&self) -> bool {
        *self == RANGE_A::LEVEL_LOW
    }
    #[doc = "Checks if the value of the field is `LEVEL_HIGH`"]
    #[inline(always)]
    pub fn is_level_high(&self) -> bool {
        *self == RANGE_A::LEVEL_HIGH
    }
    #[doc = "Checks if the value of the field is `OUT_OF_WINDOW`"]
    #[inline(always)]
    pub fn is_out_of_window(&self) -> bool {
        *self == RANGE_A::OUT_OF_WINDOW
    }
    #[doc = "Checks if the value of the field is `NEW_SAMPLE_READY`"]
    #[inline(always)]
    pub fn is_new_sample_ready(&self) -> bool {
        *self == RANGE_A::NEW_SAMPLE_READY
    }
}
impl R {
    #[doc = "Bits 6:7 - Interrupt bits for Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Interrupt bits for ALS"]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Interrupt bits for Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new((self.bits & 0x07) as u8)
    }
}
