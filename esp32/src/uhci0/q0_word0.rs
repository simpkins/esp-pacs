#[doc = "Register `Q0_WORD0` reader"]
pub type R = crate::R<Q0_WORD0_SPEC>;
#[doc = "Register `Q0_WORD0` writer"]
pub type W = crate::W<Q0_WORD0_SPEC>;
#[doc = "Field `SEND_Q0_WORD0` reader - This register stores the content of short packet's first dword"]
pub type SEND_Q0_WORD0_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q0_WORD0` writer - This register stores the content of short packet's first dword"]
pub type SEND_Q0_WORD0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the content of short packet's first dword"]
    #[inline(always)]
    pub fn send_q0_word0(&self) -> SEND_Q0_WORD0_R {
        SEND_Q0_WORD0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Q0_WORD0")
            .field(
                "send_q0_word0",
                &format_args!("{}", self.send_q0_word0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<Q0_WORD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the content of short packet's first dword"]
    #[inline(always)]
    #[must_use]
    pub fn send_q0_word0(&mut self) -> SEND_Q0_WORD0_W<Q0_WORD0_SPEC> {
        SEND_Q0_WORD0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q0_word0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q0_word0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Q0_WORD0_SPEC;
impl crate::RegisterSpec for Q0_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`q0_word0::R`](R) reader structure"]
impl crate::Readable for Q0_WORD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`q0_word0::W`](W) writer structure"]
impl crate::Writable for Q0_WORD0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Q0_WORD0 to value 0"]
impl crate::Resettable for Q0_WORD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
