#[allow(missing_docs)]
pub struct SYSRANGE_THRESH_LOW;
impl crate::Size for SYSRANGE_THRESH_LOW {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_THRESH_LOW {
    const ADDRESS: u16 = 0x1a;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_THRESH_LOW {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_THRESH_LOW {}
#[doc = "Reader of register sysrange_thresh_low"]
pub type R = crate::R<u8, SYSRANGE_THRESH_LOW>;
#[doc = "Writer for register sysrange_thresh_low"]
pub type W = crate::W<u8, SYSRANGE_THRESH_LOW>;
#[doc = "Register sysrange_thresh_low `reset()`'s with value 0"]
impl crate::ResetValue for SYSRANGE_THRESH_LOW {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
