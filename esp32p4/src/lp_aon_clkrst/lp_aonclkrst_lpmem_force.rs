#[doc = "Register `LP_AONCLKRST_LPMEM_FORCE` reader"]
pub type R = crate::R<LP_AONCLKRST_LPMEM_FORCE_SPEC>;
#[doc = "Register `LP_AONCLKRST_LPMEM_FORCE` writer"]
pub type W = crate::W<LP_AONCLKRST_LPMEM_FORCE_SPEC>;
#[doc = "Field `LP_AONCLKRST_LPMEM_CLK_FORCE_ON` reader - reserved"]
pub type LP_AONCLKRST_LPMEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LPMEM_CLK_FORCE_ON` writer - reserved"]
pub type LP_AONCLKRST_LPMEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_lpmem_clk_force_on(&self) -> LP_AONCLKRST_LPMEM_CLK_FORCE_ON_R {
        LP_AONCLKRST_LPMEM_CLK_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_LPMEM_FORCE")
            .field(
                "lp_aonclkrst_lpmem_clk_force_on",
                &format_args!("{}", self.lp_aonclkrst_lpmem_clk_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_AONCLKRST_LPMEM_FORCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_lpmem_clk_force_on(
        &mut self,
    ) -> LP_AONCLKRST_LPMEM_CLK_FORCE_ON_W<LP_AONCLKRST_LPMEM_FORCE_SPEC> {
        LP_AONCLKRST_LPMEM_CLK_FORCE_ON_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lpmem_force::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lpmem_force::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_LPMEM_FORCE_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_LPMEM_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_lpmem_force::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_LPMEM_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_lpmem_force::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_LPMEM_FORCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_LPMEM_FORCE to value 0"]
impl crate::Resettable for LP_AONCLKRST_LPMEM_FORCE_SPEC {
    const RESET_VALUE: u32 = 0;
}
