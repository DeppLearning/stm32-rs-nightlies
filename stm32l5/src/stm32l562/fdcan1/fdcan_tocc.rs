#[doc = "Register `FDCAN_TOCC` reader"]
pub type R = crate::R<FDCAN_TOCCrs>;
#[doc = "Register `FDCAN_TOCC` writer"]
pub type W = crate::W<FDCAN_TOCCrs>;
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub type ETOC_R = crate::BitReader;
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type ETOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TOS_R = crate::FieldReader;
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<FDCAN_TOCCrs> {
        ETOC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<FDCAN_TOCCrs> {
        TOS_W::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<FDCAN_TOCCrs> {
        TOP_W::new(self, 16)
    }
}
#[doc = "FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TOCCrs;
impl crate::RegisterSpec for FDCAN_TOCCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tocc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TOCCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tocc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TOCCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TOCC to value 0xffff_0000"]
impl crate::Resettable for FDCAN_TOCCrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
