#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ccr: [CCR; 16],
    _reserved1: [u8; 0x40],
    csr: CSR,
    cfr: CFR,
    _reserved3: [u8; 0x78],
    rgcr: [RGCR; 8],
    _reserved4: [u8; 0x20],
    rgsr: RGSR,
    rgcfr: RGCFR,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    #[doc = "0x00 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c0cr(&self) -> &CCR {
        self.ccr(0)
    }
    #[doc = "0x04 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c1cr(&self) -> &CCR {
        self.ccr(1)
    }
    #[doc = "0x08 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c2cr(&self) -> &CCR {
        self.ccr(2)
    }
    #[doc = "0x0c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c3cr(&self) -> &CCR {
        self.ccr(3)
    }
    #[doc = "0x10 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c4cr(&self) -> &CCR {
        self.ccr(4)
    }
    #[doc = "0x14 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c5cr(&self) -> &CCR {
        self.ccr(5)
    }
    #[doc = "0x18 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c6cr(&self) -> &CCR {
        self.ccr(6)
    }
    #[doc = "0x1c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c7cr(&self) -> &CCR {
        self.ccr(7)
    }
    #[doc = "0x20 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c8cr(&self) -> &CCR {
        self.ccr(8)
    }
    #[doc = "0x24 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c9cr(&self) -> &CCR {
        self.ccr(9)
    }
    #[doc = "0x28 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c10cr(&self) -> &CCR {
        self.ccr(10)
    }
    #[doc = "0x2c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c11cr(&self) -> &CCR {
        self.ccr(11)
    }
    #[doc = "0x30 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c12cr(&self) -> &CCR {
        self.ccr(12)
    }
    #[doc = "0x34 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c13cr(&self) -> &CCR {
        self.ccr(13)
    }
    #[doc = "0x38 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c14cr(&self) -> &CCR {
        self.ccr(14)
    }
    #[doc = "0x3c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c15cr(&self) -> &CCR {
        self.ccr(15)
    }
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    #[doc = "0x100..0x120 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rgcr(&self, n: usize) -> &RGCR {
        &self.rgcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub fn rgcr_iter(&self) -> impl Iterator<Item = &RGCR> {
        self.rgcr.iter()
    }
    #[doc = "0x100 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg0cr(&self) -> &RGCR {
        self.rgcr(0)
    }
    #[doc = "0x104 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg1cr(&self) -> &RGCR {
        self.rgcr(1)
    }
    #[doc = "0x108 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg2cr(&self) -> &RGCR {
        self.rgcr(2)
    }
    #[doc = "0x10c - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg3cr(&self) -> &RGCR {
        self.rgcr(3)
    }
    #[doc = "0x110 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg4cr(&self) -> &RGCR {
        self.rgcr(4)
    }
    #[doc = "0x114 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg5cr(&self) -> &RGCR {
        self.rgcr(5)
    }
    #[doc = "0x118 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg6cr(&self) -> &RGCR {
        self.rgcr(6)
    }
    #[doc = "0x11c - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg7cr(&self) -> &RGCR {
        self.rgcr(7)
    }
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    #[inline(always)]
    pub const fn rgcfr(&self) -> &RGCFR {
        &self.rgcfr
    }
}
#[doc = "CCR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod ccr;
#[doc = "RGCR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcr`]
module"]
pub type RGCR = crate::Reg<rgcr::RGCRrs>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rgcr;
#[doc = "RGSR (r) register accessor: DMAMux - DMA request generator status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgsr`]
module"]
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "RGCFR (w) register accessor: DMAMux - DMA request generator clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcfr`]
module"]
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;
#[doc = "CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`]
module"]
pub type CFR = crate::Reg<cfr::CFRrs>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
