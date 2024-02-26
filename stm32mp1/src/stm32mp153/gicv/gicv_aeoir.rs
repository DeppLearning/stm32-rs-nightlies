#[doc = "Register `GICV_AEOIR` writer"]
pub type W = crate::W<GICV_AEOIRrs>;
#[doc = "Field `EOIINTID` writer - EOIINTID"]
pub type EOIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CPUID` writer - CPUID"]
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:9 - EOIINTID"]
    #[inline(always)]
    #[must_use]
    pub fn eoiintid(&mut self) -> EOIINTID_W<GICV_AEOIRrs> {
        EOIINTID_W::new(self, 0)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICV_AEOIRrs> {
        CPUID_W::new(self, 10)
    }
}
#[doc = "GICV VM aliased end of interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_aeoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_AEOIRrs;
impl crate::RegisterSpec for GICV_AEOIRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicv_aeoir::W`](W) writer structure"]
impl crate::Writable for GICV_AEOIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICV_AEOIR to value 0"]
impl crate::Resettable for GICV_AEOIRrs {
    const RESET_VALUE: u32 = 0;
}
