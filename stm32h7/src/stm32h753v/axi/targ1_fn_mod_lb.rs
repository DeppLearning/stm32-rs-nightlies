#[doc = "Register `TARG1_FN_MOD_LB` reader"]
pub type R = crate::R<TARG1_FN_MOD_LBrs>;
#[doc = "Register `TARG1_FN_MOD_LB` writer"]
pub type W = crate::W<TARG1_FN_MOD_LBrs>;
#[doc = "Field `FN_MOD_LB` reader - Controls burst breaking of long bursts"]
pub type FN_MOD_LB_R = crate::BitReader;
#[doc = "Field `FN_MOD_LB` writer - Controls burst breaking of long bursts"]
pub type FN_MOD_LB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    #[must_use]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W<TARG1_FN_MOD_LBrs> {
        FN_MOD_LB_W::new(self, 0)
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ1_fn_mod_lb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ1_fn_mod_lb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARG1_FN_MOD_LBrs;
impl crate::RegisterSpec for TARG1_FN_MOD_LBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`targ1_fn_mod_lb::R`](R) reader structure"]
impl crate::Readable for TARG1_FN_MOD_LBrs {}
#[doc = "`write(|w| ..)` method takes [`targ1_fn_mod_lb::W`](W) writer structure"]
impl crate::Writable for TARG1_FN_MOD_LBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARG1_FN_MOD_LB to value 0x04"]
impl crate::Resettable for TARG1_FN_MOD_LBrs {
    const RESET_VALUE: u32 = 0x04;
}
