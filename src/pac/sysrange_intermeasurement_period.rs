#[allow(missing_docs)]
pub struct SYSRANGE_INTERMEASUREMENT_PERIOD;
impl crate::Size for SYSRANGE_INTERMEASUREMENT_PERIOD {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_INTERMEASUREMENT_PERIOD {
    const ADDRESS: u16 = 0x1b;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_INTERMEASUREMENT_PERIOD {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_INTERMEASUREMENT_PERIOD {}
#[doc = "Reader of register sysrange_intermeasurement_period"]
pub type R = crate::R<u8, SYSRANGE_INTERMEASUREMENT_PERIOD>;
#[doc = "Writer for register sysrange_intermeasurement_period"]
pub type W = crate::W<u8, SYSRANGE_INTERMEASUREMENT_PERIOD>;
#[doc = "Register sysrange_intermeasurement_period `reset()`'s with value 0xff"]
impl crate::ResetValue for SYSRANGE_INTERMEASUREMENT_PERIOD {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
impl R {}
impl W {}
