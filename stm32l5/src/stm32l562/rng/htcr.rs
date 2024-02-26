#[doc = "Register `HTCR` reader"]
pub type R = crate::R<HTCRrs>;
#[doc = "Register `HTCR` writer"]
pub type W = crate::W<HTCRrs>;
#[doc = "Field `HTCFG` reader - health test configuration"]
pub type HTCFG_R = crate::FieldReader<u32>;
#[doc = "Field `HTCFG` writer - health test configuration"]
pub type HTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    #[must_use]
    pub fn htcfg(&mut self) -> HTCFG_W<HTCRrs> {
        HTCFG_W::new(self, 0)
    }
}
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTCRrs;
impl crate::RegisterSpec for HTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htcr::R`](R) reader structure"]
impl crate::Readable for HTCRrs {}
#[doc = "`write(|w| ..)` method takes [`htcr::W`](W) writer structure"]
impl crate::Writable for HTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTCR to value 0x000c_aa74"]
impl crate::Resettable for HTCRrs {
    const RESET_VALUE: u32 = 0x000c_aa74;
}
