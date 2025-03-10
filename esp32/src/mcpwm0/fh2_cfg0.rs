#[doc = "Register `FH2_CFG0` reader"]
pub type R = crate::R<FH2_CFG0_SPEC>;
#[doc = "Register `FH2_CFG0` writer"]
pub type W = crate::W<FH2_CFG0_SPEC>;
#[doc = "Field `FH2_SW_CBC` reader - "]
pub type FH2_SW_CBC_R = crate::BitReader;
#[doc = "Field `FH2_SW_CBC` writer - "]
pub type FH2_SW_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_F2_CBC` reader - "]
pub type FH2_F2_CBC_R = crate::BitReader;
#[doc = "Field `FH2_F2_CBC` writer - "]
pub type FH2_F2_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_F1_CBC` reader - "]
pub type FH2_F1_CBC_R = crate::BitReader;
#[doc = "Field `FH2_F1_CBC` writer - "]
pub type FH2_F1_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_F0_CBC` reader - "]
pub type FH2_F0_CBC_R = crate::BitReader;
#[doc = "Field `FH2_F0_CBC` writer - "]
pub type FH2_F0_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_SW_OST` reader - "]
pub type FH2_SW_OST_R = crate::BitReader;
#[doc = "Field `FH2_SW_OST` writer - "]
pub type FH2_SW_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_F2_OST` reader - "]
pub type FH2_F2_OST_R = crate::BitReader;
#[doc = "Field `FH2_F2_OST` writer - "]
pub type FH2_F2_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_F1_OST` reader - "]
pub type FH2_F1_OST_R = crate::BitReader;
#[doc = "Field `FH2_F1_OST` writer - "]
pub type FH2_F1_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_F0_OST` reader - "]
pub type FH2_F0_OST_R = crate::BitReader;
#[doc = "Field `FH2_F0_OST` writer - "]
pub type FH2_F0_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_A_CBC_D` reader - "]
pub type FH2_A_CBC_D_R = crate::FieldReader;
#[doc = "Field `FH2_A_CBC_D` writer - "]
pub type FH2_A_CBC_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_A_CBC_U` reader - "]
pub type FH2_A_CBC_U_R = crate::FieldReader;
#[doc = "Field `FH2_A_CBC_U` writer - "]
pub type FH2_A_CBC_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_A_OST_D` reader - "]
pub type FH2_A_OST_D_R = crate::FieldReader;
#[doc = "Field `FH2_A_OST_D` writer - "]
pub type FH2_A_OST_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_A_OST_U` reader - "]
pub type FH2_A_OST_U_R = crate::FieldReader;
#[doc = "Field `FH2_A_OST_U` writer - "]
pub type FH2_A_OST_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_B_CBC_D` reader - "]
pub type FH2_B_CBC_D_R = crate::FieldReader;
#[doc = "Field `FH2_B_CBC_D` writer - "]
pub type FH2_B_CBC_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_B_CBC_U` reader - "]
pub type FH2_B_CBC_U_R = crate::FieldReader;
#[doc = "Field `FH2_B_CBC_U` writer - "]
pub type FH2_B_CBC_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_B_OST_D` reader - "]
pub type FH2_B_OST_D_R = crate::FieldReader;
#[doc = "Field `FH2_B_OST_D` writer - "]
pub type FH2_B_OST_D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_B_OST_U` reader - "]
pub type FH2_B_OST_U_R = crate::FieldReader;
#[doc = "Field `FH2_B_OST_U` writer - "]
pub type FH2_B_OST_U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_sw_cbc(&self) -> FH2_SW_CBC_R {
        FH2_SW_CBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh2_f2_cbc(&self) -> FH2_F2_CBC_R {
        FH2_F2_CBC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fh2_f1_cbc(&self) -> FH2_F1_CBC_R {
        FH2_F1_CBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_f0_cbc(&self) -> FH2_F0_CBC_R {
        FH2_F0_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_sw_ost(&self) -> FH2_SW_OST_R {
        FH2_SW_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fh2_f2_ost(&self) -> FH2_F2_OST_R {
        FH2_F2_OST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fh2_f1_ost(&self) -> FH2_F1_OST_R {
        FH2_F1_OST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fh2_f0_ost(&self) -> FH2_F0_OST_R {
        FH2_F0_OST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn fh2_a_cbc_d(&self) -> FH2_A_CBC_D_R {
        FH2_A_CBC_D_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn fh2_a_cbc_u(&self) -> FH2_A_CBC_U_R {
        FH2_A_CBC_U_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn fh2_a_ost_d(&self) -> FH2_A_OST_D_R {
        FH2_A_OST_D_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn fh2_a_ost_u(&self) -> FH2_A_OST_U_R {
        FH2_A_OST_U_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fh2_b_cbc_d(&self) -> FH2_B_CBC_D_R {
        FH2_B_CBC_D_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn fh2_b_cbc_u(&self) -> FH2_B_CBC_U_R {
        FH2_B_CBC_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn fh2_b_ost_d(&self) -> FH2_B_OST_D_R {
        FH2_B_OST_D_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn fh2_b_ost_u(&self) -> FH2_B_OST_U_R {
        FH2_B_OST_U_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH2_CFG0")
            .field("fh2_sw_cbc", &format_args!("{}", self.fh2_sw_cbc().bit()))
            .field("fh2_f2_cbc", &format_args!("{}", self.fh2_f2_cbc().bit()))
            .field("fh2_f1_cbc", &format_args!("{}", self.fh2_f1_cbc().bit()))
            .field("fh2_f0_cbc", &format_args!("{}", self.fh2_f0_cbc().bit()))
            .field("fh2_sw_ost", &format_args!("{}", self.fh2_sw_ost().bit()))
            .field("fh2_f2_ost", &format_args!("{}", self.fh2_f2_ost().bit()))
            .field("fh2_f1_ost", &format_args!("{}", self.fh2_f1_ost().bit()))
            .field("fh2_f0_ost", &format_args!("{}", self.fh2_f0_ost().bit()))
            .field(
                "fh2_a_cbc_d",
                &format_args!("{}", self.fh2_a_cbc_d().bits()),
            )
            .field(
                "fh2_a_cbc_u",
                &format_args!("{}", self.fh2_a_cbc_u().bits()),
            )
            .field(
                "fh2_a_ost_d",
                &format_args!("{}", self.fh2_a_ost_d().bits()),
            )
            .field(
                "fh2_a_ost_u",
                &format_args!("{}", self.fh2_a_ost_u().bits()),
            )
            .field(
                "fh2_b_cbc_d",
                &format_args!("{}", self.fh2_b_cbc_d().bits()),
            )
            .field(
                "fh2_b_cbc_u",
                &format_args!("{}", self.fh2_b_cbc_u().bits()),
            )
            .field(
                "fh2_b_ost_d",
                &format_args!("{}", self.fh2_b_ost_d().bits()),
            )
            .field(
                "fh2_b_ost_u",
                &format_args!("{}", self.fh2_b_ost_u().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH2_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_sw_cbc(&mut self) -> FH2_SW_CBC_W<FH2_CFG0_SPEC> {
        FH2_SW_CBC_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_f2_cbc(&mut self) -> FH2_F2_CBC_W<FH2_CFG0_SPEC> {
        FH2_F2_CBC_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_f1_cbc(&mut self) -> FH2_F1_CBC_W<FH2_CFG0_SPEC> {
        FH2_F1_CBC_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_f0_cbc(&mut self) -> FH2_F0_CBC_W<FH2_CFG0_SPEC> {
        FH2_F0_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_sw_ost(&mut self) -> FH2_SW_OST_W<FH2_CFG0_SPEC> {
        FH2_SW_OST_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_f2_ost(&mut self) -> FH2_F2_OST_W<FH2_CFG0_SPEC> {
        FH2_F2_OST_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_f1_ost(&mut self) -> FH2_F1_OST_W<FH2_CFG0_SPEC> {
        FH2_F1_OST_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_f0_ost(&mut self) -> FH2_F0_OST_W<FH2_CFG0_SPEC> {
        FH2_F0_OST_W::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_a_cbc_d(&mut self) -> FH2_A_CBC_D_W<FH2_CFG0_SPEC> {
        FH2_A_CBC_D_W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_a_cbc_u(&mut self) -> FH2_A_CBC_U_W<FH2_CFG0_SPEC> {
        FH2_A_CBC_U_W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_a_ost_d(&mut self) -> FH2_A_OST_D_W<FH2_CFG0_SPEC> {
        FH2_A_OST_D_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_a_ost_u(&mut self) -> FH2_A_OST_U_W<FH2_CFG0_SPEC> {
        FH2_A_OST_U_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_b_cbc_d(&mut self) -> FH2_B_CBC_D_W<FH2_CFG0_SPEC> {
        FH2_B_CBC_D_W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_b_cbc_u(&mut self) -> FH2_B_CBC_U_W<FH2_CFG0_SPEC> {
        FH2_B_CBC_U_W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_b_ost_d(&mut self) -> FH2_B_OST_D_W<FH2_CFG0_SPEC> {
        FH2_B_OST_D_W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_b_ost_u(&mut self) -> FH2_B_OST_U_W<FH2_CFG0_SPEC> {
        FH2_B_OST_U_W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh2_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh2_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH2_CFG0_SPEC;
impl crate::RegisterSpec for FH2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh2_cfg0::R`](R) reader structure"]
impl crate::Readable for FH2_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh2_cfg0::W`](W) writer structure"]
impl crate::Writable for FH2_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FH2_CFG0 to value 0"]
impl crate::Resettable for FH2_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
