#[allow(missing_docs)]
pub struct SYSTEM_MODE_GPIO1;
impl crate::Size for SYSTEM_MODE_GPIO1 {
    type Type = u8;
}
impl crate::I2cAddress for SYSTEM_MODE_GPIO1 {
    const ADDRESS: u16 = 0x11;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSTEM_MODE_GPIO1 {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSTEM_MODE_GPIO1 {}
#[doc = "Reader of register system_mode_gpio1"]
pub type R = crate::R<u8, SYSTEM_MODE_GPIO1>;
#[doc = "Writer for register system_mode_gpio1"]
pub type W = crate::W<u8, SYSTEM_MODE_GPIO1>;
#[doc = "Register system_mode_gpio1 `reset()`'s with value 0x20"]
impl crate::ResetValue for SYSTEM_MODE_GPIO1 {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Functional configuration options"]
pub type SELECT_A = super::system_mode_gpio0::SELECT_A;
#[doc = "Reader of field `select`"]
pub type SELECT_R = crate::R<u8, super::system_mode_gpio0::SELECT_A>;
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
        self.variant(super::system_mode_gpio0::SELECT_A::OFF)
    }
    #[doc = "GPIO Interrupt output"]
    #[inline(always)]
    pub fn interrupt_output(self) -> &'a mut W {
        self.variant(super::system_mode_gpio0::SELECT_A::INTERRUPT_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u8) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Signal Polarity Selection"]
pub type POLARITY_A = super::system_mode_gpio0::POLARITY_A;
#[doc = "Reader of field `polarity`"]
pub type POLARITY_R = crate::R<bool, super::system_mode_gpio0::POLARITY_A>;
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
        self.variant(super::system_mode_gpio0::POLARITY_A::LOW)
    }
    #[doc = "Active-high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(super::system_mode_gpio0::POLARITY_A::HIGH)
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
}
