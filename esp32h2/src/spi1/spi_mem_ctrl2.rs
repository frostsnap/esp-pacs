#[doc = "Register `SPI_MEM_CTRL2` writer"]
pub type W = crate::W<SPI_MEM_CTRL2_SPEC>;
#[doc = "Field `SPI_MEM_SYNC_RESET` writer - The FSM will be reset."]
pub type SPI_MEM_SYNC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sync_reset(&mut self) -> SPI_MEM_SYNC_RESET_W<SPI_MEM_CTRL2_SPEC> {
        SPI_MEM_SYNC_RESET_W::new(self, 31)
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
#[doc = "SPI1 control2 register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CTRL2_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mem_ctrl2::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL2 to value 0"]
impl crate::Resettable for SPI_MEM_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
