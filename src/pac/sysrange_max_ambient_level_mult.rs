#[allow(missing_docs)]
pub struct SYSRANGE_MAX_AMBIENT_LEVEL_MULT;
impl crate::Size for SYSRANGE_MAX_AMBIENT_LEVEL_MULT {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_MAX_AMBIENT_LEVEL_MULT {
    const ADDRESS: u16 = 0x2c;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_MAX_AMBIENT_LEVEL_MULT {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_MAX_AMBIENT_LEVEL_MULT {}
#[doc = "Reader of register sysrange_max_ambient_level_mult"]
pub type R = crate::R<u8, SYSRANGE_MAX_AMBIENT_LEVEL_MULT>;
#[doc = "Writer for register sysrange_max_ambient_level_mult"]
pub type W = crate::W<u8, SYSRANGE_MAX_AMBIENT_LEVEL_MULT>;
#[doc = "Register sysrange_max_ambient_level_mult `reset()`'s with value 0xa0"]
impl crate::ResetValue for SYSRANGE_MAX_AMBIENT_LEVEL_MULT {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa0
    }
}
impl R {}
impl W {}
