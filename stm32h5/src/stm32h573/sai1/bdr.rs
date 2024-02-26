#[doc = "Register `BDR` reader"]
pub type R = crate::R<BDRrs>;
#[doc = "Register `BDR` writer"]
pub type W = crate::W<BDRrs>;
#[doc = "Field `DATA` reader - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<BDRrs> {
        DATA_W::new(self, 0)
    }
}
#[doc = "SAI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDRrs;
impl crate::RegisterSpec for BDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdr::R`](R) reader structure"]
impl crate::Readable for BDRrs {}
#[doc = "`write(|w| ..)` method takes [`bdr::W`](W) writer structure"]
impl crate::Writable for BDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDR to value 0"]
impl crate::Resettable for BDRrs {
    const RESET_VALUE: u32 = 0;
}
