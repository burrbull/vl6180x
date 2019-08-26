#[allow(missing_docs)]
pub struct IDENTIFICATION_DATE_LO;
impl crate::Size for IDENTIFICATION_DATE_LO {
    type Type = u8;
}
impl crate::I2cAddress for IDENTIFICATION_DATE_LO {
    const ADDRESS: u16 = 0x07;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for IDENTIFICATION_DATE_LO {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for IDENTIFICATION_DATE_LO {}
#[doc = "Reader of register identification_date_lo"]
pub type R = crate::R<u8, IDENTIFICATION_DATE_LO>;
#[doc = "Writer for register identification_date_lo"]
pub type W = crate::W<u8, IDENTIFICATION_DATE_LO>;
#[doc = "Register identification_date_lo `reset()`'s with value 0"]
impl crate::ResetValue for IDENTIFICATION_DATE_LO {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `phase`"]
pub type PHASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `phase`"]
pub struct PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `day`"]
pub type DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `day`"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u8) & 0x1f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Manufacturing phase identification"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - Manufacturing day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Manufacturing phase identification"]
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W {
        PHASE_W { w: self }
    }
    #[doc = "Bits 3:7 - Manufacturing day"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
}
