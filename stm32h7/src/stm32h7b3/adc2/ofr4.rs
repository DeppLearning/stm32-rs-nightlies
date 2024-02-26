#[doc = "Register `OFR4` reader"]
pub type R = crate::R<OFR4rs>;
#[doc = "Register `OFR4` writer"]
pub type W = crate::W<OFR4rs>;
#[doc = "Field `OFFSET4` reader - ADC offset number 1 offset level"]
pub type OFFSET4_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET4` writer - ADC offset number 1 offset level"]
pub type OFFSET4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 26, u32>;
#[doc = "Field `OFFSET4_CH` reader - ADC offset number 1 channel selection"]
pub type OFFSET4_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET4_CH` writer - ADC offset number 1 channel selection"]
pub type OFFSET4_CH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Signed saturation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSATE {
    #[doc = "0: Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)"]
    Disabled = 0,
    #[doc = "1: Offset is subtracted and result is saturated to maintain result size"]
    Enabled = 1,
}
impl From<SSATE> for bool {
    #[inline(always)]
    fn from(variant: SSATE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSATE` reader - Signed saturation enable"]
pub type SSATE_R = crate::BitReader<SSATE>;
impl SSATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSATE {
        match self.bits {
            false => SSATE::Disabled,
            true => SSATE::Enabled,
        }
    }
    #[doc = "Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSATE::Disabled
    }
    #[doc = "Offset is subtracted and result is saturated to maintain result size"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSATE::Enabled
    }
}
#[doc = "Field `SSATE` writer - Signed saturation enable"]
pub type SSATE_W<'a, REG> = crate::BitWriter<'a, REG, SSATE>;
impl<'a, REG> SSATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSATE::Disabled)
    }
    #[doc = "Offset is subtracted and result is saturated to maintain result size"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSATE::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Signed saturation enable"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    #[must_use]
    pub fn offset4(&mut self) -> OFFSET4_W<OFR4rs> {
        OFFSET4_W::new(self, 0)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<OFR4rs> {
        OFFSET4_CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - Signed saturation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<OFR4rs> {
        SSATE_W::new(self, 31)
    }
}
#[doc = "ADC offset number 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFR4rs;
impl crate::RegisterSpec for OFR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr4::R`](R) reader structure"]
impl crate::Readable for OFR4rs {}
#[doc = "`write(|w| ..)` method takes [`ofr4::W`](W) writer structure"]
impl crate::Writable for OFR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFR4 to value 0"]
impl crate::Resettable for OFR4rs {
    const RESET_VALUE: u32 = 0;
}
