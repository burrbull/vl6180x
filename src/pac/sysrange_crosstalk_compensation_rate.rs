#[allow(missing_docs)]
pub struct SYSRANGE_CROSSTALK_COMPENSATION_RATE;
impl crate::Size for SYSRANGE_CROSSTALK_COMPENSATION_RATE {
    type Type = u16;
}
impl crate::I2cAddress for SYSRANGE_CROSSTALK_COMPENSATION_RATE {
    const ADDRESS: u16 = 0x1e;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_CROSSTALK_COMPENSATION_RATE {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_CROSSTALK_COMPENSATION_RATE {}
#[doc = "Reader of register sysrange_crosstalk_compensation_rate"]
pub type R = crate::R<u16, SYSRANGE_CROSSTALK_COMPENSATION_RATE>;
#[doc = "Writer for register sysrange_crosstalk_compensation_rate"]
pub type W = crate::W<u16, SYSRANGE_CROSSTALK_COMPENSATION_RATE>;
#[doc = "Register sysrange_crosstalk_compensation_rate `reset()`'s with value 0"]
impl crate::ResetValue for SYSRANGE_CROSSTALK_COMPENSATION_RATE {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
