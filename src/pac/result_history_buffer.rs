#[allow(missing_docs)]
pub struct RESULT_HISTORY_BUFFER;
impl crate::Size for RESULT_HISTORY_BUFFER {
    type Type = u16;
}
impl crate::I2cAddress for RESULT_HISTORY_BUFFER {
    const ADDRESS: u16 = 0x52;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_HISTORY_BUFFER {}
#[doc = "Reader of register result_history_buffer%s"]
pub type R = crate::R<u16, RESULT_HISTORY_BUFFER>;
#[doc = "Reader of field `range_newer`"]
pub type RANGE_NEWER_R = crate::R<u8, u8>;
#[doc = "Reader of field `range_older`"]
pub type RANGE_OLDER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn range_newer(&self) -> RANGE_NEWER_R {
        RANGE_NEWER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn range_older(&self) -> RANGE_OLDER_R {
        RANGE_OLDER_R::new((self.bits & 0xff) as u8)
    }
}
