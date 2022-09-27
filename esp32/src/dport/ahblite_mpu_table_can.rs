#[doc = "Register `AHBLITE_MPU_TABLE_CAN` reader"]
pub struct R(crate::R<AHBLITE_MPU_TABLE_CAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBLITE_MPU_TABLE_CAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBLITE_MPU_TABLE_CAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBLITE_MPU_TABLE_CAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBLITE_MPU_TABLE_CAN` writer"]
pub struct W(crate::W<AHBLITE_MPU_TABLE_CAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBLITE_MPU_TABLE_CAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHBLITE_MPU_TABLE_CAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBLITE_MPU_TABLE_CAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_ACCESS_GRANT_CONFIG` reader - "]
pub type CAN_ACCESS_GRANT_CONFIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_ACCESS_GRANT_CONFIG` writer - "]
pub type CAN_ACCESS_GRANT_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AHBLITE_MPU_TABLE_CAN_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn can_access_grant_config(&self) -> CAN_ACCESS_GRANT_CONFIG_R {
        CAN_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn can_access_grant_config(&mut self) -> CAN_ACCESS_GRANT_CONFIG_W<0> {
        CAN_ACCESS_GRANT_CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahblite_mpu_table_can](index.html) module"]
pub struct AHBLITE_MPU_TABLE_CAN_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_CAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahblite_mpu_table_can::R](R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_CAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahblite_mpu_table_can::W](W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_CAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBLITE_MPU_TABLE_CAN to value 0"]
impl crate::Resettable for AHBLITE_MPU_TABLE_CAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
