#[allow(missing_docs)]
pub struct SYSRANGE_THRESH_HIGH;
impl crate::Size for SYSRANGE_THRESH_HIGH {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_THRESH_HIGH {
    const ADDRESS: u16 = 0x19;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_THRESH_HIGH {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_THRESH_HIGH {}
#[doc = "Reader of register sysrange_thresh_high"]
pub type R = crate::R<u8, SYSRANGE_THRESH_HIGH>;
#[doc = "Writer for register sysrange_thresh_high"]
pub type W = crate::W<u8, SYSRANGE_THRESH_HIGH>;
#[doc = "Register sysrange_thresh_high `reset()`'s with value 0xff"]
impl crate::ResetValue for SYSRANGE_THRESH_HIGH {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
impl R {}
impl W {}
