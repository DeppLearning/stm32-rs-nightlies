#[doc = "Register `MDIOS_DINR28` reader"]
pub type R = crate::R<MDIOS_DINR28rs>;
#[doc = "Field `DIN` reader - DIN"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DIN"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr28::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR28rs;
impl crate::RegisterSpec for MDIOS_DINR28rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr28::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR28rs {}
#[doc = "`reset()` method sets MDIOS_DINR28 to value 0"]
impl crate::Resettable for MDIOS_DINR28rs {
    const RESET_VALUE: u32 = 0;
}
