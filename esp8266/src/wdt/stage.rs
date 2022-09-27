#[doc = "Register `stage` reader"]
pub struct R(crate::R<STAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `stage` writer"]
pub struct W(crate::W<STAGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STAGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAGE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The current watchdog stage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stage](index.html) module"]
pub struct STAGE_SPEC;
impl crate::RegisterSpec for STAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stage::R](R) reader structure"]
impl crate::Readable for STAGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stage::W](W) writer structure"]
impl crate::Writable for STAGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets stage to value 0"]
impl crate::Resettable for STAGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
