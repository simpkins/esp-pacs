#[doc = "Register `TZ2_CFG1` reader"]
pub type R = crate::R<TZ2_CFG1_SPEC>;
#[doc = "Register `TZ2_CFG1` writer"]
pub type W = crate::W<TZ2_CFG1_SPEC>;
#[doc = "Field `TZ2_CLR_OST` reader - a rising edge will clear on going one-shot mode action"]
pub type TZ2_CLR_OST_R = crate::BitReader;
#[doc = "Field `TZ2_CLR_OST` writer - a rising edge will clear on going one-shot mode action"]
pub type TZ2_CLR_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ2_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `TZ2_CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ2_CBCPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ2_FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ2_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `TZ2_FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ2_FORCE_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ2_FORCE_OST_R = crate::BitReader;
#[doc = "Field `TZ2_FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ2_FORCE_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz2_clr_ost(&self) -> TZ2_CLR_OST_R {
        TZ2_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    pub fn tz2_cbcpulse(&self) -> TZ2_CBCPULSE_R {
        TZ2_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz2_force_cbc(&self) -> TZ2_FORCE_CBC_R {
        TZ2_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz2_force_ost(&self) -> TZ2_FORCE_OST_R {
        TZ2_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ2_CFG1")
            .field("tz2_clr_ost", &format_args!("{}", self.tz2_clr_ost().bit()))
            .field(
                "tz2_cbcpulse",
                &format_args!("{}", self.tz2_cbcpulse().bits()),
            )
            .field(
                "tz2_force_cbc",
                &format_args!("{}", self.tz2_force_cbc().bit()),
            )
            .field(
                "tz2_force_ost",
                &format_args!("{}", self.tz2_force_ost().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TZ2_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_clr_ost(&mut self) -> TZ2_CLR_OST_W<TZ2_CFG1_SPEC> {
        TZ2_CLR_OST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_cbcpulse(&mut self) -> TZ2_CBCPULSE_W<TZ2_CFG1_SPEC> {
        TZ2_CBCPULSE_W::new(self, 1)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_force_cbc(&mut self) -> TZ2_FORCE_CBC_W<TZ2_CFG1_SPEC> {
        TZ2_FORCE_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_force_ost(&mut self) -> TZ2_FORCE_OST_W<TZ2_CFG1_SPEC> {
        TZ2_FORCE_OST_W::new(self, 4)
    }
}
#[doc = "Software triggers for fault handler actions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tz2_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tz2_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZ2_CFG1_SPEC;
impl crate::RegisterSpec for TZ2_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tz2_cfg1::R`](R) reader structure"]
impl crate::Readable for TZ2_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tz2_cfg1::W`](W) writer structure"]
impl crate::Writable for TZ2_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZ2_CFG1 to value 0"]
impl crate::Resettable for TZ2_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
