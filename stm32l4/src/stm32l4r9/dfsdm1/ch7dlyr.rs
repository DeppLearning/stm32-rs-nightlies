#[doc = "Register `CH7DLYR` reader"]
pub type R = crate::R<CH7DLYRrs>;
#[doc = "Register `CH7DLYR` writer"]
pub type W = crate::W<CH7DLYRrs>;
#[doc = "Field `PLSSKP` reader - PLSSKP"]
pub type PLSSKP_R = crate::FieldReader;
#[doc = "Field `PLSSKP` writer - PLSSKP"]
pub type PLSSKP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<CH7DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
#[doc = "channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7dlyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7dlyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH7DLYRrs;
impl crate::RegisterSpec for CH7DLYRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7dlyr::R`](R) reader structure"]
impl crate::Readable for CH7DLYRrs {}
#[doc = "`write(|w| ..)` method takes [`ch7dlyr::W`](W) writer structure"]
impl crate::Writable for CH7DLYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH7DLYR to value 0"]
impl crate::Resettable for CH7DLYRrs {
    const RESET_VALUE: u32 = 0;
}
