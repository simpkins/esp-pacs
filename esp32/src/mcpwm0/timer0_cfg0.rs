#[doc = "Register `TIMER0_CFG0` reader"]
pub type R = crate::R<TIMER0_CFG0_SPEC>;
#[doc = "Register `TIMER0_CFG0` writer"]
pub type W = crate::W<TIMER0_CFG0_SPEC>;
#[doc = "Field `TIMER0_PRESCALE` reader - "]
pub type TIMER0_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TIMER0_PRESCALE` writer - "]
pub type TIMER0_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMER0_PERIOD` reader - "]
pub type TIMER0_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER0_PERIOD` writer - "]
pub type TIMER0_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER0_PERIOD_UPMETHOD` reader - "]
pub type TIMER0_PERIOD_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `TIMER0_PERIOD_UPMETHOD` writer - "]
pub type TIMER0_PERIOD_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer0_prescale(&self) -> TIMER0_PRESCALE_R {
        TIMER0_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn timer0_period(&self) -> TIMER0_PERIOD_R {
        TIMER0_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn timer0_period_upmethod(&self) -> TIMER0_PERIOD_UPMETHOD_R {
        TIMER0_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0_CFG0")
            .field(
                "timer0_prescale",
                &format_args!("{}", self.timer0_prescale().bits()),
            )
            .field(
                "timer0_period",
                &format_args!("{}", self.timer0_period().bits()),
            )
            .field(
                "timer0_period_upmethod",
                &format_args!("{}", self.timer0_period_upmethod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER0_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_prescale(&mut self) -> TIMER0_PRESCALE_W<TIMER0_CFG0_SPEC> {
        TIMER0_PRESCALE_W::new(self, 0)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_period(&mut self) -> TIMER0_PERIOD_W<TIMER0_CFG0_SPEC> {
        TIMER0_PERIOD_W::new(self, 8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_period_upmethod(&mut self) -> TIMER0_PERIOD_UPMETHOD_W<TIMER0_CFG0_SPEC> {
        TIMER0_PERIOD_UPMETHOD_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER0_CFG0_SPEC;
impl crate::RegisterSpec for TIMER0_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0_cfg0::R`](R) reader structure"]
impl crate::Readable for TIMER0_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer0_cfg0::W`](W) writer structure"]
impl crate::Writable for TIMER0_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER0_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER0_CFG0_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
