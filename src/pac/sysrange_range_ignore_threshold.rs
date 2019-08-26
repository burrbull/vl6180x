#[allow(missing_docs)]
pub struct SYSRANGE_RANGE_IGNORE_THRESHOLD;
impl crate::Size for SYSRANGE_RANGE_IGNORE_THRESHOLD {
    type Type = u16;
}
impl crate::I2cAddress for SYSRANGE_RANGE_IGNORE_THRESHOLD {
    const ADDRESS: u16 = 0x26;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_RANGE_IGNORE_THRESHOLD {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_RANGE_IGNORE_THRESHOLD {}
#[doc = "Reader of register sysrange_range_ignore_threshold"]
pub type R = crate::R<u16, SYSRANGE_RANGE_IGNORE_THRESHOLD>;
#[doc = "Writer for register sysrange_range_ignore_threshold"]
pub type W = crate::W<u16, SYSRANGE_RANGE_IGNORE_THRESHOLD>;
#[doc = "Register sysrange_range_ignore_threshold `reset()`'s with value 0"]
impl crate::ResetValue for SYSRANGE_RANGE_IGNORE_THRESHOLD {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
