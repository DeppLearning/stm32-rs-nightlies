#[doc = "Register `RESP4R` reader"]
pub type R = crate::R<RESP4Rrs>;
#[doc = "Field `CARDSTATUS4` reader - see Table 347"]
pub type CARDSTATUS4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 347"]
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP4Rrs;
impl crate::RegisterSpec for RESP4Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp4r::R`](R) reader structure"]
impl crate::Readable for RESP4Rrs {}
#[doc = "`reset()` method sets RESP4R to value 0"]
impl crate::Resettable for RESP4Rrs {
    const RESET_VALUE: u32 = 0;
}
