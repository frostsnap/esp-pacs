#[doc = "Register `TAG_FO_CTRL` reader"]
pub type R = crate::R<TAG_FO_CTRL_SPEC>;
#[doc = "Register `TAG_FO_CTRL` writer"]
pub type W = crate::W<TAG_FO_CTRL_SPEC>;
#[doc = "Field `PRO_CACHE_TAG_FORCE_ON` reader - "]
pub type PRO_CACHE_TAG_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_TAG_FORCE_ON` writer - "]
pub type PRO_CACHE_TAG_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_TAG_PD` reader - "]
pub type PRO_CACHE_TAG_PD_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_TAG_PD` writer - "]
pub type PRO_CACHE_TAG_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_TAG_FORCE_ON` reader - "]
pub type APP_CACHE_TAG_FORCE_ON_R = crate::BitReader;
#[doc = "Field `APP_CACHE_TAG_FORCE_ON` writer - "]
pub type APP_CACHE_TAG_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_TAG_PD` reader - "]
pub type APP_CACHE_TAG_PD_R = crate::BitReader;
#[doc = "Field `APP_CACHE_TAG_PD` writer - "]
pub type APP_CACHE_TAG_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_tag_force_on(&self) -> PRO_CACHE_TAG_FORCE_ON_R {
        PRO_CACHE_TAG_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_cache_tag_pd(&self) -> PRO_CACHE_TAG_PD_R {
        PRO_CACHE_TAG_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_tag_force_on(&self) -> APP_CACHE_TAG_FORCE_ON_R {
        APP_CACHE_TAG_FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_tag_pd(&self) -> APP_CACHE_TAG_PD_R {
        APP_CACHE_TAG_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAG_FO_CTRL")
            .field(
                "pro_cache_tag_force_on",
                &format_args!("{}", self.pro_cache_tag_force_on().bit()),
            )
            .field(
                "pro_cache_tag_pd",
                &format_args!("{}", self.pro_cache_tag_pd().bit()),
            )
            .field(
                "app_cache_tag_force_on",
                &format_args!("{}", self.app_cache_tag_force_on().bit()),
            )
            .field(
                "app_cache_tag_pd",
                &format_args!("{}", self.app_cache_tag_pd().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TAG_FO_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_tag_force_on(&mut self) -> PRO_CACHE_TAG_FORCE_ON_W<TAG_FO_CTRL_SPEC> {
        PRO_CACHE_TAG_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_tag_pd(&mut self) -> PRO_CACHE_TAG_PD_W<TAG_FO_CTRL_SPEC> {
        PRO_CACHE_TAG_PD_W::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_tag_force_on(&mut self) -> APP_CACHE_TAG_FORCE_ON_W<TAG_FO_CTRL_SPEC> {
        APP_CACHE_TAG_FORCE_ON_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_tag_pd(&mut self) -> APP_CACHE_TAG_PD_W<TAG_FO_CTRL_SPEC> {
        APP_CACHE_TAG_PD_W::new(self, 9)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tag_fo_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tag_fo_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAG_FO_CTRL_SPEC;
impl crate::RegisterSpec for TAG_FO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag_fo_ctrl::R`](R) reader structure"]
impl crate::Readable for TAG_FO_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tag_fo_ctrl::W`](W) writer structure"]
impl crate::Writable for TAG_FO_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAG_FO_CTRL to value 0x0101"]
impl crate::Resettable for TAG_FO_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0101;
}
