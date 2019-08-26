#[allow(missing_docs)]
pub struct SYSRANGE_PART_TO_PART_RANGE_OFFSET;
impl crate::Size for SYSRANGE_PART_TO_PART_RANGE_OFFSET {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_PART_TO_PART_RANGE_OFFSET {
    const ADDRESS: u16 = 0x24;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_PART_TO_PART_RANGE_OFFSET {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_PART_TO_PART_RANGE_OFFSET {}
#[doc = "Reader of register sysrange_part_to_part_range_offset"]
pub type R = crate::R<u8, SYSRANGE_PART_TO_PART_RANGE_OFFSET>;
#[doc = "Writer for register sysrange_part_to_part_range_offset"]
pub type W = crate::W<u8, SYSRANGE_PART_TO_PART_RANGE_OFFSET>;
#[doc = "Register sysrange_part_to_part_range_offset `reset()`'s with value 0"]
impl crate::ResetValue for SYSRANGE_PART_TO_PART_RANGE_OFFSET {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
