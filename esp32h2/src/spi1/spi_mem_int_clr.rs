#[doc = "Register `SPI_MEM_INT_CLR` writer"]
pub type W = crate::W<SPI_MEM_INT_CLR_SPEC>;
#[doc = "Field `SPI_MEM_PER_END_INT_CLR` writer - The clear bit for SPI_MEM_PER_END_INT interrupt."]
pub type SPI_MEM_PER_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PES_END_INT_CLR` writer - The clear bit for SPI_MEM_PES_END_INT interrupt."]
pub type SPI_MEM_PES_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_WPE_END_INT_CLR` writer - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
pub type SPI_MEM_WPE_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_BROWN_OUT_INT_CLR` writer - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type SPI_MEM_BROWN_OUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_per_end_int_clr(&mut self) -> SPI_MEM_PER_END_INT_CLR_W<SPI_MEM_INT_CLR_SPEC> {
        SPI_MEM_PER_END_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The clear bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pes_end_int_clr(&mut self) -> SPI_MEM_PES_END_INT_CLR_W<SPI_MEM_INT_CLR_SPEC> {
        SPI_MEM_PES_END_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wpe_end_int_clr(&mut self) -> SPI_MEM_WPE_END_INT_CLR_W<SPI_MEM_INT_CLR_SPEC> {
        SPI_MEM_WPE_END_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_slv_st_end_int_clr(
        &mut self,
    ) -> SPI_MEM_SLV_ST_END_INT_CLR_W<SPI_MEM_INT_CLR_SPEC> {
        SPI_MEM_SLV_ST_END_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_mst_st_end_int_clr(
        &mut self,
    ) -> SPI_MEM_MST_ST_END_INT_CLR_W<SPI_MEM_INT_CLR_SPEC> {
        SPI_MEM_MST_ST_END_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 10 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_brown_out_int_clr(
        &mut self,
    ) -> SPI_MEM_BROWN_OUT_INT_CLR_W<SPI_MEM_INT_CLR_SPEC> {
        SPI_MEM_BROWN_OUT_INT_CLR_W::new(self, 10)
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
#[doc = "SPI1 interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_INT_CLR_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mem_int_clr::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_INT_CLR to value 0"]
impl crate::Resettable for SPI_MEM_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
