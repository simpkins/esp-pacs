#[doc = "Register `HOST_SLCHOST_RDCLR0` reader"]
pub struct R(crate::R<HOST_SLCHOST_RDCLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_RDCLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_RDCLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_RDCLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_RDCLR0` writer"]
pub struct W(crate::W<HOST_SLCHOST_RDCLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_RDCLR0_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_RDCLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_RDCLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_SLC0_BIT7_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC0_BIT7_CLRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HOST_SLCHOST_SLC0_BIT7_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_RDCLR0_SPEC, u16, u16, 9, O>;
#[doc = "Field `HOST_SLCHOST_SLC0_BIT6_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC0_BIT6_CLRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HOST_SLCHOST_SLC0_BIT6_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_RDCLR0_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit7_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit6_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit7_clraddr(&mut self) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<0> {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_W::new(self)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit6_clraddr(&mut self) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<9> {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_rdclr0](index.html) module"]
pub struct HOST_SLCHOST_RDCLR0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_RDCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_rdclr0::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_rdclr0::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_RDCLR0 to value 0x0003_c044"]
impl crate::Resettable for HOST_SLCHOST_RDCLR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_c044
    }
}
