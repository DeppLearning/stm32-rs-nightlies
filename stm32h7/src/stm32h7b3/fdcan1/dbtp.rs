#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DBTPrs>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DBTPrs>;
#[doc = "Field `DSJW` reader - Synchronization Jump Width"]
pub type DSJW_R = crate::FieldReader;
#[doc = "Field `DSJW` writer - Synchronization Jump Width"]
pub type DSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point"]
pub type DTSEG2_R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data time segment after sample point"]
pub type DTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - Data time segment after sample point"]
pub type DTSEG1_R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data time segment after sample point"]
pub type DTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - Data BIt Rate Prescaler"]
pub type DBRP_R = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data BIt Rate Prescaler"]
pub type DBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDC` reader - Transceiver Delay Compensation"]
pub type TDC_R = crate::BitReader;
#[doc = "Field `TDC` writer - Transceiver Delay Compensation"]
pub type TDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data BIt Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DSJW_W<DBTPrs> {
        DSJW_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> DTSEG2_W<DBTPrs> {
        DTSEG2_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> DTSEG1_W<DBTPrs> {
        DTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data BIt Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DBRP_W<DBTPrs> {
        DBRP_W::new(self, 16)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TDC_W<DBTPrs> {
        TDC_W::new(self, 23)
    }
}
#[doc = "FDCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBTPrs;
impl crate::RegisterSpec for DBTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DBTPrs {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DBTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBTP to value 0x0a33"]
impl crate::Resettable for DBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
