#[allow(missing_docs)]
pub struct IDENTIFICATION_TIME;
impl crate::Size for IDENTIFICATION_TIME {
    type Type = u16;
}
impl crate::I2cAddress for IDENTIFICATION_TIME {
    const ADDRESS: u16 = 0x08;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for IDENTIFICATION_TIME {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for IDENTIFICATION_TIME {}
#[doc = "Reader of register identification_time"]
pub type R = crate::R<u16, IDENTIFICATION_TIME>;
#[doc = "Writer for register identification_time"]
pub type W = crate::W<u16, IDENTIFICATION_TIME>;
#[doc = "Register identification_time `reset()`'s with value 0"]
impl crate::ResetValue for IDENTIFICATION_TIME {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
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
