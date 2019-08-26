#[allow(missing_docs)]
pub struct SYSALS_INTERMEASUREMENT_PERIOD;
impl crate::Size for SYSALS_INTERMEASUREMENT_PERIOD {
    type Type = u8;
}
impl crate::I2cAddress for SYSALS_INTERMEASUREMENT_PERIOD {
    const ADDRESS: u16 = 0x3e;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSALS_INTERMEASUREMENT_PERIOD {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSALS_INTERMEASUREMENT_PERIOD {}
#[doc = "Reader of register sysals_intermeasurement_period"]
pub type R = crate::R<u8, SYSALS_INTERMEASUREMENT_PERIOD>;
#[doc = "Writer for register sysals_intermeasurement_period"]
pub type W = crate::W<u8, SYSALS_INTERMEASUREMENT_PERIOD>;
#[doc = "Register sysals_intermeasurement_period `reset()`'s with value 0xff"]
impl crate::ResetValue for SYSALS_INTERMEASUREMENT_PERIOD {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
impl R {}
impl W {}
