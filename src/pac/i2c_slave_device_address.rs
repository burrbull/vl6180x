#[allow(missing_docs)]
pub struct I2C_SLAVE_DEVICE_ADDRESS;
impl crate::Size for I2C_SLAVE_DEVICE_ADDRESS {
    type Type = u8;
}
impl crate::I2cAddress for I2C_SLAVE_DEVICE_ADDRESS {
    const ADDRESS: u16 = 0x0212;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for I2C_SLAVE_DEVICE_ADDRESS {}
#[doc = "`write(|w| ..)` method takes [W](W) writer structure"]
impl crate::Writable for I2C_SLAVE_DEVICE_ADDRESS {}
#[doc = "Reader of register i2c_slave_device_address"]
pub type R = crate::R<u8, I2C_SLAVE_DEVICE_ADDRESS>;
#[doc = "Writer for register i2c_slave_device_address"]
pub type W = crate::W<u8, I2C_SLAVE_DEVICE_ADDRESS>;
#[doc = "Register i2c_slave_device_address `reset()`'s with value 0x29"]
impl crate::ResetValue for I2C_SLAVE_DEVICE_ADDRESS {
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x29
    }
}
#[doc = "Reader of field `super_address`"]
pub type SUPER_ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `super_address`"]
pub struct SUPER_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPER_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn super_address(&self) -> SUPER_ADDRESS_R {
        SUPER_ADDRESS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn super_address(&mut self) -> SUPER_ADDRESS_W {
        SUPER_ADDRESS_W { w: self }
    }
}
