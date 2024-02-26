#[doc = "Register `COUNT7_RX` reader"]
pub type R = crate::R<COUNT7_RXrs>;
#[doc = "Register `COUNT7_RX` writer"]
pub type W = crate::W<COUNT7_RXrs>;
#[doc = "Field `COUNT7_RX` reader - Reception byte count"]
pub type COUNT7_RX_R = crate::FieldReader<u16>;
#[doc = "Field `NUM_BLOCK` reader - Number of blocks"]
pub type NUM_BLOCK_R = crate::FieldReader;
#[doc = "Field `NUM_BLOCK` writer - Number of blocks"]
pub type NUM_BLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BL_SIZE` reader - Block size"]
pub type BL_SIZE_R = crate::BitReader;
#[doc = "Field `BL_SIZE` writer - Block size"]
pub type BL_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Reception byte count"]
    #[inline(always)]
    pub fn count7_rx(&self) -> COUNT7_RX_R {
        COUNT7_RX_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    pub fn num_block(&self) -> NUM_BLOCK_R {
        NUM_BLOCK_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    pub fn bl_size(&self) -> BL_SIZE_R {
        BL_SIZE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    #[must_use]
    pub fn num_block(&mut self) -> NUM_BLOCK_W<COUNT7_RXrs> {
        NUM_BLOCK_W::new(self, 10)
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    #[must_use]
    pub fn bl_size(&mut self) -> BL_SIZE_W<COUNT7_RXrs> {
        BL_SIZE_W::new(self, 15)
    }
}
#[doc = "Reception byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count7_rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count7_rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNT7_RXrs;
impl crate::RegisterSpec for COUNT7_RXrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`count7_rx::R`](R) reader structure"]
impl crate::Readable for COUNT7_RXrs {}
#[doc = "`write(|w| ..)` method takes [`count7_rx::W`](W) writer structure"]
impl crate::Writable for COUNT7_RXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COUNT7_RX to value 0"]
impl crate::Resettable for COUNT7_RXrs {
    const RESET_VALUE: u16 = 0;
}
