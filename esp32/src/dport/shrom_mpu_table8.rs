#[doc = "Register `SHROM_MPU_TABLE8` reader"]
pub type R = crate::R<SHROM_MPU_TABLE8_SPEC>;
#[doc = "Register `SHROM_MPU_TABLE8` writer"]
pub type W = crate::W<SHROM_MPU_TABLE8_SPEC>;
#[doc = "Field `SHROM_MPU_TABLE8` reader - "]
pub type SHROM_MPU_TABLE8_R = crate::FieldReader;
#[doc = "Field `SHROM_MPU_TABLE8` writer - "]
pub type SHROM_MPU_TABLE8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table8(&self) -> SHROM_MPU_TABLE8_R {
        SHROM_MPU_TABLE8_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHROM_MPU_TABLE8")
            .field(
                "shrom_mpu_table8",
                &format_args!("{}", self.shrom_mpu_table8().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHROM_MPU_TABLE8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn shrom_mpu_table8(&mut self) -> SHROM_MPU_TABLE8_W<SHROM_MPU_TABLE8_SPEC> {
        SHROM_MPU_TABLE8_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHROM_MPU_TABLE8_SPEC;
impl crate::RegisterSpec for SHROM_MPU_TABLE8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shrom_mpu_table8::R`](R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shrom_mpu_table8::W`](W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHROM_MPU_TABLE8 to value 0x01"]
impl crate::Resettable for SHROM_MPU_TABLE8_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
