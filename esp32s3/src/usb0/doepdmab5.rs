#[doc = "Register `DOEPDMAB5` reader"]
pub type R = crate::R<DOEPDMAB5_SPEC>;
#[doc = "Register `DOEPDMAB5` writer"]
pub type W = crate::W<DOEPDMAB5_SPEC>;
#[doc = "Field `DMABUFFERADDR5` reader - "]
pub type DMABUFFERADDR5_R = crate::FieldReader<u32>;
#[doc = "Field `DMABUFFERADDR5` writer - "]
pub type DMABUFFERADDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr5(&self) -> DMABUFFERADDR5_R {
        DMABUFFERADDR5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB5")
            .field(
                "dmabufferaddr5",
                &format_args!("{}", self.dmabufferaddr5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr5(&mut self) -> DMABUFFERADDR5_W<DOEPDMAB5_SPEC> {
        DMABUFFERADDR5_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB5_SPEC;
impl crate::RegisterSpec for DOEPDMAB5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab5::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdmab5::W`](W) writer structure"]
impl crate::Writable for DOEPDMAB5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMAB5 to value 0"]
impl crate::Resettable for DOEPDMAB5_SPEC {
    const RESET_VALUE: u32 = 0;
}
