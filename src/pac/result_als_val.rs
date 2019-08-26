#[allow(missing_docs)]
pub struct RESULT_ALS_VAL;
impl crate::Size for RESULT_ALS_VAL {
    type Type = u16;
}
impl crate::I2cAddress for RESULT_ALS_VAL {
    const ADDRESS: u16 = 0x50;
}
#[doc = "`read()` method returns [R](R) reader structure"]
impl crate::Readable for RESULT_ALS_VAL {}
#[doc = "Reader of register result_als_val"]
pub type R = crate::R<u16, RESULT_ALS_VAL>;
impl R {}
