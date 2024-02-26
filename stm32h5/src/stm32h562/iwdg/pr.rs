#[doc = "Register `PR` reader"]
pub type R = crate::R<PRrs>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PRrs>;
#[doc = "Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR {
    #[doc = "0: Divider /4"]
    DivideBy4 = 0,
    #[doc = "1: Divider /8"]
    DivideBy8 = 1,
    #[doc = "2: Divider /16"]
    DivideBy16 = 2,
    #[doc = "3: Divider /32"]
    DivideBy32 = 3,
    #[doc = "4: Divider /64"]
    DivideBy64 = 4,
    #[doc = "5: Divider /128"]
    DivideBy128 = 5,
    #[doc = "6: Divider /256"]
    DivideBy256 = 6,
    #[doc = "7: Divider /512"]
    DivideBy512 = 7,
    #[doc = "8: Divider /1024"]
    DivideBy1024 = 8,
}
impl From<PR> for u8 {
    #[inline(always)]
    fn from(variant: PR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR {
    type Ux = u8;
}
#[doc = "Field `PR` reader - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_R = crate::FieldReader<PR>;
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PR> {
        match self.bits {
            0 => Some(PR::DivideBy4),
            1 => Some(PR::DivideBy8),
            2 => Some(PR::DivideBy16),
            3 => Some(PR::DivideBy32),
            4 => Some(PR::DivideBy64),
            5 => Some(PR::DivideBy128),
            6 => Some(PR::DivideBy256),
            7 => Some(PR::DivideBy512),
            8 => Some(PR::DivideBy1024),
            _ => None,
        }
    }
    #[doc = "Divider /4"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == PR::DivideBy4
    }
    #[doc = "Divider /8"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == PR::DivideBy8
    }
    #[doc = "Divider /16"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == PR::DivideBy16
    }
    #[doc = "Divider /32"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == PR::DivideBy32
    }
    #[doc = "Divider /64"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == PR::DivideBy64
    }
    #[doc = "Divider /128"]
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == PR::DivideBy128
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn is_divide_by256(&self) -> bool {
        *self == PR::DivideBy256
    }
    #[doc = "Divider /512"]
    #[inline(always)]
    pub fn is_divide_by512(&self) -> bool {
        *self == PR::DivideBy512
    }
    #[doc = "Divider /1024"]
    #[inline(always)]
    pub fn is_divide_by1024(&self) -> bool {
        *self == PR::DivideBy1024
    }
}
#[doc = "Field `PR` writer - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PR>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divider /4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy4)
    }
    #[doc = "Divider /8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy8)
    }
    #[doc = "Divider /16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy16)
    }
    #[doc = "Divider /32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy32)
    }
    #[doc = "Divider /64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy64)
    }
    #[doc = "Divider /128"]
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy128)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy256)
    }
    #[doc = "Divider /512"]
    #[inline(always)]
    pub fn divide_by512(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy512)
    }
    #[doc = "Divider /1024"]
    #[inline(always)]
    pub fn divide_by1024(self) -> &'a mut crate::W<REG> {
        self.variant(PR::DivideBy1024)
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<PRrs> {
        PR_W::new(self, 0)
    }
}
#[doc = "IWDG prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PRrs {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PRrs {
    const RESET_VALUE: u32 = 0;
}
