#[doc = "Register `INT_ST_TIMERS` reader"]
pub type R = crate::R<INT_ST_TIMERS_SPEC>;
#[doc = "Field `T0_INT_ST` reader - interrupt when timer0 alarm"]
pub type T0_INT_ST_R = crate::BitReader;
#[doc = "Field `T1_INT_ST` reader - interrupt when timer1 alarm"]
pub type T1_INT_ST_R = crate::BitReader;
#[doc = "Field `WDT_INT_ST` reader - Interrupt when an interrupt stage timeout"]
pub type WDT_INT_ST_R = crate::BitReader;
#[doc = "Field `LACT_INT_ST` reader - "]
pub type LACT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0_int_st(&self) -> T0_INT_ST_R {
        T0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1_int_st(&self) -> T1_INT_ST_R {
        T1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact_int_st(&self) -> LACT_INT_ST_R {
        LACT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_TIMERS")
            .field("t0_int_st", &format_args!("{}", self.t0_int_st().bit()))
            .field("t1_int_st", &format_args!("{}", self.t1_int_st().bit()))
            .field("wdt_int_st", &format_args!("{}", self.wdt_int_st().bit()))
            .field("lact_int_st", &format_args!("{}", self.lact_int_st().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ST_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_timers::R`](R) reader structure"]
impl crate::Readable for INT_ST_TIMERS_SPEC {}
#[doc = "`reset()` method sets INT_ST_TIMERS to value 0"]
impl crate::Resettable for INT_ST_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
