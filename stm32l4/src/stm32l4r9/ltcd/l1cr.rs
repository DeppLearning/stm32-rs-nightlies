#[doc = "Register `L1CR` reader"]
pub type R = crate::R<L1CRrs>;
#[doc = "Register `L1CR` writer"]
pub type W = crate::W<L1CRrs>;
#[doc = "Field `LEN` reader - Layer Enable"]
pub type LEN_R = crate::BitReader;
#[doc = "Field `LEN` writer - Layer Enable"]
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLKEN` reader - Color Keying Enable"]
pub type COLKEN_R = crate::BitReader;
#[doc = "Field `COLKEN` writer - Color Keying Enable"]
pub type COLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLUTEN` reader - Color Look-Up Table Enable"]
pub type CLUTEN_R = crate::BitReader;
#[doc = "Field `CLUTEN` writer - Color Look-Up Table Enable"]
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<L1CRrs> {
        LEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    #[must_use]
    pub fn colken(&mut self) -> COLKEN_W<L1CRrs> {
        COLKEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cluten(&mut self) -> CLUTEN_W<L1CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
#[doc = "LTDC Layer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CRrs;
impl crate::RegisterSpec for L1CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cr::R`](R) reader structure"]
impl crate::Readable for L1CRrs {}
#[doc = "`write(|w| ..)` method takes [`l1cr::W`](W) writer structure"]
impl crate::Writable for L1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1CR to value 0"]
impl crate::Resettable for L1CRrs {
    const RESET_VALUE: u32 = 0;
}
