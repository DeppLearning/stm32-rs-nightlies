#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE {
    #[doc = "0: Peripheral disabled"]
    Disabled = 0,
    #[doc = "1: Peripheral enabled"]
    Enabled = 1,
}
impl From<SPE> for bool {
    #[inline(always)]
    fn from(variant: SPE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPE` reader - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
pub type SPE_R = crate::BitReader<SPE>;
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPE {
        match self.bits {
            false => SPE::Disabled,
            true => SPE::Enabled,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE::Disabled
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE::Enabled
    }
}
#[doc = "Field `SPE` writer - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG, SPE>;
impl<'a, REG> SPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::Disabled)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::Enabled)
    }
}
#[doc = "master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASRX {
    #[doc = "0: Automatic suspend in master receive-only mode disabled"]
    Disabled = 0,
    #[doc = "1: Automatic suspend in master receive-only mode enabled"]
    Enabled = 1,
}
impl From<MASRX> for bool {
    #[inline(always)]
    fn from(variant: MASRX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASRX` reader - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
pub type MASRX_R = crate::BitReader<MASRX>;
impl MASRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASRX {
        match self.bits {
            false => MASRX::Disabled,
            true => MASRX::Enabled,
        }
    }
    #[doc = "Automatic suspend in master receive-only mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASRX::Disabled
    }
    #[doc = "Automatic suspend in master receive-only mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASRX::Enabled
    }
}
#[doc = "Field `MASRX` writer - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG, MASRX>;
impl<'a, REG> MASRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic suspend in master receive-only mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MASRX::Disabled)
    }
    #[doc = "Automatic suspend in master receive-only mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MASRX::Enabled)
    }
}
#[doc = "master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTART {
    #[doc = "0: Do not start master transfer"]
    NotStarted = 0,
    #[doc = "1: Start master transfer"]
    Started = 1,
}
impl From<CSTART> for bool {
    #[inline(always)]
    fn from(variant: CSTART) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTART` reader - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
pub type CSTART_R = crate::BitReader<CSTART>;
impl CSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTART {
        match self.bits {
            false => CSTART::NotStarted,
            true => CSTART::Started,
        }
    }
    #[doc = "Do not start master transfer"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == CSTART::NotStarted
    }
    #[doc = "Start master transfer"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == CSTART::Started
    }
}
#[doc = "Field `CSTART` writer - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG, CSTART>;
impl<'a, REG> CSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not start master transfer"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(CSTART::NotStarted)
    }
    #[doc = "Start master transfer"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(CSTART::Started)
    }
}
#[doc = "master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before going to Low-power mode. Can be used in SPI or I2S mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSUSPW {
    #[doc = "0: Do not request master suspend"]
    NotRequested = 0,
    #[doc = "1: Request master suspend"]
    Requested = 1,
}
impl From<CSUSPW> for bool {
    #[inline(always)]
    fn from(variant: CSUSPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSUSP` writer - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before going to Low-power mode. Can be used in SPI or I2S mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts."]
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG, CSUSPW>;
impl<'a, REG> CSUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not request master suspend"]
    #[inline(always)]
    pub fn not_requested(self) -> &'a mut crate::W<REG> {
        self.variant(CSUSPW::NotRequested)
    }
    #[doc = "Request master suspend"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut crate::W<REG> {
        self.variant(CSUSPW::Requested)
    }
}
#[doc = "Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDDIR {
    #[doc = "0: Receiver in half duplex mode"]
    Receiver = 0,
    #[doc = "1: Transmitter in half duplex mode"]
    Transmitter = 1,
}
impl From<HDDIR> for bool {
    #[inline(always)]
    fn from(variant: HDDIR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
pub type HDDIR_R = crate::BitReader<HDDIR>;
impl HDDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDDIR {
        match self.bits {
            false => HDDIR::Receiver,
            true => HDDIR::Transmitter,
        }
    }
    #[doc = "Receiver in half duplex mode"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == HDDIR::Receiver
    }
    #[doc = "Transmitter in half duplex mode"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == HDDIR::Transmitter
    }
}
#[doc = "Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG, HDDIR>;
impl<'a, REG> HDDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver in half duplex mode"]
    #[inline(always)]
    pub fn receiver(self) -> &'a mut crate::W<REG> {
        self.variant(HDDIR::Receiver)
    }
    #[doc = "Transmitter in half duplex mode"]
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut crate::W<REG> {
        self.variant(HDDIR::Transmitter)
    }
}
#[doc = "internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSI {
    #[doc = "0: 0 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    SlaveSelected = 0,
    #[doc = "1: 1 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    SlaveNotSelected = 1,
}
impl From<SSI> for bool {
    #[inline(always)]
    fn from(variant: SSI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSI` reader - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
pub type SSI_R = crate::BitReader<SSI>;
impl SSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSI {
        match self.bits {
            false => SSI::SlaveSelected,
            true => SSI::SlaveNotSelected,
        }
    }
    #[doc = "0 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI::SlaveSelected
    }
    #[doc = "1 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI::SlaveNotSelected
    }
}
#[doc = "Field `SSI` writer - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG, SSI>;
impl<'a, REG> SSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SSI::SlaveSelected)
    }
    #[doc = "1 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SSI::SlaveNotSelected)
    }
}
#[doc = "32-bit CRC polynomial configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC33_17 {
    #[doc = "0: Full size (33/17 bit) CRC polynomial is not used"]
    Disabled = 0,
    #[doc = "1: Full size (33/17 bit) CRC polynomial is used"]
    Enabled = 1,
}
impl From<CRC33_17> for bool {
    #[inline(always)]
    fn from(variant: CRC33_17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC33_17` reader - 32-bit CRC polynomial configuration"]
pub type CRC33_17_R = crate::BitReader<CRC33_17>;
impl CRC33_17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRC33_17 {
        match self.bits {
            false => CRC33_17::Disabled,
            true => CRC33_17::Enabled,
        }
    }
    #[doc = "Full size (33/17 bit) CRC polynomial is not used"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRC33_17::Disabled
    }
    #[doc = "Full size (33/17 bit) CRC polynomial is used"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRC33_17::Enabled
    }
}
#[doc = "Field `CRC33_17` writer - 32-bit CRC polynomial configuration"]
pub type CRC33_17_W<'a, REG> = crate::BitWriter<'a, REG, CRC33_17>;
impl<'a, REG> CRC33_17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full size (33/17 bit) CRC polynomial is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRC33_17::Disabled)
    }
    #[doc = "Full size (33/17 bit) CRC polynomial is used"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRC33_17::Enabled)
    }
}
#[doc = "CRC calculation initialization pattern control for receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCRCINI {
    #[doc = "0: All zeros RX CRC initialization pattern"]
    AllZeros = 0,
    #[doc = "1: All ones RX CRC initialization pattern"]
    AllOnes = 1,
}
impl From<RCRCINI> for bool {
    #[inline(always)]
    fn from(variant: RCRCINI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRCINI` reader - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_R = crate::BitReader<RCRCINI>;
impl RCRCINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCRCINI {
        match self.bits {
            false => RCRCINI::AllZeros,
            true => RCRCINI::AllOnes,
        }
    }
    #[doc = "All zeros RX CRC initialization pattern"]
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == RCRCINI::AllZeros
    }
    #[doc = "All ones RX CRC initialization pattern"]
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == RCRCINI::AllOnes
    }
}
#[doc = "Field `RCRCINI` writer - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_W<'a, REG> = crate::BitWriter<'a, REG, RCRCINI>;
impl<'a, REG> RCRCINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All zeros RX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut crate::W<REG> {
        self.variant(RCRCINI::AllZeros)
    }
    #[doc = "All ones RX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut crate::W<REG> {
        self.variant(RCRCINI::AllOnes)
    }
}
#[doc = "CRC calculation initialization pattern control for transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCRCINI {
    #[doc = "0: All zeros TX CRC initialization pattern"]
    AllZeros = 0,
    #[doc = "1: All ones TX CRC initialization pattern"]
    AllOnes = 1,
}
impl From<TCRCINI> for bool {
    #[inline(always)]
    fn from(variant: TCRCINI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCRCINI` reader - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_R = crate::BitReader<TCRCINI>;
impl TCRCINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCRCINI {
        match self.bits {
            false => TCRCINI::AllZeros,
            true => TCRCINI::AllOnes,
        }
    }
    #[doc = "All zeros TX CRC initialization pattern"]
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == TCRCINI::AllZeros
    }
    #[doc = "All ones TX CRC initialization pattern"]
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == TCRCINI::AllOnes
    }
}
#[doc = "Field `TCRCINI` writer - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_W<'a, REG> = crate::BitWriter<'a, REG, TCRCINI>;
impl<'a, REG> TCRCINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All zeros TX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut crate::W<REG> {
        self.variant(TCRCINI::AllZeros)
    }
    #[doc = "All ones TX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut crate::W<REG> {
        self.variant(TCRCINI::AllOnes)
    }
}
#[doc = "locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOLOCK {
    #[doc = "0: IO configuration unlocked"]
    Unlocked = 0,
    #[doc = "1: IO configuration locked"]
    Locked = 1,
}
impl From<IOLOCK> for bool {
    #[inline(always)]
    fn from(variant: IOLOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOLOCK` reader - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
pub type IOLOCK_R = crate::BitReader<IOLOCK>;
impl IOLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOLOCK {
        match self.bits {
            false => IOLOCK::Unlocked,
            true => IOLOCK::Locked,
        }
    }
    #[doc = "IO configuration unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == IOLOCK::Unlocked
    }
    #[doc = "IO configuration locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == IOLOCK::Locked
    }
}
#[doc = "Field `IOLOCK` writer - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG, IOLOCK>;
impl<'a, REG> IOLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO configuration unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(IOLOCK::Unlocked)
    }
    #[doc = "IO configuration locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(IOLOCK::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<CR1rs> {
        SPE_W::new(self, 0)
    }
    #[doc = "Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
    #[inline(always)]
    #[must_use]
    pub fn masrx(&mut self) -> MASRX_W<CR1rs> {
        MASRX_W::new(self, 8)
    }
    #[doc = "Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn cstart(&mut self) -> CSTART_W<CR1rs> {
        CSTART_W::new(self, 9)
    }
    #[doc = "Bit 10 - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before going to Low-power mode. Can be used in SPI or I2S mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts."]
    #[inline(always)]
    #[must_use]
    pub fn csusp(&mut self) -> CSUSP_W<CR1rs> {
        CSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
    #[inline(always)]
    #[must_use]
    pub fn hddir(&mut self) -> HDDIR_W<CR1rs> {
        HDDIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<CR1rs> {
        SSI_W::new(self, 12)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    #[must_use]
    pub fn crc33_17(&mut self) -> CRC33_17_W<CR1rs> {
        CRC33_17_W::new(self, 13)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rcrcini(&mut self) -> RCRCINI_W<CR1rs> {
        RCRCINI_W::new(self, 14)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tcrcini(&mut self) -> TCRCINI_W<CR1rs> {
        TCRCINI_W::new(self, 15)
    }
    #[doc = "Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn iolock(&mut self) -> IOLOCK_W<CR1rs> {
        IOLOCK_W::new(self, 16)
    }
}
#[doc = "SPI/I2S control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
