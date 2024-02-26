#[doc = "Register `GICD_ISPENDR2` reader"]
pub type R = crate::R<GICD_ISPENDR2rs>;
#[doc = "Register `GICD_ISPENDR2` writer"]
pub type W = crate::W<GICD_ISPENDR2rs>;
#[doc = "Field `ISPENDR2` reader - ISPENDR2"]
pub type ISPENDR2_R = crate::FieldReader<u32>;
#[doc = "Field `ISPENDR2` writer - ISPENDR2"]
pub type ISPENDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISPENDR2"]
    #[inline(always)]
    pub fn ispendr2(&self) -> ISPENDR2_R {
        ISPENDR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR2"]
    #[inline(always)]
    #[must_use]
    pub fn ispendr2(&mut self) -> ISPENDR2_W<GICD_ISPENDR2rs> {
        ISPENDR2_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISPENDR2rs;
impl crate::RegisterSpec for GICD_ISPENDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ispendr2::R`](R) reader structure"]
impl crate::Readable for GICD_ISPENDR2rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_ispendr2::W`](W) writer structure"]
impl crate::Writable for GICD_ISPENDR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISPENDR2 to value 0"]
impl crate::Resettable for GICD_ISPENDR2rs {
    const RESET_VALUE: u32 = 0;
}
