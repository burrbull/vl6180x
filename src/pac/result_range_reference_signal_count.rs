#[allow(missing_docs)]
pub struct RESULT_RANGE_REFERENCE_SIGNAL_COUNT;
impl crate::Size for RESULT_RANGE_REFERENCE_SIGNAL_COUNT {
    type Type = u32;
}
impl crate::I2cAddress for RESULT_RANGE_REFERENCE_SIGNAL_COUNT {
    const ADDRESS: u16 = 0x70;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_RANGE_REFERENCE_SIGNAL_COUNT {}
#[doc = "Reader of register result_range_reference_signal_count"]
pub type R = crate::R<u32, RESULT_RANGE_REFERENCE_SIGNAL_COUNT>;
impl R {}
