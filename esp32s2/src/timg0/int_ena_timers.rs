#[doc = "Register `INT_ENA_TIMERS` reader"]
pub type R = crate::R<INT_ENA_TIMERS_SPEC>;
#[doc = "Register `INT_ENA_TIMERS` writer"]
pub type W = crate::W<INT_ENA_TIMERS_SPEC>;
#[doc = "Field `T0_INT_ENA` reader - The interrupt enable bit for the TIMG_T0_INT interrupt."]
pub type T0_INT_ENA_R = crate::BitReader;
#[doc = "Field `T0_INT_ENA` writer - The interrupt enable bit for the TIMG_T0_INT interrupt."]
pub type T0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1_INT_ENA` reader - The interrupt enable bit for the TIMG_T1_INT interrupt."]
pub type T1_INT_ENA_R = crate::BitReader;
#[doc = "Field `T1_INT_ENA` writer - The interrupt enable bit for the TIMG_T1_INT interrupt."]
pub type T1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA` reader - The interrupt enable bit for the TIMG_WDT_INT interrupt."]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - The interrupt enable bit for the TIMG_WDT_INT interrupt."]
pub type WDT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_INT_ENA` reader - The interrupt enable bit for the TIMG_LACT_INT interrupt."]
pub type LACT_INT_ENA_R = crate::BitReader;
#[doc = "Field `LACT_INT_ENA` writer - The interrupt enable bit for the TIMG_LACT_INT interrupt."]
pub type LACT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0_int_ena(&self) -> T0_INT_ENA_R {
        T0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1_int_ena(&self) -> T1_INT_ENA_R {
        T1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the TIMG_LACT_INT interrupt."]
    #[inline(always)]
    pub fn lact_int_ena(&self) -> LACT_INT_ENA_R {
        LACT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_TIMERS")
            .field("t0_int_ena", &format_args!("{}", self.t0_int_ena().bit()))
            .field("t1_int_ena", &format_args!("{}", self.t1_int_ena().bit()))
            .field("wdt_int_ena", &format_args!("{}", self.wdt_int_ena().bit()))
            .field(
                "lact_int_ena",
                &format_args!("{}", self.lact_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the TIMG_T0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn t0_int_ena(&mut self) -> T0_INT_ENA_W<INT_ENA_TIMERS_SPEC> {
        T0_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the TIMG_T1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn t1_int_ena(&mut self) -> T1_INT_ENA_W<INT_ENA_TIMERS_SPEC> {
        T1_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<INT_ENA_TIMERS_SPEC> {
        WDT_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the TIMG_LACT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lact_int_ena(&mut self) -> LACT_INT_ENA_W<INT_ENA_TIMERS_SPEC> {
        LACT_INT_ENA_W::new(self, 3)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ENA_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_timers::R`](R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_timers::W`](W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_TIMERS to value 0"]
impl crate::Resettable for INT_ENA_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
