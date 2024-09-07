#[doc = "Register `SNWTCFG2` reader"]
pub type R = crate::R<Snwtcfg2Spec>;
#[doc = "Register `SNWTCFG2` writer"]
pub type W = crate::W<Snwtcfg2Spec>;
#[doc = "Field `WASET` reader - Address setup time"]
pub type WasetR = crate::FieldReader;
#[doc = "Field `WASET` writer - Address setup time"]
pub type WasetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAHLD` reader - Address hold time"]
pub type WahldR = crate::FieldReader;
#[doc = "Field `WAHLD` writer - Address hold time"]
pub type WahldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WDSET` reader - Data setup time"]
pub type WdsetR = crate::FieldReader;
#[doc = "Field `WDSET` writer - Data setup time"]
pub type WdsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WBUSLAT` reader - Bus latency"]
pub type WbuslatR = crate::FieldReader;
#[doc = "Field `WBUSLAT` writer - Bus latency"]
pub type WbuslatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WASYNCMOD` reader - Asynchronous access mode"]
pub type WasyncmodR = crate::FieldReader;
#[doc = "Field `WASYNCMOD` writer - Asynchronous access mode"]
pub type WasyncmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn waset(&self) -> WasetR {
        WasetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn wahld(&self) -> WahldR {
        WahldR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn wdset(&self) -> WdsetR {
        WdsetR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn wbuslat(&self) -> WbuslatR {
        WbuslatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&self) -> WasyncmodR {
        WasyncmodR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn waset(&mut self) -> WasetW<Snwtcfg2Spec> {
        WasetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn wahld(&mut self) -> WahldW<Snwtcfg2Spec> {
        WahldW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn wdset(&mut self) -> WdsetW<Snwtcfg2Spec> {
        WdsetW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn wbuslat(&mut self) -> WbuslatW<Snwtcfg2Spec> {
        WbuslatW::new(self, 16)
    }
    #[doc = "Bits 28:29 - Asynchronous access mode"]
    #[inline(always)]
    #[must_use]
    pub fn wasyncmod(&mut self) -> WasyncmodW<Snwtcfg2Spec> {
        WasyncmodW::new(self, 28)
    }
}
#[doc = "SRAM/NOR flash write timing configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`snwtcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snwtcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snwtcfg2Spec;
impl crate::RegisterSpec for Snwtcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snwtcfg2::R`](R) reader structure"]
impl crate::Readable for Snwtcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`snwtcfg2::W`](W) writer structure"]
impl crate::Writable for Snwtcfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNWTCFG2 to value 0x0fff_ffff"]
impl crate::Resettable for Snwtcfg2Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
