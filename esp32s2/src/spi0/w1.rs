#[doc = "Register `W1` reader"]
pub type R = crate::R<W1_SPEC>;
#[doc = "Register `W1` writer"]
pub type W = crate::W<W1_SPEC>;
#[doc = "Field `BUF1` reader - 32 bits data buffer 1, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF1_R = crate::FieldReader<u32>;
#[doc = "Field `BUF1` writer - 32 bits data buffer 1, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 1, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf1(&self) -> BUF1_R {
        BUF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W1")
            .field("buf1", &format_args!("{}", self.buf1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 1, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf1(&mut self) -> BUF1_W<W1_SPEC> {
        BUF1_W::new(self, 0)
    }
}
#[doc = "Data buffer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W1_SPEC;
impl crate::RegisterSpec for W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w1::R`](R) reader structure"]
impl crate::Readable for W1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w1::W`](W) writer structure"]
impl crate::Writable for W1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W1 to value 0"]
impl crate::Resettable for W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
