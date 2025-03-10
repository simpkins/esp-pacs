#[doc = "Register `IN_ARB_CH2` reader"]
pub type R = crate::R<IN_ARB_CH2_SPEC>;
#[doc = "Register `IN_ARB_CH2` writer"]
pub type W = crate::W<IN_ARB_CH2_SPEC>;
#[doc = "Field `IN_ARB_TOKEN_NUM_CH2` reader - Set the max number of token count of arbiter"]
pub type IN_ARB_TOKEN_NUM_CH2_R = crate::FieldReader;
#[doc = "Field `IN_ARB_TOKEN_NUM_CH2` writer - Set the max number of token count of arbiter"]
pub type IN_ARB_TOKEN_NUM_CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTER_IN_ARB_PRIORITY_CH2` reader - Set the priority of channel"]
pub type INTER_IN_ARB_PRIORITY_CH2_R = crate::FieldReader;
#[doc = "Field `INTER_IN_ARB_PRIORITY_CH2` writer - Set the priority of channel"]
pub type INTER_IN_ARB_PRIORITY_CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn in_arb_token_num_ch2(&self) -> IN_ARB_TOKEN_NUM_CH2_R {
        IN_ARB_TOKEN_NUM_CH2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - Set the priority of channel"]
    #[inline(always)]
    pub fn inter_in_arb_priority_ch2(&self) -> INTER_IN_ARB_PRIORITY_CH2_R {
        INTER_IN_ARB_PRIORITY_CH2_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ARB_CH2")
            .field(
                "in_arb_token_num_ch2",
                &format_args!("{}", self.in_arb_token_num_ch2().bits()),
            )
            .field(
                "inter_in_arb_priority_ch2",
                &format_args!("{}", self.inter_in_arb_priority_ch2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_ARB_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    #[must_use]
    pub fn in_arb_token_num_ch2(&mut self) -> IN_ARB_TOKEN_NUM_CH2_W<IN_ARB_CH2_SPEC> {
        IN_ARB_TOKEN_NUM_CH2_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - Set the priority of channel"]
    #[inline(always)]
    #[must_use]
    pub fn inter_in_arb_priority_ch2(&mut self) -> INTER_IN_ARB_PRIORITY_CH2_W<IN_ARB_CH2_SPEC> {
        INTER_IN_ARB_PRIORITY_CH2_W::new(self, 6)
    }
}
#[doc = "RX CH2 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ARB_CH2_SPEC;
impl crate::RegisterSpec for IN_ARB_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_arb_ch2::R`](R) reader structure"]
impl crate::Readable for IN_ARB_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_arb_ch2::W`](W) writer structure"]
impl crate::Writable for IN_ARB_CH2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_ARB_CH2 to value 0x41"]
impl crate::Resettable for IN_ARB_CH2_SPEC {
    const RESET_VALUE: u32 = 0x41;
}
