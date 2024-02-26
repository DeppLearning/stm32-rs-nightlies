#[doc = "Register `ALRM%sSSR` reader"]
pub type R = crate::R<ALRMSSRrs>;
#[doc = "Register `ALRM%sSSR` writer"]
pub type W = crate::W<ALRMSSRrs>;
#[doc = "Field `SS` reader - SS"]
pub type SS_R = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - SS"]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - MASKSS"]
pub type MASKSS_R = crate::FieldReader;
#[doc = "Field `MASKSS` writer - MASKSS"]
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - SS"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - MASKSS"]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - SS"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<ALRMSSRrs> {
        SS_W::new(self, 0)
    }
    #[doc = "Bits 24:27 - MASKSS"]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<ALRMSSRrs> {
        MASKSS_W::new(self, 24)
    }
}
#[doc = "Alarm %s sub-second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMSSRrs;
impl crate::RegisterSpec for ALRMSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmssr::R`](R) reader structure"]
impl crate::Readable for ALRMSSRrs {}
#[doc = "`write(|w| ..)` method takes [`alrmssr::W`](W) writer structure"]
impl crate::Writable for ALRMSSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRM%sSSR to value 0"]
impl crate::Resettable for ALRMSSRrs {
    const RESET_VALUE: u32 = 0;
}
