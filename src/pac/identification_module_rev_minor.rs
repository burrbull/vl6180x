#[allow(missing_docs)]
pub struct IDENTIFICATION_MODULE_REV_MINOR;
impl crate::Size for IDENTIFICATION_MODULE_REV_MINOR {
    type Type = u8;
}
impl crate::I2cAddress for IDENTIFICATION_MODULE_REV_MINOR {
    const ADDRESS: u16 = 0x04;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for IDENTIFICATION_MODULE_REV_MINOR {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for IDENTIFICATION_MODULE_REV_MINOR {}
#[doc = "Reader of register identification_module_rev_minor"]
pub type R = crate::R<u8, IDENTIFICATION_MODULE_REV_MINOR>;
#[doc = "Writer for register identification_module_rev_minor"]
pub type W = crate::W<u8, IDENTIFICATION_MODULE_REV_MINOR>;
#[doc = "Register identification_module_rev_minor `reset()`'s with value 0x02"]
impl crate::ResetValue for IDENTIFICATION_MODULE_REV_MINOR {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `value`"]
pub type VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `value`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
