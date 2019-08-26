#[allow(missing_docs)]
pub struct FIRMWARE_RESULT_SCALER;
impl crate::Size for FIRMWARE_RESULT_SCALER {
    type Type = u8;
}
impl crate::I2cAddress for FIRMWARE_RESULT_SCALER {
    const ADDRESS: u16 = 0x0120;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for FIRMWARE_RESULT_SCALER {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for FIRMWARE_RESULT_SCALER {}
#[doc = "Reader of register firmware_result_scaler"]
pub type R = crate::R<u8, FIRMWARE_RESULT_SCALER>;
#[doc = "Writer for register firmware_result_scaler"]
pub type W = crate::W<u8, FIRMWARE_RESULT_SCALER>;
#[doc = "Register firmware_result_scaler `reset()`'s with value 0x01"]
impl crate::ResetValue for FIRMWARE_RESULT_SCALER {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `als`"]
pub type ALS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `als`"]
pub struct ALS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn als(&mut self) -> ALS_W {
        ALS_W { w: self }
    }
}
