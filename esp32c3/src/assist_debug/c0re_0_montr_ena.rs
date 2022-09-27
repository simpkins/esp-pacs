#[doc = "Register `C0RE_0_MONTR_ENA` reader"]
pub struct R(crate::R<C0RE_0_MONTR_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0RE_0_MONTR_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0RE_0_MONTR_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0RE_0_MONTR_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C0RE_0_MONTR_ENA` writer"]
pub struct W(crate::W<C0RE_0_MONTR_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C0RE_0_MONTR_ENA_SPEC>;
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
impl From<crate::W<C0RE_0_MONTR_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C0RE_0_MONTR_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_ENA` reader - reg_core_0_area_dram0_0_rd_ena"]
pub type CORE_0_AREA_DRAM0_0_RD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_ENA` writer - reg_core_0_area_dram0_0_rd_ena"]
pub type CORE_0_AREA_DRAM0_0_RD_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_ENA` reader - reg_core_0_area_dram0_0_wr_ena"]
pub type CORE_0_AREA_DRAM0_0_WR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_ENA` writer - reg_core_0_area_dram0_0_wr_ena"]
pub type CORE_0_AREA_DRAM0_0_WR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_ENA` reader - reg_core_0_area_dram0_1_rd_ena"]
pub type CORE_0_AREA_DRAM0_1_RD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_ENA` writer - reg_core_0_area_dram0_1_rd_ena"]
pub type CORE_0_AREA_DRAM0_1_RD_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_ENA` reader - reg_core_0_area_dram0_1_wr_ena"]
pub type CORE_0_AREA_DRAM0_1_WR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_ENA` writer - reg_core_0_area_dram0_1_wr_ena"]
pub type CORE_0_AREA_DRAM0_1_WR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_ENA` reader - reg_core_0_area_pif_0_rd_ena"]
pub type CORE_0_AREA_PIF_0_RD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_ENA` writer - reg_core_0_area_pif_0_rd_ena"]
pub type CORE_0_AREA_PIF_0_RD_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_ENA` reader - reg_core_0_area_pif_0_wr_ena"]
pub type CORE_0_AREA_PIF_0_WR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_ENA` writer - reg_core_0_area_pif_0_wr_ena"]
pub type CORE_0_AREA_PIF_0_WR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_ENA` reader - reg_core_0_area_pif_1_rd_ena"]
pub type CORE_0_AREA_PIF_1_RD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_ENA` writer - reg_core_0_area_pif_1_rd_ena"]
pub type CORE_0_AREA_PIF_1_RD_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_ENA` reader - reg_core_0_area_pif_1_wr_ena"]
pub type CORE_0_AREA_PIF_1_WR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_ENA` writer - reg_core_0_area_pif_1_wr_ena"]
pub type CORE_0_AREA_PIF_1_WR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` reader - reg_core_0_sp_spill_min_ena"]
pub type CORE_0_SP_SPILL_MIN_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_ENA` writer - reg_core_0_sp_spill_min_ena"]
pub type CORE_0_SP_SPILL_MIN_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` reader - reg_core_0_sp_spill_max_ena"]
pub type CORE_0_SP_SPILL_MAX_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_ENA` writer - reg_core_0_sp_spill_max_ena"]
pub type CORE_0_SP_SPILL_MAX_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_ENA` reader - reg_core_0_iram0_exception_monitor_ena"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_ENA` writer - reg_core_0_iram0_exception_monitor_ena"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_ENA` reader - reg_core_0_dram0_exception_monitor_ena"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_ENA` writer - reg_core_0_dram0_exception_monitor_ena"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C0RE_0_MONTR_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - reg_core_0_area_dram0_0_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_ena(&self) -> CORE_0_AREA_DRAM0_0_RD_ENA_R {
        CORE_0_AREA_DRAM0_0_RD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_core_0_area_dram0_0_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_ena(&self) -> CORE_0_AREA_DRAM0_0_WR_ENA_R {
        CORE_0_AREA_DRAM0_0_WR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_core_0_area_dram0_1_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_ena(&self) -> CORE_0_AREA_DRAM0_1_RD_ENA_R {
        CORE_0_AREA_DRAM0_1_RD_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_core_0_area_dram0_1_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_ena(&self) -> CORE_0_AREA_DRAM0_1_WR_ENA_R {
        CORE_0_AREA_DRAM0_1_WR_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_core_0_area_pif_0_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_ena(&self) -> CORE_0_AREA_PIF_0_RD_ENA_R {
        CORE_0_AREA_PIF_0_RD_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_core_0_area_pif_0_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_ena(&self) -> CORE_0_AREA_PIF_0_WR_ENA_R {
        CORE_0_AREA_PIF_0_WR_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_core_0_area_pif_1_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_ena(&self) -> CORE_0_AREA_PIF_1_RD_ENA_R {
        CORE_0_AREA_PIF_1_RD_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_core_0_area_pif_1_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_ena(&self) -> CORE_0_AREA_PIF_1_WR_ENA_R {
        CORE_0_AREA_PIF_1_WR_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_core_0_sp_spill_min_ena"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&self) -> CORE_0_SP_SPILL_MIN_ENA_R {
        CORE_0_SP_SPILL_MIN_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_core_0_sp_spill_max_ena"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&self) -> CORE_0_SP_SPILL_MAX_ENA_R {
        CORE_0_SP_SPILL_MAX_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_core_0_iram0_exception_monitor_ena"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_ena(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_core_0_dram0_exception_monitor_ena"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_ena(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_core_0_area_dram0_0_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_ena(&mut self) -> CORE_0_AREA_DRAM0_0_RD_ENA_W<0> {
        CORE_0_AREA_DRAM0_0_RD_ENA_W::new(self)
    }
    #[doc = "Bit 1 - reg_core_0_area_dram0_0_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_ena(&mut self) -> CORE_0_AREA_DRAM0_0_WR_ENA_W<1> {
        CORE_0_AREA_DRAM0_0_WR_ENA_W::new(self)
    }
    #[doc = "Bit 2 - reg_core_0_area_dram0_1_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_ena(&mut self) -> CORE_0_AREA_DRAM0_1_RD_ENA_W<2> {
        CORE_0_AREA_DRAM0_1_RD_ENA_W::new(self)
    }
    #[doc = "Bit 3 - reg_core_0_area_dram0_1_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_ena(&mut self) -> CORE_0_AREA_DRAM0_1_WR_ENA_W<3> {
        CORE_0_AREA_DRAM0_1_WR_ENA_W::new(self)
    }
    #[doc = "Bit 4 - reg_core_0_area_pif_0_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_ena(&mut self) -> CORE_0_AREA_PIF_0_RD_ENA_W<4> {
        CORE_0_AREA_PIF_0_RD_ENA_W::new(self)
    }
    #[doc = "Bit 5 - reg_core_0_area_pif_0_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_ena(&mut self) -> CORE_0_AREA_PIF_0_WR_ENA_W<5> {
        CORE_0_AREA_PIF_0_WR_ENA_W::new(self)
    }
    #[doc = "Bit 6 - reg_core_0_area_pif_1_rd_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_ena(&mut self) -> CORE_0_AREA_PIF_1_RD_ENA_W<6> {
        CORE_0_AREA_PIF_1_RD_ENA_W::new(self)
    }
    #[doc = "Bit 7 - reg_core_0_area_pif_1_wr_ena"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_ena(&mut self) -> CORE_0_AREA_PIF_1_WR_ENA_W<7> {
        CORE_0_AREA_PIF_1_WR_ENA_W::new(self)
    }
    #[doc = "Bit 8 - reg_core_0_sp_spill_min_ena"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&mut self) -> CORE_0_SP_SPILL_MIN_ENA_W<8> {
        CORE_0_SP_SPILL_MIN_ENA_W::new(self)
    }
    #[doc = "Bit 9 - reg_core_0_sp_spill_max_ena"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&mut self) -> CORE_0_SP_SPILL_MAX_ENA_W<9> {
        CORE_0_SP_SPILL_MAX_ENA_W::new(self)
    }
    #[doc = "Bit 10 - reg_core_0_iram0_exception_monitor_ena"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_ena(
        &mut self,
    ) -> CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W<10> {
        CORE_0_IRAM0_EXCEPTION_MONITOR_ENA_W::new(self)
    }
    #[doc = "Bit 11 - reg_core_0_dram0_exception_monitor_ena"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_ena(
        &mut self,
    ) -> CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W<11> {
        CORE_0_DRAM0_EXCEPTION_MONITOR_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0re_0_montr_ena](index.html) module"]
pub struct C0RE_0_MONTR_ENA_SPEC;
impl crate::RegisterSpec for C0RE_0_MONTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c0re_0_montr_ena::R](R) reader structure"]
impl crate::Readable for C0RE_0_MONTR_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c0re_0_montr_ena::W](W) writer structure"]
impl crate::Writable for C0RE_0_MONTR_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C0RE_0_MONTR_ENA to value 0"]
impl crate::Resettable for C0RE_0_MONTR_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
