#[doc = "Register `INT_RAW_RTC` reader"]
pub struct R(crate::R<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW_RTC` writer"]
pub struct W(crate::W<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_RTC_SPEC>;
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
impl From<crate::W<INT_RAW_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_RAW` reader - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLP_WAKEUP_INT_RAW` writer - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_RAW_RTC_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT_INT_RAW` reader - sleep reject interrupt raw"]
pub type SLP_REJECT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLP_REJECT_INT_RAW` writer - sleep reject interrupt raw"]
pub type SLP_REJECT_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_RAW_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_WDT_INT_RAW` reader - RTC WDT interrupt raw"]
pub type RTC_WDT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RTC_WDT_INT_RAW` writer - RTC WDT interrupt raw"]
pub type RTC_WDT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_BROWN_OUT_INT_RAW` reader - brown out interrupt raw"]
pub type RTC_BROWN_OUT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RTC_BROWN_OUT_INT_RAW` writer - brown out interrupt raw"]
pub type RTC_BROWN_OUT_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_RAW_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_MAIN_TIMER_INT_RAW` reader - RTC main timer interrupt raw"]
pub type RTC_MAIN_TIMER_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_TIMER_INT_RAW` writer - RTC main timer interrupt raw"]
pub type RTC_MAIN_TIMER_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_RAW_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_SWD_INT_RAW` reader - super watch dog interrupt raw"]
pub type RTC_SWD_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SWD_INT_RAW` writer - super watch dog interrupt raw"]
pub type RTC_SWD_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_BBPLL_CAL_INT_RAW` reader - Need add desc"]
pub type RTC_BBPLL_CAL_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RTC_BBPLL_CAL_INT_RAW` writer - Need add desc"]
pub type RTC_BBPLL_CAL_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_RAW_RTC_SPEC, bool, O>;
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
    pub fn rtc_wdt_int_raw(&self) -> RTC_WDT_INT_RAW_R {
        RTC_WDT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn rtc_brown_out_int_raw(&self) -> RTC_BROWN_OUT_INT_RAW_R {
        RTC_BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn rtc_main_timer_int_raw(&self) -> RTC_MAIN_TIMER_INT_RAW_R {
        RTC_MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn rtc_swd_int_raw(&self) -> RTC_SWD_INT_RAW_R {
        RTC_SWD_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_raw(&self) -> RTC_BBPLL_CAL_INT_RAW_R {
        RTC_BBPLL_CAL_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&mut self) -> SLP_WAKEUP_INT_RAW_W<0> {
        SLP_WAKEUP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&mut self) -> SLP_REJECT_INT_RAW_W<1> {
        SLP_REJECT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn rtc_wdt_int_raw(&mut self) -> RTC_WDT_INT_RAW_W<3> {
        RTC_WDT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn rtc_brown_out_int_raw(&mut self) -> RTC_BROWN_OUT_INT_RAW_W<9> {
        RTC_BROWN_OUT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn rtc_main_timer_int_raw(&mut self) -> RTC_MAIN_TIMER_INT_RAW_W<10> {
        RTC_MAIN_TIMER_INT_RAW_W::new(self)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn rtc_swd_int_raw(&mut self) -> RTC_SWD_INT_RAW_W<15> {
        RTC_SWD_INT_RAW_W::new(self)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_raw(&mut self) -> RTC_BBPLL_CAL_INT_RAW_W<20> {
        RTC_BBPLL_CAL_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_rtc](index.html) module"]
pub struct INT_RAW_RTC_SPEC;
impl crate::RegisterSpec for INT_RAW_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_rtc::R](R) reader structure"]
impl crate::Readable for INT_RAW_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw_rtc::W](W) writer structure"]
impl crate::Writable for INT_RAW_RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RAW_RTC to value 0"]
impl crate::Resettable for INT_RAW_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
