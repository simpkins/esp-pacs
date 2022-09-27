#[doc = "Register `LC_DMA_INT_RAW` reader"]
pub struct R(crate::R<LC_DMA_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_DMA_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_DMA_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_DMA_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCD_VSYNC_INT_RAW` reader - The raw bit for LCD frame end interrupt."]
pub type LCD_VSYNC_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `LCD_TRANS_DONE_INT_RAW` reader - The raw bit for lcd transfer end interrupt."]
pub type LCD_TRANS_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CAM_VSYNC_INT_RAW` reader - The raw bit for Camera frame end interrupt."]
pub type CAM_VSYNC_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CAM_HS_INT_RAW` reader - The raw bit for Camera line interrupt."]
pub type CAM_HS_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_raw(&self) -> LCD_VSYNC_INT_RAW_R {
        LCD_VSYNC_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_raw(&self) -> LCD_TRANS_DONE_INT_RAW_R {
        LCD_TRANS_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_raw(&self) -> CAM_VSYNC_INT_RAW_R {
        CAM_VSYNC_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for Camera line interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_raw(&self) -> CAM_HS_INT_RAW_R {
        CAM_HS_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "LCD_camera DMA raw inturrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_dma_int_raw](index.html) module"]
pub struct LC_DMA_INT_RAW_SPEC;
impl crate::RegisterSpec for LC_DMA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_dma_int_raw::R](R) reader structure"]
impl crate::Readable for LC_DMA_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LC_DMA_INT_RAW to value 0"]
impl crate::Resettable for LC_DMA_INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
