#[doc = "Register `UART_PD_CTRL` reader"]
pub type R = crate::R<UART_PD_CTRL_SPEC>;
#[doc = "Register `UART_PD_CTRL` writer"]
pub type W = crate::W<UART_PD_CTRL_SPEC>;
#[doc = "Field `UART_MEM_FORCE_PD` reader - Set this bit to power down hp uart internal memory."]
pub type UART_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `UART_MEM_FORCE_PD` writer - Set this bit to power down hp uart internal memory."]
pub type UART_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_FORCE_PU` reader - Set this bit to force power up hp uart internal memory"]
pub type UART_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `UART_MEM_FORCE_PU` writer - Set this bit to force power up hp uart internal memory"]
pub type UART_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to power down hp uart internal memory."]
    #[inline(always)]
    pub fn uart_mem_force_pd(&self) -> UART_MEM_FORCE_PD_R {
        UART_MEM_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up hp uart internal memory"]
    #[inline(always)]
    pub fn uart_mem_force_pu(&self) -> UART_MEM_FORCE_PU_R {
        UART_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_PD_CTRL")
            .field(
                "uart_mem_force_pd",
                &format_args!("{}", self.uart_mem_force_pd().bit()),
            )
            .field(
                "uart_mem_force_pu",
                &format_args!("{}", self.uart_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_PD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power down hp uart internal memory."]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_force_pd(&mut self) -> UART_MEM_FORCE_PD_W<UART_PD_CTRL_SPEC> {
        UART_MEM_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up hp uart internal memory"]
    #[inline(always)]
    #[must_use]
    pub fn uart_mem_force_pu(&mut self) -> UART_MEM_FORCE_PU_W<UART_PD_CTRL_SPEC> {
        UART_MEM_FORCE_PU_W::new(self, 1)
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
#[doc = "ecc pd ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_pd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_pd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_PD_CTRL_SPEC;
impl crate::RegisterSpec for UART_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for UART_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for UART_PD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_PD_CTRL to value 0x02"]
impl crate::Resettable for UART_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
