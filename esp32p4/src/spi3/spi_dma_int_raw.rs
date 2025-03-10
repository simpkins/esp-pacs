#[doc = "Register `SPI_DMA_INT_RAW` reader"]
pub type R = crate::R<SPI_DMA_INT_RAW_SPEC>;
#[doc = "Register `SPI_DMA_INT_RAW` writer"]
pub type W = crate::W<SPI_DMA_INT_RAW_SPEC>;
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_RAW` reader - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_RAW` writer - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW` reader - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW` writer - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_EX_QPI_INT_RAW` reader - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
pub type SPI_SLV_EX_QPI_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_EX_QPI_INT_RAW` writer - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
pub type SPI_SLV_EX_QPI_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_EN_QPI_INT_RAW` reader - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
pub type SPI_SLV_EN_QPI_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_EN_QPI_INT_RAW` writer - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
pub type SPI_SLV_EN_QPI_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD7_INT_RAW` reader - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
pub type SPI_SLV_CMD7_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD7_INT_RAW` writer - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
pub type SPI_SLV_CMD7_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD8_INT_RAW` reader - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
pub type SPI_SLV_CMD8_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD8_INT_RAW` writer - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
pub type SPI_SLV_CMD8_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD9_INT_RAW` reader - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
pub type SPI_SLV_CMD9_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD9_INT_RAW` writer - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
pub type SPI_SLV_CMD9_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMDA_INT_RAW` reader - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
pub type SPI_SLV_CMDA_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMDA_INT_RAW` writer - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
pub type SPI_SLV_CMDA_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_RAW` reader - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
pub type SPI_SLV_RD_DMA_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_RAW` writer - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
pub type SPI_SLV_RD_DMA_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_RAW` reader - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
pub type SPI_SLV_WR_DMA_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_RAW` writer - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
pub type SPI_SLV_WR_DMA_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_RAW` reader - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
pub type SPI_SLV_RD_BUF_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_RAW` writer - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
pub type SPI_SLV_RD_BUF_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_RAW` reader - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
pub type SPI_SLV_WR_BUF_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_RAW` writer - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
pub type SPI_SLV_WR_BUF_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_TRANS_DONE_INT_RAW` reader - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
pub type SPI_TRANS_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_TRANS_DONE_INT_RAW` writer - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
pub type SPI_TRANS_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_RAW` reader - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_RAW` writer - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_RAW` reader - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_RAW` writer - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_RAW` reader - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
pub type SPI_SLV_CMD_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_RAW` writer - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
pub type SPI_SLV_CMD_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW` reader - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW` writer - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW` reader - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW` writer - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_APP2_INT_RAW` reader - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
pub type SPI_APP2_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_APP2_INT_RAW` writer - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
pub type SPI_APP2_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_APP1_INT_RAW` reader - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
pub type SPI_APP1_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_APP1_INT_RAW` writer - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
pub type SPI_APP1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_raw(&self) -> SPI_DMA_INFIFO_FULL_ERR_INT_RAW_R {
        SPI_DMA_INFIFO_FULL_ERR_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_raw(&self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_R {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_raw(&self) -> SPI_SLV_EX_QPI_INT_RAW_R {
        SPI_SLV_EX_QPI_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_raw(&self) -> SPI_SLV_EN_QPI_INT_RAW_R {
        SPI_SLV_EN_QPI_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_raw(&self) -> SPI_SLV_CMD7_INT_RAW_R {
        SPI_SLV_CMD7_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_raw(&self) -> SPI_SLV_CMD8_INT_RAW_R {
        SPI_SLV_CMD8_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_raw(&self) -> SPI_SLV_CMD9_INT_RAW_R {
        SPI_SLV_CMD9_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_cmda_int_raw(&self) -> SPI_SLV_CMDA_INT_RAW_R {
        SPI_SLV_CMDA_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_raw(&self) -> SPI_SLV_RD_DMA_DONE_INT_RAW_R {
        SPI_SLV_RD_DMA_DONE_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_raw(&self) -> SPI_SLV_WR_DMA_DONE_INT_RAW_R {
        SPI_SLV_WR_DMA_DONE_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_raw(&self) -> SPI_SLV_RD_BUF_DONE_INT_RAW_R {
        SPI_SLV_RD_BUF_DONE_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_raw(&self) -> SPI_SLV_WR_BUF_DONE_INT_RAW_R {
        SPI_SLV_WR_BUF_DONE_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
    #[inline(always)]
    pub fn spi_trans_done_int_raw(&self) -> SPI_TRANS_DONE_INT_RAW_R {
        SPI_TRANS_DONE_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_raw(&self) -> SPI_DMA_SEG_TRANS_DONE_INT_RAW_R {
        SPI_DMA_SEG_TRANS_DONE_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_raw(&self) -> SPI_SLV_BUF_ADDR_ERR_INT_RAW_R {
        SPI_SLV_BUF_ADDR_ERR_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_raw(&self) -> SPI_SLV_CMD_ERR_INT_RAW_R {
        SPI_SLV_CMD_ERR_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_raw(&self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_R {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_raw(&self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    pub fn spi_app2_int_raw(&self) -> SPI_APP2_INT_RAW_R {
        SPI_APP2_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    pub fn spi_app1_int_raw(&self) -> SPI_APP1_INT_RAW_R {
        SPI_APP1_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DMA_INT_RAW")
            .field(
                "spi_dma_infifo_full_err_int_raw",
                &format_args!("{}", self.spi_dma_infifo_full_err_int_raw().bit()),
            )
            .field(
                "spi_dma_outfifo_empty_err_int_raw",
                &format_args!("{}", self.spi_dma_outfifo_empty_err_int_raw().bit()),
            )
            .field(
                "spi_slv_ex_qpi_int_raw",
                &format_args!("{}", self.spi_slv_ex_qpi_int_raw().bit()),
            )
            .field(
                "spi_slv_en_qpi_int_raw",
                &format_args!("{}", self.spi_slv_en_qpi_int_raw().bit()),
            )
            .field(
                "spi_slv_cmd7_int_raw",
                &format_args!("{}", self.spi_slv_cmd7_int_raw().bit()),
            )
            .field(
                "spi_slv_cmd8_int_raw",
                &format_args!("{}", self.spi_slv_cmd8_int_raw().bit()),
            )
            .field(
                "spi_slv_cmd9_int_raw",
                &format_args!("{}", self.spi_slv_cmd9_int_raw().bit()),
            )
            .field(
                "spi_slv_cmda_int_raw",
                &format_args!("{}", self.spi_slv_cmda_int_raw().bit()),
            )
            .field(
                "spi_slv_rd_dma_done_int_raw",
                &format_args!("{}", self.spi_slv_rd_dma_done_int_raw().bit()),
            )
            .field(
                "spi_slv_wr_dma_done_int_raw",
                &format_args!("{}", self.spi_slv_wr_dma_done_int_raw().bit()),
            )
            .field(
                "spi_slv_rd_buf_done_int_raw",
                &format_args!("{}", self.spi_slv_rd_buf_done_int_raw().bit()),
            )
            .field(
                "spi_slv_wr_buf_done_int_raw",
                &format_args!("{}", self.spi_slv_wr_buf_done_int_raw().bit()),
            )
            .field(
                "spi_trans_done_int_raw",
                &format_args!("{}", self.spi_trans_done_int_raw().bit()),
            )
            .field(
                "spi_dma_seg_trans_done_int_raw",
                &format_args!("{}", self.spi_dma_seg_trans_done_int_raw().bit()),
            )
            .field(
                "spi_slv_buf_addr_err_int_raw",
                &format_args!("{}", self.spi_slv_buf_addr_err_int_raw().bit()),
            )
            .field(
                "spi_slv_cmd_err_int_raw",
                &format_args!("{}", self.spi_slv_cmd_err_int_raw().bit()),
            )
            .field(
                "spi_mst_rx_afifo_wfull_err_int_raw",
                &format_args!("{}", self.spi_mst_rx_afifo_wfull_err_int_raw().bit()),
            )
            .field(
                "spi_mst_tx_afifo_rempty_err_int_raw",
                &format_args!("{}", self.spi_mst_tx_afifo_rempty_err_int_raw().bit()),
            )
            .field(
                "spi_app2_int_raw",
                &format_args!("{}", self.spi_app2_int_raw().bit()),
            )
            .field(
                "spi_app1_int_raw",
                &format_args!("{}", self.spi_app1_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_DMA_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_infifo_full_err_int_raw(
        &mut self,
    ) -> SPI_DMA_INFIFO_FULL_ERR_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_DMA_INFIFO_FULL_ERR_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_outfifo_empty_err_int_raw(
        &mut self,
    ) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_ex_qpi_int_raw(&mut self) -> SPI_SLV_EX_QPI_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_EX_QPI_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_en_qpi_int_raw(&mut self) -> SPI_SLV_EN_QPI_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_EN_QPI_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd7_int_raw(&mut self) -> SPI_SLV_CMD7_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_CMD7_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd8_int_raw(&mut self) -> SPI_SLV_CMD8_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_CMD8_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd9_int_raw(&mut self) -> SPI_SLV_CMD9_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_CMD9_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmda_int_raw(&mut self) -> SPI_SLV_CMDA_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_CMDA_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_rd_dma_done_int_raw(
        &mut self,
    ) -> SPI_SLV_RD_DMA_DONE_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_RD_DMA_DONE_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_wr_dma_done_int_raw(
        &mut self,
    ) -> SPI_SLV_WR_DMA_DONE_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_WR_DMA_DONE_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_rd_buf_done_int_raw(
        &mut self,
    ) -> SPI_SLV_RD_BUF_DONE_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_RD_BUF_DONE_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_wr_buf_done_int_raw(
        &mut self,
    ) -> SPI_SLV_WR_BUF_DONE_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_WR_BUF_DONE_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_trans_done_int_raw(&mut self) -> SPI_TRANS_DONE_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_TRANS_DONE_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_seg_trans_done_int_raw(
        &mut self,
    ) -> SPI_DMA_SEG_TRANS_DONE_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_DMA_SEG_TRANS_DONE_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 15 - The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_buf_addr_err_int_raw(
        &mut self,
    ) -> SPI_SLV_BUF_ADDR_ERR_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_BUF_ADDR_ERR_INT_RAW_W::new(self, 15)
    }
    #[doc = "Bit 16 - The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd_err_int_raw(&mut self) -> SPI_SLV_CMD_ERR_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_SLV_CMD_ERR_INT_RAW_W::new(self, 16)
    }
    #[doc = "Bit 17 - The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mst_rx_afifo_wfull_err_int_raw(
        &mut self,
    ) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_RAW_W::new(self, 17)
    }
    #[doc = "Bit 18 - The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mst_tx_afifo_rempty_err_int_raw(
        &mut self,
    ) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_RAW_W::new(self, 18)
    }
    #[doc = "Bit 19 - The raw bit for SPI_APP2_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi_app2_int_raw(&mut self) -> SPI_APP2_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_APP2_INT_RAW_W::new(self, 19)
    }
    #[doc = "Bit 20 - The raw bit for SPI_APP1_INT interrupt. The value is only controlled by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi_app1_int_raw(&mut self) -> SPI_APP1_INT_RAW_W<SPI_DMA_INT_RAW_SPEC> {
        SPI_APP1_INT_RAW_W::new(self, 20)
    }
}
#[doc = "SPI interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dma_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dma_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DMA_INT_RAW_SPEC;
impl crate::RegisterSpec for SPI_DMA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dma_int_raw::R`](R) reader structure"]
impl crate::Readable for SPI_DMA_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_dma_int_raw::W`](W) writer structure"]
impl crate::Writable for SPI_DMA_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_DMA_INT_RAW to value 0"]
impl crate::Resettable for SPI_DMA_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
