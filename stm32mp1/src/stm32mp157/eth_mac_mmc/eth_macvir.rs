#[doc = "Register `ETH_MACVIR` reader"]
pub type R = crate::R<ETH_MACVIRrs>;
#[doc = "Register `ETH_MACVIR` writer"]
pub type W = crate::W<ETH_MACVIRrs>;
#[doc = "Field `VLT` reader - VLT"]
pub type VLT_R = crate::FieldReader<u16>;
#[doc = "Field `VLT` writer - VLT"]
pub type VLT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLC` reader - VLC"]
pub type VLC_R = crate::FieldReader;
#[doc = "Field `VLC` writer - VLC"]
pub type VLC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VLP` reader - VLP"]
pub type VLP_R = crate::BitReader;
#[doc = "Field `VLP` writer - VLP"]
pub type VLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSVL` reader - CSVL"]
pub type CSVL_R = crate::BitReader;
#[doc = "Field `CSVL` writer - CSVL"]
pub type CSVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLTI` reader - VLTI"]
pub type VLTI_R = crate::BitReader;
#[doc = "Field `VLTI` writer - VLTI"]
pub type VLTI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    #[must_use]
    pub fn vlt(&mut self) -> VLT_W<ETH_MACVIRrs> {
        VLT_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<ETH_MACVIRrs> {
        VLC_W::new(self, 16)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    #[must_use]
    pub fn vlp(&mut self) -> VLP_W<ETH_MACVIRrs> {
        VLP_W::new(self, 18)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CSVL_W<ETH_MACVIRrs> {
        CSVL_W::new(self, 19)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VLTI_W<ETH_MACVIRrs> {
        VLTI_W::new(self, 20)
    }
}
#[doc = "The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macvir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macvir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACVIRrs;
impl crate::RegisterSpec for ETH_MACVIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macvir::R`](R) reader structure"]
impl crate::Readable for ETH_MACVIRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macvir::W`](W) writer structure"]
impl crate::Writable for ETH_MACVIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACVIR to value 0"]
impl crate::Resettable for ETH_MACVIRrs {
    const RESET_VALUE: u32 = 0;
}
