#[allow(missing_docs)]
pub struct SYSTEM_FRESH_OUT_OF_RESET;
impl crate::Size for SYSTEM_FRESH_OUT_OF_RESET {
    type Type = u8;
}
impl crate::I2cAddress for SYSTEM_FRESH_OUT_OF_RESET {
    const ADDRESS: u16 = 0x16;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSTEM_FRESH_OUT_OF_RESET {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSTEM_FRESH_OUT_OF_RESET {}
#[doc = "Reader of register system_fresh_out_of_reset"]
pub type R = crate::R<u8, SYSTEM_FRESH_OUT_OF_RESET>;
#[doc = "Writer for register system_fresh_out_of_reset"]
pub type W = crate::W<u8, SYSTEM_FRESH_OUT_OF_RESET>;
#[doc = "Register system_fresh_out_of_reset `reset()`'s with value 0x01"]
impl crate::ResetValue for SYSTEM_FRESH_OUT_OF_RESET {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `value`"]
pub type VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `value`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Fresh out of reset bit"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fresh out of reset bit"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
