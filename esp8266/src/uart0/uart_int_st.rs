#[doc = "Register `UART_INT_ST` reader"]
pub struct R(crate::R<UART_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rxfifo_full_int_st` reader - The interrupt state bit for RX fifo full event"]
pub type RXFIFO_FULL_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `txfifo_empty_int_st` reader - The interrupt state bit for TX fifo empty"]
pub type TXFIFO_EMPTY_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `parity_err_int_st` reader - The interrupt state bit for rx parity error"]
pub type PARITY_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `frm_err_int_st` reader - The interrupt state for other rx error"]
pub type FRM_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `rxfifo_ovf_int_st` reader - The interrupt state bit for RX fifo overflow"]
pub type RXFIFO_OVF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `dsr_chg_int_st` reader - The interrupt state bit for DSR changing level"]
pub type DSR_CHG_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `cts_chg_int_st` reader - The interrupt state bit for CTS changing level"]
pub type CTS_CHG_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `brk_det_int_st` reader - The interrupt state bit for rx byte start error"]
pub type BRK_DET_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `rxfifo_tout_int_st` reader - The interrupt state bit for Rx time-out event"]
pub type RXFIFO_TOUT_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The interrupt state bit for RX fifo full event"]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> RXFIFO_FULL_INT_ST_R {
        RXFIFO_FULL_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt state bit for TX fifo empty"]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> TXFIFO_EMPTY_INT_ST_R {
        TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt state bit for rx parity error"]
    #[inline(always)]
    pub fn parity_err_int_st(&self) -> PARITY_ERR_INT_ST_R {
        PARITY_ERR_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt state for other rx error"]
    #[inline(always)]
    pub fn frm_err_int_st(&self) -> FRM_ERR_INT_ST_R {
        FRM_ERR_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt state bit for RX fifo overflow"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt state bit for DSR changing level"]
    #[inline(always)]
    pub fn dsr_chg_int_st(&self) -> DSR_CHG_INT_ST_R {
        DSR_CHG_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt state bit for CTS changing level"]
    #[inline(always)]
    pub fn cts_chg_int_st(&self) -> CTS_CHG_INT_ST_R {
        CTS_CHG_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt state bit for rx byte start error"]
    #[inline(always)]
    pub fn brk_det_int_st(&self) -> BRK_DET_INT_ST_R {
        BRK_DET_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt state bit for Rx time-out event"]
    #[inline(always)]
    pub fn rxfifo_tout_int_st(&self) -> RXFIFO_TOUT_INT_ST_R {
        RXFIFO_TOUT_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "UART INTERRUPT STATEREGISTERUART_INT_RAW&UART_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_st](index.html) module"]
pub struct UART_INT_ST_SPEC;
impl crate::RegisterSpec for UART_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_st::R](R) reader structure"]
impl crate::Readable for UART_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_INT_ST to value 0"]
impl crate::Resettable for UART_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
