#[allow(missing_docs)]
pub struct IDENTIFICATION_DATE_HI;
impl crate::Size for IDENTIFICATION_DATE_HI {
    type Type = u8;
}
impl crate::I2cAddress for IDENTIFICATION_DATE_HI {
    const ADDRESS: u16 = 0x06;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for IDENTIFICATION_DATE_HI {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for IDENTIFICATION_DATE_HI {}
#[doc = "Reader of register identification_date_hi"]
pub type R = crate::R<u8, IDENTIFICATION_DATE_HI>;
#[doc = "Writer for register identification_date_hi"]
pub type W = crate::W<u8, IDENTIFICATION_DATE_HI>;
#[doc = "Register identification_date_hi `reset()`'s with value 0"]
impl crate::ResetValue for IDENTIFICATION_DATE_HI {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `month`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `month`"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `year`"]
pub type YEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `year`"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Manufacturing month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Last digit of manufacturing year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Manufacturing month"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Bits 4:7 - Last digit of manufacturing year"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
}
