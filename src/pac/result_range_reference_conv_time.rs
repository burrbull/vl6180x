#[allow(missing_docs)]
pub struct RESULT_RANGE_REFERENCE_CONV_TIME;
impl crate::Size for RESULT_RANGE_REFERENCE_CONV_TIME {
    type Type = u32;
}
impl crate::I2cAddress for RESULT_RANGE_REFERENCE_CONV_TIME {
    const ADDRESS: u16 = 0x80;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_REFERENCE_CONV_TIME {}
#[doc = "Reader of register result_range_reference_conv_time"]
pub type R = crate::R<u32, RESULT_RANGE_REFERENCE_CONV_TIME>;
impl R {}
