#[doc = "Register `L2_ROM_PWR_CTRL0` reader"]
pub type R = crate::R<L2_ROM_PWR_CTRL0_SPEC>;
#[doc = "Register `L2_ROM_PWR_CTRL0` writer"]
pub type W = crate::W<L2_ROM_PWR_CTRL0_SPEC>;
#[doc = "Field `REG_L2_ROM_CLK_FORCE_ON` reader - l2_rom clk gating force on"]
pub type REG_L2_ROM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `REG_L2_ROM_CLK_FORCE_ON` writer - l2_rom clk gating force on"]
pub type REG_L2_ROM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - l2_rom clk gating force on"]
    #[inline(always)]
    pub fn reg_l2_rom_clk_force_on(&self) -> REG_L2_ROM_CLK_FORCE_ON_R {
        REG_L2_ROM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_ROM_PWR_CTRL0")
            .field(
                "reg_l2_rom_clk_force_on",
                &format_args!("{}", self.reg_l2_rom_clk_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_ROM_PWR_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - l2_rom clk gating force on"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_rom_clk_force_on(&mut self) -> REG_L2_ROM_CLK_FORCE_ON_W<L2_ROM_PWR_CTRL0_SPEC> {
        REG_L2_ROM_CLK_FORCE_ON_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_rom_pwr_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_rom_pwr_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_ROM_PWR_CTRL0_SPEC;
impl crate::RegisterSpec for L2_ROM_PWR_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_rom_pwr_ctrl0::R`](R) reader structure"]
impl crate::Readable for L2_ROM_PWR_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_rom_pwr_ctrl0::W`](W) writer structure"]
impl crate::Writable for L2_ROM_PWR_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_ROM_PWR_CTRL0 to value 0"]
impl crate::Resettable for L2_ROM_PWR_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
