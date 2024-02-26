#[doc = "Register `KRR` writer"]
pub type W = crate::W<KRRrs>;
#[doc = "Field `b` writer - b192"]
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - b192"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<KRRrs> {
        B_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`krr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KRRrs;
impl crate::RegisterSpec for KRRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`krr::W`](W) writer structure"]
impl crate::Writable for KRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KRR to value 0"]
impl crate::Resettable for KRRrs {
    const RESET_VALUE: u32 = 0;
}
