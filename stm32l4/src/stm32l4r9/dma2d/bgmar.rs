#[doc = "Register `BGMAR` reader"]
pub type R = crate::R<BGMARrs>;
#[doc = "Register `BGMAR` writer"]
pub type W = crate::W<BGMARrs>;
#[doc = "Field `MA` reader - Memory address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<BGMARrs> {
        MA_W::new(self, 0)
    }
}
#[doc = "background memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGMARrs;
impl crate::RegisterSpec for BGMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgmar::R`](R) reader structure"]
impl crate::Readable for BGMARrs {}
#[doc = "`write(|w| ..)` method takes [`bgmar::W`](W) writer structure"]
impl crate::Writable for BGMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BGMAR to value 0"]
impl crate::Resettable for BGMARrs {
    const RESET_VALUE: u32 = 0;
}
