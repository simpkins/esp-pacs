#[doc = "Register `TIMER2_SYNC` reader"]
pub type R = crate::R<TIMER2_SYNC_SPEC>;
#[doc = "Register `TIMER2_SYNC` writer"]
pub type W = crate::W<TIMER2_SYNC_SPEC>;
#[doc = "Field `TIMER2_SYNCI_EN` reader - "]
pub type TIMER2_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `TIMER2_SYNCI_EN` writer - "]
pub type TIMER2_SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW` reader - "]
pub type SW_R = crate::BitReader;
#[doc = "Field `SW` writer - "]
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_SYNCO_SEL` reader - "]
pub type TIMER2_SYNCO_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER2_SYNCO_SEL` writer - "]
pub type TIMER2_SYNCO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER2_PHASE` reader - "]
pub type TIMER2_PHASE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER2_PHASE` writer - "]
pub type TIMER2_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER2_PHASE_DIRECTION` reader - "]
pub type TIMER2_PHASE_DIRECTION_R = crate::BitReader;
#[doc = "Field `TIMER2_PHASE_DIRECTION` writer - "]
pub type TIMER2_PHASE_DIRECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer2_synci_en(&self) -> TIMER2_SYNCI_EN_R {
        TIMER2_SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn timer2_synco_sel(&self) -> TIMER2_SYNCO_SEL_R {
        TIMER2_SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn timer2_phase(&self) -> TIMER2_PHASE_R {
        TIMER2_PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn timer2_phase_direction(&self) -> TIMER2_PHASE_DIRECTION_R {
        TIMER2_PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2_SYNC")
            .field(
                "timer2_synci_en",
                &format_args!("{}", self.timer2_synci_en().bit()),
            )
            .field("sw", &format_args!("{}", self.sw().bit()))
            .field(
                "timer2_synco_sel",
                &format_args!("{}", self.timer2_synco_sel().bits()),
            )
            .field(
                "timer2_phase",
                &format_args!("{}", self.timer2_phase().bits()),
            )
            .field(
                "timer2_phase_direction",
                &format_args!("{}", self.timer2_phase_direction().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER2_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_synci_en(&mut self) -> TIMER2_SYNCI_EN_W<TIMER2_SYNC_SPEC> {
        TIMER2_SYNCI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<TIMER2_SYNC_SPEC> {
        SW_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_synco_sel(&mut self) -> TIMER2_SYNCO_SEL_W<TIMER2_SYNC_SPEC> {
        TIMER2_SYNCO_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_phase(&mut self) -> TIMER2_PHASE_W<TIMER2_SYNC_SPEC> {
        TIMER2_PHASE_W::new(self, 4)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_phase_direction(&mut self) -> TIMER2_PHASE_DIRECTION_W<TIMER2_SYNC_SPEC> {
        TIMER2_PHASE_DIRECTION_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER2_SYNC_SPEC;
impl crate::RegisterSpec for TIMER2_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_sync::R`](R) reader structure"]
impl crate::Readable for TIMER2_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer2_sync::W`](W) writer structure"]
impl crate::Writable for TIMER2_SYNC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2_SYNC to value 0"]
impl crate::Resettable for TIMER2_SYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
