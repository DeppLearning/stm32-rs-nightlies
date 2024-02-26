#[doc = "Register `DCR4` reader"]
pub type R = crate::R<DCR4rs>;
#[doc = "Register `DCR4` writer"]
pub type W = crate::W<DCR4rs>;
#[doc = "Field `TEF` reader - Transfer error flag"]
pub type TEF_R = crate::BitReader;
#[doc = "Field `TEF` writer - Transfer error flag"]
pub type TEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCF` reader - Transfer complete flag"]
pub type TCF_R = crate::BitReader;
#[doc = "Field `TCF` writer - Transfer complete flag"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTF` reader - FIFO threshold flag"]
pub type FTF_R = crate::BitReader;
#[doc = "Field `FTF` writer - FIFO threshold flag"]
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMF` reader - Status match flag"]
pub type SMF_R = crate::BitReader;
#[doc = "Field `SMF` writer - Status match flag"]
pub type SMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOF` reader - Timeout flag"]
pub type TOF_R = crate::BitReader;
#[doc = "Field `TOF` writer - Timeout flag"]
pub type TOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLEVEL` reader - FIFO level"]
pub type FLEVEL_R = crate::FieldReader;
#[doc = "Field `FLEVEL` writer - FIFO level"]
pub type FLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - FIFO level"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<DCR4rs> {
        TEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<DCR4rs> {
        TCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<DCR4rs> {
        FTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    #[must_use]
    pub fn smf(&mut self) -> SMF_W<DCR4rs> {
        SMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<DCR4rs> {
        TOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<DCR4rs> {
        BUSY_W::new(self, 5)
    }
    #[doc = "Bits 8:13 - FIFO level"]
    #[inline(always)]
    #[must_use]
    pub fn flevel(&mut self) -> FLEVEL_W<DCR4rs> {
        FLEVEL_W::new(self, 8)
    }
}
#[doc = "DCR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR4rs;
impl crate::RegisterSpec for DCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr4::R`](R) reader structure"]
impl crate::Readable for DCR4rs {}
#[doc = "`write(|w| ..)` method takes [`dcr4::W`](W) writer structure"]
impl crate::Writable for DCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR4 to value 0"]
impl crate::Resettable for DCR4rs {
    const RESET_VALUE: u32 = 0;
}
