#[doc = "Register `RTC_CORE_INTR_MAP` reader"]
pub struct R(crate::R<RTC_CORE_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CORE_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CORE_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CORE_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CORE_INTR_MAP` writer"]
pub struct W(crate::W<RTC_CORE_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CORE_INTR_MAP_SPEC>;
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
impl From<crate::W<RTC_CORE_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CORE_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CORE_INTR_MAP` reader - reg_core0_rtc_core_intr_map"]
pub type RTC_CORE_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_CORE_INTR_MAP` writer - reg_core0_rtc_core_intr_map"]
pub type RTC_CORE_INTR_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_CORE_INTR_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_rtc_core_intr_map"]
    #[inline(always)]
    pub fn rtc_core_intr_map(&self) -> RTC_CORE_INTR_MAP_R {
        RTC_CORE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_rtc_core_intr_map"]
    #[inline(always)]
    pub fn rtc_core_intr_map(&mut self) -> RTC_CORE_INTR_MAP_W<0> {
        RTC_CORE_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc intr map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_core_intr_map](index.html) module"]
pub struct RTC_CORE_INTR_MAP_SPEC;
impl crate::RegisterSpec for RTC_CORE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_core_intr_map::R](R) reader structure"]
impl crate::Readable for RTC_CORE_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_core_intr_map::W](W) writer structure"]
impl crate::Writable for RTC_CORE_INTR_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CORE_INTR_MAP to value 0"]
impl crate::Resettable for RTC_CORE_INTR_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
