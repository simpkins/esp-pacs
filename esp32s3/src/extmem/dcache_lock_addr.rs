#[doc = "Register `DCACHE_LOCK_ADDR` reader"]
pub struct R(crate::R<DCACHE_LOCK_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_LOCK_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_LOCK_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_LOCK_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_LOCK_ADDR` writer"]
pub struct W(crate::W<DCACHE_LOCK_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_LOCK_ADDR_SPEC>;
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
impl From<crate::W<DCACHE_LOCK_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_LOCK_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_LOCK_ADDR` reader - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
pub type DCACHE_LOCK_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DCACHE_LOCK_ADDR` writer - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
pub type DCACHE_LOCK_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCACHE_LOCK_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
    #[inline(always)]
    pub fn dcache_lock_addr(&self) -> DCACHE_LOCK_ADDR_R {
        DCACHE_LOCK_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
    #[inline(always)]
    pub fn dcache_lock_addr(&mut self) -> DCACHE_LOCK_ADDR_W<0> {
        DCACHE_LOCK_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_lock_addr](index.html) module"]
pub struct DCACHE_LOCK_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_LOCK_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_lock_addr::R](R) reader structure"]
impl crate::Readable for DCACHE_LOCK_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_lock_addr::W](W) writer structure"]
impl crate::Writable for DCACHE_LOCK_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_LOCK_ADDR to value 0"]
impl crate::Resettable for DCACHE_LOCK_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
