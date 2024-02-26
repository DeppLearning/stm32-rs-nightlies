#[doc = "Register `CDR` reader"]
pub type R = crate::R<CDRrs>;
#[doc = "Field `RDATA_MST` reader - Regular data of the master ADC"]
pub type RDATA_MST_R = crate::FieldReader<u16>;
#[doc = "Field `RDATA_SLV` reader - Regular data of the slave ADC"]
pub type RDATA_SLV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data of the master ADC"]
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Regular data of the slave ADC"]
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADC common regular data register for dual and triple modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDRrs;
impl crate::RegisterSpec for CDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CDRrs {}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDRrs {
    const RESET_VALUE: u32 = 0;
}
