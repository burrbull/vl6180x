#[allow(missing_docs)]
pub struct SYSRANGE_RANGE_CHECK_ENABLES;
impl crate::Size for SYSRANGE_RANGE_CHECK_ENABLES {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_RANGE_CHECK_ENABLES {
    const ADDRESS: u16 = 0x2d;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_RANGE_CHECK_ENABLES {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_RANGE_CHECK_ENABLES {}
#[doc = "Reader of register sysrange_range_check_enables"]
pub type R = crate::R<u8, SYSRANGE_RANGE_CHECK_ENABLES>;
#[doc = "Writer for register sysrange_range_check_enables"]
pub type W = crate::W<u8, SYSRANGE_RANGE_CHECK_ENABLES>;
#[doc = "Register sysrange_range_check_enables `reset()`'s with value 0x11"]
impl crate::ResetValue for SYSRANGE_RANGE_CHECK_ENABLES {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x11
    }
}
#[doc = "Reader of field `signal_to_noise`"]
pub type SIGNAL_TO_NOISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `signal_to_noise`"]
pub struct SIGNAL_TO_NOISE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNAL_TO_NOISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `range_ignore`"]
pub type RANGE_IGNORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `range_ignore`"]
pub struct RANGE_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_IGNORE_W<'a> {
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
#[doc = "Reader of field `early_convergence`"]
pub type EARLY_CONVERGENCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `early_convergence`"]
pub struct EARLY_CONVERGENCE_W<'a> {
    w: &'a mut W,
}
impl<'a> EARLY_CONVERGENCE_W<'a> {
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
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn signal_to_noise(&self) -> SIGNAL_TO_NOISE_R {
        SIGNAL_TO_NOISE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn range_ignore(&self) -> RANGE_IGNORE_R {
        RANGE_IGNORE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn early_convergence(&self) -> EARLY_CONVERGENCE_R {
        EARLY_CONVERGENCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn signal_to_noise(&mut self) -> SIGNAL_TO_NOISE_W {
        SIGNAL_TO_NOISE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn range_ignore(&mut self) -> RANGE_IGNORE_W {
        RANGE_IGNORE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn early_convergence(&mut self) -> EARLY_CONVERGENCE_W {
        EARLY_CONVERGENCE_W { w: self }
    }
}
