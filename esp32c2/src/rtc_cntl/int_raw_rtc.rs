#[doc = "Register `INT_RAW_RTC` reader"]
pub type R = crate::R<INT_RAW_RTC_SPEC>;
#[doc = "Register `INT_RAW_RTC` writer"]
pub type W = crate::W<INT_RAW_RTC_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_RAW` reader - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP_INT_RAW` writer - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_RAW` reader - sleep reject interrupt raw"]
pub type SLP_REJECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_RAW` writer - sleep reject interrupt raw"]
pub type SLP_REJECT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_RAW` reader - RTC WDT interrupt raw"]
pub type WDT_INT_RAW_R = crate::BitReader;
#[doc = "Field `WDT_INT_RAW` writer - RTC WDT interrupt raw"]
pub type WDT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_RAW` reader - brown out interrupt raw"]
pub type BROWN_OUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_RAW` writer - brown out interrupt raw"]
pub type BROWN_OUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_RAW` reader - RTC main timer interrupt raw"]
pub type MAIN_TIMER_INT_RAW_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_RAW` writer - RTC main timer interrupt raw"]
pub type MAIN_TIMER_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_INT_RAW` reader - super watch dog interrupt raw"]
pub type SWD_INT_RAW_R = crate::BitReader;
#[doc = "Field `SWD_INT_RAW` writer - super watch dog interrupt raw"]
pub type SWD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL_INT_RAW` reader - Need add desc"]
pub type BBPLL_CAL_INT_RAW_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL_INT_RAW` writer - Need add desc"]
pub type BBPLL_CAL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&self) -> SLP_WAKEUP_INT_RAW_R {
        SLP_WAKEUP_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&self) -> SLP_REJECT_INT_RAW_R {
        SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out_int_raw(&self) -> BROWN_OUT_INT_RAW_R {
        BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer_int_raw(&self) -> MAIN_TIMER_INT_RAW_R {
        MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn swd_int_raw(&self) -> SWD_INT_RAW_R {
        SWD_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn bbpll_cal_int_raw(&self) -> BBPLL_CAL_INT_RAW_R {
        BBPLL_CAL_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW_RTC")
            .field(
                "slp_wakeup_int_raw",
                &format_args!("{}", self.slp_wakeup_int_raw().bit()),
            )
            .field(
                "slp_reject_int_raw",
                &format_args!("{}", self.slp_reject_int_raw().bit()),
            )
            .field("wdt_int_raw", &format_args!("{}", self.wdt_int_raw().bit()))
            .field(
                "brown_out_int_raw",
                &format_args!("{}", self.brown_out_int_raw().bit()),
            )
            .field(
                "main_timer_int_raw",
                &format_args!("{}", self.main_timer_int_raw().bit()),
            )
            .field("swd_int_raw", &format_args!("{}", self.swd_int_raw().bit()))
            .field(
                "bbpll_cal_int_raw",
                &format_args!("{}", self.bbpll_cal_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_raw(&mut self) -> SLP_WAKEUP_INT_RAW_W<INT_RAW_RTC_SPEC> {
        SLP_WAKEUP_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_raw(&mut self) -> SLP_REJECT_INT_RAW_W<INT_RAW_RTC_SPEC> {
        SLP_REJECT_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_raw(&mut self) -> WDT_INT_RAW_W<INT_RAW_RTC_SPEC> {
        WDT_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_raw(&mut self) -> BROWN_OUT_INT_RAW_W<INT_RAW_RTC_SPEC> {
        BROWN_OUT_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_raw(&mut self) -> MAIN_TIMER_INT_RAW_W<INT_RAW_RTC_SPEC> {
        MAIN_TIMER_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_raw(&mut self) -> SWD_INT_RAW_W<INT_RAW_RTC_SPEC> {
        SWD_INT_RAW_W::new(self, 15)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_int_raw(&mut self) -> BBPLL_CAL_INT_RAW_W<INT_RAW_RTC_SPEC> {
        BBPLL_CAL_INT_RAW_W::new(self, 20)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw_rtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw_rtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_RTC_SPEC;
impl crate::RegisterSpec for INT_RAW_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw_rtc::R`](R) reader structure"]
impl crate::Readable for INT_RAW_RTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw_rtc::W`](W) writer structure"]
impl crate::Writable for INT_RAW_RTC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW_RTC to value 0"]
impl crate::Resettable for INT_RAW_RTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
