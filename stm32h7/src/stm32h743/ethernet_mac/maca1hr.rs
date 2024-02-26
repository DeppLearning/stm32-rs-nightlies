#[doc = "Register `MACA1HR` reader"]
pub type R = crate::R<MACA1HRrs>;
#[doc = "Register `MACA1HR` writer"]
pub type W = crate::W<MACA1HRrs>;
#[doc = "Field `ADDRHI` reader - MAC Address1 \\[47:32\\]"]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - MAC Address1 \\[47:32\\]"]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask Byte Control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask Byte Control"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source Address"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source Address"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address Enable"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address Enable"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<MACA1HRrs> {
        ADDRHI_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA1HRrs> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA1HRrs> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA1HRrs> {
        AE_W::new(self, 31)
    }
}
#[doc = "Address 1 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA1HRrs;
impl crate::RegisterSpec for MACA1HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1hr::R`](R) reader structure"]
impl crate::Readable for MACA1HRrs {}
#[doc = "`write(|w| ..)` method takes [`maca1hr::W`](W) writer structure"]
impl crate::Writable for MACA1HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA1HR to value 0xffff"]
impl crate::Resettable for MACA1HRrs {
    const RESET_VALUE: u32 = 0xffff;
}
