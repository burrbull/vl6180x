#[allow(missing_docs)]
pub struct SYSTEM_INTERRUPT_CONFIG_GPIO;
impl crate::Size for SYSTEM_INTERRUPT_CONFIG_GPIO {
    type Type = u8;
}
impl crate::I2cAddress for SYSTEM_INTERRUPT_CONFIG_GPIO {
    const ADDRESS: u16 = 0x14;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSTEM_INTERRUPT_CONFIG_GPIO {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSTEM_INTERRUPT_CONFIG_GPIO {}
#[doc = "Reader of register system_interrupt_config_gpio"]
pub type R = crate::R<u8, SYSTEM_INTERRUPT_CONFIG_GPIO>;
#[doc = "Writer for register system_interrupt_config_gpio"]
pub type W = crate::W<u8, SYSTEM_INTERRUPT_CONFIG_GPIO>;
#[doc = "Register system_interrupt_config_gpio `reset()`'s with value 0"]
impl crate::ResetValue for SYSTEM_INTERRUPT_CONFIG_GPIO {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt mode source for Range readings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGE_INT_MODE_A {
    #[doc = "0: `0`"]
    DISABLED,
    #[doc = "1: value less then thresh_low"]
    LEVEL_LOW,
    #[doc = "2: value more then thresh_high"]
    LEVEL_HIGH,
    #[doc = "3: value less then thresh_low OR value more then thresh_high"]
    OUT_OF_WINDOW,
    #[doc = "4: `100`"]
    NEW_SAMPLE_READY,
}
impl From<RANGE_INT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_INT_MODE_A) -> Self {
        match variant {
            RANGE_INT_MODE_A::DISABLED => 0,
            RANGE_INT_MODE_A::LEVEL_LOW => 1,
            RANGE_INT_MODE_A::LEVEL_HIGH => 2,
            RANGE_INT_MODE_A::OUT_OF_WINDOW => 3,
            RANGE_INT_MODE_A::NEW_SAMPLE_READY => 4,
        }
    }
}
#[doc = "Reader of field `range_int_mode`"]
pub type RANGE_INT_MODE_R = crate::R<u8, RANGE_INT_MODE_A>;
impl RANGE_INT_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RANGE_INT_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RANGE_INT_MODE_A::DISABLED),
            1 => Val(RANGE_INT_MODE_A::LEVEL_LOW),
            2 => Val(RANGE_INT_MODE_A::LEVEL_HIGH),
            3 => Val(RANGE_INT_MODE_A::OUT_OF_WINDOW),
            4 => Val(RANGE_INT_MODE_A::NEW_SAMPLE_READY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RANGE_INT_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LEVEL_LOW`"]
    #[inline(always)]
    pub fn is_level_low(&self) -> bool {
        *self == RANGE_INT_MODE_A::LEVEL_LOW
    }
    #[doc = "Checks if the value of the field is `LEVEL_HIGH`"]
    #[inline(always)]
    pub fn is_level_high(&self) -> bool {
        *self == RANGE_INT_MODE_A::LEVEL_HIGH
    }
    #[doc = "Checks if the value of the field is `OUT_OF_WINDOW`"]
    #[inline(always)]
    pub fn is_out_of_window(&self) -> bool {
        *self == RANGE_INT_MODE_A::OUT_OF_WINDOW
    }
    #[doc = "Checks if the value of the field is `NEW_SAMPLE_READY`"]
    #[inline(always)]
    pub fn is_new_sample_ready(&self) -> bool {
        *self == RANGE_INT_MODE_A::NEW_SAMPLE_READY
    }
}
#[doc = "Write proxy for field `range_int_mode`"]
pub struct RANGE_INT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_INT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_INT_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::DISABLED)
    }
    #[doc = "value less then thresh_low"]
    #[inline(always)]
    pub fn level_low(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::LEVEL_LOW)
    }
    #[doc = "value more then thresh_high"]
    #[inline(always)]
    pub fn level_high(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::LEVEL_HIGH)
    }
    #[doc = "value less then thresh_low OR value more then thresh_high"]
    #[inline(always)]
    pub fn out_of_window(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::OUT_OF_WINDOW)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn new_sample_ready(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::NEW_SAMPLE_READY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Interrupt mode source for ALS readings"]
pub type ALS_INT_MODE_A = RANGE_INT_MODE_A;
#[doc = "Reader of field `als_int_mode`"]
pub type ALS_INT_MODE_R = crate::R<u8, RANGE_INT_MODE_A>;
#[doc = "Write proxy for field `als_int_mode`"]
pub struct ALS_INT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALS_INT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALS_INT_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::DISABLED)
    }
    #[doc = "value less then thresh_low"]
    #[inline(always)]
    pub fn level_low(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::LEVEL_LOW)
    }
    #[doc = "value more then thresh_high"]
    #[inline(always)]
    pub fn level_high(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::LEVEL_HIGH)
    }
    #[doc = "value less then thresh_low OR value more then thresh_high"]
    #[inline(always)]
    pub fn out_of_window(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::OUT_OF_WINDOW)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn new_sample_ready(self) -> &'a mut W {
        self.variant(RANGE_INT_MODE_A::NEW_SAMPLE_READY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u8) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Interrupt mode source for Range readings"]
    #[inline(always)]
    pub fn range_int_mode(&self) -> RANGE_INT_MODE_R {
        RANGE_INT_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Interrupt mode source for ALS readings"]
    #[inline(always)]
    pub fn als_int_mode(&self) -> ALS_INT_MODE_R {
        ALS_INT_MODE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Interrupt mode source for Range readings"]
    #[inline(always)]
    pub fn range_int_mode(&mut self) -> RANGE_INT_MODE_W {
        RANGE_INT_MODE_W { w: self }
    }
    #[doc = "Bits 3:5 - Interrupt mode source for ALS readings"]
    #[inline(always)]
    pub fn als_int_mode(&mut self) -> ALS_INT_MODE_W {
        ALS_INT_MODE_W { w: self }
    }
}
