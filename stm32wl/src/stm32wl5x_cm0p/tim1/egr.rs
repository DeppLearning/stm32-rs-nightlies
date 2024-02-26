#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGRrs>;
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    Update = 1,
}
impl From<UG> for bool {
    #[inline(always)]
    fn from(variant: UG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG, UG>;
impl<'a, REG> UG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(UG::Update)
    }
}
#[doc = "Capture/compare %s generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1GW {
    #[doc = "1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register"]
    Trigger = 1,
}
impl From<CC1GW> for bool {
    #[inline(always)]
    fn from(variant: CC1GW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCG(1-4)` writer - Capture/compare %s generation"]
pub type CCG_W<'a, REG> = crate::BitWriter<'a, REG, CC1GW>;
impl<'a, REG> CCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(CC1GW::Trigger)
    }
}
#[doc = "Capture/Compare control update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMGW {
    #[doc = "1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated"]
    Trigger = 1,
}
impl From<COMGW> for bool {
    #[inline(always)]
    fn from(variant: COMGW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG, COMGW>;
impl<'a, REG> COMG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(COMGW::Trigger)
    }
}
#[doc = "Trigger generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGW {
    #[doc = "1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled"]
    Trigger = 1,
}
impl From<TGW> for bool {
    #[inline(always)]
    fn from(variant: TGW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG, TGW>;
impl<'a, REG> TG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TGW::Trigger)
    }
}
#[doc = "Break generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGW {
    #[doc = "1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled"]
    Trigger = 1,
}
impl From<BGW> for bool {
    #[inline(always)]
    fn from(variant: BGW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BG` writer - Break generation"]
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG, BGW>;
impl<'a, REG> BG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(BGW::Trigger)
    }
}
#[doc = "Break 2 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2GW {
    #[doc = "1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled"]
    Trigger = 1,
}
impl From<B2GW> for bool {
    #[inline(always)]
    fn from(variant: B2GW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2G` writer - Break 2 generation"]
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG, B2GW>;
impl<'a, REG> B2G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(B2GW::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Capture/compare (1-4) generation"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1G` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccg(&mut self, n: u8) -> CCG_W<EGRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCG_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CCG_W<EGRrs> {
        CCG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<EGRrs> {
        COMG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<EGRrs> {
        TG_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<EGRrs> {
        BG_W::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 generation"]
    #[inline(always)]
    #[must_use]
    pub fn b2g(&mut self) -> B2G_W<EGRrs> {
        B2G_W::new(self, 8)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGRrs {
    const RESET_VALUE: u32 = 0;
}
