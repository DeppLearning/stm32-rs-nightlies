#[doc = "Register `GICD_ICPENDR0` reader"]
pub type R = crate::R<GICD_ICPENDR0rs>;
#[doc = "Register `GICD_ICPENDR0` writer"]
pub type W = crate::W<GICD_ICPENDR0rs>;
#[doc = "Field `ICPENDR0` reader - ICPENDR0"]
pub type ICPENDR0_R = crate::FieldReader<u32>;
#[doc = "Field `ICPENDR0` writer - ICPENDR0"]
pub type ICPENDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICPENDR0"]
    #[inline(always)]
    pub fn icpendr0(&self) -> ICPENDR0_R {
        ICPENDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR0"]
    #[inline(always)]
    #[must_use]
    pub fn icpendr0(&mut self) -> ICPENDR0_W<GICD_ICPENDR0rs> {
        ICPENDR0_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICPENDR0rs;
impl crate::RegisterSpec for GICD_ICPENDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icpendr0::R`](R) reader structure"]
impl crate::Readable for GICD_ICPENDR0rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icpendr0::W`](W) writer structure"]
impl crate::Writable for GICD_ICPENDR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICPENDR0 to value 0"]
impl crate::Resettable for GICD_ICPENDR0rs {
    const RESET_VALUE: u32 = 0;
}
