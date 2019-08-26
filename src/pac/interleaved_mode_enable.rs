#[allow(missing_docs)]
pub struct INTERLEAVED_MODE_ENABLE;
impl crate::Size for INTERLEAVED_MODE_ENABLE {
    type Type = u8;
}
impl crate::I2cAddress for INTERLEAVED_MODE_ENABLE {
    const ADDRESS: u16 = 0x02a3;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for INTERLEAVED_MODE_ENABLE {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for INTERLEAVED_MODE_ENABLE {}
#[doc = "Reader of register interleaved_mode_enable"]
pub type R = crate::R<u8, INTERLEAVED_MODE_ENABLE>;
#[doc = "Writer for register interleaved_mode_enable"]
pub type W = crate::W<u8, INTERLEAVED_MODE_ENABLE>;
#[doc = "Register interleaved_mode_enable `reset()`'s with value 0"]
impl crate::ResetValue for INTERLEAVED_MODE_ENABLE {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
