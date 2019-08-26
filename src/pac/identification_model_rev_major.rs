#[allow(missing_docs)]
pub struct IDENTIFICATION_MODEL_REV_MAJOR;
impl crate::Size for IDENTIFICATION_MODEL_REV_MAJOR {
    type Type = u8;
}
impl crate::I2cAddress for IDENTIFICATION_MODEL_REV_MAJOR {
    const ADDRESS: u16 = 0x01;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for IDENTIFICATION_MODEL_REV_MAJOR {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for IDENTIFICATION_MODEL_REV_MAJOR {}
#[doc = "Reader of register identification_model_rev_major"]
pub type R = crate::R<u8, IDENTIFICATION_MODEL_REV_MAJOR>;
#[doc = "Writer for register identification_model_rev_major"]
pub type W = crate::W<u8, IDENTIFICATION_MODEL_REV_MAJOR>;
#[doc = "Register identification_model_rev_major `reset()`'s with value 0x01"]
impl crate::ResetValue for IDENTIFICATION_MODEL_REV_MAJOR {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
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
