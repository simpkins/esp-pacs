#[doc = "Register `INT_ENA_RTC_W1TC` writer"]
pub type W = crate::W<INT_ENA_RTC_W1TC_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TC` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TC` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IDLE_INT_ENA_W1TC` writer - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA_W1TC` writer - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SCAN_DONE_INT_ENA_W1TC` writer - enable touch scan done interrupt"]
pub type TOUCH_SCAN_DONE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_INT_ENA_W1TC` writer - enable ULP-coprocessor interrupt"]
pub type ULP_CP_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_DONE_INT_ENA_W1TC` writer - enable touch done interrupt"]
pub type TOUCH_DONE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_ACTIVE_INT_ENA_W1TC` writer - enable touch active interrupt"]
pub type TOUCH_ACTIVE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_INACTIVE_INT_ENA_W1TC` writer - enable touch inactive interrupt"]
pub type TOUCH_INACTIVE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_ENA_W1TC` writer - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TC` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC1_INT_ENA_W1TC` writer - enable saradc1 interrupt"]
pub type SARADC1_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_INT_ENA_W1TC` writer - enable tsens interrupt"]
pub type TSENS_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_INT_ENA_W1TC` writer - enable riscV cocpu interrupt"]
pub type COCPU_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC2_INT_ENA_W1TC` writer - enable saradc2 interrupt"]
pub type SARADC2_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_INT_ENA_W1TC` writer - enable super watch dog interrupt"]
pub type SWD_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL32K_DEAD_INT_ENA_W1TC` writer - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_TRAP_INT_ENA_W1TC` writer - enable cocpu trap interrupt"]
pub type COCPU_TRAP_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_TIMEOUT_INT_ENA_W1TC` writer - enable touch timeout interrupt"]
pub type TOUCH_TIMEOUT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET_INT_ENA_W1TC` writer - enbale gitch det interrupt"]
pub type GLITCH_DET_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC` writer - enbale touch approach_loop done interrupt"]
pub type TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena_w1tc(&mut self) -> SLP_WAKEUP_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_WAKEUP_INT_ENA_W1TC_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena_w1tc(&mut self) -> SLP_REJECT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_REJECT_INT_ENA_W1TC_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_ena_w1tc(&mut self) -> SDIO_IDLE_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SDIO_IDLE_INT_ENA_W1TC_W::new(self, 2)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena_w1tc(&mut self) -> WDT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        WDT_INT_ENA_W1TC_W::new(self, 3)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_done_int_ena_w1tc(
        &mut self,
    ) -> TOUCH_SCAN_DONE_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_SCAN_DONE_INT_ENA_W1TC_W::new(self, 4)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_int_ena_w1tc(&mut self) -> ULP_CP_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        ULP_CP_INT_ENA_W1TC_W::new(self, 5)
    }
    #[doc = "Bit 6 - enable touch done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_done_int_ena_w1tc(&mut self) -> TOUCH_DONE_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_DONE_INT_ENA_W1TC_W::new(self, 6)
    }
    #[doc = "Bit 7 - enable touch active interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_active_int_ena_w1tc(
        &mut self,
    ) -> TOUCH_ACTIVE_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_ACTIVE_INT_ENA_W1TC_W::new(self, 7)
    }
    #[doc = "Bit 8 - enable touch inactive interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive_int_ena_w1tc(
        &mut self,
    ) -> TOUCH_INACTIVE_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_INACTIVE_INT_ENA_W1TC_W::new(self, 8)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena_w1tc(&mut self) -> BROWN_OUT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        BROWN_OUT_INT_ENA_W1TC_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena_w1tc(&mut self) -> MAIN_TIMER_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        MAIN_TIMER_INT_ENA_W1TC_W::new(self, 10)
    }
    #[doc = "Bit 11 - enable saradc1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn saradc1_int_ena_w1tc(&mut self) -> SARADC1_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SARADC1_INT_ENA_W1TC_W::new(self, 11)
    }
    #[doc = "Bit 12 - enable tsens interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_int_ena_w1tc(&mut self) -> TSENS_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        TSENS_INT_ENA_W1TC_W::new(self, 12)
    }
    #[doc = "Bit 13 - enable riscV cocpu interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_int_ena_w1tc(&mut self) -> COCPU_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        COCPU_INT_ENA_W1TC_W::new(self, 13)
    }
    #[doc = "Bit 14 - enable saradc2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn saradc2_int_ena_w1tc(&mut self) -> SARADC2_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SARADC2_INT_ENA_W1TC_W::new(self, 14)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_ena_w1tc(&mut self) -> SWD_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SWD_INT_ENA_W1TC_W::new(self, 15)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_ena_w1tc(
        &mut self,
    ) -> XTAL32K_DEAD_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        XTAL32K_DEAD_INT_ENA_W1TC_W::new(self, 16)
    }
    #[doc = "Bit 17 - enable cocpu trap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_trap_int_ena_w1tc(&mut self) -> COCPU_TRAP_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        COCPU_TRAP_INT_ENA_W1TC_W::new(self, 17)
    }
    #[doc = "Bit 18 - enable touch timeout interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_int_ena_w1tc(
        &mut self,
    ) -> TOUCH_TIMEOUT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_TIMEOUT_INT_ENA_W1TC_W::new(self, 18)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_ena_w1tc(&mut self) -> GLITCH_DET_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        GLITCH_DET_INT_ENA_W1TC_W::new(self, 19)
    }
    #[doc = "Bit 20 - enbale touch approach_loop done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_loop_done_int_ena_w1tc(
        &mut self,
    ) -> TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W::new(self, 20)
    }
}
#[doc = "oneset clr rtc interrupt enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc_w1tc::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TC to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
