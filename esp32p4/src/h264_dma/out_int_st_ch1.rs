#[doc = "Register `OUT_INT_ST_CH1` reader"]
pub type R = crate::R<OUT_INT_ST_CH1_SPEC>;
#[doc = "Field `OUT_DONE_CH1_INT_ST` reader - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_EOF_CH1_INT_ST` reader - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH1_INT_ST` reader - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH1_INT_ST` reader - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1_CH1_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1_CH1_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L2_CH1_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OUTFIFO_OVF_L2_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L2_CH1_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OUTFIFO_UDF_L2_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_TASK_OVF_CH1_INT_ST` reader - The raw interrupt status bit for the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
pub type OUT_DSCR_TASK_OVF_CH1_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch1_int_st(&self) -> OUT_DONE_CH1_INT_ST_R {
        OUT_DONE_CH1_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch1_int_st(&self) -> OUT_EOF_CH1_INT_ST_R {
        OUT_EOF_CH1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch1_int_st(&self) -> OUT_DSCR_ERR_CH1_INT_ST_R {
        OUT_DSCR_ERR_CH1_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch1_int_st(&self) -> OUT_TOTAL_EOF_CH1_INT_ST_R {
        OUT_TOTAL_EOF_CH1_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1_ch1_int_st(&self) -> OUTFIFO_OVF_L1_CH1_INT_ST_R {
        OUTFIFO_OVF_L1_CH1_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1_ch1_int_st(&self) -> OUTFIFO_UDF_L1_CH1_INT_ST_R {
        OUTFIFO_UDF_L1_CH1_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l2_ch1_int_st(&self) -> OUTFIFO_OVF_L2_CH1_INT_ST_R {
        OUTFIFO_OVF_L2_CH1_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l2_ch1_int_st(&self) -> OUTFIFO_UDF_L2_CH1_INT_ST_R {
        OUTFIFO_UDF_L2_CH1_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_task_ovf_ch1_int_st(&self) -> OUT_DSCR_TASK_OVF_CH1_INT_ST_R {
        OUT_DSCR_TASK_OVF_CH1_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_INT_ST_CH1")
            .field(
                "out_done_ch1_int_st",
                &format_args!("{}", self.out_done_ch1_int_st().bit()),
            )
            .field(
                "out_eof_ch1_int_st",
                &format_args!("{}", self.out_eof_ch1_int_st().bit()),
            )
            .field(
                "out_dscr_err_ch1_int_st",
                &format_args!("{}", self.out_dscr_err_ch1_int_st().bit()),
            )
            .field(
                "out_total_eof_ch1_int_st",
                &format_args!("{}", self.out_total_eof_ch1_int_st().bit()),
            )
            .field(
                "outfifo_ovf_l1_ch1_int_st",
                &format_args!("{}", self.outfifo_ovf_l1_ch1_int_st().bit()),
            )
            .field(
                "outfifo_udf_l1_ch1_int_st",
                &format_args!("{}", self.outfifo_udf_l1_ch1_int_st().bit()),
            )
            .field(
                "outfifo_ovf_l2_ch1_int_st",
                &format_args!("{}", self.outfifo_ovf_l2_ch1_int_st().bit()),
            )
            .field(
                "outfifo_udf_l2_ch1_int_st",
                &format_args!("{}", self.outfifo_udf_l2_ch1_int_st().bit()),
            )
            .field(
                "out_dscr_task_ovf_ch1_int_st",
                &format_args!("{}", self.out_dscr_task_ovf_ch1_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_ST_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CH1 interrupt st register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_ST_CH1_SPEC;
impl crate::RegisterSpec for OUT_INT_ST_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_int_st_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_INT_ST_CH1_SPEC {}
#[doc = "`reset()` method sets OUT_INT_ST_CH1 to value 0"]
impl crate::Resettable for OUT_INT_ST_CH1_SPEC {
    const RESET_VALUE: u32 = 0;
}
