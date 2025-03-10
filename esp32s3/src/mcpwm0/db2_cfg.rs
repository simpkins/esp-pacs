#[doc = "Register `DB2_CFG` reader"]
pub type R = crate::R<DB2_CFG_SPEC>;
#[doc = "Register `DB2_CFG` writer"]
pub type W = crate::W<DB2_CFG_SPEC>;
#[doc = "Field `DB2_FED_UPMETHOD` reader - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB2_FED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DB2_FED_UPMETHOD` writer - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB2_FED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DB2_RED_UPMETHOD` reader - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB2_RED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DB2_RED_UPMETHOD` writer - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
pub type DB2_RED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DB2_DEB_MODE` reader - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB2_DEB_MODE_R = crate::BitReader;
#[doc = "Field `DB2_DEB_MODE` writer - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB2_DEB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_A_OUTSWAP` reader - S6 in documentation"]
pub type DB2_A_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DB2_A_OUTSWAP` writer - S6 in documentation"]
pub type DB2_A_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_B_OUTSWAP` reader - S7 in documentation"]
pub type DB2_B_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DB2_B_OUTSWAP` writer - S7 in documentation"]
pub type DB2_B_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_RED_INSEL` reader - S4 in documentation"]
pub type DB2_RED_INSEL_R = crate::BitReader;
#[doc = "Field `DB2_RED_INSEL` writer - S4 in documentation"]
pub type DB2_RED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_FED_INSEL` reader - S5 in documentation"]
pub type DB2_FED_INSEL_R = crate::BitReader;
#[doc = "Field `DB2_FED_INSEL` writer - S5 in documentation"]
pub type DB2_FED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_RED_OUTINVERT` reader - S2 in documentation"]
pub type DB2_RED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DB2_RED_OUTINVERT` writer - S2 in documentation"]
pub type DB2_RED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_FED_OUTINVERT` reader - S3 in documentation"]
pub type DB2_FED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DB2_FED_OUTINVERT` writer - S3 in documentation"]
pub type DB2_FED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_A_OUTBYPASS` reader - S1 in documentation"]
pub type DB2_A_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DB2_A_OUTBYPASS` writer - S1 in documentation"]
pub type DB2_A_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_B_OUTBYPASS` reader - S0 in documentation"]
pub type DB2_B_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DB2_B_OUTBYPASS` writer - S0 in documentation"]
pub type DB2_B_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB2_CLK_SEL` reader - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
pub type DB2_CLK_SEL_R = crate::BitReader;
#[doc = "Field `DB2_CLK_SEL` writer - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
pub type DB2_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db2_fed_upmethod(&self) -> DB2_FED_UPMETHOD_R {
        DB2_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    pub fn db2_red_upmethod(&self) -> DB2_RED_UPMETHOD_R {
        DB2_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn db2_deb_mode(&self) -> DB2_DEB_MODE_R {
        DB2_DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn db2_a_outswap(&self) -> DB2_A_OUTSWAP_R {
        DB2_A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn db2_b_outswap(&self) -> DB2_B_OUTSWAP_R {
        DB2_B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn db2_red_insel(&self) -> DB2_RED_INSEL_R {
        DB2_RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn db2_fed_insel(&self) -> DB2_FED_INSEL_R {
        DB2_FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn db2_red_outinvert(&self) -> DB2_RED_OUTINVERT_R {
        DB2_RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn db2_fed_outinvert(&self) -> DB2_FED_OUTINVERT_R {
        DB2_FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn db2_a_outbypass(&self) -> DB2_A_OUTBYPASS_R {
        DB2_A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn db2_b_outbypass(&self) -> DB2_B_OUTBYPASS_R {
        DB2_B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    pub fn db2_clk_sel(&self) -> DB2_CLK_SEL_R {
        DB2_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB2_CFG")
            .field(
                "db2_fed_upmethod",
                &format_args!("{}", self.db2_fed_upmethod().bits()),
            )
            .field(
                "db2_red_upmethod",
                &format_args!("{}", self.db2_red_upmethod().bits()),
            )
            .field(
                "db2_deb_mode",
                &format_args!("{}", self.db2_deb_mode().bit()),
            )
            .field(
                "db2_a_outswap",
                &format_args!("{}", self.db2_a_outswap().bit()),
            )
            .field(
                "db2_b_outswap",
                &format_args!("{}", self.db2_b_outswap().bit()),
            )
            .field(
                "db2_red_insel",
                &format_args!("{}", self.db2_red_insel().bit()),
            )
            .field(
                "db2_fed_insel",
                &format_args!("{}", self.db2_fed_insel().bit()),
            )
            .field(
                "db2_red_outinvert",
                &format_args!("{}", self.db2_red_outinvert().bit()),
            )
            .field(
                "db2_fed_outinvert",
                &format_args!("{}", self.db2_fed_outinvert().bit()),
            )
            .field(
                "db2_a_outbypass",
                &format_args!("{}", self.db2_a_outbypass().bit()),
            )
            .field(
                "db2_b_outbypass",
                &format_args!("{}", self.db2_b_outbypass().bit()),
            )
            .field("db2_clk_sel", &format_args!("{}", self.db2_clk_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DB2_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    #[must_use]
    pub fn db2_fed_upmethod(&mut self) -> DB2_FED_UPMETHOD_W<DB2_CFG_SPEC> {
        DB2_FED_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate, bit0: tez, bit1: tep, bit2: sync, bit3: freeze"]
    #[inline(always)]
    #[must_use]
    pub fn db2_red_upmethod(&mut self) -> DB2_RED_UPMETHOD_W<DB2_CFG_SPEC> {
        DB2_RED_UPMETHOD_W::new(self, 4)
    }
    #[doc = "Bit 8 - S8 in documentation, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    #[must_use]
    pub fn db2_deb_mode(&mut self) -> DB2_DEB_MODE_W<DB2_CFG_SPEC> {
        DB2_DEB_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_a_outswap(&mut self) -> DB2_A_OUTSWAP_W<DB2_CFG_SPEC> {
        DB2_A_OUTSWAP_W::new(self, 9)
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_b_outswap(&mut self) -> DB2_B_OUTSWAP_W<DB2_CFG_SPEC> {
        DB2_B_OUTSWAP_W::new(self, 10)
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_red_insel(&mut self) -> DB2_RED_INSEL_W<DB2_CFG_SPEC> {
        DB2_RED_INSEL_W::new(self, 11)
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_fed_insel(&mut self) -> DB2_FED_INSEL_W<DB2_CFG_SPEC> {
        DB2_FED_INSEL_W::new(self, 12)
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_red_outinvert(&mut self) -> DB2_RED_OUTINVERT_W<DB2_CFG_SPEC> {
        DB2_RED_OUTINVERT_W::new(self, 13)
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_fed_outinvert(&mut self) -> DB2_FED_OUTINVERT_W<DB2_CFG_SPEC> {
        DB2_FED_OUTINVERT_W::new(self, 14)
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_a_outbypass(&mut self) -> DB2_A_OUTBYPASS_W<DB2_CFG_SPEC> {
        DB2_A_OUTBYPASS_W::new(self, 15)
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    #[must_use]
    pub fn db2_b_outbypass(&mut self) -> DB2_B_OUTBYPASS_W<DB2_CFG_SPEC> {
        DB2_B_OUTBYPASS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    #[must_use]
    pub fn db2_clk_sel(&mut self) -> DB2_CLK_SEL_W<DB2_CFG_SPEC> {
        DB2_CLK_SEL_W::new(self, 17)
    }
}
#[doc = "dead time type selection and configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db2_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db2_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DB2_CFG_SPEC;
impl crate::RegisterSpec for DB2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`db2_cfg::R`](R) reader structure"]
impl crate::Readable for DB2_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`db2_cfg::W`](W) writer structure"]
impl crate::Writable for DB2_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DB2_CFG to value 0x0001_8000"]
impl crate::Resettable for DB2_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0001_8000;
}
