#[allow(missing_docs)]
pub struct SYSALS_ANALOGUE_GAIN;
impl crate::Size for SYSALS_ANALOGUE_GAIN {
    type Type = u8;
}
impl crate::I2cAddress for SYSALS_ANALOGUE_GAIN {
    const ADDRESS: u16 = 0x3f;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSALS_ANALOGUE_GAIN {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSALS_ANALOGUE_GAIN {}
#[doc = "Reader of register sysals_analogue_gain"]
pub type R = crate::R<u8, SYSALS_ANALOGUE_GAIN>;
#[doc = "Writer for register sysals_analogue_gain"]
pub type W = crate::W<u8, SYSALS_ANALOGUE_GAIN>;
#[doc = "Register sysals_analogue_gain `reset()`'s with value 0x06"]
impl crate::ResetValue for SYSALS_ANALOGUE_GAIN {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "ALS analogue gain (light channel)\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIGHT_A {
    #[doc = "0: ALS Gain = 20"]
    GAIN20,
    #[doc = "1: ALS Gain = 10"]
    GAIN10,
    #[doc = "2: ALS Gain = 5.0"]
    GAIN5,
    #[doc = "3: ALS Gain = 2.5"]
    GAIN2_5,
    #[doc = "4: ALS Gain = 1.67"]
    GAIN1_67,
    #[doc = "5: ALS Gain = 1.25"]
    GAIN1_25,
    #[doc = "6: ALS Gain = 1.0"]
    GAIN1,
    #[doc = "7: ALS Gain = 40"]
    GAIN40,
}
impl From<LIGHT_A> for u8 {
    #[inline(always)]
    fn from(variant: LIGHT_A) -> Self {
        match variant {
            LIGHT_A::GAIN20 => 0,
            LIGHT_A::GAIN10 => 1,
            LIGHT_A::GAIN5 => 2,
            LIGHT_A::GAIN2_5 => 3,
            LIGHT_A::GAIN1_67 => 4,
            LIGHT_A::GAIN1_25 => 5,
            LIGHT_A::GAIN1 => 6,
            LIGHT_A::GAIN40 => 7,
        }
    }
}
#[doc = "Reader of field `light`"]
pub type LIGHT_R = crate::R<u8, LIGHT_A>;
impl LIGHT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIGHT_A {
        match self.bits {
            0 => LIGHT_A::GAIN20,
            1 => LIGHT_A::GAIN10,
            2 => LIGHT_A::GAIN5,
            3 => LIGHT_A::GAIN2_5,
            4 => LIGHT_A::GAIN1_67,
            5 => LIGHT_A::GAIN1_25,
            6 => LIGHT_A::GAIN1,
            7 => LIGHT_A::GAIN40,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GAIN20`"]
    #[inline(always)]
    pub fn is_gain20(&self) -> bool {
        *self == LIGHT_A::GAIN20
    }
    #[doc = "Checks if the value of the field is `GAIN10`"]
    #[inline(always)]
    pub fn is_gain10(&self) -> bool {
        *self == LIGHT_A::GAIN10
    }
    #[doc = "Checks if the value of the field is `GAIN5`"]
    #[inline(always)]
    pub fn is_gain5(&self) -> bool {
        *self == LIGHT_A::GAIN5
    }
    #[doc = "Checks if the value of the field is `GAIN2_5`"]
    #[inline(always)]
    pub fn is_gain2_5(&self) -> bool {
        *self == LIGHT_A::GAIN2_5
    }
    #[doc = "Checks if the value of the field is `GAIN1_67`"]
    #[inline(always)]
    pub fn is_gain1_67(&self) -> bool {
        *self == LIGHT_A::GAIN1_67
    }
    #[doc = "Checks if the value of the field is `GAIN1_25`"]
    #[inline(always)]
    pub fn is_gain1_25(&self) -> bool {
        *self == LIGHT_A::GAIN1_25
    }
    #[doc = "Checks if the value of the field is `GAIN1`"]
    #[inline(always)]
    pub fn is_gain1(&self) -> bool {
        *self == LIGHT_A::GAIN1
    }
    #[doc = "Checks if the value of the field is `GAIN40`"]
    #[inline(always)]
    pub fn is_gain40(&self) -> bool {
        *self == LIGHT_A::GAIN40
    }
}
#[doc = "Write proxy for field `light`"]
pub struct LIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> LIGHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIGHT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ALS Gain = 20"]
    #[inline(always)]
    pub fn gain20(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN20)
    }
    #[doc = "ALS Gain = 10"]
    #[inline(always)]
    pub fn gain10(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN10)
    }
    #[doc = "ALS Gain = 5.0"]
    #[inline(always)]
    pub fn gain5(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN5)
    }
    #[doc = "ALS Gain = 2.5"]
    #[inline(always)]
    pub fn gain2_5(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN2_5)
    }
    #[doc = "ALS Gain = 1.67"]
    #[inline(always)]
    pub fn gain1_67(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN1_67)
    }
    #[doc = "ALS Gain = 1.25"]
    #[inline(always)]
    pub fn gain1_25(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN1_25)
    }
    #[doc = "ALS Gain = 1.0"]
    #[inline(always)]
    pub fn gain1(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN1)
    }
    #[doc = "ALS Gain = 40"]
    #[inline(always)]
    pub fn gain40(self) -> &'a mut W {
        self.variant(LIGHT_A::GAIN40)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ALS analogue gain (light channel)"]
    #[inline(always)]
    pub fn light(&self) -> LIGHT_R {
        LIGHT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ALS analogue gain (light channel)"]
    #[inline(always)]
    pub fn light(&mut self) -> LIGHT_W {
        LIGHT_W { w: self }
    }
}
