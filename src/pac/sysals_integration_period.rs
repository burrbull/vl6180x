#[allow(missing_docs)]
pub struct SYSALS_INTEGRATION_PERIOD;
impl crate::Size for SYSALS_INTEGRATION_PERIOD {
    type Type = u8;
}
impl crate::I2cAddress for SYSALS_INTEGRATION_PERIOD {
    const ADDRESS: u16 = 0x40;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSALS_INTEGRATION_PERIOD {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSALS_INTEGRATION_PERIOD {}
#[doc = "Reader of register sysals_integration_period"]
pub type R = crate::R<u8, SYSALS_INTEGRATION_PERIOD>;
#[doc = "Writer for register sysals_integration_period"]
pub type W = crate::W<u8, SYSALS_INTEGRATION_PERIOD>;
#[doc = "Register sysals_integration_period `reset()`'s with value 0"]
impl crate::ResetValue for SYSALS_INTEGRATION_PERIOD {
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
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
