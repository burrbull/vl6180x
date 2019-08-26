#[allow(missing_docs)]
pub struct RESULT_RANGE_REFERENCE_RATE;
impl crate::Size for RESULT_RANGE_REFERENCE_RATE {
    type Type = u16;
}
impl crate::I2cAddress for RESULT_RANGE_REFERENCE_RATE {
    const ADDRESS: u16 = 0x68;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_REFERENCE_RATE {}
#[doc = "Reader of register result_range_reference_rate"]
pub type R = crate::R<u16, RESULT_RANGE_REFERENCE_RATE>;
impl R {}
