#[allow(missing_docs)]
pub struct SYSRANGE_EARLY_CONVERGENCE_ESTIMATE;
impl crate::Size for SYSRANGE_EARLY_CONVERGENCE_ESTIMATE {
    type Type = u16;
}
impl crate::I2cAddress for SYSRANGE_EARLY_CONVERGENCE_ESTIMATE {
    const ADDRESS: u16 = 0x22;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_EARLY_CONVERGENCE_ESTIMATE {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_EARLY_CONVERGENCE_ESTIMATE {}
#[doc = "Reader of register sysrange_early_convergence_estimate"]
pub type R = crate::R<u16, SYSRANGE_EARLY_CONVERGENCE_ESTIMATE>;
#[doc = "Writer for register sysrange_early_convergence_estimate"]
pub type W = crate::W<u16, SYSRANGE_EARLY_CONVERGENCE_ESTIMATE>;
#[doc = "Register sysrange_early_convergence_estimate `reset()`'s with value 0"]
impl crate::ResetValue for SYSRANGE_EARLY_CONVERGENCE_ESTIMATE {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
