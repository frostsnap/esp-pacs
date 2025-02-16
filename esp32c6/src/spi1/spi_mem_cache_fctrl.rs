#[doc = "Register `SPI_MEM_CACHE_FCTRL` reader"]
pub type R = crate::R<SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "Register `SPI_MEM_CACHE_FCTRL` writer"]
pub type W = crate::W<SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "Field `SPI_MEM_CACHE_USR_ADDR_4BYTE` reader - For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_USR_ADDR_4BYTE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CACHE_USR_ADDR_4BYTE` writer - For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_USR_ADDR_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDIN_DUAL` reader - For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDIN_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDIN_DUAL` writer - For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDOUT_DUAL` reader - For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDOUT_DUAL` writer - For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FADDR_DUAL` reader - For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FADDR_DUAL` writer - For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDIN_QUAD` reader - For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDIN_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDIN_QUAD` writer - For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FDOUT_QUAD` reader - For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDOUT_QUAD` writer - For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FADDR_QUAD` reader - For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FADDR_QUAD` writer - For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_usr_addr_4byte(&self) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_R {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdin_dual(&self) -> SPI_MEM_FDIN_DUAL_R {
        SPI_MEM_FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdout_dual(&self) -> SPI_MEM_FDOUT_DUAL_R {
        SPI_MEM_FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_faddr_dual(&self) -> SPI_MEM_FADDR_DUAL_R {
        SPI_MEM_FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdin_quad(&self) -> SPI_MEM_FDIN_QUAD_R {
        SPI_MEM_FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdout_quad(&self) -> SPI_MEM_FDOUT_QUAD_R {
        SPI_MEM_FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_faddr_quad(&self) -> SPI_MEM_FADDR_QUAD_R {
        SPI_MEM_FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CACHE_FCTRL")
            .field(
                "spi_mem_cache_usr_addr_4byte",
                &format_args!("{}", self.spi_mem_cache_usr_addr_4byte().bit()),
            )
            .field(
                "spi_mem_fdin_dual",
                &format_args!("{}", self.spi_mem_fdin_dual().bit()),
            )
            .field(
                "spi_mem_fdout_dual",
                &format_args!("{}", self.spi_mem_fdout_dual().bit()),
            )
            .field(
                "spi_mem_faddr_dual",
                &format_args!("{}", self.spi_mem_faddr_dual().bit()),
            )
            .field(
                "spi_mem_fdin_quad",
                &format_args!("{}", self.spi_mem_fdin_quad().bit()),
            )
            .field(
                "spi_mem_fdout_quad",
                &format_args!("{}", self.spi_mem_fdout_quad().bit()),
            )
            .field(
                "spi_mem_faddr_quad",
                &format_args!("{}", self.spi_mem_faddr_quad().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CACHE_FCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cache_usr_addr_4byte(
        &mut self,
    ) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_W<SPI_MEM_CACHE_FCTRL_SPEC> {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_W::new(self, 1)
    }
    #[doc = "Bit 3 - For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdin_dual(&mut self) -> SPI_MEM_FDIN_DUAL_W<SPI_MEM_CACHE_FCTRL_SPEC> {
        SPI_MEM_FDIN_DUAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdout_dual(&mut self) -> SPI_MEM_FDOUT_DUAL_W<SPI_MEM_CACHE_FCTRL_SPEC> {
        SPI_MEM_FDOUT_DUAL_W::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_faddr_dual(&mut self) -> SPI_MEM_FADDR_DUAL_W<SPI_MEM_CACHE_FCTRL_SPEC> {
        SPI_MEM_FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdin_quad(&mut self) -> SPI_MEM_FDIN_QUAD_W<SPI_MEM_CACHE_FCTRL_SPEC> {
        SPI_MEM_FDIN_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 7 - For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdout_quad(&mut self) -> SPI_MEM_FDOUT_QUAD_W<SPI_MEM_CACHE_FCTRL_SPEC> {
        SPI_MEM_FDOUT_QUAD_W::new(self, 7)
    }
    #[doc = "Bit 8 - For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_faddr_quad(&mut self) -> SPI_MEM_FADDR_QUAD_W<SPI_MEM_CACHE_FCTRL_SPEC> {
        SPI_MEM_FADDR_QUAD_W::new(self, 8)
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
#[doc = "SPI1 bit mode control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_cache_fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_cache_fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_cache_fctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_CACHE_FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_cache_fctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CACHE_FCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CACHE_FCTRL to value 0"]
impl crate::Resettable for SPI_MEM_CACHE_FCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
