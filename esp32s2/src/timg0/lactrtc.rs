#[doc = "Register `LACTRTC` reader"]
pub type R = crate::R<LACTRTC_SPEC>;
#[doc = "Register `LACTRTC` writer"]
pub type W = crate::W<LACTRTC_SPEC>;
#[doc = "Field `LACT_RTC_STEP_LEN` reader - Reserved."]
pub type LACT_RTC_STEP_LEN_R = crate::FieldReader<u32>;
#[doc = "Field `LACT_RTC_STEP_LEN` writer - Reserved."]
pub type LACT_RTC_STEP_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 6:31 - Reserved."]
    #[inline(always)]
    pub fn lact_rtc_step_len(&self) -> LACT_RTC_STEP_LEN_R {
        LACT_RTC_STEP_LEN_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTRTC")
            .field(
                "lact_rtc_step_len",
                &format_args!("{}", self.lact_rtc_step_len().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTRTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 6:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn lact_rtc_step_len(&mut self) -> LACT_RTC_STEP_LEN_W<LACTRTC_SPEC> {
        LACT_RTC_STEP_LEN_W::new(self, 6)
    }
}
#[doc = "LACT RTC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactrtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactrtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTRTC_SPEC;
impl crate::RegisterSpec for LACTRTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactrtc::R`](R) reader structure"]
impl crate::Readable for LACTRTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactrtc::W`](W) writer structure"]
impl crate::Writable for LACTRTC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTRTC to value 0"]
impl crate::Resettable for LACTRTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
