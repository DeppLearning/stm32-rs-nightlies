#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Rx-Packet available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXP {
    #[doc = "0: Rx buffer empty"]
    Empty = 0,
    #[doc = "1: Rx buffer not empty"]
    NotEmpty = 1,
}
impl From<RXP> for bool {
    #[inline(always)]
    fn from(variant: RXP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXP` reader - Rx-Packet available"]
pub type RXP_R = crate::BitReader<RXP>;
impl RXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXP {
        match self.bits {
            false => RXP::Empty,
            true => RXP::NotEmpty,
        }
    }
    #[doc = "Rx buffer empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXP::Empty
    }
    #[doc = "Rx buffer not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXP::NotEmpty
    }
}
#[doc = "Tx-Packet space available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP {
    #[doc = "0: Tx buffer full"]
    Full = 0,
    #[doc = "1: Tx buffer not full"]
    NotFull = 1,
}
impl From<TXP> for bool {
    #[inline(always)]
    fn from(variant: TXP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXP` reader - Tx-Packet space available"]
pub type TXP_R = crate::BitReader<TXP>;
impl TXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXP {
        match self.bits {
            false => TXP::Full,
            true => TXP::NotFull,
        }
    }
    #[doc = "Tx buffer full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXP::Full
    }
    #[doc = "Tx buffer not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXP::NotFull
    }
}
#[doc = "Duplex Packet\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXP {
    #[doc = "0: Duplex packet unavailable: no space for transmission and/or no data received"]
    Unavailable = 0,
    #[doc = "1: Duplex packet available: space for transmission and data received"]
    Available = 1,
}
impl From<DXP> for bool {
    #[inline(always)]
    fn from(variant: DXP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DXP` reader - Duplex Packet"]
pub type DXP_R = crate::BitReader<DXP>;
impl DXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DXP {
        match self.bits {
            false => DXP::Unavailable,
            true => DXP::Available,
        }
    }
    #[doc = "Duplex packet unavailable: no space for transmission and/or no data received"]
    #[inline(always)]
    pub fn is_unavailable(&self) -> bool {
        *self == DXP::Unavailable
    }
    #[doc = "Duplex packet available: space for transmission and data received"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == DXP::Available
    }
}
#[doc = "End Of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT {
    #[doc = "0: Transfer ongoing or not started"]
    NotCompleted = 0,
    #[doc = "1: Transfer complete"]
    Completed = 1,
}
impl From<EOT> for bool {
    #[inline(always)]
    fn from(variant: EOT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` reader - End Of Transfer"]
pub type EOT_R = crate::BitReader<EOT>;
impl EOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOT {
        match self.bits {
            false => EOT::NotCompleted,
            true => EOT::Completed,
        }
    }
    #[doc = "Transfer ongoing or not started"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == EOT::NotCompleted
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == EOT::Completed
    }
}
#[doc = "Transmission Transfer Filled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTF {
    #[doc = "0: Transmission buffer incomplete"]
    NotCompleted = 0,
    #[doc = "1: Transmission buffer filled with at least one transfer"]
    Completed = 1,
}
impl From<TXTF> for bool {
    #[inline(always)]
    fn from(variant: TXTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTF` reader - Transmission Transfer Filled"]
pub type TXTF_R = crate::BitReader<TXTF>;
impl TXTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXTF {
        match self.bits {
            false => TXTF::NotCompleted,
            true => TXTF::Completed,
        }
    }
    #[doc = "Transmission buffer incomplete"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TXTF::NotCompleted
    }
    #[doc = "Transmission buffer filled with at least one transfer"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXTF::Completed
    }
}
#[doc = "Underrun at slave transmission mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR {
    #[doc = "0: No underrun occurred"]
    NoUnderrun = 0,
    #[doc = "1: Underrun occurred"]
    Underrun = 1,
}
impl From<UDR> for bool {
    #[inline(always)]
    fn from(variant: UDR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDR` reader - Underrun at slave transmission mode"]
pub type UDR_R = crate::BitReader<UDR>;
impl UDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDR {
        match self.bits {
            false => UDR::NoUnderrun,
            true => UDR::Underrun,
        }
    }
    #[doc = "No underrun occurred"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDR::NoUnderrun
    }
    #[doc = "Underrun occurred"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDR::Underrun
    }
}
#[doc = "Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVR> for bool {
    #[inline(always)]
    fn from(variant: OVR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - Overrun"]
pub type OVR_R = crate::BitReader<OVR>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR {
        match self.bits {
            false => OVR::NoOverrun,
            true => OVR::Overrun,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR::NoOverrun
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR::Overrun
    }
}
#[doc = "CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE {
    #[doc = "0: No CRC error detected"]
    NoError = 0,
    #[doc = "1: CRC error detected"]
    Error = 1,
}
impl From<CRCE> for bool {
    #[inline(always)]
    fn from(variant: CRCE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCE` reader - CRC Error"]
pub type CRCE_R = crate::BitReader<CRCE>;
impl CRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCE {
        match self.bits {
            false => CRCE::NoError,
            true => CRCE::Error,
        }
    }
    #[doc = "No CRC error detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == CRCE::NoError
    }
    #[doc = "CRC error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == CRCE::Error
    }
}
#[doc = "TI frame format error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIFRE {
    #[doc = "0: TI frame format error detected"]
    NoError = 0,
    #[doc = "1: TI frame format error detected"]
    Error = 1,
}
impl From<TIFRE> for bool {
    #[inline(always)]
    fn from(variant: TIFRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIFRE` reader - TI frame format error"]
pub type TIFRE_R = crate::BitReader<TIFRE>;
impl TIFRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIFRE {
        match self.bits {
            false => TIFRE::NoError,
            true => TIFRE::Error,
        }
    }
    #[doc = "TI frame format error detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TIFRE::NoError
    }
    #[doc = "TI frame format error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TIFRE::Error
    }
}
#[doc = "Mode Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF {
    #[doc = "0: No mode fault detected"]
    NoFault = 0,
    #[doc = "1: Mode fault detected"]
    Fault = 1,
}
impl From<MODF> for bool {
    #[inline(always)]
    fn from(variant: MODF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Mode Fault"]
pub type MODF_R = crate::BitReader<MODF>;
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODF {
        match self.bits {
            false => MODF::NoFault,
            true => MODF::Fault,
        }
    }
    #[doc = "No mode fault detected"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODF::NoFault
    }
    #[doc = "Mode fault detected"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODF::Fault
    }
}
#[doc = "Additional number of SPI data to be transacted was reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSERF {
    #[doc = "0: Additional number of SPI data to be transacted not yet loaded"]
    NotLoaded = 0,
    #[doc = "1: Additional number of SPI data to be transacted was reloaded"]
    Loaded = 1,
}
impl From<TSERF> for bool {
    #[inline(always)]
    fn from(variant: TSERF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSERF` reader - Additional number of SPI data to be transacted was reload"]
pub type TSERF_R = crate::BitReader<TSERF>;
impl TSERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSERF {
        match self.bits {
            false => TSERF::NotLoaded,
            true => TSERF::Loaded,
        }
    }
    #[doc = "Additional number of SPI data to be transacted not yet loaded"]
    #[inline(always)]
    pub fn is_not_loaded(&self) -> bool {
        *self == TSERF::NotLoaded
    }
    #[doc = "Additional number of SPI data to be transacted was reloaded"]
    #[inline(always)]
    pub fn is_loaded(&self) -> bool {
        *self == TSERF::Loaded
    }
}
#[doc = "SUSPend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP {
    #[doc = "0: Master not suspended"]
    NotSuspended = 0,
    #[doc = "1: Master suspended"]
    Suspended = 1,
}
impl From<SUSP> for bool {
    #[inline(always)]
    fn from(variant: SUSP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - SUSPend"]
pub type SUSP_R = crate::BitReader<SUSP>;
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSP {
        match self.bits {
            false => SUSP::NotSuspended,
            true => SUSP::Suspended,
        }
    }
    #[doc = "Master not suspended"]
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP::NotSuspended
    }
    #[doc = "Master suspended"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP::Suspended
    }
}
#[doc = "TxFIFO transmission complete\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXC {
    #[doc = "0: Transmission ongoing"]
    Ongoing = 0,
    #[doc = "1: Transmission completed"]
    Completed = 1,
}
impl From<TXC> for bool {
    #[inline(always)]
    fn from(variant: TXC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXC` reader - TxFIFO transmission complete"]
pub type TXC_R = crate::BitReader<TXC>;
impl TXC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXC {
        match self.bits {
            false => TXC::Ongoing,
            true => TXC::Completed,
        }
    }
    #[doc = "Transmission ongoing"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == TXC::Ongoing
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXC::Completed
    }
}
#[doc = "RxFIFO Packing LeVeL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPLVL {
    #[doc = "0: Zero frames beyond packing ratio available"]
    ZeroFrames = 0,
    #[doc = "1: One frame beyond packing ratio available"]
    OneFrame = 1,
    #[doc = "2: Two frame beyond packing ratio available"]
    TwoFrames = 2,
    #[doc = "3: Three frame beyond packing ratio available"]
    ThreeFrames = 3,
}
impl From<RXPLVL> for u8 {
    #[inline(always)]
    fn from(variant: RXPLVL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXPLVL {
    type Ux = u8;
}
#[doc = "Field `RXPLVL` reader - RxFIFO Packing LeVeL"]
pub type RXPLVL_R = crate::FieldReader<RXPLVL>;
impl RXPLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXPLVL {
        match self.bits {
            0 => RXPLVL::ZeroFrames,
            1 => RXPLVL::OneFrame,
            2 => RXPLVL::TwoFrames,
            3 => RXPLVL::ThreeFrames,
            _ => unreachable!(),
        }
    }
    #[doc = "Zero frames beyond packing ratio available"]
    #[inline(always)]
    pub fn is_zero_frames(&self) -> bool {
        *self == RXPLVL::ZeroFrames
    }
    #[doc = "One frame beyond packing ratio available"]
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == RXPLVL::OneFrame
    }
    #[doc = "Two frame beyond packing ratio available"]
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == RXPLVL::TwoFrames
    }
    #[doc = "Three frame beyond packing ratio available"]
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == RXPLVL::ThreeFrames
    }
}
#[doc = "RxFIFO Word Not Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXWNE {
    #[doc = "0: Less than 32-bit data frame received"]
    LessThan32 = 0,
    #[doc = "1: At least 32-bit data frame received"]
    AtLeast32 = 1,
}
impl From<RXWNE> for bool {
    #[inline(always)]
    fn from(variant: RXWNE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXWNE` reader - RxFIFO Word Not Empty"]
pub type RXWNE_R = crate::BitReader<RXWNE>;
impl RXWNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXWNE {
        match self.bits {
            false => RXWNE::LessThan32,
            true => RXWNE::AtLeast32,
        }
    }
    #[doc = "Less than 32-bit data frame received"]
    #[inline(always)]
    pub fn is_less_than32(&self) -> bool {
        *self == RXWNE::LessThan32
    }
    #[doc = "At least 32-bit data frame received"]
    #[inline(always)]
    pub fn is_at_least32(&self) -> bool {
        *self == RXWNE::AtLeast32
    }
}
#[doc = "Field `CTSIZE` reader - Number of data frames remaining in current TSIZE session"]
pub type CTSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Rx-Packet available"]
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx-Packet space available"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Duplex Packet"]
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Of Transfer"]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Transfer Filled"]
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underrun at slave transmission mode"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC Error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mode Fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Additional number of SPI data to be transacted was reload"]
    #[inline(always)]
    pub fn tserf(&self) -> TSERF_R {
        TSERF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSPend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TxFIFO transmission complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - RxFIFO Packing LeVeL"]
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RxFIFO Word Not Empty"]
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Number of data frames remaining in current TSIZE session"]
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0x1002"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x1002;
}
