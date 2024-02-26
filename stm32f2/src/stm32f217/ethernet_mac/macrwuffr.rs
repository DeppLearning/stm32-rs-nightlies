#[doc = "Register `MACRWUFFR` reader"]
pub type R = crate::R<MACRWUFFRrs>;
#[doc = "Register `MACRWUFFR` writer"]
pub type W = crate::W<MACRWUFFRrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACRWUFFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Ethernet MAC remote wakeup frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwuffr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwuffr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRWUFFRrs;
impl crate::RegisterSpec for MACRWUFFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwuffr::R`](R) reader structure"]
impl crate::Readable for MACRWUFFRrs {}
#[doc = "`write(|w| ..)` method takes [`macrwuffr::W`](W) writer structure"]
impl crate::Writable for MACRWUFFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACRWUFFR to value 0"]
impl crate::Resettable for MACRWUFFRrs {
    const RESET_VALUE: u32 = 0;
}
