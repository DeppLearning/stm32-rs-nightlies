#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dinr: DINR,
    doutr: DOUTR,
    keyr0: KEYR0,
    keyr1: KEYR1,
    keyr2: KEYR2,
    keyr3: KEYR3,
    ivr0: IVR0,
    ivr1: IVR1,
    ivr2: IVR2,
    ivr3: IVR3,
    keyr4: KEYR4,
    keyr5: KEYR5,
    keyr6: KEYR6,
    keyr7: KEYR7,
    susp0r: SUSP0R,
    susp1r: SUSP1R,
    susp2r: SUSP2R,
    susp3r: SUSP3R,
    susp4r: SUSP4R,
    susp5r: SUSP5R,
    susp6r: SUSP6R,
    susp7r: SUSP7R,
    _reserved24: [u8; 0x02a0],
    ier: IER,
    isr: ISR,
    icr: ICR,
}
impl RegisterBlock {
    #[doc = "0x00 - AES control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - AES status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - AES data input register"]
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    #[doc = "0x0c - AES data output register"]
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    #[doc = "0x10 - AES key register 0"]
    #[inline(always)]
    pub const fn keyr0(&self) -> &KEYR0 {
        &self.keyr0
    }
    #[doc = "0x14 - AES key register 1"]
    #[inline(always)]
    pub const fn keyr1(&self) -> &KEYR1 {
        &self.keyr1
    }
    #[doc = "0x18 - AES key register 2"]
    #[inline(always)]
    pub const fn keyr2(&self) -> &KEYR2 {
        &self.keyr2
    }
    #[doc = "0x1c - AES key register 3"]
    #[inline(always)]
    pub const fn keyr3(&self) -> &KEYR3 {
        &self.keyr3
    }
    #[doc = "0x20 - AES initialization vector register 0"]
    #[inline(always)]
    pub const fn ivr0(&self) -> &IVR0 {
        &self.ivr0
    }
    #[doc = "0x24 - AES initialization vector register 1"]
    #[inline(always)]
    pub const fn ivr1(&self) -> &IVR1 {
        &self.ivr1
    }
    #[doc = "0x28 - AES initialization vector register 2"]
    #[inline(always)]
    pub const fn ivr2(&self) -> &IVR2 {
        &self.ivr2
    }
    #[doc = "0x2c - AES initialization vector register 3"]
    #[inline(always)]
    pub const fn ivr3(&self) -> &IVR3 {
        &self.ivr3
    }
    #[doc = "0x30 - AES key register 4"]
    #[inline(always)]
    pub const fn keyr4(&self) -> &KEYR4 {
        &self.keyr4
    }
    #[doc = "0x34 - AES key register 5"]
    #[inline(always)]
    pub const fn keyr5(&self) -> &KEYR5 {
        &self.keyr5
    }
    #[doc = "0x38 - AES key register 6"]
    #[inline(always)]
    pub const fn keyr6(&self) -> &KEYR6 {
        &self.keyr6
    }
    #[doc = "0x3c - AES key register 7"]
    #[inline(always)]
    pub const fn keyr7(&self) -> &KEYR7 {
        &self.keyr7
    }
    #[doc = "0x40 - AES suspend registers"]
    #[inline(always)]
    pub const fn susp0r(&self) -> &SUSP0R {
        &self.susp0r
    }
    #[doc = "0x44 - AES suspend registers"]
    #[inline(always)]
    pub const fn susp1r(&self) -> &SUSP1R {
        &self.susp1r
    }
    #[doc = "0x48 - AES suspend registers"]
    #[inline(always)]
    pub const fn susp2r(&self) -> &SUSP2R {
        &self.susp2r
    }
    #[doc = "0x4c - AES suspend registers"]
    #[inline(always)]
    pub const fn susp3r(&self) -> &SUSP3R {
        &self.susp3r
    }
    #[doc = "0x50 - AES suspend registers"]
    #[inline(always)]
    pub const fn susp4r(&self) -> &SUSP4R {
        &self.susp4r
    }
    #[doc = "0x54 - AES suspend registers"]
    #[inline(always)]
    pub const fn susp5r(&self) -> &SUSP5R {
        &self.susp5r
    }
    #[doc = "0x58 - AES suspend registers"]
    #[inline(always)]
    pub const fn susp6r(&self) -> &SUSP6R {
        &self.susp6r
    }
    #[doc = "0x5c - AES suspend registers"]
    #[inline(always)]
    pub const fn susp7r(&self) -> &SUSP7R {
        &self.susp7r
    }
    #[doc = "0x300 - AES interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x304 - AES interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x308 - AES interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
}
#[doc = "CR (rw) register accessor: AES control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "AES control register"]
pub mod cr;
#[doc = "SR (r) register accessor: AES status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "AES status register"]
pub mod sr;
#[doc = "DINR (w) register accessor: AES data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr`]
module"]
pub type DINR = crate::Reg<dinr::DINRrs>;
#[doc = "AES data input register"]
pub mod dinr;
#[doc = "DOUTR (r) register accessor: AES data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr`]
module"]
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
#[doc = "AES data output register"]
pub mod doutr;
#[doc = "KEYR0 (w) register accessor: AES key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr0`]
module"]
pub type KEYR0 = crate::Reg<keyr0::KEYR0rs>;
#[doc = "AES key register 0"]
pub mod keyr0;
#[doc = "KEYR1 (w) register accessor: AES key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr1`]
module"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1rs>;
#[doc = "AES key register 1"]
pub mod keyr1;
#[doc = "KEYR2 (w) register accessor: AES key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr2`]
module"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2rs>;
#[doc = "AES key register 2"]
pub mod keyr2;
#[doc = "KEYR3 (w) register accessor: AES key register 3\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr3`]
module"]
pub type KEYR3 = crate::Reg<keyr3::KEYR3rs>;
#[doc = "AES key register 3"]
pub mod keyr3;
#[doc = "IVR0 (rw) register accessor: AES initialization vector register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr0`]
module"]
pub type IVR0 = crate::Reg<ivr0::IVR0rs>;
#[doc = "AES initialization vector register 0"]
pub mod ivr0;
#[doc = "IVR1 (rw) register accessor: AES initialization vector register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr1`]
module"]
pub type IVR1 = crate::Reg<ivr1::IVR1rs>;
#[doc = "AES initialization vector register 1"]
pub mod ivr1;
#[doc = "IVR2 (rw) register accessor: AES initialization vector register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr2`]
module"]
pub type IVR2 = crate::Reg<ivr2::IVR2rs>;
#[doc = "AES initialization vector register 2"]
pub mod ivr2;
#[doc = "IVR3 (rw) register accessor: AES initialization vector register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr3`]
module"]
pub type IVR3 = crate::Reg<ivr3::IVR3rs>;
#[doc = "AES initialization vector register 3"]
pub mod ivr3;
#[doc = "KEYR4 (w) register accessor: AES key register 4\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr4`]
module"]
pub type KEYR4 = crate::Reg<keyr4::KEYR4rs>;
#[doc = "AES key register 4"]
pub mod keyr4;
#[doc = "KEYR5 (w) register accessor: AES key register 5\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr5`]
module"]
pub type KEYR5 = crate::Reg<keyr5::KEYR5rs>;
#[doc = "AES key register 5"]
pub mod keyr5;
#[doc = "KEYR6 (w) register accessor: AES key register 6\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr6`]
module"]
pub type KEYR6 = crate::Reg<keyr6::KEYR6rs>;
#[doc = "AES key register 6"]
pub mod keyr6;
#[doc = "KEYR7 (w) register accessor: AES key register 7\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr7`]
module"]
pub type KEYR7 = crate::Reg<keyr7::KEYR7rs>;
#[doc = "AES key register 7"]
pub mod keyr7;
#[doc = "SUSP0R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp0r`]
module"]
pub type SUSP0R = crate::Reg<susp0r::SUSP0Rrs>;
#[doc = "AES suspend registers"]
pub mod susp0r;
#[doc = "SUSP1R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp1r`]
module"]
pub type SUSP1R = crate::Reg<susp1r::SUSP1Rrs>;
#[doc = "AES suspend registers"]
pub mod susp1r;
#[doc = "SUSP2R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp2r`]
module"]
pub type SUSP2R = crate::Reg<susp2r::SUSP2Rrs>;
#[doc = "AES suspend registers"]
pub mod susp2r;
#[doc = "SUSP3R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp3r`]
module"]
pub type SUSP3R = crate::Reg<susp3r::SUSP3Rrs>;
#[doc = "AES suspend registers"]
pub mod susp3r;
#[doc = "SUSP4R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp4r`]
module"]
pub type SUSP4R = crate::Reg<susp4r::SUSP4Rrs>;
#[doc = "AES suspend registers"]
pub mod susp4r;
#[doc = "SUSP5R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp5r`]
module"]
pub type SUSP5R = crate::Reg<susp5r::SUSP5Rrs>;
#[doc = "AES suspend registers"]
pub mod susp5r;
#[doc = "SUSP6R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp6r`]
module"]
pub type SUSP6R = crate::Reg<susp6r::SUSP6Rrs>;
#[doc = "AES suspend registers"]
pub mod susp6r;
#[doc = "SUSP7R (rw) register accessor: AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp7r`]
module"]
pub type SUSP7R = crate::Reg<susp7r::SUSP7Rrs>;
#[doc = "AES suspend registers"]
pub mod susp7r;
#[doc = "IER (rw) register accessor: AES interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "AES interrupt enable register"]
pub mod ier;
#[doc = "ISR (r) register accessor: AES interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "AES interrupt status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: AES interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "AES interrupt clear register"]
pub mod icr;
