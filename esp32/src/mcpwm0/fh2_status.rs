#[doc = "Register `FH2_STATUS` reader"]
pub type R = crate::R<FH2_STATUS_SPEC>;
#[doc = "Field `FH2_CBC_ON` reader - "]
pub type FH2_CBC_ON_R = crate::BitReader;
#[doc = "Field `FH2_OST_ON` reader - "]
pub type FH2_OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_cbc_on(&self) -> FH2_CBC_ON_R {
        FH2_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fh2_ost_on(&self) -> FH2_OST_ON_R {
        FH2_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH2_STATUS")
            .field("fh2_cbc_on", &format_args!("{}", self.fh2_cbc_on().bit()))
            .field("fh2_ost_on", &format_args!("{}", self.fh2_ost_on().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH2_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh2_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH2_STATUS_SPEC;
impl crate::RegisterSpec for FH2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh2_status::R`](R) reader structure"]
impl crate::Readable for FH2_STATUS_SPEC {}
#[doc = "`reset()` method sets FH2_STATUS to value 0"]
impl crate::Resettable for FH2_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
