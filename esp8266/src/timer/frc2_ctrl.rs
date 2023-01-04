#[doc = "Register `FRC2_CTRL` reader"]
pub struct R(crate::R<FRC2_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRC2_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRC2_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRC2_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRC2_CTRL` writer"]
pub struct W(crate::W<FRC2_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRC2_CTRL_SPEC>;
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
impl From<crate::W<FRC2_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRC2_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frc2_ctrl` reader - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
pub type FRC2_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `frc2_ctrl` writer - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
pub type FRC2_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRC2_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `interrupt_type` reader - Configure the interrupt type"]
pub type INTERRUPT_TYPE_R = crate::BitReader<INTERRUPT_TYPE_A>;
#[doc = "Configure the interrupt type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_TYPE_A {
    #[doc = "0: edge"]
    EDGE = 0,
    #[doc = "1: level"]
    LEVEL = 1,
}
impl From<INTERRUPT_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl INTERRUPT_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERRUPT_TYPE_A {
        match self.bits {
            false => INTERRUPT_TYPE_A::EDGE,
            true => INTERRUPT_TYPE_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INTERRUPT_TYPE_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INTERRUPT_TYPE_A::LEVEL
    }
}
#[doc = "Field `interrupt_type` writer - Configure the interrupt type"]
pub type INTERRUPT_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRC2_CTRL_SPEC, INTERRUPT_TYPE_A, O>;
impl<'a, const O: u8> INTERRUPT_TYPE_W<'a, O> {
    #[doc = "edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INTERRUPT_TYPE_A::EDGE)
    }
    #[doc = "level"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INTERRUPT_TYPE_A::LEVEL)
    }
}
#[doc = "Field `prescale_divider` reader - Pre-scale divider for the timer"]
pub type PRESCALE_DIVIDER_R = crate::FieldReader<u8, PRESCALE_DIVIDER_A>;
#[doc = "Pre-scale divider for the timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALE_DIVIDER_A {
    #[doc = "0: divided by 1"]
    DEVIDED_BY_1 = 0,
    #[doc = "1: divided by 16"]
    DEVIDED_BY_16 = 1,
    #[doc = "2: divided by 256"]
    DEVIDED_BY_256 = 2,
}
impl From<PRESCALE_DIVIDER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_DIVIDER_A) -> Self {
        variant as _
    }
}
impl PRESCALE_DIVIDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALE_DIVIDER_A> {
        match self.bits {
            0 => Some(PRESCALE_DIVIDER_A::DEVIDED_BY_1),
            1 => Some(PRESCALE_DIVIDER_A::DEVIDED_BY_16),
            2 => Some(PRESCALE_DIVIDER_A::DEVIDED_BY_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEVIDED_BY_1`"]
    #[inline(always)]
    pub fn is_devided_by_1(&self) -> bool {
        *self == PRESCALE_DIVIDER_A::DEVIDED_BY_1
    }
    #[doc = "Checks if the value of the field is `DEVIDED_BY_16`"]
    #[inline(always)]
    pub fn is_devided_by_16(&self) -> bool {
        *self == PRESCALE_DIVIDER_A::DEVIDED_BY_16
    }
    #[doc = "Checks if the value of the field is `DEVIDED_BY_256`"]
    #[inline(always)]
    pub fn is_devided_by_256(&self) -> bool {
        *self == PRESCALE_DIVIDER_A::DEVIDED_BY_256
    }
}
#[doc = "Field `prescale_divider` writer - Pre-scale divider for the timer"]
pub type PRESCALE_DIVIDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRC2_CTRL_SPEC, u8, PRESCALE_DIVIDER_A, 2, O>;
impl<'a, const O: u8> PRESCALE_DIVIDER_W<'a, O> {
    #[doc = "divided by 1"]
    #[inline(always)]
    pub fn devided_by_1(self) -> &'a mut W {
        self.variant(PRESCALE_DIVIDER_A::DEVIDED_BY_1)
    }
    #[doc = "divided by 16"]
    #[inline(always)]
    pub fn devided_by_16(self) -> &'a mut W {
        self.variant(PRESCALE_DIVIDER_A::DEVIDED_BY_16)
    }
    #[doc = "divided by 256"]
    #[inline(always)]
    pub fn devided_by_256(self) -> &'a mut W {
        self.variant(PRESCALE_DIVIDER_A::DEVIDED_BY_256)
    }
}
#[doc = "Field `rollover` reader - Automatically reload when the counter hits zero"]
pub type ROLLOVER_R = crate::BitReader<bool>;
#[doc = "Field `rollover` writer - Automatically reload when the counter hits zero"]
pub type ROLLOVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRC2_CTRL_SPEC, bool, O>;
#[doc = "Field `timer_enable` reader - Enable or disable the timer"]
pub type TIMER_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `timer_enable` writer - Enable or disable the timer"]
pub type TIMER_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRC2_CTRL_SPEC, bool, O>;
#[doc = "Field `frc2_int` reader - the status of the interrupt, when the count is equal tothe alarm value"]
pub type FRC2_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
    #[inline(always)]
    pub fn frc2_ctrl(&self) -> FRC2_CTRL_R {
        FRC2_CTRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 0 - Configure the interrupt type"]
    #[inline(always)]
    pub fn interrupt_type(&self) -> INTERRUPT_TYPE_R {
        INTERRUPT_TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pre-scale divider for the timer"]
    #[inline(always)]
    pub fn prescale_divider(&self) -> PRESCALE_DIVIDER_R {
        PRESCALE_DIVIDER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Automatically reload when the counter hits zero"]
    #[inline(always)]
    pub fn rollover(&self) -> ROLLOVER_R {
        ROLLOVER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable the timer"]
    #[inline(always)]
    pub fn timer_enable(&self) -> TIMER_ENABLE_R {
        TIMER_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the status of the interrupt, when the count is equal tothe alarm value"]
    #[inline(always)]
    pub fn frc2_int(&self) -> FRC2_INT_R {
        FRC2_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
    #[inline(always)]
    #[must_use]
    pub fn frc2_ctrl(&mut self) -> FRC2_CTRL_W<0> {
        FRC2_CTRL_W::new(self)
    }
    #[doc = "Bit 0 - Configure the interrupt type"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_type(&mut self) -> INTERRUPT_TYPE_W<0> {
        INTERRUPT_TYPE_W::new(self)
    }
    #[doc = "Bits 2:3 - Pre-scale divider for the timer"]
    #[inline(always)]
    #[must_use]
    pub fn prescale_divider(&mut self) -> PRESCALE_DIVIDER_W<2> {
        PRESCALE_DIVIDER_W::new(self)
    }
    #[doc = "Bit 6 - Automatically reload when the counter hits zero"]
    #[inline(always)]
    #[must_use]
    pub fn rollover(&mut self) -> ROLLOVER_W<6> {
        ROLLOVER_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable the timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer_enable(&mut self) -> TIMER_ENABLE_W<7> {
        TIMER_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRC2_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frc2_ctrl](index.html) module"]
pub struct FRC2_CTRL_SPEC;
impl crate::RegisterSpec for FRC2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frc2_ctrl::R](R) reader structure"]
impl crate::Readable for FRC2_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frc2_ctrl::W](W) writer structure"]
impl crate::Writable for FRC2_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC2_CTRL to value 0"]
impl crate::Resettable for FRC2_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
