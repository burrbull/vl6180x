#[allow(missing_docs)]
pub struct READOUT_AVERAGING_SAMPLE_PERIOD;
impl crate::Size for READOUT_AVERAGING_SAMPLE_PERIOD {
    type Type = u8;
}
impl crate::I2cAddress for READOUT_AVERAGING_SAMPLE_PERIOD {
    const ADDRESS: u16 = 0x010a;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for READOUT_AVERAGING_SAMPLE_PERIOD {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for READOUT_AVERAGING_SAMPLE_PERIOD {}
#[doc = "Reader of register readout_averaging_sample_period"]
pub type R = crate::R<u8, READOUT_AVERAGING_SAMPLE_PERIOD>;
#[doc = "Writer for register readout_averaging_sample_period"]
pub type W = crate::W<u8, READOUT_AVERAGING_SAMPLE_PERIOD>;
#[doc = "Register readout_averaging_sample_period `reset()`'s with value 0x30"]
impl crate::ResetValue for READOUT_AVERAGING_SAMPLE_PERIOD {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
impl R {}
impl W {}
