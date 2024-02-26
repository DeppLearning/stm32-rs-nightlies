#[doc = "Register `HASH_CSR37` reader"]
pub type R = crate::R<HASH_CSR37rs>;
#[doc = "Register `HASH_CSR37` writer"]
pub type W = crate::W<HASH_CSR37rs>;
#[doc = "Field `CS37` reader - CS37"]
pub type CS37_R = crate::FieldReader<u32>;
#[doc = "Field `CS37` writer - CS37"]
pub type CS37_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS37"]
    #[inline(always)]
    pub fn cs37(&self) -> CS37_R {
        CS37_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS37"]
    #[inline(always)]
    #[must_use]
    pub fn cs37(&mut self) -> CS37_W<HASH_CSR37rs> {
        CS37_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR37rs;
impl crate::RegisterSpec for HASH_CSR37rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr37::R`](R) reader structure"]
impl crate::Readable for HASH_CSR37rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr37::W`](W) writer structure"]
impl crate::Writable for HASH_CSR37rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR37 to value 0"]
impl crate::Resettable for HASH_CSR37rs {
    const RESET_VALUE: u32 = 0;
}
