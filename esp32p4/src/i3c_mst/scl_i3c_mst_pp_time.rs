#[doc = "Register `SCL_I3C_MST_PP_TIME` reader"]
pub type R = crate::R<SCL_I3C_MST_PP_TIME_SPEC>;
#[doc = "Register `SCL_I3C_MST_PP_TIME` writer"]
pub type W = crate::W<SCL_I3C_MST_PP_TIME_SPEC>;
#[doc = "Field `REG_I3C_MST_PP_LOW_PERIOD` reader - NA"]
pub type REG_I3C_MST_PP_LOW_PERIOD_R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_PP_LOW_PERIOD` writer - NA"]
pub type REG_I3C_MST_PP_LOW_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I3C_MST_PP_HIGH_PERIOD` reader - NA"]
pub type REG_I3C_MST_PP_HIGH_PERIOD_R = crate::FieldReader;
#[doc = "Field `REG_I3C_MST_PP_HIGH_PERIOD` writer - NA"]
pub type REG_I3C_MST_PP_HIGH_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_pp_low_period(&self) -> REG_I3C_MST_PP_LOW_PERIOD_R {
        REG_I3C_MST_PP_LOW_PERIOD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i3c_mst_pp_high_period(&self) -> REG_I3C_MST_PP_HIGH_PERIOD_R {
        REG_I3C_MST_PP_HIGH_PERIOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_I3C_MST_PP_TIME")
            .field(
                "reg_i3c_mst_pp_low_period",
                &format_args!("{}", self.reg_i3c_mst_pp_low_period().bits()),
            )
            .field(
                "reg_i3c_mst_pp_high_period",
                &format_args!("{}", self.reg_i3c_mst_pp_high_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_I3C_MST_PP_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_pp_low_period(
        &mut self,
    ) -> REG_I3C_MST_PP_LOW_PERIOD_W<SCL_I3C_MST_PP_TIME_SPEC> {
        REG_I3C_MST_PP_LOW_PERIOD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_pp_high_period(
        &mut self,
    ) -> REG_I3C_MST_PP_HIGH_PERIOD_W<SCL_I3C_MST_PP_TIME_SPEC> {
        REG_I3C_MST_PP_HIGH_PERIOD_W::new(self, 16)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_i3c_mst_pp_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_i3c_mst_pp_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_I3C_MST_PP_TIME_SPEC;
impl crate::RegisterSpec for SCL_I3C_MST_PP_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_i3c_mst_pp_time::R`](R) reader structure"]
impl crate::Readable for SCL_I3C_MST_PP_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_i3c_mst_pp_time::W`](W) writer structure"]
impl crate::Writable for SCL_I3C_MST_PP_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_I3C_MST_PP_TIME to value 0x0005_0005"]
impl crate::Resettable for SCL_I3C_MST_PP_TIME_SPEC {
    const RESET_VALUE: u32 = 0x0005_0005;
}
