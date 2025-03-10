#[doc = "Register `OUT_INT_RAW_CH3` reader"]
pub type R = crate::R<OUT_INT_RAW_CH3_SPEC>;
#[doc = "Register `OUT_INT_RAW_CH3` writer"]
pub type W = crate::W<OUT_INT_RAW_CH3_SPEC>;
#[doc = "Field `OUT_DONE_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DONE_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L1_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OUTFIFO_OVF_L1_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OUTFIFO_OVF_L1_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L1_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OUTFIFO_UDF_L1_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OUTFIFO_UDF_L1_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L2_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OUTFIFO_OVF_L2_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L2_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OUTFIFO_OVF_L2_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L2_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OUTFIFO_UDF_L2_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L2_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OUTFIFO_UDF_L2_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_TASK_OVF_CH3_INT_RAW` reader - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type OUT_DSCR_TASK_OVF_CH3_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_TASK_OVF_CH3_INT_RAW` writer - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type OUT_DSCR_TASK_OVF_CH3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done_ch3_int_raw(&self) -> OUT_DONE_CH3_INT_RAW_R {
        OUT_DONE_CH3_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof_ch3_int_raw(&self) -> OUT_EOF_CH3_INT_RAW_R {
        OUT_EOF_CH3_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err_ch3_int_raw(&self) -> OUT_DSCR_ERR_CH3_INT_RAW_R {
        OUT_DSCR_ERR_CH3_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof_ch3_int_raw(&self) -> OUT_TOTAL_EOF_CH3_INT_RAW_R {
        OUT_TOTAL_EOF_CH3_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l1_ch3_int_raw(&self) -> OUTFIFO_OVF_L1_CH3_INT_RAW_R {
        OUTFIFO_OVF_L1_CH3_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l1_ch3_int_raw(&self) -> OUTFIFO_UDF_L1_CH3_INT_RAW_R {
        OUTFIFO_UDF_L1_CH3_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l2_ch3_int_raw(&self) -> OUTFIFO_OVF_L2_CH3_INT_RAW_R {
        OUTFIFO_OVF_L2_CH3_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l2_ch3_int_raw(&self) -> OUTFIFO_UDF_L2_CH3_INT_RAW_R {
        OUTFIFO_UDF_L2_CH3_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    pub fn out_dscr_task_ovf_ch3_int_raw(&self) -> OUT_DSCR_TASK_OVF_CH3_INT_RAW_R {
        OUT_DSCR_TASK_OVF_CH3_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_INT_RAW_CH3")
            .field(
                "out_done_ch3_int_raw",
                &format_args!("{}", self.out_done_ch3_int_raw().bit()),
            )
            .field(
                "out_eof_ch3_int_raw",
                &format_args!("{}", self.out_eof_ch3_int_raw().bit()),
            )
            .field(
                "out_dscr_err_ch3_int_raw",
                &format_args!("{}", self.out_dscr_err_ch3_int_raw().bit()),
            )
            .field(
                "out_total_eof_ch3_int_raw",
                &format_args!("{}", self.out_total_eof_ch3_int_raw().bit()),
            )
            .field(
                "outfifo_ovf_l1_ch3_int_raw",
                &format_args!("{}", self.outfifo_ovf_l1_ch3_int_raw().bit()),
            )
            .field(
                "outfifo_udf_l1_ch3_int_raw",
                &format_args!("{}", self.outfifo_udf_l1_ch3_int_raw().bit()),
            )
            .field(
                "outfifo_ovf_l2_ch3_int_raw",
                &format_args!("{}", self.outfifo_ovf_l2_ch3_int_raw().bit()),
            )
            .field(
                "outfifo_udf_l2_ch3_int_raw",
                &format_args!("{}", self.outfifo_udf_l2_ch3_int_raw().bit()),
            )
            .field(
                "out_dscr_task_ovf_ch3_int_raw",
                &format_args!("{}", self.out_dscr_task_ovf_ch3_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_RAW_CH3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_ch3_int_raw(&mut self) -> OUT_DONE_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUT_DONE_CH3_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_ch3_int_raw(&mut self) -> OUT_EOF_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUT_EOF_CH3_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_ch3_int_raw(&mut self) -> OUT_DSCR_ERR_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUT_DSCR_ERR_CH3_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_ch3_int_raw(
        &mut self,
    ) -> OUT_TOTAL_EOF_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUT_TOTAL_EOF_CH3_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l1_ch3_int_raw(
        &mut self,
    ) -> OUTFIFO_OVF_L1_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUTFIFO_OVF_L1_CH3_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l1_ch3_int_raw(
        &mut self,
    ) -> OUTFIFO_UDF_L1_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUTFIFO_UDF_L1_CH3_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l2_ch3_int_raw(
        &mut self,
    ) -> OUTFIFO_OVF_L2_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUTFIFO_OVF_L2_CH3_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l2_ch3_int_raw(
        &mut self,
    ) -> OUTFIFO_UDF_L2_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUTFIFO_UDF_L2_CH3_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_task_ovf_ch3_int_raw(
        &mut self,
    ) -> OUT_DSCR_TASK_OVF_CH3_INT_RAW_W<OUT_INT_RAW_CH3_SPEC> {
        OUT_DSCR_TASK_OVF_CH3_INT_RAW_W::new(self, 8)
    }
}
#[doc = "TX CH3 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_RAW_CH3_SPEC;
impl crate::RegisterSpec for OUT_INT_RAW_CH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_int_raw_ch3::R`](R) reader structure"]
impl crate::Readable for OUT_INT_RAW_CH3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_int_raw_ch3::W`](W) writer structure"]
impl crate::Writable for OUT_INT_RAW_CH3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_INT_RAW_CH3 to value 0"]
impl crate::Resettable for OUT_INT_RAW_CH3_SPEC {
    const RESET_VALUE: u32 = 0;
}
