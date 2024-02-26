#[doc = "Register `HRA0` reader"]
pub type R = crate::R<HRA0rs>;
#[doc = "Field `H0` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn h0(&self) -> H0_R {
        H0_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hra0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA0rs;
impl crate::RegisterSpec for HRA0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra0::R`](R) reader structure"]
impl crate::Readable for HRA0rs {}
#[doc = "`reset()` method sets HRA0 to value 0"]
impl crate::Resettable for HRA0rs {
    const RESET_VALUE: u32 = 0;
}
