#[doc = "Register `RWBT_IRQ_MAP` reader"]
pub struct R(crate::R<RWBT_IRQ_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWBT_IRQ_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWBT_IRQ_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWBT_IRQ_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWBT_IRQ_MAP` writer"]
pub struct W(crate::W<RWBT_IRQ_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWBT_IRQ_MAP_SPEC>;
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
impl From<crate::W<RWBT_IRQ_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWBT_IRQ_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWBT_IRQ_MAP` reader - this register used to map rwbt_irq interrupt to one of core0's external interrupt"]
pub type RWBT_IRQ_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWBT_IRQ_MAP` writer - this register used to map rwbt_irq interrupt to one of core0's external interrupt"]
pub type RWBT_IRQ_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RWBT_IRQ_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map rwbt_irq interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn rwbt_irq_map(&self) -> RWBT_IRQ_MAP_R {
        RWBT_IRQ_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map rwbt_irq interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn rwbt_irq_map(&mut self) -> RWBT_IRQ_MAP_W<0> {
        RWBT_IRQ_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rwbt_irq interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwbt_irq_map](index.html) module"]
pub struct RWBT_IRQ_MAP_SPEC;
impl crate::RegisterSpec for RWBT_IRQ_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwbt_irq_map::R](R) reader structure"]
impl crate::Readable for RWBT_IRQ_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwbt_irq_map::W](W) writer structure"]
impl crate::Writable for RWBT_IRQ_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RWBT_IRQ_MAP to value 0x10"]
impl crate::Resettable for RWBT_IRQ_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
