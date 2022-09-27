#[doc = "Register `I2S_FIFO_CONF` reader"]
pub struct R(crate::R<I2S_FIFO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_FIFO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_FIFO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_FIFO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_FIFO_CONF` writer"]
pub struct W(crate::W<I2S_FIFO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_FIFO_CONF_SPEC>;
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
impl From<crate::W<I2S_FIFO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_FIFO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_I2S_RX_DATA_NUM` reader - "]
pub type I2S_I2S_RX_DATA_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_I2S_RX_DATA_NUM` writer - "]
pub type I2S_I2S_RX_DATA_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_FIFO_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `I2S_I2S_TX_DATA_NUM` reader - "]
pub type I2S_I2S_TX_DATA_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_I2S_TX_DATA_NUM` writer - "]
pub type I2S_I2S_TX_DATA_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_FIFO_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `I2S_I2S_DSCR_EN` reader - "]
pub type I2S_I2S_DSCR_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2S_I2S_DSCR_EN` writer - "]
pub type I2S_I2S_DSCR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONF_SPEC, bool, O>;
#[doc = "Field `I2S_I2S_TX_FIFO_MOD` reader - "]
pub type I2S_I2S_TX_FIFO_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_I2S_TX_FIFO_MOD` writer - "]
pub type I2S_I2S_TX_FIFO_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_FIFO_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `I2S_I2S_RX_FIFO_MOD` reader - "]
pub type I2S_I2S_RX_FIFO_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_I2S_RX_FIFO_MOD` writer - "]
pub type I2S_I2S_RX_FIFO_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_FIFO_CONF_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2s_i2s_rx_data_num(&self) -> I2S_I2S_RX_DATA_NUM_R {
        I2S_I2S_RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn i2s_i2s_tx_data_num(&self) -> I2S_I2S_TX_DATA_NUM_R {
        I2S_I2S_TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_i2s_dscr_en(&self) -> I2S_I2S_DSCR_EN_R {
        I2S_I2S_DSCR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn i2s_i2s_tx_fifo_mod(&self) -> I2S_I2S_TX_FIFO_MOD_R {
        I2S_I2S_TX_FIFO_MOD_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn i2s_i2s_rx_fifo_mod(&self) -> I2S_I2S_RX_FIFO_MOD_R {
        I2S_I2S_RX_FIFO_MOD_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2s_i2s_rx_data_num(&mut self) -> I2S_I2S_RX_DATA_NUM_W<0> {
        I2S_I2S_RX_DATA_NUM_W::new(self)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn i2s_i2s_tx_data_num(&mut self) -> I2S_I2S_TX_DATA_NUM_W<6> {
        I2S_I2S_TX_DATA_NUM_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_i2s_dscr_en(&mut self) -> I2S_I2S_DSCR_EN_W<12> {
        I2S_I2S_DSCR_EN_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn i2s_i2s_tx_fifo_mod(&mut self) -> I2S_I2S_TX_FIFO_MOD_W<13> {
        I2S_I2S_TX_FIFO_MOD_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn i2s_i2s_rx_fifo_mod(&mut self) -> I2S_I2S_RX_FIFO_MOD_W<16> {
        I2S_I2S_RX_FIFO_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_FIFO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_fifo_conf](index.html) module"]
pub struct I2S_FIFO_CONF_SPEC;
impl crate::RegisterSpec for I2S_FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_fifo_conf::R](R) reader structure"]
impl crate::Readable for I2S_FIFO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_fifo_conf::W](W) writer structure"]
impl crate::Writable for I2S_FIFO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_FIFO_CONF to value 0"]
impl crate::Resettable for I2S_FIFO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
