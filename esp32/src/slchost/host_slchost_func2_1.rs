#[doc = "Register `HOST_SLCHOST_FUNC2_1` reader"]
pub type R = crate::R<HOST_SLCHOST_FUNC2_1_SPEC>;
#[doc = "Register `HOST_SLCHOST_FUNC2_1` writer"]
pub type W = crate::W<HOST_SLCHOST_FUNC2_1_SPEC>;
#[doc = "Field `HOST_SLC_FUNC2_INT_EN` reader - "]
pub type HOST_SLC_FUNC2_INT_EN_R = crate::BitReader;
#[doc = "Field `HOST_SLC_FUNC2_INT_EN` writer - "]
pub type HOST_SLC_FUNC2_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc_func2_int_en(&self) -> HOST_SLC_FUNC2_INT_EN_R {
        HOST_SLC_FUNC2_INT_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_FUNC2_1")
            .field(
                "host_slc_func2_int_en",
                &format_args!("{}", self.host_slc_func2_int_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_FUNC2_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc_func2_int_en(&mut self) -> HOST_SLC_FUNC2_INT_EN_W<HOST_SLCHOST_FUNC2_1_SPEC> {
        HOST_SLC_FUNC2_INT_EN_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_func2_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_func2_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_FUNC2_1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_FUNC2_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_func2_1::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_func2_1::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_FUNC2_1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_FUNC2_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
