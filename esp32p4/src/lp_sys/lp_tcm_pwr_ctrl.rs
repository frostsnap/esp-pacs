#[doc = "Register `LP_TCM_PWR_CTRL` reader"]
pub type R = crate::R<LP_TCM_PWR_CTRL_SPEC>;
#[doc = "Register `LP_TCM_PWR_CTRL` writer"]
pub type W = crate::W<LP_TCM_PWR_CTRL_SPEC>;
#[doc = "Field `LP_TCM_ROM_CLK_FORCE_ON` reader - need_des"]
pub type LP_TCM_ROM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_TCM_ROM_CLK_FORCE_ON` writer - need_des"]
pub type LP_TCM_ROM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TCM_RAM_CLK_FORCE_ON` reader - need_des"]
pub type LP_TCM_RAM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_TCM_RAM_CLK_FORCE_ON` writer - need_des"]
pub type LP_TCM_RAM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_rom_clk_force_on(&self) -> LP_TCM_ROM_CLK_FORCE_ON_R {
        LP_TCM_ROM_CLK_FORCE_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_ram_clk_force_on(&self) -> LP_TCM_RAM_CLK_FORCE_ON_R {
        LP_TCM_RAM_CLK_FORCE_ON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TCM_PWR_CTRL")
            .field(
                "lp_tcm_rom_clk_force_on",
                &format_args!("{}", self.lp_tcm_rom_clk_force_on().bit()),
            )
            .field(
                "lp_tcm_ram_clk_force_on",
                &format_args!("{}", self.lp_tcm_ram_clk_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_TCM_PWR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_tcm_rom_clk_force_on(&mut self) -> LP_TCM_ROM_CLK_FORCE_ON_W<LP_TCM_PWR_CTRL_SPEC> {
        LP_TCM_ROM_CLK_FORCE_ON_W::new(self, 5)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_tcm_ram_clk_force_on(&mut self) -> LP_TCM_RAM_CLK_FORCE_ON_W<LP_TCM_PWR_CTRL_SPEC> {
        LP_TCM_RAM_CLK_FORCE_ON_W::new(self, 7)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_pwr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_pwr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_TCM_PWR_CTRL_SPEC;
impl crate::RegisterSpec for LP_TCM_PWR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_tcm_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_TCM_PWR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_tcm_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_TCM_PWR_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_TCM_PWR_CTRL to value 0"]
impl crate::Resettable for LP_TCM_PWR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
