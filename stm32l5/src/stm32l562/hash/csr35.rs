#[doc = "Register `CSR35` reader"]
pub type R = crate::R<CSR35rs>;
#[doc = "Register `CSR35` writer"]
pub type W = crate::W<CSR35rs>;
#[doc = "Field `CSR35` reader - CSR35"]
pub type CSR35_R = crate::FieldReader<u32>;
#[doc = "Field `CSR35` writer - CSR35"]
pub type CSR35_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR35"]
    #[inline(always)]
    pub fn csr35(&self) -> CSR35_R {
        CSR35_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR35"]
    #[inline(always)]
    #[must_use]
    pub fn csr35(&mut self) -> CSR35_W<CSR35rs> {
        CSR35_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR35rs;
impl crate::RegisterSpec for CSR35rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr35::R`](R) reader structure"]
impl crate::Readable for CSR35rs {}
#[doc = "`write(|w| ..)` method takes [`csr35::W`](W) writer structure"]
impl crate::Writable for CSR35rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR35 to value 0"]
impl crate::Resettable for CSR35rs {
    const RESET_VALUE: u32 = 0;
}
