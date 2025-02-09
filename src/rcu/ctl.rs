#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `IRC16MEN` reader - Internal 16MHz RC oscillator Enable"]
pub type Irc16menR = crate::BitReader;
#[doc = "Field `IRC16MEN` writer - Internal 16MHz RC oscillator Enable"]
pub type Irc16menW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC16MSTB` reader - IRC16M Internal 16MHz RC Oscillator stabilization Flag"]
pub type Irc16mstbR = crate::BitReader;
#[doc = "Field `IRC16MADJ` reader - Internal 16MHz RC Oscillator clock trim adjust value"]
pub type Irc16madjR = crate::FieldReader;
#[doc = "Field `IRC16MADJ` writer - Internal 16MHz RC Oscillator clock trim adjust value"]
pub type Irc16madjW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IRC16MCALIB` reader - Internal 16MHz RC Oscillator calibration value register"]
pub type Irc16mcalibR = crate::FieldReader;
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub type HxtalenR = crate::BitReader;
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub type HxtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HxtalstbR = crate::BitReader;
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsR = crate::BitReader;
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CkmenR = crate::BitReader;
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CkmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PllenR = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PllstbR = crate::BitReader;
#[doc = "Field `PLLI2SEN` reader - PLLI2S enable"]
pub type Plli2senR = crate::BitReader;
#[doc = "Field `PLLI2SEN` writer - PLLI2S enable"]
pub type Plli2senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLI2SSTB` reader - PLLI2S Clock Stabilization Flag"]
pub type Plli2sstbR = crate::BitReader;
#[doc = "Field `PLLSAIEN` reader - PLLSAI enable"]
pub type PllsaienR = crate::BitReader;
#[doc = "Field `PLLSAIEN` writer - PLLSAI enable"]
pub type PllsaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAISTB` reader - PLLSAI Clock Stabilization Flag"]
pub type PllsaistbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Internal 16MHz RC oscillator Enable"]
    #[inline(always)]
    pub fn irc16men(&self) -> Irc16menR {
        Irc16menR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC16M Internal 16MHz RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc16mstb(&self) -> Irc16mstbR {
        Irc16mstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 16MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc16madj(&self) -> Irc16madjR {
        Irc16madjR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 16MHz RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc16mcalib(&self) -> Irc16mcalibR {
        Irc16mcalibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HxtalenR {
        HxtalenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HxtalstbR {
        HxtalstbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HxtalbpsR {
        HxtalbpsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CkmenR {
        CkmenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllstb(&self) -> PllstbR {
        PllstbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline(always)]
    pub fn plli2sen(&self) -> Plli2senR {
        Plli2senR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLLI2S Clock Stabilization Flag"]
    #[inline(always)]
    pub fn plli2sstb(&self) -> Plli2sstbR {
        Plli2sstbR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLLSAI enable"]
    #[inline(always)]
    pub fn pllsaien(&self) -> PllsaienR {
        PllsaienR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLLSAI Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllsaistb(&self) -> PllsaistbR {
        PllsaistbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal 16MHz RC oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc16men(&mut self) -> Irc16menW<CtlSpec> {
        Irc16menW::new(self, 0)
    }
    #[doc = "Bits 3:7 - Internal 16MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    #[must_use]
    pub fn irc16madj(&mut self) -> Irc16madjW<CtlSpec> {
        Irc16madjW::new(self, 3)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalen(&mut self) -> HxtalenW<CtlSpec> {
        HxtalenW::new(self, 16)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalbps(&mut self) -> HxtalbpsW<CtlSpec> {
        HxtalbpsW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckmen(&mut self) -> CkmenW<CtlSpec> {
        CkmenW::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PllenW<CtlSpec> {
        PllenW::new(self, 24)
    }
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sen(&mut self) -> Plli2senW<CtlSpec> {
        Plli2senW::new(self, 26)
    }
    #[doc = "Bit 28 - PLLSAI enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaien(&mut self) -> PllsaienW<CtlSpec> {
        PllsaienW::new(self, 28)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x83"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x83;
}
