#[doc = "Register `SRDCR` reader"]
pub type R = crate::R<SRDCRrs>;
#[doc = "Register `SRDCR` writer"]
pub type W = crate::W<SRDCRrs>;
#[doc = "Field `VOSRDY` reader - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
pub type VOSRDY_R = crate::BitReader;
#[doc = "Field `VOS` reader - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
pub type VOS_R = crate::FieldReader;
#[doc = "Field `VOS` writer - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 13 - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<SRDCRrs> {
        VOS_W::new(self, 14)
    }
}
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRDCRrs;
impl crate::RegisterSpec for SRDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdcr::R`](R) reader structure"]
impl crate::Readable for SRDCRrs {}
#[doc = "`write(|w| ..)` method takes [`srdcr::W`](W) writer structure"]
impl crate::Writable for SRDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRDCR to value 0x4000"]
impl crate::Resettable for SRDCRrs {
    const RESET_VALUE: u32 = 0x4000;
}
