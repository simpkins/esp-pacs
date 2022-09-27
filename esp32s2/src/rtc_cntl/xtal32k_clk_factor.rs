#[doc = "Register `XTAL32K_CLK_FACTOR` reader"]
pub struct R(crate::R<XTAL32K_CLK_FACTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_CLK_FACTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_CLK_FACTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_CLK_FACTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K_CLK_FACTOR` writer"]
pub struct W(crate::W<XTAL32K_CLK_FACTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_CLK_FACTOR_SPEC>;
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
impl From<crate::W<XTAL32K_CLK_FACTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_CLK_FACTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_CLK_FACTOR` reader - Configures the divider factor for the 32 kHz crystal oscillator."]
pub type XTAL32K_CLK_FACTOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XTAL32K_CLK_FACTOR` writer - Configures the divider factor for the 32 kHz crystal oscillator."]
pub type XTAL32K_CLK_FACTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL32K_CLK_FACTOR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Configures the divider factor for the 32 kHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal32k_clk_factor(&self) -> XTAL32K_CLK_FACTOR_R {
        XTAL32K_CLK_FACTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the divider factor for the 32 kHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal32k_clk_factor(&mut self) -> XTAL32K_CLK_FACTOR_W<0> {
        XTAL32K_CLK_FACTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the divider factor for the backup clock of 32 kHz crystal oscillator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_clk_factor](index.html) module"]
pub struct XTAL32K_CLK_FACTOR_SPEC;
impl crate::RegisterSpec for XTAL32K_CLK_FACTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k_clk_factor::R](R) reader structure"]
impl crate::Readable for XTAL32K_CLK_FACTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k_clk_factor::W](W) writer structure"]
impl crate::Writable for XTAL32K_CLK_FACTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32K_CLK_FACTOR to value 0"]
impl crate::Resettable for XTAL32K_CLK_FACTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
