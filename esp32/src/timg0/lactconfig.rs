#[doc = "Register `LACTCONFIG` reader"]
pub type R = crate::R<LACTCONFIG_SPEC>;
#[doc = "Register `LACTCONFIG` writer"]
pub type W = crate::W<LACTCONFIG_SPEC>;
#[doc = "Field `LACT_RTC_ONLY` reader - "]
pub type LACT_RTC_ONLY_R = crate::BitReader;
#[doc = "Field `LACT_RTC_ONLY` writer - "]
pub type LACT_RTC_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_CPST_EN` reader - "]
pub type LACT_CPST_EN_R = crate::BitReader;
#[doc = "Field `LACT_CPST_EN` writer - "]
pub type LACT_CPST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_LAC_EN` reader - "]
pub type LACT_LAC_EN_R = crate::BitReader;
#[doc = "Field `LACT_LAC_EN` writer - "]
pub type LACT_LAC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_ALARM_EN` reader - "]
pub type LACT_ALARM_EN_R = crate::BitReader;
#[doc = "Field `LACT_ALARM_EN` writer - "]
pub type LACT_ALARM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_LEVEL_INT_EN` reader - "]
pub type LACT_LEVEL_INT_EN_R = crate::BitReader;
#[doc = "Field `LACT_LEVEL_INT_EN` writer - "]
pub type LACT_LEVEL_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_EDGE_INT_EN` reader - "]
pub type LACT_EDGE_INT_EN_R = crate::BitReader;
#[doc = "Field `LACT_EDGE_INT_EN` writer - "]
pub type LACT_EDGE_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_DIVIDER` reader - "]
pub type LACT_DIVIDER_R = crate::FieldReader<u16>;
#[doc = "Field `LACT_DIVIDER` writer - "]
pub type LACT_DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LACT_AUTORELOAD` reader - "]
pub type LACT_AUTORELOAD_R = crate::BitReader;
#[doc = "Field `LACT_AUTORELOAD` writer - "]
pub type LACT_AUTORELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_INCREASE` reader - "]
pub type LACT_INCREASE_R = crate::BitReader;
#[doc = "Field `LACT_INCREASE` writer - "]
pub type LACT_INCREASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACT_EN` reader - "]
pub type LACT_EN_R = crate::BitReader;
#[doc = "Field `LACT_EN` writer - "]
pub type LACT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lact_rtc_only(&self) -> LACT_RTC_ONLY_R {
        LACT_RTC_ONLY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lact_cpst_en(&self) -> LACT_CPST_EN_R {
        LACT_CPST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lact_lac_en(&self) -> LACT_LAC_EN_R {
        LACT_LAC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lact_alarm_en(&self) -> LACT_ALARM_EN_R {
        LACT_ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lact_level_int_en(&self) -> LACT_LEVEL_INT_EN_R {
        LACT_LEVEL_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lact_edge_int_en(&self) -> LACT_EDGE_INT_EN_R {
        LACT_EDGE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn lact_divider(&self) -> LACT_DIVIDER_R {
        LACT_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lact_autoreload(&self) -> LACT_AUTORELOAD_R {
        LACT_AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lact_increase(&self) -> LACT_INCREASE_R {
        LACT_INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lact_en(&self) -> LACT_EN_R {
        LACT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTCONFIG")
            .field(
                "lact_rtc_only",
                &format_args!("{}", self.lact_rtc_only().bit()),
            )
            .field(
                "lact_cpst_en",
                &format_args!("{}", self.lact_cpst_en().bit()),
            )
            .field("lact_lac_en", &format_args!("{}", self.lact_lac_en().bit()))
            .field(
                "lact_alarm_en",
                &format_args!("{}", self.lact_alarm_en().bit()),
            )
            .field(
                "lact_level_int_en",
                &format_args!("{}", self.lact_level_int_en().bit()),
            )
            .field(
                "lact_edge_int_en",
                &format_args!("{}", self.lact_edge_int_en().bit()),
            )
            .field(
                "lact_divider",
                &format_args!("{}", self.lact_divider().bits()),
            )
            .field(
                "lact_autoreload",
                &format_args!("{}", self.lact_autoreload().bit()),
            )
            .field(
                "lact_increase",
                &format_args!("{}", self.lact_increase().bit()),
            )
            .field("lact_en", &format_args!("{}", self.lact_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTCONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn lact_rtc_only(&mut self) -> LACT_RTC_ONLY_W<LACTCONFIG_SPEC> {
        LACT_RTC_ONLY_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lact_cpst_en(&mut self) -> LACT_CPST_EN_W<LACTCONFIG_SPEC> {
        LACT_CPST_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn lact_lac_en(&mut self) -> LACT_LAC_EN_W<LACTCONFIG_SPEC> {
        LACT_LAC_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn lact_alarm_en(&mut self) -> LACT_ALARM_EN_W<LACTCONFIG_SPEC> {
        LACT_ALARM_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn lact_level_int_en(&mut self) -> LACT_LEVEL_INT_EN_W<LACTCONFIG_SPEC> {
        LACT_LEVEL_INT_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn lact_edge_int_en(&mut self) -> LACT_EDGE_INT_EN_W<LACTCONFIG_SPEC> {
        LACT_EDGE_INT_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    #[must_use]
    pub fn lact_divider(&mut self) -> LACT_DIVIDER_W<LACTCONFIG_SPEC> {
        LACT_DIVIDER_W::new(self, 13)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn lact_autoreload(&mut self) -> LACT_AUTORELOAD_W<LACTCONFIG_SPEC> {
        LACT_AUTORELOAD_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn lact_increase(&mut self) -> LACT_INCREASE_W<LACTCONFIG_SPEC> {
        LACT_INCREASE_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn lact_en(&mut self) -> LACT_EN_W<LACTCONFIG_SPEC> {
        LACT_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTCONFIG_SPEC;
impl crate::RegisterSpec for LACTCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactconfig::R`](R) reader structure"]
impl crate::Readable for LACTCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactconfig::W`](W) writer structure"]
impl crate::Writable for LACTCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTCONFIG to value 0x6000_2300"]
impl crate::Resettable for LACTCONFIG_SPEC {
    const RESET_VALUE: u32 = 0x6000_2300;
}
