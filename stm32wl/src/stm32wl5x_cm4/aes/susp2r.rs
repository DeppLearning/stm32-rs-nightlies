#[doc = "Register `SUSP2R` reader"]
pub type R = crate::R<SUSP2Rrs>;
#[doc = "Register `SUSP2R` writer"]
pub type W = crate::W<SUSP2Rrs>;
#[doc = "Field `SUSP` reader - AES suspend register 2"]
pub type SUSP_R = crate::FieldReader<u32>;
#[doc = "Field `SUSP` writer - AES suspend register 2"]
pub type SUSP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 2"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 2"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<SUSP2Rrs> {
        SUSP_W::new(self, 0)
    }
}
#[doc = "AES suspend register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP2Rrs;
impl crate::RegisterSpec for SUSP2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp2r::R`](R) reader structure"]
impl crate::Readable for SUSP2Rrs {}
#[doc = "`write(|w| ..)` method takes [`susp2r::W`](W) writer structure"]
impl crate::Writable for SUSP2Rrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP2R to value 0"]
impl crate::Resettable for SUSP2Rrs {
    const RESET_VALUE: u32 = 0;
}
