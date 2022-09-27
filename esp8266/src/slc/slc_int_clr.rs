#[doc = "Register `SLC_INT_CLR` reader"]
pub struct R(crate::R<SLC_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_INT_CLR` writer"]
pub struct W(crate::W<SLC_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_INT_CLR_SPEC>;
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
impl From<crate::W<SLC_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_FRHOST_BIT0_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT0_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT0_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_FRHOST_BIT1_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT1_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT1_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_FRHOST_BIT2_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT2_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT2_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT2_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_FRHOST_BIT3_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT3_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT3_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT3_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_FRHOST_BIT4_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT4_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT4_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT4_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_FRHOST_BIT5_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT5_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT5_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT5_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_FRHOST_BIT6_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT6_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT6_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT6_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_FRHOST_BIT7_INT_CLR` reader - "]
pub type SLC_FRHOST_BIT7_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_FRHOST_BIT7_INT_CLR` writer - "]
pub type SLC_FRHOST_BIT7_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_RX_START_INT_CLR` reader - "]
pub type SLC_RX_START_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_RX_START_INT_CLR` writer - "]
pub type SLC_RX_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TX_START_INT_CLR` reader - "]
pub type SLC_TX_START_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_START_INT_CLR` writer - "]
pub type SLC_TX_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_RX_UDF_INT_CLR` reader - "]
pub type SLC_RX_UDF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_RX_UDF_INT_CLR` writer - "]
pub type SLC_RX_UDF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TX_OVF_INT_CLR` reader - "]
pub type SLC_TX_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_OVF_INT_CLR` writer - "]
pub type SLC_TX_OVF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TOKEN0_1TO0_INT_CLR` reader - "]
pub type SLC_TOKEN0_1TO0_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TOKEN0_1TO0_INT_CLR` writer - "]
pub type SLC_TOKEN0_1TO0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TOKEN1_1TO0_INT_CLR` reader - "]
pub type SLC_TOKEN1_1TO0_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TOKEN1_1TO0_INT_CLR` writer - "]
pub type SLC_TOKEN1_1TO0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TX_DONE_INT_CLR` reader - "]
pub type SLC_TX_DONE_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_DONE_INT_CLR` writer - "]
pub type SLC_TX_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TX_EOF_INT_CLR` reader - "]
pub type SLC_TX_EOF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_EOF_INT_CLR` writer - "]
pub type SLC_TX_EOF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_RX_DONE_INT_CLR` reader - "]
pub type SLC_RX_DONE_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_RX_DONE_INT_CLR` writer - "]
pub type SLC_RX_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_RX_EOF_INT_CLR` reader - "]
pub type SLC_RX_EOF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_RX_EOF_INT_CLR` writer - "]
pub type SLC_RX_EOF_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TOHOST_INT_CLR` reader - "]
pub type SLC_TOHOST_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TOHOST_INT_CLR` writer - "]
pub type SLC_TOHOST_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TX_DSCR_ERR_INT_CLR` reader - "]
pub type SLC_TX_DSCR_ERR_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC_TX_DSCR_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_RX_DSCR_ERR_INT_CLR` reader - "]
pub type SLC_RX_DSCR_ERR_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_RX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC_RX_DSCR_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLC_TX_DSCR_EMPTY_INT_CLR` reader - "]
pub type SLC_TX_DSCR_EMPTY_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_DSCR_EMPTY_INT_CLR` writer - "]
pub type SLC_TX_DSCR_EMPTY_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_INT_CLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_frhost_bit0_int_clr(&self) -> SLC_FRHOST_BIT0_INT_CLR_R {
        SLC_FRHOST_BIT0_INT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_frhost_bit1_int_clr(&self) -> SLC_FRHOST_BIT1_INT_CLR_R {
        SLC_FRHOST_BIT1_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_frhost_bit2_int_clr(&self) -> SLC_FRHOST_BIT2_INT_CLR_R {
        SLC_FRHOST_BIT2_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_frhost_bit3_int_clr(&self) -> SLC_FRHOST_BIT3_INT_CLR_R {
        SLC_FRHOST_BIT3_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_frhost_bit4_int_clr(&self) -> SLC_FRHOST_BIT4_INT_CLR_R {
        SLC_FRHOST_BIT4_INT_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_frhost_bit5_int_clr(&self) -> SLC_FRHOST_BIT5_INT_CLR_R {
        SLC_FRHOST_BIT5_INT_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_frhost_bit6_int_clr(&self) -> SLC_FRHOST_BIT6_INT_CLR_R {
        SLC_FRHOST_BIT6_INT_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_frhost_bit7_int_clr(&self) -> SLC_FRHOST_BIT7_INT_CLR_R {
        SLC_FRHOST_BIT7_INT_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_rx_start_int_clr(&self) -> SLC_RX_START_INT_CLR_R {
        SLC_RX_START_INT_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_tx_start_int_clr(&self) -> SLC_TX_START_INT_CLR_R {
        SLC_TX_START_INT_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_rx_udf_int_clr(&self) -> SLC_RX_UDF_INT_CLR_R {
        SLC_RX_UDF_INT_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_tx_ovf_int_clr(&self) -> SLC_TX_OVF_INT_CLR_R {
        SLC_TX_OVF_INT_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_token0_1to0_int_clr(&self) -> SLC_TOKEN0_1TO0_INT_CLR_R {
        SLC_TOKEN0_1TO0_INT_CLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_token1_1to0_int_clr(&self) -> SLC_TOKEN1_1TO0_INT_CLR_R {
        SLC_TOKEN1_1TO0_INT_CLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_tx_done_int_clr(&self) -> SLC_TX_DONE_INT_CLR_R {
        SLC_TX_DONE_INT_CLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_tx_eof_int_clr(&self) -> SLC_TX_EOF_INT_CLR_R {
        SLC_TX_EOF_INT_CLR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_rx_done_int_clr(&self) -> SLC_RX_DONE_INT_CLR_R {
        SLC_RX_DONE_INT_CLR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_rx_eof_int_clr(&self) -> SLC_RX_EOF_INT_CLR_R {
        SLC_RX_EOF_INT_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_tohost_int_clr(&self) -> SLC_TOHOST_INT_CLR_R {
        SLC_TOHOST_INT_CLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_tx_dscr_err_int_clr(&self) -> SLC_TX_DSCR_ERR_INT_CLR_R {
        SLC_TX_DSCR_ERR_INT_CLR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_rx_dscr_err_int_clr(&self) -> SLC_RX_DSCR_ERR_INT_CLR_R {
        SLC_RX_DSCR_ERR_INT_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_tx_dscr_empty_int_clr(&self) -> SLC_TX_DSCR_EMPTY_INT_CLR_R {
        SLC_TX_DSCR_EMPTY_INT_CLR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_frhost_bit0_int_clr(&mut self) -> SLC_FRHOST_BIT0_INT_CLR_W<0> {
        SLC_FRHOST_BIT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_frhost_bit1_int_clr(&mut self) -> SLC_FRHOST_BIT1_INT_CLR_W<1> {
        SLC_FRHOST_BIT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_frhost_bit2_int_clr(&mut self) -> SLC_FRHOST_BIT2_INT_CLR_W<2> {
        SLC_FRHOST_BIT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_frhost_bit3_int_clr(&mut self) -> SLC_FRHOST_BIT3_INT_CLR_W<3> {
        SLC_FRHOST_BIT3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_frhost_bit4_int_clr(&mut self) -> SLC_FRHOST_BIT4_INT_CLR_W<4> {
        SLC_FRHOST_BIT4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_frhost_bit5_int_clr(&mut self) -> SLC_FRHOST_BIT5_INT_CLR_W<5> {
        SLC_FRHOST_BIT5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_frhost_bit6_int_clr(&mut self) -> SLC_FRHOST_BIT6_INT_CLR_W<6> {
        SLC_FRHOST_BIT6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_frhost_bit7_int_clr(&mut self) -> SLC_FRHOST_BIT7_INT_CLR_W<7> {
        SLC_FRHOST_BIT7_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_rx_start_int_clr(&mut self) -> SLC_RX_START_INT_CLR_W<8> {
        SLC_RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_tx_start_int_clr(&mut self) -> SLC_TX_START_INT_CLR_W<9> {
        SLC_TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc_rx_udf_int_clr(&mut self) -> SLC_RX_UDF_INT_CLR_W<10> {
        SLC_RX_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc_tx_ovf_int_clr(&mut self) -> SLC_TX_OVF_INT_CLR_W<11> {
        SLC_TX_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_token0_1to0_int_clr(&mut self) -> SLC_TOKEN0_1TO0_INT_CLR_W<12> {
        SLC_TOKEN0_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_token1_1to0_int_clr(&mut self) -> SLC_TOKEN1_1TO0_INT_CLR_W<13> {
        SLC_TOKEN1_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_tx_done_int_clr(&mut self) -> SLC_TX_DONE_INT_CLR_W<14> {
        SLC_TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc_tx_eof_int_clr(&mut self) -> SLC_TX_EOF_INT_CLR_W<15> {
        SLC_TX_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_rx_done_int_clr(&mut self) -> SLC_RX_DONE_INT_CLR_W<16> {
        SLC_RX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_rx_eof_int_clr(&mut self) -> SLC_RX_EOF_INT_CLR_W<17> {
        SLC_RX_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_tohost_int_clr(&mut self) -> SLC_TOHOST_INT_CLR_W<18> {
        SLC_TOHOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_tx_dscr_err_int_clr(&mut self) -> SLC_TX_DSCR_ERR_INT_CLR_W<19> {
        SLC_TX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_rx_dscr_err_int_clr(&mut self) -> SLC_RX_DSCR_ERR_INT_CLR_W<20> {
        SLC_RX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc_tx_dscr_empty_int_clr(&mut self) -> SLC_TX_DSCR_EMPTY_INT_CLR_W<21> {
        SLC_TX_DSCR_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_INT_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_int_clr](index.html) module"]
pub struct SLC_INT_CLR_SPEC;
impl crate::RegisterSpec for SLC_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_int_clr::R](R) reader structure"]
impl crate::Readable for SLC_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_int_clr::W](W) writer structure"]
impl crate::Writable for SLC_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLC_INT_CLR to value 0"]
impl crate::Resettable for SLC_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
