#[doc = "Register `GICD_ICENABLER8` reader"]
pub type R = crate::R<GICD_ICENABLER8rs>;
#[doc = "Register `GICD_ICENABLER8` writer"]
pub type W = crate::W<GICD_ICENABLER8rs>;
#[doc = "Field `ICENABLER8` reader - ICENABLER8"]
pub type ICENABLER8_R = crate::FieldReader<u32>;
#[doc = "Field `ICENABLER8` writer - ICENABLER8"]
pub type ICENABLER8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICENABLER8"]
    #[inline(always)]
    pub fn icenabler8(&self) -> ICENABLER8_R {
        ICENABLER8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICENABLER8"]
    #[inline(always)]
    #[must_use]
    pub fn icenabler8(&mut self) -> ICENABLER8_W<GICD_ICENABLER8rs> {
        ICENABLER8_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICENABLER8rs;
impl crate::RegisterSpec for GICD_ICENABLER8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icenabler8::R`](R) reader structure"]
impl crate::Readable for GICD_ICENABLER8rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icenabler8::W`](W) writer structure"]
impl crate::Writable for GICD_ICENABLER8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICENABLER8 to value 0"]
impl crate::Resettable for GICD_ICENABLER8rs {
    const RESET_VALUE: u32 = 0;
}
