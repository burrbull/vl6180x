#[allow(missing_docs)]
pub struct RESULT_RANGE_RAW;
impl crate::Size for RESULT_RANGE_RAW {
    type Type = u8;
}
impl crate::I2cAddress for RESULT_RANGE_RAW {
    const ADDRESS: u16 = 0x64;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_RAW {}
#[doc = "Reader of register result_range_raw"]
pub type R = crate::R<u8, RESULT_RANGE_RAW>;
impl R {}
