#[allow(missing_docs)]
pub struct SYSRANGE_CROSSTALK_VALID_HEIGHT;
impl crate::Size for SYSRANGE_CROSSTALK_VALID_HEIGHT {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_CROSSTALK_VALID_HEIGHT {
    const ADDRESS: u16 = 0x21;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_CROSSTALK_VALID_HEIGHT {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_CROSSTALK_VALID_HEIGHT {}
#[doc = "Reader of register sysrange_crosstalk_valid_height"]
pub type R = crate::R<u8, SYSRANGE_CROSSTALK_VALID_HEIGHT>;
#[doc = "Writer for register sysrange_crosstalk_valid_height"]
pub type W = crate::W<u8, SYSRANGE_CROSSTALK_VALID_HEIGHT>;
#[doc = "Register sysrange_crosstalk_valid_height `reset()`'s with value 0x14"]
impl crate::ResetValue for SYSRANGE_CROSSTALK_VALID_HEIGHT {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x14
    }
}
impl R {}
impl W {}
