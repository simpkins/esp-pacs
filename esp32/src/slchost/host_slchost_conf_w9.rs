#[doc = "Register `HOST_SLCHOST_CONF_W9` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W9` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W9_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF36` reader - "]
pub type HOST_SLCHOST_CONF36_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF36` writer - "]
pub type HOST_SLCHOST_CONF36_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W9_SPEC, u8, u8, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF37` reader - "]
pub type HOST_SLCHOST_CONF37_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF37` writer - "]
pub type HOST_SLCHOST_CONF37_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W9_SPEC, u8, u8, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF38` reader - "]
pub type HOST_SLCHOST_CONF38_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF38` writer - "]
pub type HOST_SLCHOST_CONF38_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W9_SPEC, u8, u8, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF39` reader - "]
pub type HOST_SLCHOST_CONF39_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF39` writer - "]
pub type HOST_SLCHOST_CONF39_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W9_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf36(&self) -> HOST_SLCHOST_CONF36_R {
        HOST_SLCHOST_CONF36_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf37(&self) -> HOST_SLCHOST_CONF37_R {
        HOST_SLCHOST_CONF37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf38(&self) -> HOST_SLCHOST_CONF38_R {
        HOST_SLCHOST_CONF38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf39(&self) -> HOST_SLCHOST_CONF39_R {
        HOST_SLCHOST_CONF39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf36(&mut self) -> HOST_SLCHOST_CONF36_W<0> {
        HOST_SLCHOST_CONF36_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf37(&mut self) -> HOST_SLCHOST_CONF37_W<8> {
        HOST_SLCHOST_CONF37_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf38(&mut self) -> HOST_SLCHOST_CONF38_W<16> {
        HOST_SLCHOST_CONF38_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf39(&mut self) -> HOST_SLCHOST_CONF39_W<24> {
        HOST_SLCHOST_CONF39_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w9](index.html) module"]
pub struct HOST_SLCHOST_CONF_W9_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w9::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w9::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W9 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
