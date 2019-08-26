#[allow(missing_docs)]
pub struct SYSTEM_GROUPED_PARAMETER_HOLD;
impl crate::Size for SYSTEM_GROUPED_PARAMETER_HOLD {
    type Type = u8;
}
impl crate::I2cAddress for SYSTEM_GROUPED_PARAMETER_HOLD {
    const ADDRESS: u16 = 0x17;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSTEM_GROUPED_PARAMETER_HOLD {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSTEM_GROUPED_PARAMETER_HOLD {}
#[doc = "Reader of register system_grouped_parameter_hold"]
pub type R = crate::R<u8, SYSTEM_GROUPED_PARAMETER_HOLD>;
#[doc = "Writer for register system_grouped_parameter_hold"]
pub type W = crate::W<u8, SYSTEM_GROUPED_PARAMETER_HOLD>;
#[doc = "Register system_grouped_parameter_hold `reset()`'s with value 0"]
impl crate::ResetValue for SYSTEM_GROUPED_PARAMETER_HOLD {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GROUPED_PARAMETER_HOLD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALUE_A {
    #[doc = "0: Data is stable - FW is safe to copy"]
    STABLE,
    #[doc = "1: Data being updated - FW not safe to copy"]
    UPDATED,
}
impl From<VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: VALUE_A) -> Self {
        match variant {
            VALUE_A::STABLE => false,
            VALUE_A::UPDATED => true,
        }
    }
}
#[doc = "Reader of field `value`"]
pub type VALUE_R = crate::R<bool, VALUE_A>;
impl VALUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALUE_A {
        match self.bits {
            false => VALUE_A::STABLE,
            true => VALUE_A::UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == VALUE_A::STABLE
    }
    #[doc = "Checks if the value of the field is `UPDATED`"]
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == VALUE_A::UPDATED
    }
}
#[doc = "Write proxy for field `value`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALUE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is stable - FW is safe to copy"]
    #[inline(always)]
    pub fn stable(self) -> &'a mut W {
        self.variant(VALUE_A::STABLE)
    }
    #[doc = "Data being updated - FW not safe to copy"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(VALUE_A::UPDATED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GROUPED_PARAMETER_HOLD"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GROUPED_PARAMETER_HOLD"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
