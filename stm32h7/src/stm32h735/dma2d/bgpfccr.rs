#[doc = "Register `BGPFCCR` reader"]
pub type R = crate::R<BGPFCCRrs>;
#[doc = "Register `BGPFCCR` writer"]
pub type W = crate::W<BGPFCCRrs>;
#[doc = "Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM {
    #[doc = "0: Color mode ARGB8888"]
    Argb8888 = 0,
    #[doc = "1: Color mode RGB888"]
    Rgb888 = 1,
    #[doc = "2: Color mode RGB565"]
    Rgb565 = 2,
    #[doc = "3: Color mode ARGB1555"]
    Argb1555 = 3,
    #[doc = "4: Color mode ARGB4444"]
    Argb4444 = 4,
    #[doc = "5: Color mode L8"]
    L8 = 5,
    #[doc = "6: Color mode AL44"]
    Al44 = 6,
    #[doc = "7: Color mode AL88"]
    Al88 = 7,
    #[doc = "8: Color mode L4"]
    L4 = 8,
    #[doc = "9: Color mode A8"]
    A8 = 9,
    #[doc = "10: Color mode A4"]
    A4 = 10,
}
impl From<CM> for u8 {
    #[inline(always)]
    fn from(variant: CM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CM {
    type Ux = u8;
}
#[doc = "Field `CM` reader - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CM_R = crate::FieldReader<CM>;
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CM> {
        match self.bits {
            0 => Some(CM::Argb8888),
            1 => Some(CM::Rgb888),
            2 => Some(CM::Rgb565),
            3 => Some(CM::Argb1555),
            4 => Some(CM::Argb4444),
            5 => Some(CM::L8),
            6 => Some(CM::Al44),
            7 => Some(CM::Al88),
            8 => Some(CM::L4),
            9 => Some(CM::A8),
            10 => Some(CM::A4),
            _ => None,
        }
    }
    #[doc = "Color mode ARGB8888"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM::Argb8888
    }
    #[doc = "Color mode RGB888"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM::Rgb888
    }
    #[doc = "Color mode RGB565"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM::Rgb565
    }
    #[doc = "Color mode ARGB1555"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM::Argb1555
    }
    #[doc = "Color mode ARGB4444"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM::Argb4444
    }
    #[doc = "Color mode L8"]
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == CM::L8
    }
    #[doc = "Color mode AL44"]
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == CM::Al44
    }
    #[doc = "Color mode AL88"]
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == CM::Al88
    }
    #[doc = "Color mode L4"]
    #[inline(always)]
    pub fn is_l4(&self) -> bool {
        *self == CM::L4
    }
    #[doc = "Color mode A8"]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == CM::A8
    }
    #[doc = "Color mode A4"]
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == CM::A4
    }
}
#[doc = "Field `CM` writer - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CM>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Color mode ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb8888)
    }
    #[doc = "Color mode RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb888)
    }
    #[doc = "Color mode RGB565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb565)
    }
    #[doc = "Color mode ARGB1555"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb1555)
    }
    #[doc = "Color mode ARGB4444"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb4444)
    }
    #[doc = "Color mode L8"]
    #[inline(always)]
    pub fn l8(self) -> &'a mut crate::W<REG> {
        self.variant(CM::L8)
    }
    #[doc = "Color mode AL44"]
    #[inline(always)]
    pub fn al44(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Al44)
    }
    #[doc = "Color mode AL88"]
    #[inline(always)]
    pub fn al88(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Al88)
    }
    #[doc = "Color mode L4"]
    #[inline(always)]
    pub fn l4(self) -> &'a mut crate::W<REG> {
        self.variant(CM::L4)
    }
    #[doc = "Color mode A8"]
    #[inline(always)]
    pub fn a8(self) -> &'a mut crate::W<REG> {
        self.variant(CM::A8)
    }
    #[doc = "Color mode A4"]
    #[inline(always)]
    pub fn a4(self) -> &'a mut crate::W<REG> {
        self.variant(CM::A4)
    }
}
#[doc = "CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCM {
    #[doc = "0: CLUT color format ARGB8888"]
    Argb8888 = 0,
    #[doc = "1: CLUT color format RGB888"]
    Rgb888 = 1,
}
impl From<CCM> for bool {
    #[inline(always)]
    fn from(variant: CCM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCM` reader - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
pub type CCM_R = crate::BitReader<CCM>;
impl CCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCM {
        match self.bits {
            false => CCM::Argb8888,
            true => CCM::Rgb888,
        }
    }
    #[doc = "CLUT color format ARGB8888"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CCM::Argb8888
    }
    #[doc = "CLUT color format RGB888"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CCM::Rgb888
    }
}
#[doc = "Field `CCM` writer - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
pub type CCM_W<'a, REG> = crate::BitWriter<'a, REG, CCM>;
impl<'a, REG> CCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLUT color format ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(CCM::Argb8888)
    }
    #[doc = "CLUT color format RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(CCM::Rgb888)
    }
}
#[doc = "Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START {
    #[doc = "1: Start the automatic loading of the CLUT"]
    Start = 1,
}
impl From<START> for bool {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
pub type START_R = crate::BitReader<START>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<START> {
        match self.bits {
            true => Some(START::Start),
            _ => None,
        }
    }
    #[doc = "Start the automatic loading of the CLUT"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START::Start
    }
}
#[doc = "Field `START` writer - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start the automatic loading of the CLUT"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START::Start)
    }
}
#[doc = "Field `CS` reader - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
pub type CS_R = crate::FieldReader;
#[doc = "Field `CS` writer - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
pub type CS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AM {
    #[doc = "0: No modification of alpha channel"]
    NoModify = 0,
    #[doc = "1: Replace with value in ALPHA\\[7:0\\]"]
    Replace = 1,
    #[doc = "2: Multiply with value in ALPHA\\[7:0\\]"]
    Multiply = 2,
}
impl From<AM> for u8 {
    #[inline(always)]
    fn from(variant: AM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AM {
    type Ux = u8;
}
#[doc = "Field `AM` reader - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type AM_R = crate::FieldReader<AM>;
impl AM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AM> {
        match self.bits {
            0 => Some(AM::NoModify),
            1 => Some(AM::Replace),
            2 => Some(AM::Multiply),
            _ => None,
        }
    }
    #[doc = "No modification of alpha channel"]
    #[inline(always)]
    pub fn is_no_modify(&self) -> bool {
        *self == AM::NoModify
    }
    #[doc = "Replace with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn is_replace(&self) -> bool {
        *self == AM::Replace
    }
    #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn is_multiply(&self) -> bool {
        *self == AM::Multiply
    }
}
#[doc = "Field `AM` writer - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type AM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, AM>;
impl<'a, REG> AM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No modification of alpha channel"]
    #[inline(always)]
    pub fn no_modify(self) -> &'a mut crate::W<REG> {
        self.variant(AM::NoModify)
    }
    #[doc = "Replace with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn replace(self) -> &'a mut crate::W<REG> {
        self.variant(AM::Replace)
    }
    #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn multiply(self) -> &'a mut crate::W<REG> {
        self.variant(AM::Multiply)
    }
}
#[doc = "Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI {
    #[doc = "0: Regular alpha"]
    RegularAlpha = 0,
    #[doc = "1: Inverted alpha"]
    InvertedAlpha = 1,
}
impl From<AI> for bool {
    #[inline(always)]
    fn from(variant: AI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` reader - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_R = crate::BitReader<AI>;
impl AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AI {
        match self.bits {
            false => AI::RegularAlpha,
            true => AI::InvertedAlpha,
        }
    }
    #[doc = "Regular alpha"]
    #[inline(always)]
    pub fn is_regular_alpha(&self) -> bool {
        *self == AI::RegularAlpha
    }
    #[doc = "Inverted alpha"]
    #[inline(always)]
    pub fn is_inverted_alpha(&self) -> bool {
        *self == AI::InvertedAlpha
    }
}
#[doc = "Field `AI` writer - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG, AI>;
impl<'a, REG> AI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular alpha"]
    #[inline(always)]
    pub fn regular_alpha(self) -> &'a mut crate::W<REG> {
        self.variant(AI::RegularAlpha)
    }
    #[doc = "Inverted alpha"]
    #[inline(always)]
    pub fn inverted_alpha(self) -> &'a mut crate::W<REG> {
        self.variant(AI::InvertedAlpha)
    }
}
#[doc = "Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBS {
    #[doc = "0: No Red Blue Swap (RGB or ARGB)"]
    Regular = 0,
    #[doc = "1: Red Blue Swap (BGR or ABGR)"]
    Swap = 1,
}
impl From<RBS> for bool {
    #[inline(always)]
    fn from(variant: RBS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBS` reader - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_R = crate::BitReader<RBS>;
impl RBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBS {
        match self.bits {
            false => RBS::Regular,
            true => RBS::Swap,
        }
    }
    #[doc = "No Red Blue Swap (RGB or ARGB)"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == RBS::Regular
    }
    #[doc = "Red Blue Swap (BGR or ABGR)"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == RBS::Swap
    }
}
#[doc = "Field `RBS` writer - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG, RBS>;
impl<'a, REG> RBS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Red Blue Swap (RGB or ARGB)"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(RBS::Regular)
    }
    #[doc = "Red Blue Swap (BGR or ABGR)"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut crate::W<REG> {
        self.variant(RBS::Swap)
    }
}
#[doc = "Field `ALPHA` reader - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type ALPHA_R = crate::FieldReader;
#[doc = "Field `ALPHA` writer - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type ALPHA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<BGPFCCRrs> {
        CM_W::new(self, 0)
    }
    #[doc = "Bit 4 - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<BGPFCCRrs> {
        CCM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<BGPFCCRrs> {
        START_W::new(self, 5)
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<BGPFCCRrs> {
        CS_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<BGPFCCRrs> {
        AM_W::new(self, 16)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<BGPFCCRrs> {
        AI_W::new(self, 20)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<BGPFCCRrs> {
        RBS_W::new(self, 21)
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<BGPFCCRrs> {
        ALPHA_W::new(self, 24)
    }
}
#[doc = "DMA2D background PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgpfccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgpfccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGPFCCRrs;
impl crate::RegisterSpec for BGPFCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgpfccr::R`](R) reader structure"]
impl crate::Readable for BGPFCCRrs {}
#[doc = "`write(|w| ..)` method takes [`bgpfccr::W`](W) writer structure"]
impl crate::Writable for BGPFCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BGPFCCR to value 0"]
impl crate::Resettable for BGPFCCRrs {
    const RESET_VALUE: u32 = 0;
}
