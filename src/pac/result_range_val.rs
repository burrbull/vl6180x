#[allow(missing_docs)]
pub struct RESULT_RANGE_VAL;
impl crate::Size for RESULT_RANGE_VAL {
    type Type = u8;
}
impl crate::I2cAddress for RESULT_RANGE_VAL {
    const ADDRESS: u16 = 0x62;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_VAL {}
#[doc = "Reader of register result_range_val"]
pub type R = crate::R<u8, RESULT_RANGE_VAL>;
impl R {}
