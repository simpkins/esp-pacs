#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `HSTIMER0_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER0_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub type HSTIMER0_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTIMER1_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER1_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub type HSTIMER1_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTIMER2_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER2_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub type HSTIMER2_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTIMER3_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `HSTIMER3_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub type HSTIMER3_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER0_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER0_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub type LSTIMER0_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER1_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER1_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub type LSTIMER1_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER2_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER2_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub type LSTIMER2_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTIMER3_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSTIMER3_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub type LSTIMER3_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_ENA` reader - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_ENA` writer - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_ENA` reader - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_ENA` writer - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_ENA` reader - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_ENA` writer - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_ENA` reader - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_ENA` writer - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH3_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_ENA` reader - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_ENA` writer - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_ENA` reader - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_ENA` writer - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH5_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_ENA` reader - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_ENA` writer - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH6_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_ENA` reader - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_ENA` writer - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_HSCH7_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` reader - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` writer - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` reader - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` writer - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` reader - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` writer - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` reader - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` writer - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH3_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` reader - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` writer - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` reader - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` writer - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH5_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_ENA` reader - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_ENA` writer - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH6_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_ENA` reader - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_INT_ENA_R = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_ENA` writer - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub type DUTY_CHNG_END_LSCH7_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_ena(&self) -> HSTIMER0_OVF_INT_ENA_R {
        HSTIMER0_OVF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_ena(&self) -> HSTIMER1_OVF_INT_ENA_R {
        HSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_ena(&self) -> HSTIMER2_OVF_INT_ENA_R {
        HSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_ena(&self) -> HSTIMER3_OVF_INT_ENA_R {
        HSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_ena(&self) -> LSTIMER0_OVF_INT_ENA_R {
        LSTIMER0_OVF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_ena(&self) -> LSTIMER1_OVF_INT_ENA_R {
        LSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_ena(&self) -> LSTIMER2_OVF_INT_ENA_R {
        LSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_ena(&self) -> LSTIMER3_OVF_INT_ENA_R {
        LSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_ena(&self) -> DUTY_CHNG_END_HSCH0_INT_ENA_R {
        DUTY_CHNG_END_HSCH0_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_ena(&self) -> DUTY_CHNG_END_HSCH1_INT_ENA_R {
        DUTY_CHNG_END_HSCH1_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_ena(&self) -> DUTY_CHNG_END_HSCH2_INT_ENA_R {
        DUTY_CHNG_END_HSCH2_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_ena(&self) -> DUTY_CHNG_END_HSCH3_INT_ENA_R {
        DUTY_CHNG_END_HSCH3_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_ena(&self) -> DUTY_CHNG_END_HSCH4_INT_ENA_R {
        DUTY_CHNG_END_HSCH4_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_ena(&self) -> DUTY_CHNG_END_HSCH5_INT_ENA_R {
        DUTY_CHNG_END_HSCH5_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_ena(&self) -> DUTY_CHNG_END_HSCH6_INT_ENA_R {
        DUTY_CHNG_END_HSCH6_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_ena(&self) -> DUTY_CHNG_END_HSCH7_INT_ENA_R {
        DUTY_CHNG_END_HSCH7_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_ena(&self) -> DUTY_CHNG_END_LSCH0_INT_ENA_R {
        DUTY_CHNG_END_LSCH0_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_ena(&self) -> DUTY_CHNG_END_LSCH1_INT_ENA_R {
        DUTY_CHNG_END_LSCH1_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_ena(&self) -> DUTY_CHNG_END_LSCH2_INT_ENA_R {
        DUTY_CHNG_END_LSCH2_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_ena(&self) -> DUTY_CHNG_END_LSCH3_INT_ENA_R {
        DUTY_CHNG_END_LSCH3_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_ena(&self) -> DUTY_CHNG_END_LSCH4_INT_ENA_R {
        DUTY_CHNG_END_LSCH4_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_ena(&self) -> DUTY_CHNG_END_LSCH5_INT_ENA_R {
        DUTY_CHNG_END_LSCH5_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_ena(&self) -> DUTY_CHNG_END_LSCH6_INT_ENA_R {
        DUTY_CHNG_END_LSCH6_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_ena(&self) -> DUTY_CHNG_END_LSCH7_INT_ENA_R {
        DUTY_CHNG_END_LSCH7_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "hstimer0_ovf_int_ena",
                &format_args!("{}", self.hstimer0_ovf_int_ena().bit()),
            )
            .field(
                "hstimer1_ovf_int_ena",
                &format_args!("{}", self.hstimer1_ovf_int_ena().bit()),
            )
            .field(
                "hstimer2_ovf_int_ena",
                &format_args!("{}", self.hstimer2_ovf_int_ena().bit()),
            )
            .field(
                "hstimer3_ovf_int_ena",
                &format_args!("{}", self.hstimer3_ovf_int_ena().bit()),
            )
            .field(
                "lstimer0_ovf_int_ena",
                &format_args!("{}", self.lstimer0_ovf_int_ena().bit()),
            )
            .field(
                "lstimer1_ovf_int_ena",
                &format_args!("{}", self.lstimer1_ovf_int_ena().bit()),
            )
            .field(
                "lstimer2_ovf_int_ena",
                &format_args!("{}", self.lstimer2_ovf_int_ena().bit()),
            )
            .field(
                "lstimer3_ovf_int_ena",
                &format_args!("{}", self.lstimer3_ovf_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch0_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch0_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch1_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch1_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch2_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch2_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch3_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch3_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch4_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch4_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch5_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch5_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch6_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch6_int_ena().bit()),
            )
            .field(
                "duty_chng_end_hsch7_int_ena",
                &format_args!("{}", self.duty_chng_end_hsch7_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch0_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch0_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch1_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch1_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch2_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch2_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch3_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch3_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch4_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch4_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch5_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch5_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch6_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch6_int_ena().bit()),
            )
            .field(
                "duty_chng_end_lsch7_int_ena",
                &format_args!("{}", self.duty_chng_end_lsch7_int_ena().bit()),
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
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer0_ovf_int_ena(&mut self) -> HSTIMER0_OVF_INT_ENA_W<INT_ENA_SPEC> {
        HSTIMER0_OVF_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer1_ovf_int_ena(&mut self) -> HSTIMER1_OVF_INT_ENA_W<INT_ENA_SPEC> {
        HSTIMER1_OVF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer2_ovf_int_ena(&mut self) -> HSTIMER2_OVF_INT_ENA_W<INT_ENA_SPEC> {
        HSTIMER2_OVF_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn hstimer3_ovf_int_ena(&mut self) -> HSTIMER3_OVF_INT_ENA_W<INT_ENA_SPEC> {
        HSTIMER3_OVF_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer0_ovf_int_ena(&mut self) -> LSTIMER0_OVF_INT_ENA_W<INT_ENA_SPEC> {
        LSTIMER0_OVF_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer1_ovf_int_ena(&mut self) -> LSTIMER1_OVF_INT_ENA_W<INT_ENA_SPEC> {
        LSTIMER1_OVF_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer2_ovf_int_ena(&mut self) -> LSTIMER2_OVF_INT_ENA_W<INT_ENA_SPEC> {
        LSTIMER2_OVF_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn lstimer3_ovf_int_ena(&mut self) -> LSTIMER3_OVF_INT_ENA_W<INT_ENA_SPEC> {
        LSTIMER3_OVF_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch0_int_ena(&mut self) -> DUTY_CHNG_END_HSCH0_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH0_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch1_int_ena(&mut self) -> DUTY_CHNG_END_HSCH1_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH1_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch2_int_ena(&mut self) -> DUTY_CHNG_END_HSCH2_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH2_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch3_int_ena(&mut self) -> DUTY_CHNG_END_HSCH3_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH3_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch4_int_ena(&mut self) -> DUTY_CHNG_END_HSCH4_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH4_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch5_int_ena(&mut self) -> DUTY_CHNG_END_HSCH5_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH5_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch6_int_ena(&mut self) -> DUTY_CHNG_END_HSCH6_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH6_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_hsch7_int_ena(&mut self) -> DUTY_CHNG_END_HSCH7_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_HSCH7_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch0_int_ena(&mut self) -> DUTY_CHNG_END_LSCH0_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH0_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch1_int_ena(&mut self) -> DUTY_CHNG_END_LSCH1_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH1_INT_ENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch2_int_ena(&mut self) -> DUTY_CHNG_END_LSCH2_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH2_INT_ENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch3_int_ena(&mut self) -> DUTY_CHNG_END_LSCH3_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH3_INT_ENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch4_int_ena(&mut self) -> DUTY_CHNG_END_LSCH4_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH4_INT_ENA_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch5_int_ena(&mut self) -> DUTY_CHNG_END_LSCH5_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH5_INT_ENA_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch6_int_ena(&mut self) -> DUTY_CHNG_END_LSCH6_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH6_INT_ENA_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn duty_chng_end_lsch7_int_ena(&mut self) -> DUTY_CHNG_END_LSCH7_INT_ENA_W<INT_ENA_SPEC> {
        DUTY_CHNG_END_LSCH7_INT_ENA_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
