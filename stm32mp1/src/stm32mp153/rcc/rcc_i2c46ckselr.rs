#[doc = "Register `RCC_I2C46CKSELR` reader"]
pub type R = crate::R<RCC_I2C46CKSELRrs>;
#[doc = "Register `RCC_I2C46CKSELR` writer"]
pub type W = crate::W<RCC_I2C46CKSELRrs>;
#[doc = "Field `I2C46SRC` reader - I2C46SRC"]
pub type I2C46SRC_R = crate::FieldReader;
#[doc = "Field `I2C46SRC` writer - I2C46SRC"]
pub type I2C46SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - I2C46SRC"]
    #[inline(always)]
    pub fn i2c46src(&self) -> I2C46SRC_R {
        I2C46SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - I2C46SRC"]
    #[inline(always)]
    #[must_use]
    pub fn i2c46src(&mut self) -> I2C46SRC_W<RCC_I2C46CKSELRrs> {
        I2C46SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_i2c46ckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_i2c46ckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_I2C46CKSELRrs;
impl crate::RegisterSpec for RCC_I2C46CKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_i2c46ckselr::R`](R) reader structure"]
impl crate::Readable for RCC_I2C46CKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_i2c46ckselr::W`](W) writer structure"]
impl crate::Writable for RCC_I2C46CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_I2C46CKSELR to value 0"]
impl crate::Resettable for RCC_I2C46CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
