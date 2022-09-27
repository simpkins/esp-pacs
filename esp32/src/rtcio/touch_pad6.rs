#[doc = "Register `TOUCH_PAD6` reader"]
pub struct R(crate::R<TOUCH_PAD6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_PAD6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_PAD6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_PAD6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_PAD6` writer"]
pub struct W(crate::W<TOUCH_PAD6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_PAD6_SPEC>;
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
impl From<crate::W<TOUCH_PAD6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_PAD6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO_GPIO` reader - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS"]
pub type TO_GPIO_R = crate::BitReader<bool>;
#[doc = "Field `TO_GPIO` writer - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS"]
pub type TO_GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `FUN_IE` reader - the input enable of the pad"]
pub type FUN_IE_R = crate::BitReader<bool>;
#[doc = "Field `FUN_IE` writer - the input enable of the pad"]
pub type FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `SLP_OE` reader - the output enable of the pad in sleep status"]
pub type SLP_OE_R = crate::BitReader<bool>;
#[doc = "Field `SLP_OE` writer - the output enable of the pad in sleep status"]
pub type SLP_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SLP_IE_R = crate::BitReader<bool>;
#[doc = "Field `SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `FUN_SEL` reader - the functional selection signal of the pad"]
pub type FUN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUN_SEL` writer - the functional selection signal of the pad"]
pub type FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOUCH_PAD6_SPEC, u8, u8, 2, O>;
#[doc = "Field `MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type MUX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `XPD` reader - touch sensor power on."]
pub type XPD_R = crate::BitReader<bool>;
#[doc = "Field `XPD` writer - touch sensor power on."]
pub type XPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `TIE_OPT` reader - default touch sensor tie option. 0: tie low 1: tie high."]
pub type TIE_OPT_R = crate::BitReader<bool>;
#[doc = "Field `TIE_OPT` writer - default touch sensor tie option. 0: tie low 1: tie high."]
pub type TIE_OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `START` reader - start touch sensor."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - start touch sensor."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `DAC` reader - touch sensor slope control. 3-bit for each touch panel default 100."]
pub type DAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC` writer - touch sensor slope control. 3-bit for each touch panel default 100."]
pub type DAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOUCH_PAD6_SPEC, u8, u8, 3, O>;
#[doc = "Field `RUE` reader - the pull up enable of the pad"]
pub type RUE_R = crate::BitReader<bool>;
#[doc = "Field `RUE` writer - the pull up enable of the pad"]
pub type RUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `RDE` reader - the pull down enable of the pad"]
pub type RDE_R = crate::BitReader<bool>;
#[doc = "Field `RDE` writer - the pull down enable of the pad"]
pub type RDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
#[doc = "Field `DRV` reader - the driver strength of the pad"]
pub type DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRV` writer - the driver strength of the pad"]
pub type DRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOUCH_PAD6_SPEC, u8, u8, 2, O>;
#[doc = "Field `HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type HOLD_R = crate::BitReader<bool>;
#[doc = "Field `HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOUCH_PAD6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS"]
    #[inline(always)]
    pub fn to_gpio(&self) -> TO_GPIO_R {
        TO_GPIO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - the driver strength of the pad"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS"]
    #[inline(always)]
    pub fn to_gpio(&mut self) -> TO_GPIO_W<12> {
        TO_GPIO_W::new(self)
    }
    #[doc = "Bit 13 - the input enable of the pad"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<13> {
        FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W<14> {
        SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W<15> {
        SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<16> {
        SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<17> {
        FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<19> {
        MUX_SEL_W::new(self)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&mut self) -> XPD_W<20> {
        XPD_W::new(self)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&mut self) -> TIE_OPT_W<21> {
        TIE_OPT_W::new(self)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<22> {
        START_W::new(self)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<23> {
        DAC_W::new(self)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W<27> {
        RUE_W::new(self)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W<28> {
        RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - the driver strength of the pad"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<29> {
        DRV_W::new(self)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W<31> {
        HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_pad6](index.html) module"]
pub struct TOUCH_PAD6_SPEC;
impl crate::RegisterSpec for TOUCH_PAD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_pad6::R](R) reader structure"]
impl crate::Readable for TOUCH_PAD6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_pad6::W](W) writer structure"]
impl crate::Writable for TOUCH_PAD6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_PAD6 to value 0x4a00_0000"]
impl crate::Resettable for TOUCH_PAD6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4a00_0000
    }
}
