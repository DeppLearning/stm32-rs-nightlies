#[doc = "Register `MACL3L4C0R` reader"]
pub type R = crate::R<MACL3L4C0Rrs>;
#[doc = "Register `MACL3L4C0R` writer"]
pub type W = crate::W<MACL3L4C0Rrs>;
#[doc = "Field `L3PEN0` reader - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM0 or L3DAM0 bit is set."]
pub type L3PEN0_R = crate::BitReader;
#[doc = "Field `L3PEN0` writer - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM0 or L3DAM0 bit is set."]
pub type L3PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAM0` reader - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3DAM0 bit because either IPv6 SA or DA can be checked for filtering."]
pub type L3SAM0_R = crate::BitReader;
#[doc = "Field `L3SAM0` writer - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3DAM0 bit because either IPv6 SA or DA can be checked for filtering."]
pub type L3SAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAIM0` reader - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM0 bit is set."]
pub type L3SAIM0_R = crate::BitReader;
#[doc = "Field `L3SAIM0` writer - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM0 bit is set."]
pub type L3SAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAM0` reader - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3SAM0 bit because either IPv6 DA or SA can be checked for filtering."]
pub type L3DAM0_R = crate::BitReader;
#[doc = "Field `L3DAM0` writer - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3SAM0 bit because either IPv6 DA or SA can be checked for filtering."]
pub type L3DAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAIM0` reader - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM0 bit is set high."]
pub type L3DAIM0_R = crate::BitReader;
#[doc = "Field `L3DAIM0` writer - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM0 bit is set high."]
pub type L3DAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3HSBM0` reader - Layer 3 IP SA higher bits match This field contains the number of lower bits of IP source address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Condition: IPv6 packets: This field contains Bits\\[4:0\\]
of L3HSBM0. These bits indicate the number of higher bits of IP source or destination address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set high."]
pub type L3HSBM0_R = crate::FieldReader;
#[doc = "Field `L3HSBM0` writer - Layer 3 IP SA higher bits match This field contains the number of lower bits of IP source address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Condition: IPv6 packets: This field contains Bits\\[4:0\\]
of L3HSBM0. These bits indicate the number of higher bits of IP source or destination address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set high."]
pub type L3HSBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L3HDBM0` reader - Layer 3 IP DA higher bits match This field contains the number of higher bits of IP Destination Address that are masked in the IPv4 packets: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM0 which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. Number of bits masked is given by concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: .. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set."]
pub type L3HDBM0_R = crate::FieldReader;
#[doc = "Field `L3HDBM0` writer - Layer 3 IP DA higher bits match This field contains the number of higher bits of IP Destination Address that are masked in the IPv4 packets: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM0 which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. Number of bits masked is given by concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: .. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set."]
pub type L3HDBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L4PEN0` reader - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM0 or L4DPM0 bit is set."]
pub type L4PEN0_R = crate::BitReader;
#[doc = "Field `L4PEN0` writer - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM0 or L4DPM0 bit is set."]
pub type L4PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPM0` reader - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
pub type L4SPM0_R = crate::BitReader;
#[doc = "Field `L4SPM0` writer - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
pub type L4SPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPIM0` reader - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM0 bit is set high."]
pub type L4SPIM0_R = crate::BitReader;
#[doc = "Field `L4SPIM0` writer - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM0 bit is set high."]
pub type L4SPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPM0` reader - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
pub type L4DPM0_R = crate::BitReader;
#[doc = "Field `L4DPM0` writer - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
pub type L4DPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPIM0` reader - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM0 bit is set high."]
pub type L4DPIM0_R = crate::BitReader;
#[doc = "Field `L4DPIM0` writer - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM0 bit is set high."]
pub type L4DPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM0 or L3DAM0 bit is set."]
    #[inline(always)]
    pub fn l3pen0(&self) -> L3PEN0_R {
        L3PEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3DAM0 bit because either IPv6 SA or DA can be checked for filtering."]
    #[inline(always)]
    pub fn l3sam0(&self) -> L3SAM0_R {
        L3SAM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM0 bit is set."]
    #[inline(always)]
    pub fn l3saim0(&self) -> L3SAIM0_R {
        L3SAIM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3SAM0 bit because either IPv6 DA or SA can be checked for filtering."]
    #[inline(always)]
    pub fn l3dam0(&self) -> L3DAM0_R {
        L3DAM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM0 bit is set high."]
    #[inline(always)]
    pub fn l3daim0(&self) -> L3DAIM0_R {
        L3DAIM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA higher bits match This field contains the number of lower bits of IP source address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Condition: IPv6 packets: This field contains Bits\\[4:0\\]
of L3HSBM0. These bits indicate the number of higher bits of IP source or destination address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set high."]
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3HSBM0_R {
        L3HSBM0_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA higher bits match This field contains the number of higher bits of IP Destination Address that are masked in the IPv4 packets: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM0 which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. Number of bits masked is given by concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: .. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set."]
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3HDBM0_R {
        L3HDBM0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM0 or L4DPM0 bit is set."]
    #[inline(always)]
    pub fn l4pen0(&self) -> L4PEN0_R {
        L4PEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
    #[inline(always)]
    pub fn l4spm0(&self) -> L4SPM0_R {
        L4SPM0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM0 bit is set high."]
    #[inline(always)]
    pub fn l4spim0(&self) -> L4SPIM0_R {
        L4SPIM0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4DPM0_R {
        L4DPM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM0 bit is set high."]
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4DPIM0_R {
        L4DPIM0_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM0 or L3DAM0 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l3pen0(&mut self) -> L3PEN0_W<MACL3L4C0Rrs> {
        L3PEN0_W::new(self, 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3DAM0 bit because either IPv6 SA or DA can be checked for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn l3sam0(&mut self) -> L3SAM0_W<MACL3L4C0Rrs> {
        L3SAM0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM0 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l3saim0(&mut self) -> L3SAIM0_W<MACL3L4C0Rrs> {
        L3SAIM0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN0 bit is set, you should set either this bit or the L3SAM0 bit because either IPv6 DA or SA can be checked for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn l3dam0(&mut self) -> L3DAM0_W<MACL3L4C0Rrs> {
        L3DAM0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM0 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3daim0(&mut self) -> L3DAIM0_W<MACL3L4C0Rrs> {
        L3DAIM0_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA higher bits match This field contains the number of lower bits of IP source address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Condition: IPv6 packets: This field contains Bits\\[4:0\\]
of L3HSBM0. These bits indicate the number of higher bits of IP source or destination address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3hsbm0(&mut self) -> L3HSBM0_W<MACL3L4C0Rrs> {
        L3HSBM0_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA higher bits match This field contains the number of higher bits of IP Destination Address that are masked in the IPv4 packets: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM0 which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. Number of bits masked is given by concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: .. This field is valid and applicable only when the L3DAM0 or L3SAM0 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l3hdbm0(&mut self) -> L3HDBM0_W<MACL3L4C0Rrs> {
        L3HDBM0_W::new(self, 11)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM0 or L4DPM0 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l4pen0(&mut self) -> L4PEN0_W<MACL3L4C0Rrs> {
        L4PEN0_W::new(self, 16)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
    #[inline(always)]
    #[must_use]
    pub fn l4spm0(&mut self) -> L4SPM0_W<MACL3L4C0Rrs> {
        L4SPM0_W::new(self, 18)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM0 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l4spim0(&mut self) -> L4SPIM0_W<MACL3L4C0Rrs> {
        L4SPIM0_W::new(self, 19)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
    #[inline(always)]
    #[must_use]
    pub fn l4dpm0(&mut self) -> L4DPM0_W<MACL3L4C0Rrs> {
        L4DPM0_W::new(self, 20)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM0 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l4dpim0(&mut self) -> L4DPIM0_W<MACL3L4C0Rrs> {
        L4DPIM0_W::new(self, 21)
    }
}
#[doc = "L3 and L4 control 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3l4c0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3l4c0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3L4C0Rrs;
impl crate::RegisterSpec for MACL3L4C0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3l4c0r::R`](R) reader structure"]
impl crate::Readable for MACL3L4C0Rrs {}
#[doc = "`write(|w| ..)` method takes [`macl3l4c0r::W`](W) writer structure"]
impl crate::Writable for MACL3L4C0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACL3L4C0R to value 0"]
impl crate::Resettable for MACL3L4C0Rrs {
    const RESET_VALUE: u32 = 0;
}
