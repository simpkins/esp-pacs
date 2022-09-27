#[doc = "Register `SAR1_PATT_TAB2` reader"]
pub struct R(crate::R<SAR1_PATT_TAB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR1_PATT_TAB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR1_PATT_TAB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR1_PATT_TAB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR1_PATT_TAB2` writer"]
pub struct W(crate::W<SAR1_PATT_TAB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR1_PATT_TAB2_SPEC>;
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
impl From<crate::W<SAR1_PATT_TAB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR1_PATT_TAB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_PATT_TAB2` reader - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
pub type SAR1_PATT_TAB2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR1_PATT_TAB2` writer - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
pub type SAR1_PATT_TAB2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR1_PATT_TAB2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn sar1_patt_tab2(&self) -> SAR1_PATT_TAB2_R {
        SAR1_PATT_TAB2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn sar1_patt_tab2(&mut self) -> SAR1_PATT_TAB2_W<0> {
        SAR1_PATT_TAB2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Item 4 ~ 7 for pattern table 1 (each item one byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar1_patt_tab2](index.html) module"]
pub struct SAR1_PATT_TAB2_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar1_patt_tab2::R](R) reader structure"]
impl crate::Readable for SAR1_PATT_TAB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar1_patt_tab2::W](W) writer structure"]
impl crate::Writable for SAR1_PATT_TAB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR1_PATT_TAB2 to value 0x0f0f_0f0f"]
impl crate::Resettable for SAR1_PATT_TAB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f0f_0f0f
    }
}
