#[doc = "Register `VDDBAT_CFG` reader"]
pub type R = crate::R<VDDBAT_CFG_SPEC>;
#[doc = "Register `VDDBAT_CFG` writer"]
pub type W = crate::W<VDDBAT_CFG_SPEC>;
#[doc = "Field `ANA_VDDBAT_MODE` reader - need_des"]
pub type ANA_VDDBAT_MODE_R = crate::FieldReader;
#[doc = "Field `VDDBAT_SW_UPDATE` writer - need_des"]
pub type VDDBAT_SW_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn ana_vddbat_mode(&self) -> ANA_VDDBAT_MODE_R {
        ANA_VDDBAT_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDDBAT_CFG")
            .field(
                "ana_vddbat_mode",
                &format_args!("{}", self.ana_vddbat_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VDDBAT_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_sw_update(&mut self) -> VDDBAT_SW_UPDATE_W<VDDBAT_CFG_SPEC> {
        VDDBAT_SW_UPDATE_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vddbat_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddbat_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDDBAT_CFG_SPEC;
impl crate::RegisterSpec for VDDBAT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vddbat_cfg::R`](R) reader structure"]
impl crate::Readable for VDDBAT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vddbat_cfg::W`](W) writer structure"]
impl crate::Writable for VDDBAT_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDDBAT_CFG to value 0"]
impl crate::Resettable for VDDBAT_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
