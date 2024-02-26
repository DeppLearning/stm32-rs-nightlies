#[doc = "Register `HRA1` reader"]
pub type R = crate::R<HRA1rs>;
#[doc = "Field `H1` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hra1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA1rs;
impl crate::RegisterSpec for HRA1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra1::R`](R) reader structure"]
impl crate::Readable for HRA1rs {}
#[doc = "`reset()` method sets HRA1 to value 0"]
impl crate::Resettable for HRA1rs {
    const RESET_VALUE: u32 = 0;
}
