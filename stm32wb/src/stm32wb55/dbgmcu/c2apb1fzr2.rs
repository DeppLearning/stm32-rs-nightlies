#[doc = "Register `C2APB1FZR2` reader"]
pub type R = crate::R<C2APB1FZR2rs>;
#[doc = "Register `C2APB1FZR2` writer"]
pub type W = crate::W<C2APB1FZR2rs>;
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<C2APB1FZR2rs> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
}
#[doc = "APB1 High Freeze Register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb1fzr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb1fzr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB1FZR2rs;
impl crate::RegisterSpec for C2APB1FZR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb1fzr2::R`](R) reader structure"]
impl crate::Readable for C2APB1FZR2rs {}
#[doc = "`write(|w| ..)` method takes [`c2apb1fzr2::W`](W) writer structure"]
impl crate::Writable for C2APB1FZR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB1FZR2 to value 0"]
impl crate::Resettable for C2APB1FZR2rs {
    const RESET_VALUE: u32 = 0;
}
