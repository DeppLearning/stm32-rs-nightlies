#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DADDRrs>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DADDRrs>;
#[doc = "Field `ADD` reader - Device address"]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Device address"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Enable function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EF {
    #[doc = "0: USB device disabled"]
    Disabled = 0,
    #[doc = "1: USB device enabled"]
    Enabled = 1,
}
impl From<EF> for bool {
    #[inline(always)]
    fn from(variant: EF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EF` reader - Enable function"]
pub type EF_R = crate::BitReader<EF>;
impl EF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EF {
        match self.bits {
            false => EF::Disabled,
            true => EF::Enabled,
        }
    }
    #[doc = "USB device disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EF::Disabled
    }
    #[doc = "USB device enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EF::Enabled
    }
}
#[doc = "Field `EF` writer - Enable function"]
pub type EF_W<'a, REG> = crate::BitWriter<'a, REG, EF>;
impl<'a, REG> EF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB device disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EF::Disabled)
    }
    #[doc = "USB device enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EF::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<DADDRrs> {
        ADD_W::new(self, 0)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    #[must_use]
    pub fn ef(&mut self) -> EF_W<DADDRrs> {
        EF_W::new(self, 7)
    }
}
#[doc = "device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DADDRrs;
impl crate::RegisterSpec for DADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DADDRrs {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DADDRrs {
    const RESET_VALUE: u32 = 0;
}
