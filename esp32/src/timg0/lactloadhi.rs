#[doc = "Register `LACTLOADHI` reader"]
pub type R = crate::R<LACTLOADHI_SPEC>;
#[doc = "Register `LACTLOADHI` writer"]
pub type W = crate::W<LACTLOADHI_SPEC>;
#[doc = "Field `LACT_LOAD_HI` reader - "]
pub type LACT_LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `LACT_LOAD_HI` writer - "]
pub type LACT_LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_load_hi(&self) -> LACT_LOAD_HI_R {
        LACT_LOAD_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLOADHI")
            .field(
                "lact_load_hi",
                &format_args!("{}", self.lact_load_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTLOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lact_load_hi(&mut self) -> LACT_LOAD_HI_W<LACTLOADHI_SPEC> {
        LACT_LOAD_HI_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactloadhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTLOADHI_SPEC;
impl crate::RegisterSpec for LACTLOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactloadhi::R`](R) reader structure"]
impl crate::Readable for LACTLOADHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactloadhi::W`](W) writer structure"]
impl crate::Writable for LACTLOADHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTLOADHI to value 0"]
impl crate::Resettable for LACTLOADHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
