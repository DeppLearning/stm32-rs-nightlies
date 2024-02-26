#[doc = "Register `CID` reader"]
pub type R = crate::R<CIDrs>;
#[doc = "Register `CID` writer"]
pub type W = crate::W<CIDrs>;
#[doc = "Field `PRODUCT_ID` reader - Product ID field"]
pub type PRODUCT_ID_R = crate::FieldReader<u32>;
#[doc = "Field `PRODUCT_ID` writer - Product ID field"]
pub type PRODUCT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<CIDrs> {
        PRODUCT_ID_W::new(self, 0)
    }
}
#[doc = "core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDrs;
impl crate::RegisterSpec for CIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid::R`](R) reader structure"]
impl crate::Readable for CIDrs {}
#[doc = "`write(|w| ..)` method takes [`cid::W`](W) writer structure"]
impl crate::Writable for CIDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CID to value 0x2000"]
impl crate::Resettable for CIDrs {
    const RESET_VALUE: u32 = 0x2000;
}
