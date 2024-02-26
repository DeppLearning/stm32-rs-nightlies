#[doc = "Register `B3CR` reader"]
pub type R = crate::R<B3CRrs>;
#[doc = "Register `B3CR` writer"]
pub type W = crate::W<B3CRrs>;
#[doc = "Field `PBO` reader - Physical buffer offset"]
pub type PBO_R = crate::FieldReader<u32>;
#[doc = "Field `PBO` writer - Physical buffer offset"]
pub type PBO_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PBBA` reader - Physical buffer base address"]
pub type PBBA_R = crate::FieldReader<u16>;
#[doc = "Field `PBBA` writer - Physical buffer base address"]
pub type PBBA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 4:22 - Physical buffer offset"]
    #[inline(always)]
    pub fn pbo(&self) -> PBO_R {
        PBO_R::new((self.bits >> 4) & 0x0007_ffff)
    }
    #[doc = "Bits 23:31 - Physical buffer base address"]
    #[inline(always)]
    pub fn pbba(&self) -> PBBA_R {
        PBBA_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:22 - Physical buffer offset"]
    #[inline(always)]
    #[must_use]
    pub fn pbo(&mut self) -> PBO_W<B3CRrs> {
        PBO_W::new(self, 4)
    }
    #[doc = "Bits 23:31 - Physical buffer base address"]
    #[inline(always)]
    #[must_use]
    pub fn pbba(&mut self) -> PBBA_W<B3CRrs> {
        PBBA_W::new(self, 23)
    }
}
#[doc = "Graphic MMU buffer 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b3cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b3cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B3CRrs;
impl crate::RegisterSpec for B3CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b3cr::R`](R) reader structure"]
impl crate::Readable for B3CRrs {}
#[doc = "`write(|w| ..)` method takes [`b3cr::W`](W) writer structure"]
impl crate::Writable for B3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B3CR to value 0"]
impl crate::Resettable for B3CRrs {
    const RESET_VALUE: u32 = 0;
}
