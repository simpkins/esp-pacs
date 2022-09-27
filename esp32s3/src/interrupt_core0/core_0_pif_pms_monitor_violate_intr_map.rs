#[doc = "Register `CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP` reader - this register used to map core0_PIF_pms_monitor_violatile interrupt to one of core0's external interrupt"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP` writer - this register used to map core0_PIF_pms_monitor_violatile interrupt to one of core0's external interrupt"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map core0_PIF_pms_monitor_violatile interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_violate_intr_map(
        &self,
    ) -> CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_R {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map core0_PIF_pms_monitor_violatile interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn core_0_pif_pms_monitor_violate_intr_map(
        &mut self,
    ) -> CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_W<0> {
        CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0_PIF_pms_monitor_violatile interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_monitor_violate_intr_map](index.html) module"]
pub struct CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_monitor_violate_intr_map::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_monitor_violate_intr_map::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP to value 0x10"]
impl crate::Resettable for CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
