#[doc = "Register `DINR6` reader"]
pub type R = crate::R<DINR6rs>;
#[doc = "Field `DIN6` reader - Input data received from MDIO Master during write frames"]
pub type DIN6_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din6(&self) -> DIN6_R {
        DIN6_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DINR6rs;
impl crate::RegisterSpec for DINR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr6::R`](R) reader structure"]
impl crate::Readable for DINR6rs {}
#[doc = "`reset()` method sets DINR6 to value 0"]
impl crate::Resettable for DINR6rs {
    const RESET_VALUE: u32 = 0;
}
