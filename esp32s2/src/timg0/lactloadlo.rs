#[doc = "Register `LACTLOADLO` reader"]
pub type R = crate::R<LACTLOADLO_SPEC>;
#[doc = "Register `LACTLOADLO` writer"]
pub type W = crate::W<LACTLOADLO_SPEC>;
#[doc = "Field `LACT_LOAD_LO` reader - Reserved."]
pub type LACT_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LACT_LOAD_LO` writer - Reserved."]
pub type LACT_LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn lact_load_lo(&self) -> LACT_LOAD_LO_R {
        LACT_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLOADLO")
            .field(
                "lact_load_lo",
                &format_args!("{}", self.lact_load_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTLOADLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn lact_load_lo(&mut self) -> LACT_LOAD_LO_W<LACTLOADLO_SPEC> {
        LACT_LOAD_LO_W::new(self, 0)
    }
}
#[doc = "LACT load low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTLOADLO_SPEC;
impl crate::RegisterSpec for LACTLOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactloadlo::R`](R) reader structure"]
impl crate::Readable for LACTLOADLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactloadlo::W`](W) writer structure"]
impl crate::Writable for LACTLOADLO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTLOADLO to value 0"]
impl crate::Resettable for LACTLOADLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
