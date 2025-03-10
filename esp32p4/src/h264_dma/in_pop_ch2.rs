#[doc = "Register `IN_POP_CH2` reader"]
pub type R = crate::R<IN_POP_CH2_SPEC>;
#[doc = "Register `IN_POP_CH2` writer"]
pub type W = crate::W<IN_POP_CH2_SPEC>;
#[doc = "Field `INFIFO_RDATA_CH2` reader - This register stores the data popping from DMA Rx FIFO."]
pub type INFIFO_RDATA_CH2_R = crate::FieldReader<u16>;
#[doc = "Field `INFIFO_POP_CH2` reader - Set this bit to pop data from DMA Rx FIFO."]
pub type INFIFO_POP_CH2_R = crate::BitReader;
#[doc = "Field `INFIFO_POP_CH2` writer - Set this bit to pop data from DMA Rx FIFO."]
pub type INFIFO_POP_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - This register stores the data popping from DMA Rx FIFO."]
    #[inline(always)]
    pub fn infifo_rdata_ch2(&self) -> INFIFO_RDATA_CH2_R {
        INFIFO_RDATA_CH2_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Set this bit to pop data from DMA Rx FIFO."]
    #[inline(always)]
    pub fn infifo_pop_ch2(&self) -> INFIFO_POP_CH2_R {
        INFIFO_POP_CH2_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_POP_CH2")
            .field(
                "infifo_rdata_ch2",
                &format_args!("{}", self.infifo_rdata_ch2().bits()),
            )
            .field(
                "infifo_pop_ch2",
                &format_args!("{}", self.infifo_pop_ch2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_POP_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 11 - Set this bit to pop data from DMA Rx FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_pop_ch2(&mut self) -> INFIFO_POP_CH2_W<IN_POP_CH2_SPEC> {
        INFIFO_POP_CH2_W::new(self, 11)
    }
}
#[doc = "RX CH2 INFIFO pop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_POP_CH2_SPEC;
impl crate::RegisterSpec for IN_POP_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pop_ch2::R`](R) reader structure"]
impl crate::Readable for IN_POP_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_pop_ch2::W`](W) writer structure"]
impl crate::Writable for IN_POP_CH2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_POP_CH2 to value 0x0400"]
impl crate::Resettable for IN_POP_CH2_SPEC {
    const RESET_VALUE: u32 = 0x0400;
}
