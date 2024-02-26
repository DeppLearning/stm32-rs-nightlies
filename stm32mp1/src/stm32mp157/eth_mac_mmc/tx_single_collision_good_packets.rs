#[doc = "Register `TX_SINGLE_COLLISION_GOOD_PACKETS` reader"]
pub type R = crate::R<TX_SINGLE_COLLISION_GOOD_PACKETSrs>;
#[doc = "Field `TXSNGLCOLG` reader - TXSNGLCOLG"]
pub type TXSNGLCOLG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXSNGLCOLG"]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new(self.bits)
    }
}
#[doc = "This register provides the number of successfully transmitted packets by Ethernet peripheral after a single collision in the half-duplex mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_single_collision_good_packets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_SINGLE_COLLISION_GOOD_PACKETSrs;
impl crate::RegisterSpec for TX_SINGLE_COLLISION_GOOD_PACKETSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_single_collision_good_packets::R`](R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_PACKETSrs {}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_PACKETS to value 0"]
impl crate::Resettable for TX_SINGLE_COLLISION_GOOD_PACKETSrs {
    const RESET_VALUE: u32 = 0;
}
