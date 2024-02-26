#[doc = "Register `MDMA_C25MAR` reader"]
pub type R = crate::R<MDMA_C25MARrs>;
#[doc = "Register `MDMA_C25MAR` writer"]
pub type W = crate::W<MDMA_C25MARrs>;
#[doc = "Field `MAR` reader - MAR"]
pub type MAR_R = crate::FieldReader<u32>;
#[doc = "Field `MAR` writer - MAR"]
pub type MAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAR"]
    #[inline(always)]
    pub fn mar(&self) -> MAR_R {
        MAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAR"]
    #[inline(always)]
    #[must_use]
    pub fn mar(&mut self) -> MAR_W<MDMA_C25MARrs> {
        MAR_W::new(self, 0)
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c25mar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c25mar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C25MARrs;
impl crate::RegisterSpec for MDMA_C25MARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c25mar::R`](R) reader structure"]
impl crate::Readable for MDMA_C25MARrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c25mar::W`](W) writer structure"]
impl crate::Writable for MDMA_C25MARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C25MAR to value 0"]
impl crate::Resettable for MDMA_C25MARrs {
    const RESET_VALUE: u32 = 0;
}
