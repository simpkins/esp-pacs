#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RX_TAKE_DATA_INT_ENA` reader - The interrupt enable bit for I2S_RX_TAKE_DATA_INT interrupt."]
pub type RX_TAKE_DATA_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_TAKE_DATA_INT_ENA` writer - The interrupt enable bit for I2S_RX_TAKE_DATA_INT interrupt."]
pub type RX_TAKE_DATA_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PUT_DATA_INT_ENA` reader - The interrupt enable bit for I2S_TX_PUT_DATA_INT interrupt."]
pub type TX_PUT_DATA_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_PUT_DATA_INT_ENA` writer - The interrupt enable bit for I2S_TX_PUT_DATA_INT interrupt."]
pub type TX_PUT_DATA_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_WFULL_INT_ENA` reader - The interrupt enable bit for I2S_RX_WFULL_INT interrupt."]
pub type RX_WFULL_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_WFULL_INT_ENA` writer - The interrupt enable bit for I2S_RX_WFULL_INT interrupt."]
pub type RX_WFULL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REMPTY_INT_ENA` reader - The interrupt enable bit for I2S_RX_REMPTY_INT interrupt."]
pub type RX_REMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_REMPTY_INT_ENA` writer - The interrupt enable bit for I2S_RX_REMPTY_INT interrupt."]
pub type RX_REMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_WFULL_INT_ENA` reader - The interrupt enable bit for I2S_TX_WFULL_INT interrupt."]
pub type TX_WFULL_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_WFULL_INT_ENA` writer - The interrupt enable bit for I2S_TX_WFULL_INT interrupt."]
pub type TX_WFULL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_REMPTY_INT_ENA` reader - The interrupt enable bit for I2S_TX_REMPTY_INT interrupt."]
pub type TX_REMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_REMPTY_INT_ENA` writer - The interrupt enable bit for I2S_TX_REMPTY_INT interrupt."]
pub type TX_REMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_ENA` reader - The interrupt enable bit for I2S_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_ENA` writer - The interrupt enable bit for I2S_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG_INT_ENA` reader - The interrupt enable bit for I2S_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_ENA` writer - The interrupt enable bit for I2S_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DONE_INT_ENA` reader - The interrupt enable bit for I2S_IN_DONE_INT interrupt."]
pub type IN_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_ENA` writer - The interrupt enable bit for I2S_IN_DONE_INT interrupt."]
pub type IN_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_INT_ENA` reader - The interrupt enable bit for I2S_IN_SUC_EOF_INT interrupt."]
pub type IN_SUC_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_ENA` writer - The interrupt enable bit for I2S_IN_SUC_EOF_INT interrupt."]
pub type IN_SUC_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_INT_ENA` reader - Reserved."]
pub type IN_ERR_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_ENA` writer - Reserved."]
pub type IN_ERR_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DONE_INT_ENA` reader - The interrupt enable bit for I2S_OUT_DONE_INT interrupt."]
pub type OUT_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_ENA` writer - The interrupt enable bit for I2S_OUT_DONE_INT interrupt."]
pub type OUT_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_ENA` reader - The interrupt enable bit for I2S_OUT_EOF_INT interrupt."]
pub type OUT_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_ENA` writer - The interrupt enable bit for I2S_OUT_EOF_INT interrupt."]
pub type OUT_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_INT_ENA` reader - The interrupt enable bit for I2S_IN_DSCR_ERR_INT interrupt."]
pub type IN_DSCR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_INT_ENA` writer - The interrupt enable bit for I2S_IN_DSCR_ERR_INT interrupt."]
pub type IN_DSCR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_INT_ENA` reader - The interrupt enable bit for I2S_OUT_DSCR_ERR_INT interrupt."]
pub type OUT_DSCR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_INT_ENA` writer - The interrupt enable bit for I2S_OUT_DSCR_ERR_INT interrupt."]
pub type OUT_DSCR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_INT_ENA` reader - The interrupt enable bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
pub type IN_DSCR_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_INT_ENA` writer - The interrupt enable bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
pub type IN_DSCR_EMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` reader - The interrupt enable bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
pub type OUT_TOTAL_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` writer - The interrupt enable bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
pub type OUT_TOTAL_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V_SYNC_INT_ENA` reader - The interrupt enable bit for I2S_V_SYNC_INT interrupt."]
pub type V_SYNC_INT_ENA_R = crate::BitReader;
#[doc = "Field `V_SYNC_INT_ENA` writer - The interrupt enable bit for I2S_V_SYNC_INT interrupt."]
pub type V_SYNC_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for I2S_RX_TAKE_DATA_INT interrupt."]
    #[inline(always)]
    pub fn rx_take_data_int_ena(&self) -> RX_TAKE_DATA_INT_ENA_R {
        RX_TAKE_DATA_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2S_TX_PUT_DATA_INT interrupt."]
    #[inline(always)]
    pub fn tx_put_data_int_ena(&self) -> TX_PUT_DATA_INT_ENA_R {
        TX_PUT_DATA_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2S_RX_WFULL_INT interrupt."]
    #[inline(always)]
    pub fn rx_wfull_int_ena(&self) -> RX_WFULL_INT_ENA_R {
        RX_WFULL_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for I2S_RX_REMPTY_INT interrupt."]
    #[inline(always)]
    pub fn rx_rempty_int_ena(&self) -> RX_REMPTY_INT_ENA_R {
        RX_REMPTY_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for I2S_TX_WFULL_INT interrupt."]
    #[inline(always)]
    pub fn tx_wfull_int_ena(&self) -> TX_WFULL_INT_ENA_R {
        TX_WFULL_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for I2S_TX_REMPTY_INT interrupt."]
    #[inline(always)]
    pub fn tx_rempty_int_ena(&self) -> TX_REMPTY_INT_ENA_R {
        TX_REMPTY_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2S_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for I2S_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for I2S_IN_DONE_INT interrupt."]
    #[inline(always)]
    pub fn in_done_int_ena(&self) -> IN_DONE_INT_ENA_R {
        IN_DONE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for I2S_IN_SUC_EOF_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_int_ena(&self) -> IN_SUC_EOF_INT_ENA_R {
        IN_SUC_EOF_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved."]
    #[inline(always)]
    pub fn in_err_eof_int_ena(&self) -> IN_ERR_EOF_INT_ENA_R {
        IN_ERR_EOF_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2S_OUT_DONE_INT interrupt."]
    #[inline(always)]
    pub fn out_done_int_ena(&self) -> OUT_DONE_INT_ENA_R {
        OUT_DONE_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2S_OUT_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_int_ena(&self) -> OUT_EOF_INT_ENA_R {
        OUT_EOF_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2S_IN_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_int_ena(&self) -> IN_DSCR_ERR_INT_ENA_R {
        IN_DSCR_ERR_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2S_OUT_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_int_ena(&self) -> OUT_DSCR_ERR_INT_ENA_R {
        OUT_DSCR_ERR_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_int_ena(&self) -> IN_DSCR_EMPTY_INT_ENA_R {
        IN_DSCR_EMPTY_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_int_ena(&self) -> OUT_TOTAL_EOF_INT_ENA_R {
        OUT_TOTAL_EOF_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for I2S_V_SYNC_INT interrupt."]
    #[inline(always)]
    pub fn v_sync_int_ena(&self) -> V_SYNC_INT_ENA_R {
        V_SYNC_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "rx_take_data_int_ena",
                &format_args!("{}", self.rx_take_data_int_ena().bit()),
            )
            .field(
                "tx_put_data_int_ena",
                &format_args!("{}", self.tx_put_data_int_ena().bit()),
            )
            .field(
                "rx_wfull_int_ena",
                &format_args!("{}", self.rx_wfull_int_ena().bit()),
            )
            .field(
                "rx_rempty_int_ena",
                &format_args!("{}", self.rx_rempty_int_ena().bit()),
            )
            .field(
                "tx_wfull_int_ena",
                &format_args!("{}", self.tx_wfull_int_ena().bit()),
            )
            .field(
                "tx_rempty_int_ena",
                &format_args!("{}", self.tx_rempty_int_ena().bit()),
            )
            .field(
                "rx_hung_int_ena",
                &format_args!("{}", self.rx_hung_int_ena().bit()),
            )
            .field(
                "tx_hung_int_ena",
                &format_args!("{}", self.tx_hung_int_ena().bit()),
            )
            .field(
                "in_done_int_ena",
                &format_args!("{}", self.in_done_int_ena().bit()),
            )
            .field(
                "in_suc_eof_int_ena",
                &format_args!("{}", self.in_suc_eof_int_ena().bit()),
            )
            .field(
                "in_err_eof_int_ena",
                &format_args!("{}", self.in_err_eof_int_ena().bit()),
            )
            .field(
                "out_done_int_ena",
                &format_args!("{}", self.out_done_int_ena().bit()),
            )
            .field(
                "out_eof_int_ena",
                &format_args!("{}", self.out_eof_int_ena().bit()),
            )
            .field(
                "in_dscr_err_int_ena",
                &format_args!("{}", self.in_dscr_err_int_ena().bit()),
            )
            .field(
                "out_dscr_err_int_ena",
                &format_args!("{}", self.out_dscr_err_int_ena().bit()),
            )
            .field(
                "in_dscr_empty_int_ena",
                &format_args!("{}", self.in_dscr_empty_int_ena().bit()),
            )
            .field(
                "out_total_eof_int_ena",
                &format_args!("{}", self.out_total_eof_int_ena().bit()),
            )
            .field(
                "v_sync_int_ena",
                &format_args!("{}", self.v_sync_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for I2S_RX_TAKE_DATA_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_take_data_int_ena(&mut self) -> RX_TAKE_DATA_INT_ENA_W<INT_ENA_SPEC> {
        RX_TAKE_DATA_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2S_TX_PUT_DATA_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_put_data_int_ena(&mut self) -> TX_PUT_DATA_INT_ENA_W<INT_ENA_SPEC> {
        TX_PUT_DATA_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2S_RX_WFULL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wfull_int_ena(&mut self) -> RX_WFULL_INT_ENA_W<INT_ENA_SPEC> {
        RX_WFULL_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for I2S_RX_REMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_rempty_int_ena(&mut self) -> RX_REMPTY_INT_ENA_W<INT_ENA_SPEC> {
        RX_REMPTY_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for I2S_TX_WFULL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wfull_int_ena(&mut self) -> TX_WFULL_INT_ENA_W<INT_ENA_SPEC> {
        TX_WFULL_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for I2S_TX_REMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_rempty_int_ena(&mut self) -> TX_REMPTY_INT_ENA_W<INT_ENA_SPEC> {
        TX_REMPTY_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2S_RX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        RX_HUNG_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for I2S_TX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        TX_HUNG_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for I2S_IN_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_int_ena(&mut self) -> IN_DONE_INT_ENA_W<INT_ENA_SPEC> {
        IN_DONE_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for I2S_IN_SUC_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_int_ena(&mut self) -> IN_SUC_EOF_INT_ENA_W<INT_ENA_SPEC> {
        IN_SUC_EOF_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_int_ena(&mut self) -> IN_ERR_EOF_INT_ENA_W<INT_ENA_SPEC> {
        IN_ERR_EOF_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2S_OUT_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_int_ena(&mut self) -> OUT_DONE_INT_ENA_W<INT_ENA_SPEC> {
        OUT_DONE_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2S_OUT_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_ena(&mut self) -> OUT_EOF_INT_ENA_W<INT_ENA_SPEC> {
        OUT_EOF_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2S_IN_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_int_ena(&mut self) -> IN_DSCR_ERR_INT_ENA_W<INT_ENA_SPEC> {
        IN_DSCR_ERR_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2S_OUT_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_int_ena(&mut self) -> OUT_DSCR_ERR_INT_ENA_W<INT_ENA_SPEC> {
        OUT_DSCR_ERR_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_int_ena(&mut self) -> IN_DSCR_EMPTY_INT_ENA_W<INT_ENA_SPEC> {
        IN_DSCR_EMPTY_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt enable bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_int_ena(&mut self) -> OUT_TOTAL_EOF_INT_ENA_W<INT_ENA_SPEC> {
        OUT_TOTAL_EOF_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enable bit for I2S_V_SYNC_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn v_sync_int_ena(&mut self) -> V_SYNC_INT_ENA_W<INT_ENA_SPEC> {
        V_SYNC_INT_ENA_W::new(self, 17)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
