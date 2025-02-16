#[doc = "Register `SPI_MEM_USER1` reader"]
pub type R = crate::R<SPI_MEM_USER1_SPEC>;
#[doc = "Register `SPI_MEM_USER1` writer"]
pub type W = crate::W<SPI_MEM_USER1_SPEC>;
#[doc = "Field `SPI_MEM_USR_DUMMY_CYCLELEN` reader - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type SPI_MEM_USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_USR_DUMMY_CYCLELEN` writer - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type SPI_MEM_USR_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_MEM_USR_DBYTELEN` reader - SPI0 USR_CMD read or write data byte length -1"]
pub type SPI_MEM_USR_DBYTELEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_cyclelen(&self) -> SPI_MEM_USR_DUMMY_CYCLELEN_R {
        SPI_MEM_USR_DUMMY_CYCLELEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - SPI0 USR_CMD read or write data byte length -1"]
    #[inline(always)]
    pub fn spi_mem_usr_dbytelen(&self) -> SPI_MEM_USR_DBYTELEN_R {
        SPI_MEM_USR_DBYTELEN_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_addr_bitlen(&self) -> SPI_MEM_USR_ADDR_BITLEN_R {
        SPI_MEM_USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_USER1")
            .field(
                "spi_mem_usr_dummy_cyclelen",
                &format_args!("{}", self.spi_mem_usr_dummy_cyclelen().bits()),
            )
            .field(
                "spi_mem_usr_dbytelen",
                &format_args!("{}", self.spi_mem_usr_dbytelen().bits()),
            )
            .field(
                "spi_mem_usr_addr_bitlen",
                &format_args!("{}", self.spi_mem_usr_addr_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_USER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_dummy_cyclelen(
        &mut self,
    ) -> SPI_MEM_USR_DUMMY_CYCLELEN_W<SPI_MEM_USER1_SPEC> {
        SPI_MEM_USR_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_addr_bitlen(&mut self) -> SPI_MEM_USR_ADDR_BITLEN_W<SPI_MEM_USER1_SPEC> {
        SPI_MEM_USR_ADDR_BITLEN_W::new(self, 26)
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
#[doc = "SPI0 user1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_USER1_SPEC;
impl crate::RegisterSpec for SPI_MEM_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_user1::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_USER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_user1::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_USER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_USER1 to value 0x5c00_0047"]
impl crate::Resettable for SPI_MEM_USER1_SPEC {
    const RESET_VALUE: u32 = 0x5c00_0047;
}
