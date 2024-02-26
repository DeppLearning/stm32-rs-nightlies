#[doc = "Register `DCR2` reader"]
pub type R = crate::R<DCR2rs>;
#[doc = "Register `DCR2` writer"]
pub type W = crate::W<DCR2rs>;
#[doc = "Field `PRESCALER` reader - Clock prescaler"]
pub type PRESCALER_R = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - Clock prescaler"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Wrap size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRAPSIZE {
    #[doc = "0: Wrapped reads are not supported by the memory"]
    NoWrappingSupport = 0,
    #[doc = "2: External memory supports wrap size of 16 bytes"]
    WrappingSize16 = 2,
    #[doc = "3: External memory supports wrap size of 16 bytes"]
    WrappingSize32 = 3,
    #[doc = "4: External memory supports wrap size of 16 bytes"]
    WrappingSize64 = 4,
    #[doc = "5: External memory supports wrap size of 16 bytes"]
    WrappingSize128 = 5,
}
impl From<WRAPSIZE> for u8 {
    #[inline(always)]
    fn from(variant: WRAPSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRAPSIZE {
    type Ux = u8;
}
#[doc = "Field `WRAPSIZE` reader - Wrap size"]
pub type WRAPSIZE_R = crate::FieldReader<WRAPSIZE>;
impl WRAPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WRAPSIZE> {
        match self.bits {
            0 => Some(WRAPSIZE::NoWrappingSupport),
            2 => Some(WRAPSIZE::WrappingSize16),
            3 => Some(WRAPSIZE::WrappingSize32),
            4 => Some(WRAPSIZE::WrappingSize64),
            5 => Some(WRAPSIZE::WrappingSize128),
            _ => None,
        }
    }
    #[doc = "Wrapped reads are not supported by the memory"]
    #[inline(always)]
    pub fn is_no_wrapping_support(&self) -> bool {
        *self == WRAPSIZE::NoWrappingSupport
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn is_wrapping_size16(&self) -> bool {
        *self == WRAPSIZE::WrappingSize16
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn is_wrapping_size32(&self) -> bool {
        *self == WRAPSIZE::WrappingSize32
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn is_wrapping_size64(&self) -> bool {
        *self == WRAPSIZE::WrappingSize64
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn is_wrapping_size128(&self) -> bool {
        *self == WRAPSIZE::WrappingSize128
    }
}
#[doc = "Field `WRAPSIZE` writer - Wrap size"]
pub type WRAPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WRAPSIZE>;
impl<'a, REG> WRAPSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wrapped reads are not supported by the memory"]
    #[inline(always)]
    pub fn no_wrapping_support(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::NoWrappingSupport)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size16(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize16)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size32(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize32)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size64(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize64)
    }
    #[doc = "External memory supports wrap size of 16 bytes"]
    #[inline(always)]
    pub fn wrapping_size128(self) -> &'a mut crate::W<REG> {
        self.variant(WRAPSIZE::WrappingSize128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Wrap size"]
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<DCR2rs> {
        PRESCALER_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - Wrap size"]
    #[inline(always)]
    #[must_use]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W<DCR2rs> {
        WRAPSIZE_W::new(self, 16)
    }
}
#[doc = "device configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR2rs;
impl crate::RegisterSpec for DCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr2::R`](R) reader structure"]
impl crate::Readable for DCR2rs {}
#[doc = "`write(|w| ..)` method takes [`dcr2::W`](W) writer structure"]
impl crate::Writable for DCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR2 to value 0"]
impl crate::Resettable for DCR2rs {
    const RESET_VALUE: u32 = 0;
}
