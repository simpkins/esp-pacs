#[doc = "Register `CH%s_CONF0` reader"]
pub struct R(crate::R<CH_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_CONF0` writer"]
pub struct W(crate::W<CH_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CONF0_SPEC>;
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
impl From<crate::W<CH_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL` reader - reg_timer_sel_lsch0."]
pub type TIMER_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_SEL` writer - reg_timer_sel_lsch0."]
pub type TIMER_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SIG_OUT_EN` reader - reg_sig_out_en_lsch0."]
pub type SIG_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SIG_OUT_EN` writer - reg_sig_out_en_lsch0."]
pub type SIG_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF0_SPEC, bool, O>;
#[doc = "Field `IDLE_LV` reader - reg_idle_lv_lsch0."]
pub type IDLE_LV_R = crate::BitReader<bool>;
#[doc = "Field `IDLE_LV` writer - reg_idle_lv_lsch0."]
pub type IDLE_LV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF0_SPEC, bool, O>;
#[doc = "Field `PARA_UP` writer - reg_para_up_lsch0."]
pub type PARA_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF0_SPEC, bool, O>;
#[doc = "Field `OVF_NUM` reader - reg_ovf_num_lsch0."]
pub type OVF_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVF_NUM` writer - reg_ovf_num_lsch0."]
pub type OVF_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CONF0_SPEC, u16, u16, 10, O>;
#[doc = "Field `OVF_CNT_EN` reader - reg_ovf_cnt_en_lsch0."]
pub type OVF_CNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_EN` writer - reg_ovf_cnt_en_lsch0."]
pub type OVF_CNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF0_SPEC, bool, O>;
#[doc = "Field `OVF_CNT_RESET` writer - reg_ovf_cnt_reset_lsch0."]
pub type OVF_CNT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch0."]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch0."]
    #[inline(always)]
    pub fn sig_out_en(&self) -> SIG_OUT_EN_R {
        SIG_OUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch0."]
    #[inline(always)]
    pub fn idle_lv(&self) -> IDLE_LV_R {
        IDLE_LV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch0."]
    #[inline(always)]
    pub fn ovf_num(&self) -> OVF_NUM_R {
        OVF_NUM_R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_en(&self) -> OVF_CNT_EN_R {
        OVF_CNT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_timer_sel_lsch0."]
    #[inline(always)]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<0> {
        TIMER_SEL_W::new(self)
    }
    #[doc = "Bit 2 - reg_sig_out_en_lsch0."]
    #[inline(always)]
    pub fn sig_out_en(&mut self) -> SIG_OUT_EN_W<2> {
        SIG_OUT_EN_W::new(self)
    }
    #[doc = "Bit 3 - reg_idle_lv_lsch0."]
    #[inline(always)]
    pub fn idle_lv(&mut self) -> IDLE_LV_W<3> {
        IDLE_LV_W::new(self)
    }
    #[doc = "Bit 4 - reg_para_up_lsch0."]
    #[inline(always)]
    pub fn para_up(&mut self) -> PARA_UP_W<4> {
        PARA_UP_W::new(self)
    }
    #[doc = "Bits 5:14 - reg_ovf_num_lsch0."]
    #[inline(always)]
    pub fn ovf_num(&mut self) -> OVF_NUM_W<5> {
        OVF_NUM_W::new(self)
    }
    #[doc = "Bit 15 - reg_ovf_cnt_en_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_en(&mut self) -> OVF_CNT_EN_W<15> {
        OVF_CNT_EN_W::new(self)
    }
    #[doc = "Bit 16 - reg_ovf_cnt_reset_lsch0."]
    #[inline(always)]
    pub fn ovf_cnt_reset(&mut self) -> OVF_CNT_RESET_W<16> {
        OVF_CNT_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH%s_CONF%s.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_conf0](index.html) module"]
pub struct CH_CONF0_SPEC;
impl crate::RegisterSpec for CH_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_conf0::R](R) reader structure"]
impl crate::Readable for CH_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_conf0::W](W) writer structure"]
impl crate::Writable for CH_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_CONF0 to value 0"]
impl crate::Resettable for CH_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
