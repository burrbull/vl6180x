#[allow(missing_docs)]
pub struct SYSTEM_MODE_GPIO0;
impl crate::Size for SYSTEM_MODE_GPIO0 {
    type Type = u8;
}
impl crate::I2cAddress for SYSTEM_MODE_GPIO0 {
    const ADDRESS: u16 = 0x10;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSTEM_MODE_GPIO0 {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSTEM_MODE_GPIO0 {}
#[doc = "Reader of register system_mode_gpio0"]
pub type R = crate::R<u8, SYSTEM_MODE_GPIO0>;
#[doc = "Writer for register system_mode_gpio0"]
pub type W = crate::W<u8, SYSTEM_MODE_GPIO0>;
#[doc = "Register system_mode_gpio0 `reset()`'s with value 0x60"]
impl crate::ResetValue for SYSTEM_MODE_GPIO0 {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x60
    }
}
#[doc = "Functional configuration options\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECT_A {
    #[doc = "0: Hi-Z"]
    OFF,
    #[doc = "8: GPIO Interrupt output"]
    INTERRUPT_OUTPUT,
}
impl From<SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELECT_A) -> Self {
        match variant {
            SELECT_A::OFF => 0,
            SELECT_A::INTERRUPT_OUTPUT => 8,
        }
    }
}
#[doc = "Reader of field `select`"]
pub type SELECT_R = crate::R<u8, SELECT_A>;
impl SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SELECT_A::OFF),
            8 => Val(SELECT_A::INTERRUPT_OUTPUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_OUTPUT`"]
    #[inline(always)]
    pub fn is_interrupt_output(&self) -> bool {
        *self == SELECT_A::INTERRUPT_OUTPUT
    }
}
#[doc = "Write proxy for field `select`"]
pub struct SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Hi-Z"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SELECT_A::OFF)
    }
    #[doc = "GPIO Interrupt output"]
    #[inline(always)]
    pub fn interrupt_output(self) -> &'a mut W {
        self.variant(SELECT_A::INTERRUPT_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u8) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Signal Polarity Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLARITY_A {
    #[doc = "0: Active-low"]
    LOW,
    #[doc = "1: Active-high"]
    HIGH,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        match variant {
            POLARITY_A::LOW => false,
            POLARITY_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `polarity`"]
pub type POLARITY_R = crate::R<bool, POLARITY_A>;
impl POLARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::LOW,
            true => POLARITY_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POLARITY_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POLARITY_A::HIGH
    }
}
#[doc = "Write proxy for field `polarity`"]
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLARITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active-low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POLARITY_A::LOW)
    }
    #[doc = "Active-high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POLARITY_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Priority mode - when enabled, other bits of the register are ignored. GPIO0 is main XSHUTDOWN input.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_XSHUTDOWN_A {
    #[doc = "0: `0`"]
    DISABLED,
    #[doc = "1: GPIO0 is main XSHUTDOWN input"]
    ENABLED,
}
impl From<IS_XSHUTDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: IS_XSHUTDOWN_A) -> Self {
        match variant {
            IS_XSHUTDOWN_A::DISABLED => false,
            IS_XSHUTDOWN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `is_xshutdown`"]
pub type IS_XSHUTDOWN_R = crate::R<bool, IS_XSHUTDOWN_A>;
impl IS_XSHUTDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_XSHUTDOWN_A {
        match self.bits {
            false => IS_XSHUTDOWN_A::DISABLED,
            true => IS_XSHUTDOWN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IS_XSHUTDOWN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IS_XSHUTDOWN_A::ENABLED
    }
}
#[doc = "Write proxy for field `is_xshutdown`"]
pub struct IS_XSHUTDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_XSHUTDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_XSHUTDOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IS_XSHUTDOWN_A::DISABLED)
    }
    #[doc = "GPIO0 is main XSHUTDOWN input"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IS_XSHUTDOWN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:4 - Functional configuration options"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Signal Polarity Selection"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Priority mode - when enabled, other bits of the register are ignored. GPIO0 is main XSHUTDOWN input."]
    #[inline(always)]
    pub fn is_xshutdown(&self) -> IS_XSHUTDOWN_R {
        IS_XSHUTDOWN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - Functional configuration options"]
    #[inline(always)]
    pub fn select(&mut self) -> SELECT_W {
        SELECT_W { w: self }
    }
    #[doc = "Bit 5 - Signal Polarity Selection"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    #[doc = "Bit 6 - Priority mode - when enabled, other bits of the register are ignored. GPIO0 is main XSHUTDOWN input."]
    #[inline(always)]
    pub fn is_xshutdown(&mut self) -> IS_XSHUTDOWN_W {
        IS_XSHUTDOWN_W { w: self }
    }
}
