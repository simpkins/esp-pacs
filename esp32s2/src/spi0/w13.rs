#[doc = "Register `W13` reader"]
pub struct R(crate::R<W13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W13` writer"]
pub struct W(crate::W<W13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W13_SPEC>;
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
impl From<crate::W<W13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF13` reader - 32 bits data buffer 13, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUF13` writer - 32 bits data buffer 13, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, W13_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 13, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf13(&self) -> BUF13_R {
        BUF13_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 13, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf13(&mut self) -> BUF13_W<0> {
        BUF13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w13](index.html) module"]
pub struct W13_SPEC;
impl crate::RegisterSpec for W13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w13::R](R) reader structure"]
impl crate::Readable for W13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w13::W](W) writer structure"]
impl crate::Writable for W13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W13 to value 0"]
impl crate::Resettable for W13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
