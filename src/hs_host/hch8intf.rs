#[doc = "Register `HCH8INTF` reader"]
pub type R = crate::R<Hch8intfSpec>;
#[doc = "Register `HCH8INTF` writer"]
pub type W = crate::W<Hch8intfSpec>;
#[doc = "Field `TF` reader - Transfer completed"]
pub type TfR = crate::BitReader;
#[doc = "Field `TF` writer - Transfer completed"]
pub type TfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH` reader - Channel halted"]
pub type ChR = crate::BitReader;
#[doc = "Field `CH` writer - Channel halted"]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAER` reader - DMA Error"]
pub type DmaerR = crate::BitReader;
#[doc = "Field `DMAER` writer - DMA Error"]
pub type DmaerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - NYET"]
pub type NyetR = crate::BitReader;
#[doc = "Field `NYET` writer - NYET"]
pub type NyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBER` reader - USB bus error"]
pub type UsberR = crate::BitReader;
#[doc = "Field `USBER` writer - USB bus error"]
pub type UsberW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBER` reader - Babble error"]
pub type BberR = crate::BitReader;
#[doc = "Field `BBER` writer - Babble error"]
pub type BberW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQOVR` reader - Request queue overrun"]
pub type ReqovrR = crate::BitReader;
#[doc = "Field `REQOVR` writer - Request queue overrun"]
pub type ReqovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTER` reader - Data toggle error"]
pub type DterR = crate::BitReader;
#[doc = "Field `DTER` writer - Data toggle error"]
pub type DterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Error"]
    #[inline(always)]
    pub fn dmaer(&self) -> DmaerR {
        DmaerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    pub fn nyet(&self) -> NyetR {
        NyetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    pub fn usber(&self) -> UsberR {
        UsberR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bber(&self) -> BberR {
        BberR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    pub fn reqovr(&self) -> ReqovrR {
        ReqovrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dter(&self) -> DterR {
        DterR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TfW<Hch8intfSpec> {
        TfW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> ChW<Hch8intfSpec> {
        ChW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn dmaer(&mut self) -> DmaerW<Hch8intfSpec> {
        DmaerW::new(self, 2)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<Hch8intfSpec> {
        StallW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NakW<Hch8intfSpec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<Hch8intfSpec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NyetW<Hch8intfSpec> {
        NyetW::new(self, 6)
    }
    #[doc = "Bit 7 - USB bus error"]
    #[inline(always)]
    #[must_use]
    pub fn usber(&mut self) -> UsberW<Hch8intfSpec> {
        UsberW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bber(&mut self) -> BberW<Hch8intfSpec> {
        BberW::new(self, 8)
    }
    #[doc = "Bit 9 - Request queue overrun"]
    #[inline(always)]
    #[must_use]
    pub fn reqovr(&mut self) -> ReqovrW<Hch8intfSpec> {
        ReqovrW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dter(&mut self) -> DterW<Hch8intfSpec> {
        DterW::new(self, 10)
    }
}
#[doc = "host channel-8 interrupt flag register (HCH8INTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`hch8intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch8intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch8intfSpec;
impl crate::RegisterSpec for Hch8intfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch8intf::R`](R) reader structure"]
impl crate::Readable for Hch8intfSpec {}
#[doc = "`write(|w| ..)` method takes [`hch8intf::W`](W) writer structure"]
impl crate::Writable for Hch8intfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCH8INTF to value 0"]
impl crate::Resettable for Hch8intfSpec {
    const RESET_VALUE: u32 = 0;
}
