#[allow(missing_docs)]
pub struct SYSRANGE_VHV_REPEATE_RATE;
impl crate::Size for SYSRANGE_VHV_REPEATE_RATE {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_VHV_REPEATE_RATE {
    const ADDRESS: u16 = 0x31;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_VHV_REPEATE_RATE {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_VHV_REPEATE_RATE {}
#[doc = "Reader of register sysrange_vhv_repeate_rate"]
pub type R = crate::R<u8, SYSRANGE_VHV_REPEATE_RATE>;
#[doc = "Writer for register sysrange_vhv_repeate_rate"]
pub type W = crate::W<u8, SYSRANGE_VHV_REPEATE_RATE>;
#[doc = "Register sysrange_vhv_repeate_rate `reset()`'s with value 0"]
impl crate::ResetValue for SYSRANGE_VHV_REPEATE_RATE {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
