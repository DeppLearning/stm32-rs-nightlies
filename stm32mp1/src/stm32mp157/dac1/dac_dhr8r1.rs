#[doc = "Register `DAC_DHR8R1` reader"]
pub type R = crate::R<DAC_DHR8R1rs>;
#[doc = "Register `DAC_DHR8R1` writer"]
pub type W = crate::W<DAC_DHR8R1rs>;
#[doc = "Field `DACC1DHR` reader - DACC1DHR"]
pub type DACC1DHR_R = crate::FieldReader;
#[doc = "Field `DACC1DHR` writer - DACC1DHR"]
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DACC1DHR"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DACC1DHR"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DAC_DHR8R1rs> {
        DACC1DHR_W::new(self, 0)
    }
}
#[doc = "DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8r1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8r1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_DHR8R1rs;
impl crate::RegisterSpec for DAC_DHR8R1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr8r1::R`](R) reader structure"]
impl crate::Readable for DAC_DHR8R1rs {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr8r1::W`](W) writer structure"]
impl crate::Writable for DAC_DHR8R1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_DHR8R1 to value 0"]
impl crate::Resettable for DAC_DHR8R1rs {
    const RESET_VALUE: u32 = 0;
}
