#[doc = "Register `Q2_WORD1` reader"]
pub type R = crate::R<Q2_WORD1_SPEC>;
#[doc = "Register `Q2_WORD1` writer"]
pub type W = crate::W<Q2_WORD1_SPEC>;
#[doc = "Field `SEND_Q2_WORD1` reader - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q2_WORD1_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q2_WORD1` writer - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q2_WORD1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_q2_word1(&self) -> SEND_Q2_WORD1_R {
        SEND_Q2_WORD1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Q2_WORD1")
            .field(
                "send_q2_word1",
                &format_args!("{}", self.send_q2_word1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<Q2_WORD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn send_q2_word1(&mut self) -> SEND_Q2_WORD1_W<Q2_WORD1_SPEC> {
        SEND_Q2_WORD1_W::new(self, 0)
    }
}
#[doc = "Q2_WORD1 quick_sent register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q2_word1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q2_word1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Q2_WORD1_SPEC;
impl crate::RegisterSpec for Q2_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`q2_word1::R`](R) reader structure"]
impl crate::Readable for Q2_WORD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`q2_word1::W`](W) writer structure"]
impl crate::Writable for Q2_WORD1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Q2_WORD1 to value 0"]
impl crate::Resettable for Q2_WORD1_SPEC {
    const RESET_VALUE: u32 = 0;
}
