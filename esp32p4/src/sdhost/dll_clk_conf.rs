#[doc = "Register `DLL_CLK_CONF` reader"]
pub type R = crate::R<DLL_CLK_CONF_SPEC>;
#[doc = "Register `DLL_CLK_CONF` writer"]
pub type W = crate::W<DLL_CLK_CONF_SPEC>;
#[doc = "Field `DLL_CCLK_IN_SLF_EN` reader - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SLF_EN_R = crate::BitReader;
#[doc = "Field `DLL_CCLK_IN_SLF_EN` writer - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SLF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CCLK_IN_DRV_EN` reader - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_DRV_EN_R = crate::BitReader;
#[doc = "Field `DLL_CCLK_IN_DRV_EN` writer - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_DRV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CCLK_IN_SAM_EN` reader - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SAM_EN_R = crate::BitReader;
#[doc = "Field `DLL_CCLK_IN_SAM_EN` writer - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SAM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CCLK_IN_SLF_PHASE` reader - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SLF_PHASE_R = crate::FieldReader;
#[doc = "Field `DLL_CCLK_IN_SLF_PHASE` writer - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SLF_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DLL_CCLK_IN_DRV_PHASE` reader - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_DRV_PHASE_R = crate::FieldReader;
#[doc = "Field `DLL_CCLK_IN_DRV_PHASE` writer - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_DRV_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DLL_CCLK_IN_SAM_PHASE` reader - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SAM_PHASE_R = crate::FieldReader;
#[doc = "Field `DLL_CCLK_IN_SAM_PHASE` writer - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DLL_CCLK_IN_SAM_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_slf_en(&self) -> DLL_CCLK_IN_SLF_EN_R {
        DLL_CCLK_IN_SLF_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_drv_en(&self) -> DLL_CCLK_IN_DRV_EN_R {
        DLL_CCLK_IN_DRV_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_sam_en(&self) -> DLL_CCLK_IN_SAM_EN_R {
        DLL_CCLK_IN_SAM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_slf_phase(&self) -> DLL_CCLK_IN_SLF_PHASE_R {
        DLL_CCLK_IN_SLF_PHASE_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:14 - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_drv_phase(&self) -> DLL_CCLK_IN_DRV_PHASE_R {
        DLL_CCLK_IN_DRV_PHASE_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 15:20 - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_sam_phase(&self) -> DLL_CCLK_IN_SAM_PHASE_R {
        DLL_CCLK_IN_SAM_PHASE_R::new(((self.bits >> 15) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_CLK_CONF")
            .field(
                "dll_cclk_in_slf_en",
                &format_args!("{}", self.dll_cclk_in_slf_en().bit()),
            )
            .field(
                "dll_cclk_in_drv_en",
                &format_args!("{}", self.dll_cclk_in_drv_en().bit()),
            )
            .field(
                "dll_cclk_in_sam_en",
                &format_args!("{}", self.dll_cclk_in_sam_en().bit()),
            )
            .field(
                "dll_cclk_in_slf_phase",
                &format_args!("{}", self.dll_cclk_in_slf_phase().bits()),
            )
            .field(
                "dll_cclk_in_drv_phase",
                &format_args!("{}", self.dll_cclk_in_drv_phase().bits()),
            )
            .field(
                "dll_cclk_in_sam_phase",
                &format_args!("{}", self.dll_cclk_in_sam_phase().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DLL_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    #[must_use]
    pub fn dll_cclk_in_slf_en(&mut self) -> DLL_CCLK_IN_SLF_EN_W<DLL_CLK_CONF_SPEC> {
        DLL_CCLK_IN_SLF_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    #[must_use]
    pub fn dll_cclk_in_drv_en(&mut self) -> DLL_CCLK_IN_DRV_EN_W<DLL_CLK_CONF_SPEC> {
        DLL_CCLK_IN_DRV_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    #[must_use]
    pub fn dll_cclk_in_sam_en(&mut self) -> DLL_CCLK_IN_SAM_EN_W<DLL_CLK_CONF_SPEC> {
        DLL_CCLK_IN_SAM_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:8 - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    #[must_use]
    pub fn dll_cclk_in_slf_phase(&mut self) -> DLL_CCLK_IN_SLF_PHASE_W<DLL_CLK_CONF_SPEC> {
        DLL_CCLK_IN_SLF_PHASE_W::new(self, 3)
    }
    #[doc = "Bits 9:14 - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    #[must_use]
    pub fn dll_cclk_in_drv_phase(&mut self) -> DLL_CCLK_IN_DRV_PHASE_W<DLL_CLK_CONF_SPEC> {
        DLL_CCLK_IN_DRV_PHASE_W::new(self, 9)
    }
    #[doc = "Bits 15:20 - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    #[must_use]
    pub fn dll_cclk_in_sam_phase(&mut self) -> DLL_CCLK_IN_SAM_PHASE_W<DLL_CLK_CONF_SPEC> {
        DLL_CCLK_IN_SAM_PHASE_W::new(self, 15)
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
#[doc = "SDIO DLL clock control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_CLK_CONF_SPEC;
impl crate::RegisterSpec for DLL_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_clk_conf::R`](R) reader structure"]
impl crate::Readable for DLL_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dll_clk_conf::W`](W) writer structure"]
impl crate::Writable for DLL_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CLK_CONF to value 0"]
impl crate::Resettable for DLL_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
