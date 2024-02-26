#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Cortex-M0 LOCKUP bit enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_LOCK {
    #[doc = "0: Cortex-M4 LOCKUP output disconnected from TIM1/15/16/17 Break inputs and HRTIM1 SYSFLT."]
    Disconnected = 0,
    #[doc = "1: Cortex-M4 LOCKUP output connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    Connected = 1,
}
impl From<LOCKUP_LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0 LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_R = crate::BitReader<LOCKUP_LOCK>;
impl LOCKUP_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCKUP_LOCK {
        match self.bits {
            false => LOCKUP_LOCK::Disconnected,
            true => LOCKUP_LOCK::Connected,
        }
    }
    #[doc = "Cortex-M4 LOCKUP output disconnected from TIM1/15/16/17 Break inputs and HRTIM1 SYSFLT."]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == LOCKUP_LOCK::Disconnected
    }
    #[doc = "Cortex-M4 LOCKUP output connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == LOCKUP_LOCK::Connected
    }
}
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0 LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKUP_LOCK>;
impl<'a, REG> LOCKUP_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cortex-M4 LOCKUP output disconnected from TIM1/15/16/17 Break inputs and HRTIM1 SYSFLT."]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK::Disconnected)
    }
    #[doc = "Cortex-M4 LOCKUP output connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUP_LOCK::Connected)
    }
}
#[doc = "SRAM parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PARITY_LOCK {
    #[doc = "0: SRAM parity error signal disconnected from TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    Disconnected = 0,
    #[doc = "1: SRAM parity error signal connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    Connected = 1,
}
impl From<SRAM_PARITY_LOCK> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PARITY_LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_R = crate::BitReader<SRAM_PARITY_LOCK>;
impl SRAM_PARITY_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM_PARITY_LOCK {
        match self.bits {
            false => SRAM_PARITY_LOCK::Disconnected,
            true => SRAM_PARITY_LOCK::Connected,
        }
    }
    #[doc = "SRAM parity error signal disconnected from TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == SRAM_PARITY_LOCK::Disconnected
    }
    #[doc = "SRAM parity error signal connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == SRAM_PARITY_LOCK::Connected
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, SRAM_PARITY_LOCK>;
impl<'a, REG> SRAM_PARITY_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM parity error signal disconnected from TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PARITY_LOCK::Disconnected)
    }
    #[doc = "SRAM parity error signal connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PARITY_LOCK::Connected)
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVD_LOCK {
    #[doc = "0: PVD interrupt disconnected from TIM15/16/17 Break input"]
    Disconnected = 0,
    #[doc = "1: PVD interrupt connected to TIM15/16/17 Break input"]
    Connected = 1,
}
impl From<PVD_LOCK> for bool {
    #[inline(always)]
    fn from(variant: PVD_LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PVD_LOCK_R = crate::BitReader<PVD_LOCK>;
impl PVD_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVD_LOCK {
        match self.bits {
            false => PVD_LOCK::Disconnected,
            true => PVD_LOCK::Connected,
        }
    }
    #[doc = "PVD interrupt disconnected from TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PVD_LOCK::Disconnected
    }
    #[doc = "PVD interrupt connected to TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVD_LOCK::Connected
    }
}
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PVD_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, PVD_LOCK>;
impl<'a, REG> PVD_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVD interrupt disconnected from TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(PVD_LOCK::Disconnected)
    }
    #[doc = "PVD interrupt connected to TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(PVD_LOCK::Connected)
    }
}
#[doc = "Bypass address bit 29 in parity calculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYP_ADDR_PAR {
    #[doc = "0: The ramload operation is performed taking into consideration bit 29 of the address when the parity is calculated"]
    NoBypass = 0,
    #[doc = "1: The ramload operation is performed without taking into consideration bit 29 of the address when the parity is calculated"]
    Bypass = 1,
}
impl From<BYP_ADDR_PAR> for bool {
    #[inline(always)]
    fn from(variant: BYP_ADDR_PAR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYP_ADDR_PAR` reader - Bypass address bit 29 in parity calculation"]
pub type BYP_ADDR_PAR_R = crate::BitReader<BYP_ADDR_PAR>;
impl BYP_ADDR_PAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYP_ADDR_PAR {
        match self.bits {
            false => BYP_ADDR_PAR::NoBypass,
            true => BYP_ADDR_PAR::Bypass,
        }
    }
    #[doc = "The ramload operation is performed taking into consideration bit 29 of the address when the parity is calculated"]
    #[inline(always)]
    pub fn is_no_bypass(&self) -> bool {
        *self == BYP_ADDR_PAR::NoBypass
    }
    #[doc = "The ramload operation is performed without taking into consideration bit 29 of the address when the parity is calculated"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == BYP_ADDR_PAR::Bypass
    }
}
#[doc = "Field `BYP_ADDR_PAR` writer - Bypass address bit 29 in parity calculation"]
pub type BYP_ADDR_PAR_W<'a, REG> = crate::BitWriter<'a, REG, BYP_ADDR_PAR>;
impl<'a, REG> BYP_ADDR_PAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The ramload operation is performed taking into consideration bit 29 of the address when the parity is calculated"]
    #[inline(always)]
    pub fn no_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(BYP_ADDR_PAR::NoBypass)
    }
    #[doc = "The ramload operation is performed without taking into consideration bit 29 of the address when the parity is calculated"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(BYP_ADDR_PAR::Bypass)
    }
}
#[doc = "SRAM parity flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PEFR {
    #[doc = "0: No SRAM parity error detected"]
    NoParityError = 0,
    #[doc = "1: SRAM parity error detected"]
    ParityErrorDetected = 1,
}
impl From<SRAM_PEFR> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PEFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PEF` reader - SRAM parity flag"]
pub type SRAM_PEF_R = crate::BitReader<SRAM_PEFR>;
impl SRAM_PEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM_PEFR {
        match self.bits {
            false => SRAM_PEFR::NoParityError,
            true => SRAM_PEFR::ParityErrorDetected,
        }
    }
    #[doc = "No SRAM parity error detected"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == SRAM_PEFR::NoParityError
    }
    #[doc = "SRAM parity error detected"]
    #[inline(always)]
    pub fn is_parity_error_detected(&self) -> bool {
        *self == SRAM_PEFR::ParityErrorDetected
    }
}
#[doc = "SRAM parity flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PEFW {
    #[doc = "1: Clear SRAM parity error flag"]
    Clear = 1,
}
impl From<SRAM_PEFW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PEFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PEF` writer - SRAM parity flag"]
pub type SRAM_PEF_W<'a, REG> = crate::BitWriter<'a, REG, SRAM_PEFW>;
impl<'a, REG> SRAM_PEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear SRAM parity error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM_PEFW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    pub fn byp_addr_par(&self) -> BYP_ADDR_PAR_R {
        BYP_ADDR_PAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<CFGR2rs> {
        LOCKUP_LOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W<CFGR2rs> {
        SRAM_PARITY_LOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<CFGR2rs> {
        PVD_LOCK_W::new(self, 2)
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    #[must_use]
    pub fn byp_addr_par(&mut self) -> BYP_ADDR_PAR_W<CFGR2rs> {
        BYP_ADDR_PAR_W::new(self, 4)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W<CFGR2rs> {
        SRAM_PEF_W::new(self, 8)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
