#[doc = "Register `SPI_W4` reader"]
pub type R = crate::R<SPI_W4_SPEC>;
#[doc = "Register `SPI_W4` writer"]
pub type W = crate::W<SPI_W4_SPEC>;
#[doc = "Field `SPI_BUF4` reader - data buffer"]
pub type SPI_BUF4_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_BUF4` writer - data buffer"]
pub type SPI_BUF4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf4(&self) -> SPI_BUF4_R {
        SPI_BUF4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W4")
            .field("spi_buf4", &format_args!("{}", self.spi_buf4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn spi_buf4(&mut self) -> SPI_BUF4_W<SPI_W4_SPEC> {
        SPI_BUF4_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_w4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_w4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_W4_SPEC;
impl crate::RegisterSpec for SPI_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_w4::R`](R) reader structure"]
impl crate::Readable for SPI_W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_w4::W`](W) writer structure"]
impl crate::Writable for SPI_W4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_W4 to value 0"]
impl crate::Resettable for SPI_W4_SPEC {
    const RESET_VALUE: u32 = 0;
}
