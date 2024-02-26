#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    x1bufcfg: X1BUFCFG,
    x2bufcfg: X2BUFCFG,
    ybufcfg: YBUFCFG,
    param: PARAM,
    cr: CR,
    sr: SR,
    wdata: WDATA,
    rdata: RDATA,
}
impl RegisterBlock {
    #[doc = "0x00 - FMAC X1 buffer configuration register"]
    #[inline(always)]
    pub const fn x1bufcfg(&self) -> &X1BUFCFG {
        &self.x1bufcfg
    }
    #[doc = "0x04 - FMAC X2 buffer configuration register"]
    #[inline(always)]
    pub const fn x2bufcfg(&self) -> &X2BUFCFG {
        &self.x2bufcfg
    }
    #[doc = "0x08 - FMAC Y buffer configuration register"]
    #[inline(always)]
    pub const fn ybufcfg(&self) -> &YBUFCFG {
        &self.ybufcfg
    }
    #[doc = "0x0c - FMAC parameter register"]
    #[inline(always)]
    pub const fn param(&self) -> &PARAM {
        &self.param
    }
    #[doc = "0x10 - FMAC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x14 - FMAC status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - FMAC write data register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    #[doc = "0x1c - FMAC read data register"]
    #[inline(always)]
    pub const fn rdata(&self) -> &RDATA {
        &self.rdata
    }
}
#[doc = "X1BUFCFG (rw) register accessor: FMAC X1 buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x1bufcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x1bufcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@x1bufcfg`]
module"]
pub type X1BUFCFG = crate::Reg<x1bufcfg::X1BUFCFGrs>;
#[doc = "FMAC X1 buffer configuration register"]
pub mod x1bufcfg;
#[doc = "X2BUFCFG (rw) register accessor: FMAC X2 buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x2bufcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x2bufcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@x2bufcfg`]
module"]
pub type X2BUFCFG = crate::Reg<x2bufcfg::X2BUFCFGrs>;
#[doc = "FMAC X2 buffer configuration register"]
pub mod x2bufcfg;
#[doc = "YBUFCFG (rw) register accessor: FMAC Y buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ybufcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ybufcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ybufcfg`]
module"]
pub type YBUFCFG = crate::Reg<ybufcfg::YBUFCFGrs>;
#[doc = "FMAC Y buffer configuration register"]
pub mod ybufcfg;
#[doc = "PARAM (rw) register accessor: FMAC parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`param::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`param::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`]
module"]
pub type PARAM = crate::Reg<param::PARAMrs>;
#[doc = "FMAC parameter register"]
pub mod param;
#[doc = "CR (rw) register accessor: FMAC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "FMAC control register"]
pub mod cr;
#[doc = "SR (r) register accessor: FMAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "FMAC status register"]
pub mod sr;
#[doc = "WDATA (w) register accessor: FMAC write data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`]
module"]
pub type WDATA = crate::Reg<wdata::WDATArs>;
#[doc = "FMAC write data register"]
pub mod wdata;
#[doc = "RDATA (r) register accessor: FMAC read data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`]
module"]
pub type RDATA = crate::Reg<rdata::RDATArs>;
#[doc = "FMAC read data register"]
pub mod rdata;
