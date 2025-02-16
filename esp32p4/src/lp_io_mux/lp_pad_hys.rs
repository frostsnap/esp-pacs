#[doc = "Register `LP_PAD_HYS` reader"]
pub type R = crate::R<LP_PAD_HYS_SPEC>;
#[doc = "Register `LP_PAD_HYS` writer"]
pub type W = crate::W<LP_PAD_HYS_SPEC>;
#[doc = "Field `REG_LP_GPIO_HYS` reader - Reserved"]
pub type REG_LP_GPIO_HYS_R = crate::FieldReader<u16>;
#[doc = "Field `REG_LP_GPIO_HYS` writer - Reserved"]
pub type REG_LP_GPIO_HYS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_lp_gpio_hys(&self) -> REG_LP_GPIO_HYS_R {
        REG_LP_GPIO_HYS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PAD_HYS")
            .field(
                "reg_lp_gpio_hys",
                &format_args!("{}", self.reg_lp_gpio_hys().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_PAD_HYS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_lp_gpio_hys(&mut self) -> REG_LP_GPIO_HYS_W<LP_PAD_HYS_SPEC> {
        REG_LP_GPIO_HYS_W::new(self, 0)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_pad_hys::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_pad_hys::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PAD_HYS_SPEC;
impl crate::RegisterSpec for LP_PAD_HYS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_pad_hys::R`](R) reader structure"]
impl crate::Readable for LP_PAD_HYS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_pad_hys::W`](W) writer structure"]
impl crate::Writable for LP_PAD_HYS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_PAD_HYS to value 0"]
impl crate::Resettable for LP_PAD_HYS_SPEC {
    const RESET_VALUE: u32 = 0;
}
