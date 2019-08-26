#[allow(missing_docs)]
pub struct RESULT_RANGE_RETURN_AMB_COUNT;
impl crate::Size for RESULT_RANGE_RETURN_AMB_COUNT {
    type Type = u32;
}
impl crate::I2cAddress for RESULT_RANGE_RETURN_AMB_COUNT {
    const ADDRESS: u16 = 0x74;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_RETURN_AMB_COUNT {}
#[doc = "Reader of register result_range_return_amb_count"]
pub type R = crate::R<u32, RESULT_RANGE_RETURN_AMB_COUNT>;
impl R {}
