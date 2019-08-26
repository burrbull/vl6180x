#[allow(missing_docs)]
pub struct IDENTIFICATION_MODEL_ID;
impl crate::Size for IDENTIFICATION_MODEL_ID {
    type Type = u8;
}
impl crate::I2cAddress for IDENTIFICATION_MODEL_ID {
    const ADDRESS: u16 = 0;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for IDENTIFICATION_MODEL_ID {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for IDENTIFICATION_MODEL_ID {}
#[doc = "Reader of register identification_model_id"]
pub type R = crate::R<u8, IDENTIFICATION_MODEL_ID>;
#[doc = "Writer for register identification_model_id"]
pub type W = crate::W<u8, IDENTIFICATION_MODEL_ID>;
#[doc = "Register identification_model_id `reset()`'s with value 0xb4"]
impl crate::ResetValue for IDENTIFICATION_MODEL_ID {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb4
    }
}
impl R {}
impl W {}
