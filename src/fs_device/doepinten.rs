#[doc = "Register `DOEPINTEN` reader"]
pub type R = crate::R<DoepintenSpec>;
#[doc = "Register `DOEPINTEN` writer"]
pub type W = crate::W<DoepintenSpec>;
#[doc = "Field `TFEN` reader - Transfer finished interrupt enable"]
pub type TfenR = crate::BitReader;
#[doc = "Field `TFEN` writer - Transfer finished interrupt enable"]
pub type TfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISEN` reader - Endpoint disabled interrupt enable"]
pub type EpdisenR = crate::BitReader;
#[doc = "Field `EPDISEN` writer - Endpoint disabled interrupt enable"]
pub type EpdisenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPFEN` reader - SETUP phase finished interrupt enable"]
pub type StpfenR = crate::BitReader;
#[doc = "Field `STPFEN` writer - SETUP phase finished interrupt enable"]
pub type StpfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRXFOVREN` reader - Endpoint Rx FIFO overrun interrupt enable"]
pub type EprxfovrenR = crate::BitReader;
#[doc = "Field `EPRXFOVREN` writer - Endpoint Rx FIFO overrun interrupt enable"]
pub type EprxfovrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTBSTPEN` reader - Back-to-back SETUP packets interrupt enable"]
pub type BtbstpenR = crate::BitReader;
#[doc = "Field `BTBSTPEN` writer - Back-to-back SETUP packets interrupt enable"]
pub type BtbstpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    pub fn tfen(&self) -> TfenR {
        TfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    pub fn epdisen(&self) -> EpdisenR {
        EpdisenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable"]
    #[inline(always)]
    pub fn stpfen(&self) -> StpfenR {
        StpfenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn eprxfovren(&self) -> EprxfovrenR {
        EprxfovrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable"]
    #[inline(always)]
    pub fn btbstpen(&self) -> BtbstpenR {
        BtbstpenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfen(&mut self) -> TfenW<DoepintenSpec> {
        TfenW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdisen(&mut self) -> EpdisenW<DoepintenSpec> {
        EpdisenW::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpfen(&mut self) -> StpfenW<DoepintenSpec> {
        StpfenW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eprxfovren(&mut self) -> EprxfovrenW<DoepintenSpec> {
        EprxfovrenW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn btbstpen(&mut self) -> BtbstpenW<DoepintenSpec> {
        BtbstpenW::new(self, 6)
    }
}
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`doepinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepintenSpec;
impl crate::RegisterSpec for DoepintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepinten::R`](R) reader structure"]
impl crate::Readable for DoepintenSpec {}
#[doc = "`write(|w| ..)` method takes [`doepinten::W`](W) writer structure"]
impl crate::Writable for DoepintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINTEN to value 0"]
impl crate::Resettable for DoepintenSpec {
    const RESET_VALUE: u32 = 0;
}
