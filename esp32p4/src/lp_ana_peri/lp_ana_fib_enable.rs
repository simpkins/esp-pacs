#[doc = "Register `LP_ANA_FIB_ENABLE` reader"]
pub type R = crate::R<LP_ANA_FIB_ENABLE_SPEC>;
#[doc = "Register `LP_ANA_FIB_ENABLE` writer"]
pub type W = crate::W<LP_ANA_FIB_ENABLE_SPEC>;
#[doc = "Field `LP_ANA_ANA_FIB_ENA` reader - need_des"]
pub type LP_ANA_ANA_FIB_ENA_R = crate::FieldReader<u32>;
#[doc = "Field `LP_ANA_ANA_FIB_ENA` writer - need_des"]
pub type LP_ANA_ANA_FIB_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_ana_fib_ena(&self) -> LP_ANA_ANA_FIB_ENA_R {
        LP_ANA_ANA_FIB_ENA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_FIB_ENABLE")
            .field(
                "lp_ana_ana_fib_ena",
                &format_args!("{}", self.lp_ana_ana_fib_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_FIB_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_ana_fib_ena(&mut self) -> LP_ANA_ANA_FIB_ENA_W<LP_ANA_FIB_ENABLE_SPEC> {
        LP_ANA_ANA_FIB_ENA_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_fib_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_fib_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_FIB_ENABLE_SPEC;
impl crate::RegisterSpec for LP_ANA_FIB_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_fib_enable::R`](R) reader structure"]
impl crate::Readable for LP_ANA_FIB_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_fib_enable::W`](W) writer structure"]
impl crate::Writable for LP_ANA_FIB_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_FIB_ENABLE to value 0xffff_ffff"]
impl crate::Resettable for LP_ANA_FIB_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
