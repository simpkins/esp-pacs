#[doc = "Register `SPI_MEM_DIN_NUM` reader"]
pub type R = crate::R<SPI_MEM_DIN_NUM_SPEC>;
#[doc = "Register `SPI_MEM_DIN_NUM` writer"]
pub type W = crate::W<SPI_MEM_DIN_NUM_SPEC>;
#[doc = "Field `SPI_MEM_DIN0_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN0_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN0_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DIN1_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN1_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN1_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DIN2_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN2_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN2_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DIN3_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN3_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN3_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DIN4_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN4_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN4_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN4_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DIN5_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN5_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN5_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN5_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DIN6_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN6_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN6_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN6_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DIN7_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN7_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DIN7_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DIN7_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_DINS_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DINS_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_DINS_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SPI_MEM_DINS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din0_num(&self) -> SPI_MEM_DIN0_NUM_R {
        SPI_MEM_DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din1_num(&self) -> SPI_MEM_DIN1_NUM_R {
        SPI_MEM_DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din2_num(&self) -> SPI_MEM_DIN2_NUM_R {
        SPI_MEM_DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din3_num(&self) -> SPI_MEM_DIN3_NUM_R {
        SPI_MEM_DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din4_num(&self) -> SPI_MEM_DIN4_NUM_R {
        SPI_MEM_DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din5_num(&self) -> SPI_MEM_DIN5_NUM_R {
        SPI_MEM_DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din6_num(&self) -> SPI_MEM_DIN6_NUM_R {
        SPI_MEM_DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_din7_num(&self) -> SPI_MEM_DIN7_NUM_R {
        SPI_MEM_DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_mem_dins_num(&self) -> SPI_MEM_DINS_NUM_R {
        SPI_MEM_DINS_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_DIN_NUM")
            .field(
                "spi_mem_din0_num",
                &format_args!("{}", self.spi_mem_din0_num().bits()),
            )
            .field(
                "spi_mem_din1_num",
                &format_args!("{}", self.spi_mem_din1_num().bits()),
            )
            .field(
                "spi_mem_din2_num",
                &format_args!("{}", self.spi_mem_din2_num().bits()),
            )
            .field(
                "spi_mem_din3_num",
                &format_args!("{}", self.spi_mem_din3_num().bits()),
            )
            .field(
                "spi_mem_din4_num",
                &format_args!("{}", self.spi_mem_din4_num().bits()),
            )
            .field(
                "spi_mem_din5_num",
                &format_args!("{}", self.spi_mem_din5_num().bits()),
            )
            .field(
                "spi_mem_din6_num",
                &format_args!("{}", self.spi_mem_din6_num().bits()),
            )
            .field(
                "spi_mem_din7_num",
                &format_args!("{}", self.spi_mem_din7_num().bits()),
            )
            .field(
                "spi_mem_dins_num",
                &format_args!("{}", self.spi_mem_dins_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_DIN_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din0_num(&mut self) -> SPI_MEM_DIN0_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din1_num(&mut self) -> SPI_MEM_DIN1_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din2_num(&mut self) -> SPI_MEM_DIN2_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din3_num(&mut self) -> SPI_MEM_DIN3_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN3_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din4_num(&mut self) -> SPI_MEM_DIN4_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN4_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din5_num(&mut self) -> SPI_MEM_DIN5_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN5_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din6_num(&mut self) -> SPI_MEM_DIN6_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN6_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_din7_num(&mut self) -> SPI_MEM_DIN7_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DIN7_NUM_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dins_num(&mut self) -> SPI_MEM_DINS_NUM_W<SPI_MEM_DIN_NUM_SPEC> {
        SPI_MEM_DINS_NUM_W::new(self, 16)
    }
}
#[doc = "MSPI flash input timing delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_din_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_din_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_DIN_NUM_SPEC;
impl crate::RegisterSpec for SPI_MEM_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_din_num::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_DIN_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_din_num::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_DIN_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_DIN_NUM to value 0"]
impl crate::Resettable for SPI_MEM_DIN_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
