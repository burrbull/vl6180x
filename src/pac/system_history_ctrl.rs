#[allow(missing_docs)]
pub struct SYSTEM_HISTORY_CTRL;
impl crate::Size for SYSTEM_HISTORY_CTRL {
    type Type = u8;
}
impl crate::I2cAddress for SYSTEM_HISTORY_CTRL {
    const ADDRESS: u16 = 0x12;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSTEM_HISTORY_CTRL {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSTEM_HISTORY_CTRL {}
#[doc = "Reader of register system_history_ctrl"]
pub type R = crate::R<u8, SYSTEM_HISTORY_CTRL>;
#[doc = "Writer for register system_history_ctrl"]
pub type W = crate::W<u8, SYSTEM_HISTORY_CTRL>;
#[doc = "Register system_history_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for SYSTEM_HISTORY_CTRL {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable History buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLED,
    #[doc = "1: `1`"]
    ENABLED,
}
impl From<BUFFER_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_ENABLE_A) -> Self {
        match variant {
            BUFFER_ENABLE_A::DISABLED => false,
            BUFFER_ENABLE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `buffer_enable`"]
pub type BUFFER_ENABLE_R = crate::R<bool, BUFFER_ENABLE_A>;
impl BUFFER_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFER_ENABLE_A {
        match self.bits {
            false => BUFFER_ENABLE_A::DISABLED,
            true => BUFFER_ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BUFFER_ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BUFFER_ENABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `buffer_enable`"]
pub struct BUFFER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFFER_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BUFFER_ENABLE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BUFFER_ENABLE_A::ENABLED)
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
#[doc = "Select mode buffer results for:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_MODE_A {
    #[doc = "0: `0`"]
    RANGING,
    #[doc = "1: `1`"]
    ALS,
}
impl From<BUFFER_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_MODE_A) -> Self {
        match variant {
            BUFFER_MODE_A::RANGING => false,
            BUFFER_MODE_A::ALS => true,
        }
    }
}
#[doc = "Reader of field `buffer_mode`"]
pub type BUFFER_MODE_R = crate::R<bool, BUFFER_MODE_A>;
impl BUFFER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFER_MODE_A {
        match self.bits {
            false => BUFFER_MODE_A::RANGING,
            true => BUFFER_MODE_A::ALS,
        }
    }
    #[doc = "Checks if the value of the field is `RANGING`"]
    #[inline(always)]
    pub fn is_ranging(&self) -> bool {
        *self == BUFFER_MODE_A::RANGING
    }
    #[doc = "Checks if the value of the field is `ALS`"]
    #[inline(always)]
    pub fn is_als(&self) -> bool {
        *self == BUFFER_MODE_A::ALS
    }
}
#[doc = "Write proxy for field `buffer_mode`"]
pub struct BUFFER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFFER_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ranging(self) -> &'a mut W {
        self.variant(BUFFER_MODE_A::RANGING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn als(self) -> &'a mut W {
        self.variant(BUFFER_MODE_A::ALS)
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
#[doc = "User-command to clear history (FW will auto-clear this bit when clear has completed)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_CLEAR_A {
    #[doc = "0: `0`"]
    DISABLED,
    #[doc = "1: `1`"]
    CLEAR_ALL,
}
impl From<BUFFER_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_CLEAR_A) -> Self {
        match variant {
            BUFFER_CLEAR_A::DISABLED => false,
            BUFFER_CLEAR_A::CLEAR_ALL => true,
        }
    }
}
#[doc = "Reader of field `buffer_clear`"]
pub type BUFFER_CLEAR_R = crate::R<bool, BUFFER_CLEAR_A>;
impl BUFFER_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFER_CLEAR_A {
        match self.bits {
            false => BUFFER_CLEAR_A::DISABLED,
            true => BUFFER_CLEAR_A::CLEAR_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BUFFER_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `CLEAR_ALL`"]
    #[inline(always)]
    pub fn is_clear_all(&self) -> bool {
        *self == BUFFER_CLEAR_A::CLEAR_ALL
    }
}
#[doc = "Write proxy for field `buffer_clear`"]
pub struct BUFFER_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFFER_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BUFFER_CLEAR_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear_all(self) -> &'a mut W {
        self.variant(BUFFER_CLEAR_A::CLEAR_ALL)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable History buffering"]
    #[inline(always)]
    pub fn buffer_enable(&self) -> BUFFER_ENABLE_R {
        BUFFER_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select mode buffer results for:"]
    #[inline(always)]
    pub fn buffer_mode(&self) -> BUFFER_MODE_R {
        BUFFER_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - User-command to clear history (FW will auto-clear this bit when clear has completed)"]
    #[inline(always)]
    pub fn buffer_clear(&self) -> BUFFER_CLEAR_R {
        BUFFER_CLEAR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable History buffering"]
    #[inline(always)]
    pub fn buffer_enable(&mut self) -> BUFFER_ENABLE_W {
        BUFFER_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Select mode buffer results for:"]
    #[inline(always)]
    pub fn buffer_mode(&mut self) -> BUFFER_MODE_W {
        BUFFER_MODE_W { w: self }
    }
    #[doc = "Bit 2 - User-command to clear history (FW will auto-clear this bit when clear has completed)"]
    #[inline(always)]
    pub fn buffer_clear(&mut self) -> BUFFER_CLEAR_W {
        BUFFER_CLEAR_W { w: self }
    }
}
