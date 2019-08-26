#[allow(missing_docs)]
pub struct SYSRANGE_VHV_RECALIBRATE;
impl crate::Size for SYSRANGE_VHV_RECALIBRATE {
    type Type = u8;
}
impl crate::I2cAddress for SYSRANGE_VHV_RECALIBRATE {
    const ADDRESS: u16 = 0x2e;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for SYSRANGE_VHV_RECALIBRATE {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for SYSRANGE_VHV_RECALIBRATE {}
#[doc = "Reader of register sysrange_vhv_recalibrate"]
pub type R = crate::R<u8, SYSRANGE_VHV_RECALIBRATE>;
#[doc = "Writer for register sysrange_vhv_recalibrate"]
pub type W = crate::W<u8, SYSRANGE_VHV_RECALIBRATE>;
#[doc = "Register sysrange_vhv_recalibrate `reset()`'s with value 0"]
impl crate::ResetValue for SYSRANGE_VHV_RECALIBRATE {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FW controlled status bit showing when FW has completed auto-vhv process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VHV_STATUS_A {
    #[doc = "0: FW has finished autoVHV operation"]
    FINISHED,
    #[doc = "1: During autoVHV operation"]
    OPERATION,
}
impl From<VHV_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: VHV_STATUS_A) -> Self {
        match variant {
            VHV_STATUS_A::FINISHED => false,
            VHV_STATUS_A::OPERATION => true,
        }
    }
}
#[doc = "Reader of field `vhv_status`"]
pub type VHV_STATUS_R = crate::R<bool, VHV_STATUS_A>;
impl VHV_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VHV_STATUS_A {
        match self.bits {
            false => VHV_STATUS_A::FINISHED,
            true => VHV_STATUS_A::OPERATION,
        }
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == VHV_STATUS_A::FINISHED
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == VHV_STATUS_A::OPERATION
    }
}
#[doc = "Write proxy for field `vhv_status`"]
pub struct VHV_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> VHV_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VHV_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FW has finished autoVHV operation"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(VHV_STATUS_A::FINISHED)
    }
    #[doc = "During autoVHV operation"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(VHV_STATUS_A::OPERATION)
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
#[doc = "User-Controlled enable bit to force FW to carry out recalibration of the VHV setting for sensor array. FW clears bit after operation carried out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VHV_RECALIBRATE_A {
    #[doc = "1: Manual trigger for VHV recalibration. Can only be called when ALS and ranging are in STOP mode"]
    START,
}
impl From<VHV_RECALIBRATE_A> for bool {
    #[inline(always)]
    fn from(variant: VHV_RECALIBRATE_A) -> Self {
        match variant {
            VHV_RECALIBRATE_A::START => true,
        }
    }
}
#[doc = "Reader of field `vhv_recalibrate`"]
pub type VHV_RECALIBRATE_R = crate::R<bool, VHV_RECALIBRATE_A>;
impl VHV_RECALIBRATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VHV_RECALIBRATE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(VHV_RECALIBRATE_A::START),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == VHV_RECALIBRATE_A::START
    }
}
#[doc = "Write proxy for field `vhv_recalibrate`"]
pub struct VHV_RECALIBRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> VHV_RECALIBRATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VHV_RECALIBRATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Manual trigger for VHV recalibration. Can only be called when ALS and ranging are in STOP mode"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(VHV_RECALIBRATE_A::START)
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
    #[doc = "Bit 1 - FW controlled status bit showing when FW has completed auto-vhv process"]
    #[inline(always)]
    pub fn vhv_status(&self) -> VHV_STATUS_R {
        VHV_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - User-Controlled enable bit to force FW to carry out recalibration of the VHV setting for sensor array. FW clears bit after operation carried out"]
    #[inline(always)]
    pub fn vhv_recalibrate(&self) -> VHV_RECALIBRATE_R {
        VHV_RECALIBRATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FW controlled status bit showing when FW has completed auto-vhv process"]
    #[inline(always)]
    pub fn vhv_status(&mut self) -> VHV_STATUS_W {
        VHV_STATUS_W { w: self }
    }
    #[doc = "Bit 0 - User-Controlled enable bit to force FW to carry out recalibration of the VHV setting for sensor array. FW clears bit after operation carried out"]
    #[inline(always)]
    pub fn vhv_recalibrate(&mut self) -> VHV_RECALIBRATE_W {
        VHV_RECALIBRATE_W { w: self }
    }
}
