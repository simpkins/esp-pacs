#[doc = "Register `DOEPINT1` reader"]
pub type R = crate::R<DOEPINT1_SPEC>;
#[doc = "Register `DOEPINT1` writer"]
pub type W = crate::W<DOEPINT1_SPEC>;
#[doc = "Field `XFERCOMPL1` reader - "]
pub type XFERCOMPL1_R = crate::BitReader;
#[doc = "Field `XFERCOMPL1` writer - "]
pub type XFERCOMPL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLD1` reader - "]
pub type EPDISBLD1_R = crate::BitReader;
#[doc = "Field `EPDISBLD1` writer - "]
pub type EPDISBLD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR1` reader - "]
pub type AHBERR1_R = crate::BitReader;
#[doc = "Field `AHBERR1` writer - "]
pub type AHBERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP1` reader - "]
pub type SETUP1_R = crate::BitReader;
#[doc = "Field `SETUP1` writer - "]
pub type SETUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDIS1` reader - "]
pub type OUTTKNEPDIS1_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS1` writer - "]
pub type OUTTKNEPDIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVD1` reader - "]
pub type STSPHSERCVD1_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD1` writer - "]
pub type STSPHSERCVD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP1` reader - "]
pub type BACK2BACKSETUP1_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP1` writer - "]
pub type BACK2BACKSETUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERR1` reader - "]
pub type OUTPKTERR1_R = crate::BitReader;
#[doc = "Field `OUTPKTERR1` writer - "]
pub type OUTPKTERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTR1` reader - "]
pub type BNAINTR1_R = crate::BitReader;
#[doc = "Field `BNAINTR1` writer - "]
pub type BNAINTR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS1` reader - "]
pub type PKTDRPSTS1_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS1` writer - "]
pub type PKTDRPSTS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR1` reader - "]
pub type BBLEERR1_R = crate::BitReader;
#[doc = "Field `BBLEERR1` writer - "]
pub type BBLEERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT1` reader - "]
pub type NAKINTRPT1_R = crate::BitReader;
#[doc = "Field `NAKINTRPT1` writer - "]
pub type NAKINTRPT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYEPINTRPT1` reader - "]
pub type NYEPINTRPT1_R = crate::BitReader;
#[doc = "Field `NYEPINTRPT1` writer - "]
pub type NYEPINTRPT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPPKTRCVD1` reader - "]
pub type STUPPKTRCVD1_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD1` writer - "]
pub type STUPPKTRCVD1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl1(&self) -> XFERCOMPL1_R {
        XFERCOMPL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld1(&self) -> EPDISBLD1_R {
        EPDISBLD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr1(&self) -> AHBERR1_R {
        AHBERR1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup1(&self) -> SETUP1_R {
        SETUP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis1(&self) -> OUTTKNEPDIS1_R {
        OUTTKNEPDIS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd1(&self) -> STSPHSERCVD1_R {
        STSPHSERCVD1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup1(&self) -> BACK2BACKSETUP1_R {
        BACK2BACKSETUP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr1(&self) -> OUTPKTERR1_R {
        OUTPKTERR1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr1(&self) -> BNAINTR1_R {
        BNAINTR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts1(&self) -> PKTDRPSTS1_R {
        PKTDRPSTS1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr1(&self) -> BBLEERR1_R {
        BBLEERR1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt1(&self) -> NAKINTRPT1_R {
        NAKINTRPT1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt1(&self) -> NYEPINTRPT1_R {
        NYEPINTRPT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd1(&self) -> STUPPKTRCVD1_R {
        STUPPKTRCVD1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT1")
            .field("xfercompl1", &format_args!("{}", self.xfercompl1().bit()))
            .field("epdisbld1", &format_args!("{}", self.epdisbld1().bit()))
            .field("ahberr1", &format_args!("{}", self.ahberr1().bit()))
            .field("setup1", &format_args!("{}", self.setup1().bit()))
            .field(
                "outtknepdis1",
                &format_args!("{}", self.outtknepdis1().bit()),
            )
            .field(
                "stsphsercvd1",
                &format_args!("{}", self.stsphsercvd1().bit()),
            )
            .field(
                "back2backsetup1",
                &format_args!("{}", self.back2backsetup1().bit()),
            )
            .field("outpkterr1", &format_args!("{}", self.outpkterr1().bit()))
            .field("bnaintr1", &format_args!("{}", self.bnaintr1().bit()))
            .field("pktdrpsts1", &format_args!("{}", self.pktdrpsts1().bit()))
            .field("bbleerr1", &format_args!("{}", self.bbleerr1().bit()))
            .field("nakintrpt1", &format_args!("{}", self.nakintrpt1().bit()))
            .field("nyepintrpt1", &format_args!("{}", self.nyepintrpt1().bit()))
            .field(
                "stuppktrcvd1",
                &format_args!("{}", self.stuppktrcvd1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPINT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl1(&mut self) -> XFERCOMPL1_W<DOEPINT1_SPEC> {
        XFERCOMPL1_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld1(&mut self) -> EPDISBLD1_W<DOEPINT1_SPEC> {
        EPDISBLD1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr1(&mut self) -> AHBERR1_W<DOEPINT1_SPEC> {
        AHBERR1_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setup1(&mut self) -> SETUP1_W<DOEPINT1_SPEC> {
        SETUP1_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis1(&mut self) -> OUTTKNEPDIS1_W<DOEPINT1_SPEC> {
        OUTTKNEPDIS1_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd1(&mut self) -> STSPHSERCVD1_W<DOEPINT1_SPEC> {
        STSPHSERCVD1_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup1(&mut self) -> BACK2BACKSETUP1_W<DOEPINT1_SPEC> {
        BACK2BACKSETUP1_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr1(&mut self) -> OUTPKTERR1_W<DOEPINT1_SPEC> {
        OUTPKTERR1_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr1(&mut self) -> BNAINTR1_W<DOEPINT1_SPEC> {
        BNAINTR1_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts1(&mut self) -> PKTDRPSTS1_W<DOEPINT1_SPEC> {
        PKTDRPSTS1_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr1(&mut self) -> BBLEERR1_W<DOEPINT1_SPEC> {
        BBLEERR1_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt1(&mut self) -> NAKINTRPT1_W<DOEPINT1_SPEC> {
        NAKINTRPT1_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyepintrpt1(&mut self) -> NYEPINTRPT1_W<DOEPINT1_SPEC> {
        NYEPINTRPT1_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd1(&mut self) -> STUPPKTRCVD1_W<DOEPINT1_SPEC> {
        STUPPKTRCVD1_W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT1_SPEC;
impl crate::RegisterSpec for DOEPINT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint1::R`](R) reader structure"]
impl crate::Readable for DOEPINT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint1::W`](W) writer structure"]
impl crate::Writable for DOEPINT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT1 to value 0"]
impl crate::Resettable for DOEPINT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
