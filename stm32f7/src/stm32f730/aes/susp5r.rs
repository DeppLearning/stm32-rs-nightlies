#[doc = "Register `SUSP5R` reader"]
pub type R = crate::R<SUSP5Rrs>;
#[doc = "Register `SUSP5R` writer"]
pub type W = crate::W<SUSP5Rrs>;
#[doc = "Field `AES_SUSP5R` reader - AES suspend register 5"]
pub type AES_SUSP5R_R = crate::FieldReader<u32>;
#[doc = "Field `AES_SUSP5R` writer - AES suspend register 5"]
pub type AES_SUSP5R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 5"]
    #[inline(always)]
    pub fn aes_susp5r(&self) -> AES_SUSP5R_R {
        AES_SUSP5R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 5"]
    #[inline(always)]
    #[must_use]
    pub fn aes_susp5r(&mut self) -> AES_SUSP5R_W<SUSP5Rrs> {
        AES_SUSP5R_W::new(self, 0)
    }
}
#[doc = "AES suspend register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp5r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp5r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP5Rrs;
impl crate::RegisterSpec for SUSP5Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp5r::R`](R) reader structure"]
impl crate::Readable for SUSP5Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp5r::W`](W) writer structure"]
impl crate::Writable for SUSP5Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP5R to value 0"]
impl crate::Resettable for SUSP5Rrs {
    const RESET_VALUE: u32 = 0;
}
