#[doc = "Register `DB0_FED_CFG` reader"]
pub type R = crate::R<DB0_FED_CFG_SPEC>;
#[doc = "Register `DB0_FED_CFG` writer"]
pub type W = crate::W<DB0_FED_CFG_SPEC>;
#[doc = "Field `DB0_FED` reader - Shadow register for FED"]
pub type DB0_FED_R = crate::FieldReader<u16>;
#[doc = "Field `DB0_FED` writer - Shadow register for FED"]
pub type DB0_FED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    pub fn db0_fed(&self) -> DB0_FED_R {
        DB0_FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB0_FED_CFG")
            .field("db0_fed", &format_args!("{}", self.db0_fed().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DB0_FED_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    #[must_use]
    pub fn db0_fed(&mut self) -> DB0_FED_W<DB0_FED_CFG_SPEC> {
        DB0_FED_W::new(self, 0)
    }
}
#[doc = "Shadow register for falling edge delay (FED).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db0_fed_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db0_fed_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DB0_FED_CFG_SPEC;
impl crate::RegisterSpec for DB0_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`db0_fed_cfg::R`](R) reader structure"]
impl crate::Readable for DB0_FED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`db0_fed_cfg::W`](W) writer structure"]
impl crate::Writable for DB0_FED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DB0_FED_CFG to value 0"]
impl crate::Resettable for DB0_FED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
