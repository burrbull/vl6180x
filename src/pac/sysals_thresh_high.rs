#[allow(missing_docs)]
pub struct SYSALS_THRESH_HIGH;
impl crate::Size for SYSALS_THRESH_HIGH {
    type Type = u16;
}
impl crate::I2cAddress for SYSALS_THRESH_HIGH {
    const ADDRESS: u16 = 0x3a;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSALS_THRESH_HIGH {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSALS_THRESH_HIGH {}
#[doc = "Reader of register sysals_thresh_high"]
pub type R = crate::R<u16, SYSALS_THRESH_HIGH>;
#[doc = "Writer for register sysals_thresh_high"]
pub type W = crate::W<u16, SYSALS_THRESH_HIGH>;
#[doc = "Register sysals_thresh_high `reset()`'s with value 0xffff"]
impl crate::ResetValue for SYSALS_THRESH_HIGH {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
impl R {}
impl W {}
