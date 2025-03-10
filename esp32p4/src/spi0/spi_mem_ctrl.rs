#[doc = "Register `SPI_MEM_CTRL` reader"]
pub type R = crate::R<SPI_MEM_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_CTRL` writer"]
pub type W = crate::W<SPI_MEM_CTRL_SPEC>;
#[doc = "Field `SPI_MEM_WDUMMY_DQS_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller."]
pub type SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WDUMMY_DQS_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller."]
pub type SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_WDUMMY_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SPI_MEM_WDUMMY_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WDUMMY_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SPI_MEM_WDUMMY_ALWAYS_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDUMMY_RIN` reader - In an MSPI read data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase."]
pub type SPI_MEM_FDUMMY_RIN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDUMMY_RIN` writer - In an MSPI read data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase."]
pub type SPI_MEM_FDUMMY_RIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDUMMY_WOUT` reader - In an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash."]
pub type SPI_MEM_FDUMMY_WOUT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDUMMY_WOUT` writer - In an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash."]
pub type SPI_MEM_FDUMMY_WOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDOUT_OCT` reader - Apply 8 signals during write-data phase 1:enable 0: disable"]
pub type SPI_MEM_FDOUT_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDOUT_OCT` writer - Apply 8 signals during write-data phase 1:enable 0: disable"]
pub type SPI_MEM_FDOUT_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDIN_OCT` reader - Apply 8 signals during read-data phase 1:enable 0: disable"]
pub type SPI_MEM_FDIN_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDIN_OCT` writer - Apply 8 signals during read-data phase 1:enable 0: disable"]
pub type SPI_MEM_FDIN_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FADDR_OCT` reader - Apply 8 signals during address phase 1:enable 0: disable"]
pub type SPI_MEM_FADDR_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FADDR_OCT` writer - Apply 8 signals during address phase 1:enable 0: disable"]
pub type SPI_MEM_FADDR_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FCMD_QUAD` reader - Apply 4 signals during command phase 1:enable 0: disable"]
pub type SPI_MEM_FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FCMD_QUAD` writer - Apply 4 signals during command phase 1:enable 0: disable"]
pub type SPI_MEM_FCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FCMD_OCT` reader - Apply 8 signals during command phase 1:enable 0: disable"]
pub type SPI_MEM_FCMD_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FCMD_OCT` writer - Apply 8 signals during command phase 1:enable 0: disable"]
pub type SPI_MEM_FCMD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FASTRD_MODE` reader - This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable."]
pub type SPI_MEM_FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FASTRD_MODE` writer - This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable."]
pub type SPI_MEM_FASTRD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FREAD_DUAL` reader - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_DUAL` writer - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type SPI_MEM_Q_POL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type SPI_MEM_Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type SPI_MEM_D_POL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type SPI_MEM_D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type SPI_MEM_WP_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type SPI_MEM_WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FREAD_DIO` reader - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_DIO` writer - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FREAD_QIO` reader - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_QIO` writer - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DQS_IE_ALWAYS_ON` reader - When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SPI_MEM_DQS_IE_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DQS_IE_ALWAYS_ON` writer - When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SPI_MEM_DQS_IE_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DATA_IE_ALWAYS_ON` reader - When accesses to flash, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SPI_MEM_DATA_IE_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DATA_IE_ALWAYS_ON` writer - When accesses to flash, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SPI_MEM_DATA_IE_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_mem_wdummy_dqs_always_out(&self) -> SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_R {
        SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_mem_wdummy_always_out(&self) -> SPI_MEM_WDUMMY_ALWAYS_OUT_R {
        SPI_MEM_WDUMMY_ALWAYS_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In an MSPI read data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase."]
    #[inline(always)]
    pub fn spi_mem_fdummy_rin(&self) -> SPI_MEM_FDUMMY_RIN_R {
        SPI_MEM_FDUMMY_RIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash."]
    #[inline(always)]
    pub fn spi_mem_fdummy_wout(&self) -> SPI_MEM_FDUMMY_WOUT_R {
        SPI_MEM_FDUMMY_WOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Apply 8 signals during write-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fdout_oct(&self) -> SPI_MEM_FDOUT_OCT_R {
        SPI_MEM_FDOUT_OCT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Apply 8 signals during read-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fdin_oct(&self) -> SPI_MEM_FDIN_OCT_R {
        SPI_MEM_FDIN_OCT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Apply 8 signals during address phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_faddr_oct(&self) -> SPI_MEM_FADDR_OCT_R {
        SPI_MEM_FADDR_OCT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fcmd_quad(&self) -> SPI_MEM_FCMD_QUAD_R {
        SPI_MEM_FCMD_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Apply 8 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fcmd_oct(&self) -> SPI_MEM_FCMD_OCT_R {
        SPI_MEM_FCMD_OCT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fastrd_mode(&self) -> SPI_MEM_FASTRD_MODE_R {
        SPI_MEM_FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_dual(&self) -> SPI_MEM_FREAD_DUAL_R {
        SPI_MEM_FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn spi_mem_q_pol(&self) -> SPI_MEM_Q_POL_R {
        SPI_MEM_Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn spi_mem_d_pol(&self) -> SPI_MEM_D_POL_R {
        SPI_MEM_D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_quad(&self) -> SPI_MEM_FREAD_QUAD_R {
        SPI_MEM_FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn spi_mem_wp(&self) -> SPI_MEM_WP_R {
        SPI_MEM_WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_dio(&self) -> SPI_MEM_FREAD_DIO_R {
        SPI_MEM_FREAD_DIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_qio(&self) -> SPI_MEM_FREAD_QIO_R {
        SPI_MEM_FREAD_QIO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_dqs_ie_always_on(&self) -> SPI_MEM_DQS_IE_ALWAYS_ON_R {
        SPI_MEM_DQS_IE_ALWAYS_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When accesses to flash, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_data_ie_always_on(&self) -> SPI_MEM_DATA_IE_ALWAYS_ON_R {
        SPI_MEM_DATA_IE_ALWAYS_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CTRL")
            .field(
                "spi_mem_wdummy_dqs_always_out",
                &format_args!("{}", self.spi_mem_wdummy_dqs_always_out().bit()),
            )
            .field(
                "spi_mem_wdummy_always_out",
                &format_args!("{}", self.spi_mem_wdummy_always_out().bit()),
            )
            .field(
                "spi_mem_fdummy_rin",
                &format_args!("{}", self.spi_mem_fdummy_rin().bit()),
            )
            .field(
                "spi_mem_fdummy_wout",
                &format_args!("{}", self.spi_mem_fdummy_wout().bit()),
            )
            .field(
                "spi_mem_fdout_oct",
                &format_args!("{}", self.spi_mem_fdout_oct().bit()),
            )
            .field(
                "spi_mem_fdin_oct",
                &format_args!("{}", self.spi_mem_fdin_oct().bit()),
            )
            .field(
                "spi_mem_faddr_oct",
                &format_args!("{}", self.spi_mem_faddr_oct().bit()),
            )
            .field(
                "spi_mem_fcmd_quad",
                &format_args!("{}", self.spi_mem_fcmd_quad().bit()),
            )
            .field(
                "spi_mem_fcmd_oct",
                &format_args!("{}", self.spi_mem_fcmd_oct().bit()),
            )
            .field(
                "spi_mem_fastrd_mode",
                &format_args!("{}", self.spi_mem_fastrd_mode().bit()),
            )
            .field(
                "spi_mem_fread_dual",
                &format_args!("{}", self.spi_mem_fread_dual().bit()),
            )
            .field(
                "spi_mem_q_pol",
                &format_args!("{}", self.spi_mem_q_pol().bit()),
            )
            .field(
                "spi_mem_d_pol",
                &format_args!("{}", self.spi_mem_d_pol().bit()),
            )
            .field(
                "spi_mem_fread_quad",
                &format_args!("{}", self.spi_mem_fread_quad().bit()),
            )
            .field("spi_mem_wp", &format_args!("{}", self.spi_mem_wp().bit()))
            .field(
                "spi_mem_fread_dio",
                &format_args!("{}", self.spi_mem_fread_dio().bit()),
            )
            .field(
                "spi_mem_fread_qio",
                &format_args!("{}", self.spi_mem_fread_qio().bit()),
            )
            .field(
                "spi_mem_dqs_ie_always_on",
                &format_args!("{}", self.spi_mem_dqs_ie_always_on().bit()),
            )
            .field(
                "spi_mem_data_ie_always_on",
                &format_args!("{}", self.spi_mem_data_ie_always_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wdummy_dqs_always_out(
        &mut self,
    ) -> SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_WDUMMY_DQS_ALWAYS_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - In the dummy phase of an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wdummy_always_out(&mut self) -> SPI_MEM_WDUMMY_ALWAYS_OUT_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_WDUMMY_ALWAYS_OUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - In an MSPI read data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the first half part of dummy phase. It is used to mask invalid SPI_DQS in the half part of dummy phase."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdummy_rin(&mut self) -> SPI_MEM_FDUMMY_RIN_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FDUMMY_RIN_W::new(self, 2)
    }
    #[doc = "Bit 3 - In an MSPI write data transfer when accesses to flash, the level of SPI_IO\\[7:0\\] is output by the MSPI controller in the second half part of dummy phase. It is used to pre-drive flash."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdummy_wout(&mut self) -> SPI_MEM_FDUMMY_WOUT_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FDUMMY_WOUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Apply 8 signals during write-data phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdout_oct(&mut self) -> SPI_MEM_FDOUT_OCT_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FDOUT_OCT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Apply 8 signals during read-data phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdin_oct(&mut self) -> SPI_MEM_FDIN_OCT_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FDIN_OCT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Apply 8 signals during address phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_faddr_oct(&mut self) -> SPI_MEM_FADDR_OCT_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FADDR_OCT_W::new(self, 6)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fcmd_quad(&mut self) -> SPI_MEM_FCMD_QUAD_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FCMD_QUAD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Apply 8 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fcmd_oct(&mut self) -> SPI_MEM_FCMD_OCT_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FCMD_OCT_W::new(self, 9)
    }
    #[doc = "Bit 13 - This bit enable the bits: SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QOUT and SPI_MEM_FREAD_DOUT. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fastrd_mode(&mut self) -> SPI_MEM_FASTRD_MODE_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FASTRD_MODE_W::new(self, 13)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_dual(&mut self) -> SPI_MEM_FREAD_DUAL_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_q_pol(&mut self) -> SPI_MEM_Q_POL_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_d_pol(&mut self) -> SPI_MEM_D_POL_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_D_POL_W::new(self, 19)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_quad(&mut self) -> SPI_MEM_FREAD_QUAD_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FREAD_QUAD_W::new(self, 20)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wp(&mut self) -> SPI_MEM_WP_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_WP_W::new(self, 21)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_dio(&mut self) -> SPI_MEM_FREAD_DIO_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FREAD_DIO_W::new(self, 23)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_qio(&mut self) -> SPI_MEM_FREAD_QIO_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_FREAD_QIO_W::new(self, 24)
    }
    #[doc = "Bit 30 - When accesses to flash, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dqs_ie_always_on(&mut self) -> SPI_MEM_DQS_IE_ALWAYS_ON_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_DQS_IE_ALWAYS_ON_W::new(self, 30)
    }
    #[doc = "Bit 31 - When accesses to flash, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_data_ie_always_on(&mut self) -> SPI_MEM_DATA_IE_ALWAYS_ON_W<SPI_MEM_CTRL_SPEC> {
        SPI_MEM_DATA_IE_ALWAYS_ON_W::new(self, 31)
    }
}
#[doc = "SPI0 control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL to value 0x802c_200c"]
impl crate::Resettable for SPI_MEM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x802c_200c;
}
