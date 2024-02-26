#[doc = "Register `OCOLR` reader"]
pub type R = crate::R<OCOLRrs>;
#[doc = "Register `OCOLR` writer"]
pub type W = crate::W<OCOLRrs>;
#[doc = "Field `BLUE` reader - Blue Value These bits define the blue value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type BLUE_R = crate::FieldReader;
#[doc = "Field `BLUE` writer - Blue Value These bits define the blue value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` reader - Green Value These bits define the green value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type GREEN_R = crate::FieldReader;
#[doc = "Field `GREEN` writer - Green Value These bits define the green value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED` reader - Red Value These bits define the red value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type RED_R = crate::FieldReader;
#[doc = "Field `RED` writer - Red Value These bits define the red value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALPHA` reader - Alpha Channel Value These bits define the alpha channel of the output color. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type ALPHA_R = crate::FieldReader;
#[doc = "Field `ALPHA` writer - Alpha Channel Value These bits define the alpha channel of the output color. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Blue Value These bits define the blue value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green Value These bits define the green value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red Value These bits define the red value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Alpha Channel Value These bits define the alpha channel of the output color. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue Value These bits define the blue value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<OCOLRrs> {
        BLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green Value These bits define the green value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<OCOLRrs> {
        GREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red Value These bits define the red value of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<OCOLRrs> {
        RED_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Alpha Channel Value These bits define the alpha channel of the output color. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<OCOLRrs> {
        ALPHA_W::new(self, 24)
    }
}
#[doc = "DMA2D output color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocolr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocolr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCOLRrs;
impl crate::RegisterSpec for OCOLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocolr::R`](R) reader structure"]
impl crate::Readable for OCOLRrs {}
#[doc = "`write(|w| ..)` method takes [`ocolr::W`](W) writer structure"]
impl crate::Writable for OCOLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCOLR to value 0"]
impl crate::Resettable for OCOLRrs {
    const RESET_VALUE: u32 = 0;
}
