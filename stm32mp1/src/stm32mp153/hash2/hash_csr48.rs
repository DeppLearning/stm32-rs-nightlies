#[doc = "Register `HASH_CSR48` reader"]
pub type R = crate::R<HASH_CSR48rs>;
#[doc = "Register `HASH_CSR48` writer"]
pub type W = crate::W<HASH_CSR48rs>;
#[doc = "Field `CS48` reader - CS48"]
pub type CS48_R = crate::FieldReader<u32>;
#[doc = "Field `CS48` writer - CS48"]
pub type CS48_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS48"]
    #[inline(always)]
    pub fn cs48(&self) -> CS48_R {
        CS48_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS48"]
    #[inline(always)]
    #[must_use]
    pub fn cs48(&mut self) -> CS48_W<HASH_CSR48rs> {
        CS48_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR48rs;
impl crate::RegisterSpec for HASH_CSR48rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr48::R`](R) reader structure"]
impl crate::Readable for HASH_CSR48rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr48::W`](W) writer structure"]
impl crate::Writable for HASH_CSR48rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR48 to value 0"]
impl crate::Resettable for HASH_CSR48rs {
    const RESET_VALUE: u32 = 0;
}
