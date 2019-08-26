#[allow(missing_docs)]
pub struct RESULT_RANGE_RETURN_RATE;
impl crate::Size for RESULT_RANGE_RETURN_RATE {
    type Type = u16;
}
impl crate::I2cAddress for RESULT_RANGE_RETURN_RATE {
    const ADDRESS: u16 = 0x66;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_RETURN_RATE {}
#[doc = "Reader of register result_range_return_rate"]
pub type R = crate::R<u16, RESULT_RANGE_RETURN_RATE>;
impl R {}
