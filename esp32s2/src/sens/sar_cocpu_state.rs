#[doc = "Register `SAR_COCPU_STATE` reader"]
pub type R = crate::R<SAR_COCPU_STATE_SPEC>;
#[doc = "Register `SAR_COCPU_STATE` writer"]
pub type W = crate::W<SAR_COCPU_STATE_SPEC>;
#[doc = "Field `COCPU_DBG_TRIGGER` writer - Trigger ULP-RISCV debug registers"]
pub type COCPU_DBG_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_CLK_EN` reader - Check ULP-RISCV whether clk on"]
pub type COCPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `COCPU_RESET_N` reader - Check ULP-RISCV whether in reset state"]
pub type COCPU_RESET_N_R = crate::BitReader;
#[doc = "Field `COCPU_EOI` reader - Check ULP-RISCV whether in interrupt state"]
pub type COCPU_EOI_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP` reader - Check ULP-RISCV whether in trap state"]
pub type COCPU_TRAP_R = crate::BitReader;
#[doc = "Field `COCPU_EBREAK` reader - Check ULP-RISCV whether in ebreak"]
pub type COCPU_EBREAK_R = crate::BitReader;
impl R {
    #[doc = "Bit 26 - Check ULP-RISCV whether clk on"]
    #[inline(always)]
    pub fn cocpu_clk_en(&self) -> COCPU_CLK_EN_R {
        COCPU_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Check ULP-RISCV whether in reset state"]
    #[inline(always)]
    pub fn cocpu_reset_n(&self) -> COCPU_RESET_N_R {
        COCPU_RESET_N_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Check ULP-RISCV whether in interrupt state"]
    #[inline(always)]
    pub fn cocpu_eoi(&self) -> COCPU_EOI_R {
        COCPU_EOI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Check ULP-RISCV whether in trap state"]
    #[inline(always)]
    pub fn cocpu_trap(&self) -> COCPU_TRAP_R {
        COCPU_TRAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Check ULP-RISCV whether in ebreak"]
    #[inline(always)]
    pub fn cocpu_ebreak(&self) -> COCPU_EBREAK_R {
        COCPU_EBREAK_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_STATE")
            .field(
                "cocpu_clk_en",
                &format_args!("{}", self.cocpu_clk_en().bit()),
            )
            .field(
                "cocpu_reset_n",
                &format_args!("{}", self.cocpu_reset_n().bit()),
            )
            .field("cocpu_eoi", &format_args!("{}", self.cocpu_eoi().bit()))
            .field("cocpu_trap", &format_args!("{}", self.cocpu_trap().bit()))
            .field(
                "cocpu_ebreak",
                &format_args!("{}", self.cocpu_ebreak().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 25 - Trigger ULP-RISCV debug registers"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_dbg_trigger(&mut self) -> COCPU_DBG_TRIGGER_W<SAR_COCPU_STATE_SPEC> {
        COCPU_DBG_TRIGGER_W::new(self, 25)
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
#[doc = "ULP-RISCV status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_STATE_SPEC;
impl crate::RegisterSpec for SAR_COCPU_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_cocpu_state::R`](R) reader structure"]
impl crate::Readable for SAR_COCPU_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_cocpu_state::W`](W) writer structure"]
impl crate::Writable for SAR_COCPU_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_COCPU_STATE to value 0"]
impl crate::Resettable for SAR_COCPU_STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
