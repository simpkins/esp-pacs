#[doc = "Register `CH%s_HPOINT` reader"]
pub type R = crate::R<CH_HPOINT_SPEC>;
#[doc = "Register `CH%s_HPOINT` writer"]
pub type W = crate::W<CH_HPOINT_SPEC>;
#[doc = "Field `HPOINT` reader - reg_hpoint_lsch0."]
pub type HPOINT_R = crate::FieldReader<u16>;
#[doc = "Field `HPOINT` writer - reg_hpoint_lsch0."]
pub type HPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - reg_hpoint_lsch0."]
    #[inline(always)]
    pub fn hpoint(&self) -> HPOINT_R {
        HPOINT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_HPOINT")
            .field("hpoint", &format_args!("{}", self.hpoint().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_HPOINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - reg_hpoint_lsch0."]
    #[inline(always)]
    #[must_use]
    pub fn hpoint(&mut self) -> HPOINT_W<CH_HPOINT_SPEC> {
        HPOINT_W::new(self, 0)
    }
}
#[doc = "LEDC_LSCH%s_HPOINT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_hpoint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_hpoint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_HPOINT_SPEC;
impl crate::RegisterSpec for CH_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_hpoint::R`](R) reader structure"]
impl crate::Readable for CH_HPOINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_hpoint::W`](W) writer structure"]
impl crate::Writable for CH_HPOINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_HPOINT to value 0"]
impl crate::Resettable for CH_HPOINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
