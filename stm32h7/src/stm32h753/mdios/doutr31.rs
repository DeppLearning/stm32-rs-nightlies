#[doc = "Register `DOUTR31` reader"]
pub type R = crate::R<DOUTR31rs>;
#[doc = "Register `DOUTR31` writer"]
pub type W = crate::W<DOUTR31rs>;
#[doc = "Field `DOUT31` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT31_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT31` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT31_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout31(&self) -> DOUT31_R {
        DOUT31_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout31(&mut self) -> DOUT31_W<DOUTR31rs> {
        DOUT31_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR31rs;
impl crate::RegisterSpec for DOUTR31rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr31::R`](R) reader structure"]
impl crate::Readable for DOUTR31rs {}
#[doc = "`write(|w| ..)` method takes [`doutr31::W`](W) writer structure"]
impl crate::Writable for DOUTR31rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR31 to value 0"]
impl crate::Resettable for DOUTR31rs {
    const RESET_VALUE: u32 = 0;
}
