#[doc = "Register `APB2_FZ` reader"]
pub type R = crate::R<APB2_FZrs>;
#[doc = "Register `APB2_FZ` writer"]
pub type W = crate::W<APB2_FZrs>;
#[doc = "Field `DBG_TIM9_STOP` reader - TIM counter stopped when core is halted"]
pub type DBG_TIM9_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM9_STOP` writer - TIM counter stopped when core is halted"]
pub type DBG_TIM9_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM10_STOP` reader - TIM counter stopped when core is halted"]
pub type DBG_TIM10_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM10_STOP` writer - TIM counter stopped when core is halted"]
pub type DBG_TIM10_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM11_STOP` reader - TIM counter stopped when core is halted"]
pub type DBG_TIM11_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM11_STOP` writer - TIM counter stopped when core is halted"]
pub type DBG_TIM11_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TIM counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W<APB2_FZrs> {
        DBG_TIM9_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W<APB2_FZrs> {
        DBG_TIM10_STOP_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W<APB2_FZrs> {
        DBG_TIM11_STOP_W::new(self, 4)
    }
}
#[doc = "Debug MCU APB1 freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2_fz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2_fz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2_FZrs;
impl crate::RegisterSpec for APB2_FZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2_fz::R`](R) reader structure"]
impl crate::Readable for APB2_FZrs {}
#[doc = "`write(|w| ..)` method takes [`apb2_fz::W`](W) writer structure"]
impl crate::Writable for APB2_FZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2_FZ to value 0"]
impl crate::Resettable for APB2_FZrs {
    const RESET_VALUE: u32 = 0;
}
