#[doc = "Register `X2BUFCFG` reader"]
pub type R = crate::R<X2BUFCFGrs>;
#[doc = "Register `X2BUFCFG` writer"]
pub type W = crate::W<X2BUFCFGrs>;
#[doc = "Field `X2_BASE` reader - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result."]
pub type X2_BASE_R = crate::FieldReader;
#[doc = "Field `X2_BASE` writer - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result."]
pub type X2_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `X2_BUF_SIZE` reader - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1)."]
pub type X2_BUF_SIZE_R = crate::FieldReader;
#[doc = "Field `X2_BUF_SIZE` writer - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1)."]
pub type X2_BUF_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result."]
    #[inline(always)]
    pub fn x2_base(&self) -> X2_BASE_R {
        X2_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1)."]
    #[inline(always)]
    pub fn x2_buf_size(&self) -> X2_BUF_SIZE_R {
        X2_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base address of X2 buffer The X2 buffer base address can be modified while START=1, for example to change coefficient values. The filter should be stalled when doing this, since changing the coefficients while a calculation is ongoing affects the result."]
    #[inline(always)]
    #[must_use]
    pub fn x2_base(&mut self) -> X2_BASE_W<X2BUFCFGrs> {
        X2_BASE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Size of X2 buffer in 16-bit words This bitfield can not be modified when a function is ongoing (START = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn x2_buf_size(&mut self) -> X2_BUF_SIZE_W<X2BUFCFGrs> {
        X2_BUF_SIZE_W::new(self, 8)
    }
}
#[doc = "FMAC X2 buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`x2bufcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`x2bufcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct X2BUFCFGrs;
impl crate::RegisterSpec for X2BUFCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`x2bufcfg::R`](R) reader structure"]
impl crate::Readable for X2BUFCFGrs {}
#[doc = "`write(|w| ..)` method takes [`x2bufcfg::W`](W) writer structure"]
impl crate::Writable for X2BUFCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets X2BUFCFG to value 0"]
impl crate::Resettable for X2BUFCFGrs {
    const RESET_VALUE: u32 = 0;
}
