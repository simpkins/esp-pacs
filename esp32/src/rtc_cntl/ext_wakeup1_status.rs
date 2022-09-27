#[doc = "Register `EXT_WAKEUP1_STATUS` reader"]
pub struct R(crate::R<EXT_WAKEUP1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXT_WAKEUP1_STATUS` reader - ext wakeup1 status"]
pub type EXT_WAKEUP1_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - ext wakeup1 status"]
    #[inline(always)]
    pub fn ext_wakeup1_status(&self) -> EXT_WAKEUP1_STATUS_R {
        EXT_WAKEUP1_STATUS_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup1_status](index.html) module"]
pub struct EXT_WAKEUP1_STATUS_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup1_status::R](R) reader structure"]
impl crate::Readable for EXT_WAKEUP1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXT_WAKEUP1_STATUS to value 0"]
impl crate::Resettable for EXT_WAKEUP1_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
