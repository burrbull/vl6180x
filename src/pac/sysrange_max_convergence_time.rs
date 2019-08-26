#[allow(missing_docs)]
pub struct SYSRANGE_MAX_CONVERGENCE_TIME;
impl crate::Size for SYSRANGE_MAX_CONVERGENCE_TIME {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_MAX_CONVERGENCE_TIME {
    const ADDRESS: u16 = 0x1c;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_MAX_CONVERGENCE_TIME {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_MAX_CONVERGENCE_TIME {}
#[doc = "Reader of register sysrange_max_convergence_time"]
pub type R = crate::R<u8, SYSRANGE_MAX_CONVERGENCE_TIME>;
#[doc = "Writer for register sysrange_max_convergence_time"]
pub type W = crate::W<u8, SYSRANGE_MAX_CONVERGENCE_TIME>;
#[doc = "Register sysrange_max_convergence_time `reset()`'s with value 0x31"]
impl crate::ResetValue for SYSRANGE_MAX_CONVERGENCE_TIME {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x31
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
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
