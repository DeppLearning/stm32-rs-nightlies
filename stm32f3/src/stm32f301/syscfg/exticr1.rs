#[doc = "Register `EXTICR1` reader"]
pub type R = crate::R<EXTICR1rs>;
#[doc = "Register `EXTICR1` writer"]
pub type W = crate::W<EXTICR1rs>;
#[doc = "EXTI 0 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0 {
    #[doc = "0: Select PA0 as the source input for the EXTI0 external interrupt"]
    Pa0 = 0,
    #[doc = "1: Select PB0 as the source input for the EXTI0 external interrupt"]
    Pb0 = 1,
    #[doc = "2: Select PC0 as the source input for the EXTI0 external interrupt"]
    Pc0 = 2,
    #[doc = "5: Select PF0 as the source input for the EXTI0 external interrupt"]
    Pf0 = 5,
}
impl From<EXTI0> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0 {
    type Ux = u8;
}
#[doc = "Field `EXTI0` reader - EXTI 0 configuration bits"]
pub type EXTI0_R = crate::FieldReader<EXTI0>;
impl EXTI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI0> {
        match self.bits {
            0 => Some(EXTI0::Pa0),
            1 => Some(EXTI0::Pb0),
            2 => Some(EXTI0::Pc0),
            5 => Some(EXTI0::Pf0),
            _ => None,
        }
    }
    #[doc = "Select PA0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0::Pa0
    }
    #[doc = "Select PB0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0::Pb0
    }
    #[doc = "Select PC0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0::Pc0
    }
    #[doc = "Select PF0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == EXTI0::Pf0
    }
}
#[doc = "Field `EXTI0` writer - EXTI 0 configuration bits"]
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI0>;
impl<'a, REG> EXTI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pa0)
    }
    #[doc = "Select PB0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pb0)
    }
    #[doc = "Select PC0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pc0)
    }
    #[doc = "Select PF0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pf0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pf0)
    }
}
#[doc = "EXTI 1 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1 {
    #[doc = "0: Select PA1 as the source input for the EXTI1 external interrupt"]
    Pa1 = 0,
    #[doc = "1: Select PB1 as the source input for the EXTI1 external interrupt"]
    Pb1 = 1,
    #[doc = "2: Select PC1 as the source input for the EXTI1 external interrupt"]
    Pc1 = 2,
    #[doc = "5: Select PF1 as the source input for the EXTI1 external interrupt"]
    Pf1 = 5,
}
impl From<EXTI1> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI1 {
    type Ux = u8;
}
#[doc = "Field `EXTI1` reader - EXTI 1 configuration bits"]
pub type EXTI1_R = crate::FieldReader<EXTI1>;
impl EXTI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI1> {
        match self.bits {
            0 => Some(EXTI1::Pa1),
            1 => Some(EXTI1::Pb1),
            2 => Some(EXTI1::Pc1),
            5 => Some(EXTI1::Pf1),
            _ => None,
        }
    }
    #[doc = "Select PA1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1::Pa1
    }
    #[doc = "Select PB1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1::Pb1
    }
    #[doc = "Select PC1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1::Pc1
    }
    #[doc = "Select PF1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == EXTI1::Pf1
    }
}
#[doc = "Field `EXTI1` writer - EXTI 1 configuration bits"]
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI1>;
impl<'a, REG> EXTI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pa1)
    }
    #[doc = "Select PB1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pb1)
    }
    #[doc = "Select PC1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pc1)
    }
    #[doc = "Select PF1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pf1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pf1)
    }
}
#[doc = "EXTI 2 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2 {
    #[doc = "0: Select PA2 as the source input for the EXTI2 external interrupt"]
    Pa2 = 0,
    #[doc = "1: Select PB2 as the source input for the EXTI2 external interrupt"]
    Pb2 = 1,
    #[doc = "2: Select PC2 as the source input for the EXTI2 external interrupt"]
    Pc2 = 2,
    #[doc = "3: Select PD2 as the source input for the EXTI2 external interrupt"]
    Pd2 = 3,
}
impl From<EXTI2> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI2 {
    type Ux = u8;
}
#[doc = "Field `EXTI2` reader - EXTI 2 configuration bits"]
pub type EXTI2_R = crate::FieldReader<EXTI2>;
impl EXTI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI2> {
        match self.bits {
            0 => Some(EXTI2::Pa2),
            1 => Some(EXTI2::Pb2),
            2 => Some(EXTI2::Pc2),
            3 => Some(EXTI2::Pd2),
            _ => None,
        }
    }
    #[doc = "Select PA2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2::Pa2
    }
    #[doc = "Select PB2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2::Pb2
    }
    #[doc = "Select PC2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2::Pc2
    }
    #[doc = "Select PD2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2::Pd2
    }
}
#[doc = "Field `EXTI2` writer - EXTI 2 configuration bits"]
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI2>;
impl<'a, REG> EXTI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pa2)
    }
    #[doc = "Select PB2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pb2)
    }
    #[doc = "Select PC2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pc2)
    }
    #[doc = "Select PD2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pd2)
    }
}
#[doc = "EXTI 3 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3 {
    #[doc = "0: Select PA3 as the source input for the EXTI3 external interrupt"]
    Pa3 = 0,
    #[doc = "1: Select PB3 as the source input for the EXTI3 external interrupt"]
    Pb3 = 1,
    #[doc = "2: Select PC3 as the source input for the EXTI3 external interrupt"]
    Pc3 = 2,
}
impl From<EXTI3> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI3 {
    type Ux = u8;
}
#[doc = "Field `EXTI3` reader - EXTI 3 configuration bits"]
pub type EXTI3_R = crate::FieldReader<EXTI3>;
impl EXTI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI3> {
        match self.bits {
            0 => Some(EXTI3::Pa3),
            1 => Some(EXTI3::Pb3),
            2 => Some(EXTI3::Pc3),
            _ => None,
        }
    }
    #[doc = "Select PA3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3::Pa3
    }
    #[doc = "Select PB3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3::Pb3
    }
    #[doc = "Select PC3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3::Pc3
    }
}
#[doc = "Field `EXTI3` writer - EXTI 3 configuration bits"]
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI3>;
impl<'a, REG> EXTI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pa3)
    }
    #[doc = "Select PB3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pb3)
    }
    #[doc = "Select PC3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pc3)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 0 configuration bits"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration bits"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration bits"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration bits"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1rs> {
        EXTI1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1rs> {
        EXTI2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1rs> {
        EXTI3_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR1rs;
impl crate::RegisterSpec for EXTICR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr1::R`](R) reader structure"]
impl crate::Readable for EXTICR1rs {}
#[doc = "`write(|w| ..)` method takes [`exticr1::W`](W) writer structure"]
impl crate::Writable for EXTICR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1rs {
    const RESET_VALUE: u32 = 0;
}
