#[doc = "Register `OUT_INT_CLR_CH2` writer"]
pub type W = crate::W<OUT_INT_CLR_CH2_SPEC>;
#[doc = "Field `OUT_DONE_CH2_INT_CLR` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH2_INT_CLR` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH2_INT_CLR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH2_INT_CLR` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L1_CH2_INT_CLR` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L1_CH2_INT_CLR` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L2_CH2_INT_CLR` writer - Set this bit to clear the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OUTFIFO_OVF_L2_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L2_CH2_INT_CLR` writer - Set this bit to clear the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OUTFIFO_UDF_L2_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_TASK_OVF_CH2_INT_CLR` writer - Set this bit to clear the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
pub type OUT_DSCR_TASK_OVF_CH2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_CLR_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_ch2_int_clr(&mut self) -> OUT_DONE_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUT_DONE_CH2_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_ch2_int_clr(&mut self) -> OUT_EOF_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUT_EOF_CH2_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_ch2_int_clr(&mut self) -> OUT_DSCR_ERR_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUT_DSCR_ERR_CH2_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_ch2_int_clr(
        &mut self,
    ) -> OUT_TOTAL_EOF_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUT_TOTAL_EOF_CH2_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l1_ch2_int_clr(
        &mut self,
    ) -> OUTFIFO_OVF_L1_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUTFIFO_OVF_L1_CH2_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l1_ch2_int_clr(
        &mut self,
    ) -> OUTFIFO_UDF_L1_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUTFIFO_UDF_L1_CH2_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l2_ch2_int_clr(
        &mut self,
    ) -> OUTFIFO_OVF_L2_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUTFIFO_OVF_L2_CH2_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l2_ch2_int_clr(
        &mut self,
    ) -> OUTFIFO_UDF_L2_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUTFIFO_UDF_L2_CH2_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_task_ovf_ch2_int_clr(
        &mut self,
    ) -> OUT_DSCR_TASK_OVF_CH2_INT_CLR_W<OUT_INT_CLR_CH2_SPEC> {
        OUT_DSCR_TASK_OVF_CH2_INT_CLR_W::new(self, 8)
    }
}
#[doc = "TX CH2 interrupt clr register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_CLR_CH2_SPEC;
impl crate::RegisterSpec for OUT_INT_CLR_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_int_clr_ch2::W`](W) writer structure"]
impl crate::Writable for OUT_INT_CLR_CH2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_INT_CLR_CH2 to value 0"]
impl crate::Resettable for OUT_INT_CLR_CH2_SPEC {
    const RESET_VALUE: u32 = 0;
}
