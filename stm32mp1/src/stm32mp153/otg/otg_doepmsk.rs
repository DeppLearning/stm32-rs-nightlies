#[doc = "Register `OTG_DOEPMSK` reader"]
pub type R = crate::R<OTG_DOEPMSKrs>;
#[doc = "Register `OTG_DOEPMSK` writer"]
pub type W = crate::W<OTG_DOEPMSKrs>;
#[doc = "Field `XFRCM` reader - XFRCM"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - XFRCM"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - EPDM"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - EPDM"]
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRM` reader - AHBERRM"]
pub type AHBERRM_R = crate::BitReader;
#[doc = "Field `AHBERRM` writer - AHBERRM"]
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPM` reader - STUPM"]
pub type STUPM_R = crate::BitReader;
#[doc = "Field `STUPM` writer - STUPM"]
pub type STUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDM` reader - OTEPDM"]
pub type OTEPDM_R = crate::BitReader;
#[doc = "Field `OTEPDM` writer - OTEPDM"]
pub type OTEPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSRXM` reader - STSPHSRXM"]
pub type STSPHSRXM_R = crate::BitReader;
#[doc = "Field `STSPHSRXM` writer - STSPHSRXM"]
pub type STSPHSRXM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUPM` reader - B2BSTUPM"]
pub type B2BSTUPM_R = crate::BitReader;
#[doc = "Field `B2BSTUPM` writer - B2BSTUPM"]
pub type B2BSTUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERRM` reader - OUTPKTERRM"]
pub type OUTPKTERRM_R = crate::BitReader;
#[doc = "Field `OUTPKTERRM` writer - OUTPKTERRM"]
pub type OUTPKTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAM` reader - BNAM"]
pub type BNAM_R = crate::BitReader;
#[doc = "Field `BNAM` writer - BNAM"]
pub type BNAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRM` reader - BERRM"]
pub type BERRM_R = crate::BitReader;
#[doc = "Field `BERRM` writer - BERRM"]
pub type BERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAKMSK"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAKMSK"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMSK` reader - NYETMSK"]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMSK` writer - NYETMSK"]
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STUPM"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OTEPDM"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STSPHSRXM"]
    #[inline(always)]
    pub fn stsphsrxm(&self) -> STSPHSRXM_R {
        STSPHSRXM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - B2BSTUPM"]
    #[inline(always)]
    pub fn b2bstupm(&self) -> B2BSTUPM_R {
        B2BSTUPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUTPKTERRM"]
    #[inline(always)]
    pub fn outpkterrm(&self) -> OUTPKTERRM_R {
        OUTPKTERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - BERRM"]
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAKMSK"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYETMSK"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<OTG_DOEPMSKrs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<OTG_DOEPMSKrs> {
        EPDM_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrm(&mut self) -> AHBERRM_W<OTG_DOEPMSKrs> {
        AHBERRM_W::new(self, 2)
    }
    #[doc = "Bit 3 - STUPM"]
    #[inline(always)]
    #[must_use]
    pub fn stupm(&mut self) -> STUPM_W<OTG_DOEPMSKrs> {
        STUPM_W::new(self, 3)
    }
    #[doc = "Bit 4 - OTEPDM"]
    #[inline(always)]
    #[must_use]
    pub fn otepdm(&mut self) -> OTEPDM_W<OTG_DOEPMSKrs> {
        OTEPDM_W::new(self, 4)
    }
    #[doc = "Bit 5 - STSPHSRXM"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsrxm(&mut self) -> STSPHSRXM_W<OTG_DOEPMSKrs> {
        STSPHSRXM_W::new(self, 5)
    }
    #[doc = "Bit 6 - B2BSTUPM"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstupm(&mut self) -> B2BSTUPM_W<OTG_DOEPMSKrs> {
        B2BSTUPM_W::new(self, 6)
    }
    #[doc = "Bit 8 - OUTPKTERRM"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterrm(&mut self) -> OUTPKTERRM_W<OTG_DOEPMSKrs> {
        OUTPKTERRM_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    #[must_use]
    pub fn bnam(&mut self) -> BNAM_W<OTG_DOEPMSKrs> {
        BNAM_W::new(self, 9)
    }
    #[doc = "Bit 12 - BERRM"]
    #[inline(always)]
    #[must_use]
    pub fn berrm(&mut self) -> BERRM_W<OTG_DOEPMSKrs> {
        BERRM_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAKMSK"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<OTG_DOEPMSKrs> {
        NAKMSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYETMSK"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<OTG_DOEPMSKrs> {
        NYETMSK_W::new(self, 14)
    }
}
#[doc = "This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DOEPMSKrs;
impl crate::RegisterSpec for OTG_DOEPMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_doepmsk::R`](R) reader structure"]
impl crate::Readable for OTG_DOEPMSKrs {}
#[doc = "`write(|w| ..)` method takes [`otg_doepmsk::W`](W) writer structure"]
impl crate::Writable for OTG_DOEPMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DOEPMSK to value 0"]
impl crate::Resettable for OTG_DOEPMSKrs {
    const RESET_VALUE: u32 = 0;
}
