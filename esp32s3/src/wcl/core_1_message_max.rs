#[doc = "Register `Core_1_MESSAGE_MAX` reader"]
pub struct R(crate::R<CORE_1_MESSAGE_MAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_MESSAGE_MAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_MESSAGE_MAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_MESSAGE_MAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_MESSAGE_MAX` writer"]
pub struct W(crate::W<CORE_1_MESSAGE_MAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_MESSAGE_MAX_SPEC>;
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
impl From<crate::W<CORE_1_MESSAGE_MAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_MESSAGE_MAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_MESSAGE_MAX` reader - This filed is used to set the max value of clear write_buffer"]
pub type CORE_1_MESSAGE_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_1_MESSAGE_MAX` writer - This filed is used to set the max value of clear write_buffer"]
pub type CORE_1_MESSAGE_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_MESSAGE_MAX_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - This filed is used to set the max value of clear write_buffer"]
    #[inline(always)]
    pub fn core_1_message_max(&self) -> CORE_1_MESSAGE_MAX_R {
        CORE_1_MESSAGE_MAX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This filed is used to set the max value of clear write_buffer"]
    #[inline(always)]
    pub fn core_1_message_max(&mut self) -> CORE_1_MESSAGE_MAX_W<0> {
        CORE_1_MESSAGE_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear writer_buffer write number configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_message_max](index.html) module"]
pub struct CORE_1_MESSAGE_MAX_SPEC;
impl crate::RegisterSpec for CORE_1_MESSAGE_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_message_max::R](R) reader structure"]
impl crate::Readable for CORE_1_MESSAGE_MAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_message_max::W](W) writer structure"]
impl crate::Writable for CORE_1_MESSAGE_MAX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_1_MESSAGE_MAX to value 0"]
impl crate::Resettable for CORE_1_MESSAGE_MAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
