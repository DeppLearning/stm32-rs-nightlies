#[doc = "Register `IVR0` reader"]
pub type R = crate::R<IVR0rs>;
#[doc = "Register `IVR0` writer"]
pub type W = crate::W<IVR0rs>;
#[doc = "Field `IVR0` reader - Initialization Vector Register"]
pub type IVR0_R = crate::FieldReader<u32>;
#[doc = "Field `IVR0` writer - Initialization Vector Register"]
pub type IVR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register"]
    #[inline(always)]
    pub fn ivr0(&self) -> IVR0_R {
        IVR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register"]
    #[inline(always)]
    #[must_use]
    pub fn ivr0(&mut self) -> IVR0_W<IVR0rs> {
        IVR0_W::new(self, 0)
    }
}
#[doc = "Initialization Vector Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVR0rs;
impl crate::RegisterSpec for IVR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr0::R`](R) reader structure"]
impl crate::Readable for IVR0rs {}
#[doc = "`write(|w| ..)` method takes [`ivr0::W`](W) writer structure"]
impl crate::Writable for IVR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVR0 to value 0"]
impl crate::Resettable for IVR0rs {
    const RESET_VALUE: u32 = 0;
}
