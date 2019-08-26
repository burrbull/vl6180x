#[allow(missing_docs)]
pub struct SYSTEM_INTERRUPT_CLEAR;
impl crate::Size for SYSTEM_INTERRUPT_CLEAR {
    type Type = u8;
}
impl crate::I2cAddress for SYSTEM_INTERRUPT_CLEAR {
    const ADDRESS: u16 = 0x15;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSTEM_INTERRUPT_CLEAR {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSTEM_INTERRUPT_CLEAR {}
#[doc = "Reader of register system_interrupt_clear"]
pub type R = crate::R<u8, SYSTEM_INTERRUPT_CLEAR>;
#[doc = "Writer for register system_interrupt_clear"]
pub type W = crate::W<u8, SYSTEM_INTERRUPT_CLEAR>;
#[doc = "Register system_interrupt_clear `reset()`'s with value 0"]
impl crate::ResetValue for SYSTEM_INTERRUPT_CLEAR {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `range`"]
pub type RANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `range`"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
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
#[doc = "Reader of field `als`"]
pub type ALS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `als`"]
pub struct ALS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `error`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `error`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear Range Int"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear ALS Int"]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear Error Int"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Range Int"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Bit 1 - Clear ALS Int"]
    #[inline(always)]
    pub fn als(&mut self) -> ALS_W {
        ALS_W { w: self }
    }
    #[doc = "Bit 2 - Clear Error Int"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
}
