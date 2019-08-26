#[allow(missing_docs)]
pub struct SYSALS_START;
impl crate::Size for SYSALS_START {
    type Type = u8;
}
impl crate::I2cAddress for SYSALS_START {
    const ADDRESS: u16 = 0x38;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSALS_START {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSALS_START {}
#[doc = "Reader of register sysals_start"]
pub type R = crate::R<u8, SYSALS_START>;
#[doc = "Writer for register sysals_start"]
pub type W = crate::W<u8, SYSALS_START>;
#[doc = "Register sysals_start `reset()`'s with value 0"]
impl crate::ResetValue for SYSALS_START {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Device Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: ALS Mode Single-Shot"]
    SINGLE_SHOT,
    #[doc = "1: ALS Mode Continuous"]
    CONTINIOUS,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::SINGLE_SHOT => false,
            MODE_A::CONTINIOUS => true,
        }
    }
}
#[doc = "Reader of field `mode`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SINGLE_SHOT,
            true => MODE_A::CONTINIOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_SHOT`"]
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == MODE_A::SINGLE_SHOT
    }
    #[doc = "Checks if the value of the field is `CONTINIOUS`"]
    #[inline(always)]
    pub fn is_continious(&self) -> bool {
        *self == MODE_A::CONTINIOUS
    }
}
#[doc = "Write proxy for field `mode`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ALS Mode Single-Shot"]
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut W {
        self.variant(MODE_A::SINGLE_SHOT)
    }
    #[doc = "ALS Mode Continuous"]
    #[inline(always)]
    pub fn continious(self) -> &'a mut W {
        self.variant(MODE_A::CONTINIOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Start/Stop trigger based on current mode and system configuration of device_ready. FW clears register automatically\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTSTOP_A {
    #[doc = "1: In single-shot mode starts a single measurement. In continuous mode will either start continuous operation (if stopped) or halt continuous operation (if started)"]
    ENABLE,
}
impl From<STARTSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: STARTSTOP_A) -> Self {
        match variant {
            STARTSTOP_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `startstop`"]
pub type STARTSTOP_R = crate::R<bool, STARTSTOP_A>;
impl STARTSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, STARTSTOP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(STARTSTOP_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STARTSTOP_A::ENABLE
    }
}
#[doc = "Write proxy for field `startstop`"]
pub struct STARTSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In single-shot mode starts a single measurement. In continuous mode will either start continuous operation (if stopped) or halt continuous operation (if started)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STARTSTOP_A::ENABLE)
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
    #[doc = "Bit 1 - Device Mode select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Start/Stop trigger based on current mode and system configuration of device_ready. FW clears register automatically"]
    #[inline(always)]
    pub fn startstop(&self) -> STARTSTOP_R {
        STARTSTOP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Device Mode select"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 0 - Start/Stop trigger based on current mode and system configuration of device_ready. FW clears register automatically"]
    #[inline(always)]
    pub fn startstop(&mut self) -> STARTSTOP_W {
        STARTSTOP_W { w: self }
    }
}
