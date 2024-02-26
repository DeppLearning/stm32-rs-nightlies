#[doc = "Register `HASH_CSR14` reader"]
pub type R = crate::R<HASH_CSR14rs>;
#[doc = "Register `HASH_CSR14` writer"]
pub type W = crate::W<HASH_CSR14rs>;
#[doc = "Field `CS14` reader - CS14"]
pub type CS14_R = crate::FieldReader<u32>;
#[doc = "Field `CS14` writer - CS14"]
pub type CS14_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS14"]
    #[inline(always)]
    pub fn cs14(&self) -> CS14_R {
        CS14_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS14"]
    #[inline(always)]
    #[must_use]
    pub fn cs14(&mut self) -> CS14_W<HASH_CSR14rs> {
        CS14_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR14rs;
impl crate::RegisterSpec for HASH_CSR14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr14::R`](R) reader structure"]
impl crate::Readable for HASH_CSR14rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr14::W`](W) writer structure"]
impl crate::Writable for HASH_CSR14rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR14 to value 0"]
impl crate::Resettable for HASH_CSR14rs {
    const RESET_VALUE: u32 = 0;
}
