#[doc = "Register `HWCFGR3` reader"]
pub type R = crate::R<HWCFGR3rs>;
#[doc = "Register `HWCFGR3` writer"]
pub type W = crate::W<HWCFGR3rs>;
#[doc = "Field `EVENT_TRG` reader - HW configuration event trigger type"]
pub type EVENT_TRG_R = crate::FieldReader<u32>;
#[doc = "Field `EVENT_TRG` writer - HW configuration event trigger type"]
pub type EVENT_TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    #[must_use]
    pub fn event_trg(&mut self) -> EVENT_TRG_W<HWCFGR3rs> {
        EVENT_TRG_W::new(self, 0)
    }
}
#[doc = "Hardware configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR3rs;
impl crate::RegisterSpec for HWCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr3::R`](R) reader structure"]
impl crate::Readable for HWCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr3::W`](W) writer structure"]
impl crate::Writable for HWCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR3 to value 0"]
impl crate::Resettable for HWCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
