#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `ENVR` reader - Enable Voltage Reference"]
pub type ENVR_R = crate::BitReader;
#[doc = "Field `ENVR` writer - Enable Voltage Reference"]
pub type ENVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIZ` reader - High impedence mode for the VREF_BUF"]
pub type HIZ_R = crate::BitReader;
#[doc = "Field `HIZ` writer - High impedence mode for the VREF_BUF"]
pub type HIZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub type VRR_R = crate::BitReader;
#[doc = "Field `VRS` reader - Voltage reference scale"]
pub type VRS_R = crate::FieldReader;
#[doc = "Field `VRS` writer - Voltage reference scale"]
pub type VRS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable Voltage Reference"]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High impedence mode for the VREF_BUF"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Voltage Reference"]
    #[inline(always)]
    #[must_use]
    pub fn envr(&mut self) -> ENVR_W<CSRrs> {
        ENVR_W::new(self, 0)
    }
    #[doc = "Bit 1 - High impedence mode for the VREF_BUF"]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<CSRrs> {
        HIZ_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - Voltage reference scale"]
    #[inline(always)]
    #[must_use]
    pub fn vrs(&mut self) -> VRS_W<CSRrs> {
        VRS_W::new(self, 4)
    }
}
#[doc = "VREF_BUF Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x02"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x02;
}
