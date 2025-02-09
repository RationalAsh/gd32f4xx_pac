#[doc = "Register `TG` writer"]
pub type W = crate::W<TgSpec>;
#[doc = "Field `TG0` writer - Port toggle bit"]
pub type Tg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG1` writer - Port toggle bit"]
pub type Tg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG2` writer - Port toggle bit"]
pub type Tg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG3` writer - Port toggle bit"]
pub type Tg3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG4` writer - Port toggle bit"]
pub type Tg4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG5` writer - Port toggle bit"]
pub type Tg5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG6` writer - Port toggle bit"]
pub type Tg6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG7` writer - Port toggle bit"]
pub type Tg7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG8` writer - Port toggle bit"]
pub type Tg8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG9` writer - Port toggle bit"]
pub type Tg9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG10` writer - Port toggle bit"]
pub type Tg10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG11` writer - Port toggle bit"]
pub type Tg11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG12` writer - Port toggle bit"]
pub type Tg12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG13` writer - Port toggle bit"]
pub type Tg13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG14` writer - Port toggle bit"]
pub type Tg14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG15` writer - Port toggle bit"]
pub type Tg15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg0(&mut self) -> Tg0W<TgSpec> {
        Tg0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg1(&mut self) -> Tg1W<TgSpec> {
        Tg1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg2(&mut self) -> Tg2W<TgSpec> {
        Tg2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg3(&mut self) -> Tg3W<TgSpec> {
        Tg3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg4(&mut self) -> Tg4W<TgSpec> {
        Tg4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg5(&mut self) -> Tg5W<TgSpec> {
        Tg5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg6(&mut self) -> Tg6W<TgSpec> {
        Tg6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg7(&mut self) -> Tg7W<TgSpec> {
        Tg7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg8(&mut self) -> Tg8W<TgSpec> {
        Tg8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg9(&mut self) -> Tg9W<TgSpec> {
        Tg9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg10(&mut self) -> Tg10W<TgSpec> {
        Tg10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg11(&mut self) -> Tg11W<TgSpec> {
        Tg11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg12(&mut self) -> Tg12W<TgSpec> {
        Tg12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg13(&mut self) -> Tg13W<TgSpec> {
        Tg13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg14(&mut self) -> Tg14W<TgSpec> {
        Tg14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tg15(&mut self) -> Tg15W<TgSpec> {
        Tg15W::new(self, 15)
    }
}
#[doc = "Port bit toggle register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgSpec;
impl crate::RegisterSpec for TgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tg::W`](W) writer structure"]
impl crate::Writable for TgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TG to value 0"]
impl crate::Resettable for TgSpec {
    const RESET_VALUE: u32 = 0;
}
