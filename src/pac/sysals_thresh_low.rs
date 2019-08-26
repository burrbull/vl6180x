#[allow(missing_docs)]
pub struct SYSALS_THRESH_LOW;
impl crate::Size for SYSALS_THRESH_LOW {
    type Type = u16;
}
impl crate::I2cAddress for SYSALS_THRESH_LOW {
    const ADDRESS: u16 = 0x3c;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSALS_THRESH_LOW {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSALS_THRESH_LOW {}
#[doc = "Reader of register sysals_thresh_low"]
pub type R = crate::R<u16, SYSALS_THRESH_LOW>;
#[doc = "Writer for register sysals_thresh_low"]
pub type W = crate::W<u16, SYSALS_THRESH_LOW>;
#[doc = "Register sysals_thresh_low `reset()`'s with value 0"]
impl crate::ResetValue for SYSALS_THRESH_LOW {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
