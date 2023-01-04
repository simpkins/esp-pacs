#[doc = "Register `SPI_RD_STATUS` reader"]
pub struct R(crate::R<SPI_RD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_RD_STATUS` writer"]
pub struct W(crate::W<SPI_RD_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_RD_STATUS_SPEC>;
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
impl From<crate::W<SPI_RD_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_RD_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slv_rd_status` reader - In the slave mode, this register are the status register for the master to read out."]
pub type SLV_RD_STATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `slv_rd_status` writer - In the slave mode, this register are the status register for the master to read out."]
pub type SLV_RD_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_RD_STATUS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - In the slave mode, this register are the status register for the master to read out."]
    #[inline(always)]
    pub fn slv_rd_status(&self) -> SLV_RD_STATUS_R {
        SLV_RD_STATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In the slave mode, this register are the status register for the master to read out."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_status(&mut self) -> SLV_RD_STATUS_W<0> {
        SLV_RD_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In the slave mode, this register are the status register for the master to read out.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rd_status](index.html) module"]
pub struct SPI_RD_STATUS_SPEC;
impl crate::RegisterSpec for SPI_RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_rd_status::R](R) reader structure"]
impl crate::Readable for SPI_RD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_rd_status::W](W) writer structure"]
impl crate::Writable for SPI_RD_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_RD_STATUS to value 0"]
impl crate::Resettable for SPI_RD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
