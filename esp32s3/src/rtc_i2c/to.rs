#[doc = "Register `TO` reader"]
pub struct R(crate::R<TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TO` writer"]
pub struct W(crate::W<TO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TO_SPEC>;
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
impl From<crate::W<TO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_OUT` reader - time out threshold"]
pub type TIME_OUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIME_OUT` writer - time out threshold"]
pub type TIME_OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TO_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - time out threshold"]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - time out threshold"]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W<0> {
        TIME_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure time out\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [to](index.html) module"]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [to::R](R) reader structure"]
impl crate::Readable for TO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [to::W](W) writer structure"]
impl crate::Writable for TO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TO to value 0x0001_0000"]
impl crate::Resettable for TO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
