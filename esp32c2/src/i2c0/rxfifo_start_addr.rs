#[doc = "Register `RXFIFO_START_ADDR` reader"]
pub struct R(crate::R<RXFIFO_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFO_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFO_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFO_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFIFO_START_ADDR` writer"]
pub struct W(crate::W<RXFIFO_START_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFIFO_START_ADDR_SPEC>;
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
impl From<crate::W<RXFIFO_START_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFIFO_START_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_START_ADDR` reader - This is the I2C rxfifo first address."]
pub type RXFIFO_START_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RXFIFO_START_ADDR` writer - This is the I2C rxfifo first address."]
pub type RXFIFO_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXFIFO_START_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This is the I2C rxfifo first address."]
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> RXFIFO_START_ADDR_R {
        RXFIFO_START_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the I2C rxfifo first address."]
    #[inline(always)]
    pub fn rxfifo_start_addr(&mut self) -> RXFIFO_START_ADDR_W<0> {
        RXFIFO_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C RXFIFO base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo_start_addr](index.html) module"]
pub struct RXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for RXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifo_start_addr::R](R) reader structure"]
impl crate::Readable for RXFIFO_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfifo_start_addr::W](W) writer structure"]
impl crate::Writable for RXFIFO_START_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXFIFO_START_ADDR to value 0"]
impl crate::Resettable for RXFIFO_START_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
