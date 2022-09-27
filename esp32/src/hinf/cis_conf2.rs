#[doc = "Register `CIS_CONF2` reader"]
pub struct R(crate::R<CIS_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIS_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIS_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIS_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIS_CONF2` writer"]
pub struct W(crate::W<CIS_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIS_CONF2_SPEC>;
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
impl From<crate::W<CIS_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIS_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CIS_CONF_W2` reader - "]
pub type CIS_CONF_W2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CIS_CONF_W2` writer - "]
pub type CIS_CONF_W2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CIS_CONF2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w2(&self) -> CIS_CONF_W2_R {
        CIS_CONF_W2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w2(&mut self) -> CIS_CONF_W2_W<0> {
        CIS_CONF_W2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis_conf2](index.html) module"]
pub struct CIS_CONF2_SPEC;
impl crate::RegisterSpec for CIS_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cis_conf2::R](R) reader structure"]
impl crate::Readable for CIS_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cis_conf2::W](W) writer structure"]
impl crate::Writable for CIS_CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIS_CONF2 to value 0xffff_ffff"]
impl crate::Resettable for CIS_CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
