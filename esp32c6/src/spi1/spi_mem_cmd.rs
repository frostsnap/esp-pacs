#[doc = "Register `SPI_MEM_CMD` reader"]
pub type R = crate::R<SPI_MEM_CMD_SPEC>;
#[doc = "Register `SPI_MEM_CMD` writer"]
pub type W = crate::W<SPI_MEM_CMD_SPEC>;
#[doc = "Field `SPI_MEM_MST_ST` reader - The current status of SPI1 master FSM."]
pub type SPI_MEM_MST_ST_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SLV_ST` reader - The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
pub type SPI_MEM_SLV_ST_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_FLASH_PE` reader - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_PE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PE` writer - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_USR` reader - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_USR_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR` writer - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_USR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_HPM` reader - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_HPM_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_HPM` writer - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_RES` reader - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_RES_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_RES` writer - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_RES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_DP` reader - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_DP_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_DP` writer - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_CE` reader - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_CE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_CE` writer - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_BE` reader - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_BE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_BE` writer - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_SE` reader - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_SE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_SE` writer - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_PP` reader - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type SPI_MEM_FLASH_PP_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PP` writer - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type SPI_MEM_FLASH_PP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_WRSR` reader - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_WRSR_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_WRSR` writer - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_WRSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_RDSR` reader - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_RDSR_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_RDSR` writer - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_RDSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_RDID` reader - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_RDID_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_RDID` writer - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_RDID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_WRDI` reader - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_WRDI_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_WRDI` writer - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_WRDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_WREN` reader - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_WREN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_WREN` writer - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_READ` reader - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_READ_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_READ` writer - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type SPI_MEM_FLASH_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI1 master FSM."]
    #[inline(always)]
    pub fn spi_mem_mst_st(&self) -> SPI_MEM_MST_ST_R {
        SPI_MEM_MST_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    #[inline(always)]
    pub fn spi_mem_slv_st(&self) -> SPI_MEM_SLV_ST_R {
        SPI_MEM_SLV_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_pe(&self) -> SPI_MEM_FLASH_PE_R {
        SPI_MEM_FLASH_PE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_usr(&self) -> SPI_MEM_USR_R {
        SPI_MEM_USR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_hpm(&self) -> SPI_MEM_FLASH_HPM_R {
        SPI_MEM_FLASH_HPM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_res(&self) -> SPI_MEM_FLASH_RES_R {
        SPI_MEM_FLASH_RES_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_dp(&self) -> SPI_MEM_FLASH_DP_R {
        SPI_MEM_FLASH_DP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_ce(&self) -> SPI_MEM_FLASH_CE_R {
        SPI_MEM_FLASH_CE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_be(&self) -> SPI_MEM_FLASH_BE_R {
        SPI_MEM_FLASH_BE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_se(&self) -> SPI_MEM_FLASH_SE_R {
        SPI_MEM_FLASH_SE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_pp(&self) -> SPI_MEM_FLASH_PP_R {
        SPI_MEM_FLASH_PP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_wrsr(&self) -> SPI_MEM_FLASH_WRSR_R {
        SPI_MEM_FLASH_WRSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_rdsr(&self) -> SPI_MEM_FLASH_RDSR_R {
        SPI_MEM_FLASH_RDSR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_rdid(&self) -> SPI_MEM_FLASH_RDID_R {
        SPI_MEM_FLASH_RDID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_wrdi(&self) -> SPI_MEM_FLASH_WRDI_R {
        SPI_MEM_FLASH_WRDI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_wren(&self) -> SPI_MEM_FLASH_WREN_R {
        SPI_MEM_FLASH_WREN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_read(&self) -> SPI_MEM_FLASH_READ_R {
        SPI_MEM_FLASH_READ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CMD")
            .field(
                "spi_mem_mst_st",
                &format_args!("{}", self.spi_mem_mst_st().bits()),
            )
            .field(
                "spi_mem_slv_st",
                &format_args!("{}", self.spi_mem_slv_st().bits()),
            )
            .field(
                "spi_mem_flash_pe",
                &format_args!("{}", self.spi_mem_flash_pe().bit()),
            )
            .field("spi_mem_usr", &format_args!("{}", self.spi_mem_usr().bit()))
            .field(
                "spi_mem_flash_hpm",
                &format_args!("{}", self.spi_mem_flash_hpm().bit()),
            )
            .field(
                "spi_mem_flash_res",
                &format_args!("{}", self.spi_mem_flash_res().bit()),
            )
            .field(
                "spi_mem_flash_dp",
                &format_args!("{}", self.spi_mem_flash_dp().bit()),
            )
            .field(
                "spi_mem_flash_ce",
                &format_args!("{}", self.spi_mem_flash_ce().bit()),
            )
            .field(
                "spi_mem_flash_be",
                &format_args!("{}", self.spi_mem_flash_be().bit()),
            )
            .field(
                "spi_mem_flash_se",
                &format_args!("{}", self.spi_mem_flash_se().bit()),
            )
            .field(
                "spi_mem_flash_pp",
                &format_args!("{}", self.spi_mem_flash_pp().bit()),
            )
            .field(
                "spi_mem_flash_wrsr",
                &format_args!("{}", self.spi_mem_flash_wrsr().bit()),
            )
            .field(
                "spi_mem_flash_rdsr",
                &format_args!("{}", self.spi_mem_flash_rdsr().bit()),
            )
            .field(
                "spi_mem_flash_rdid",
                &format_args!("{}", self.spi_mem_flash_rdid().bit()),
            )
            .field(
                "spi_mem_flash_wrdi",
                &format_args!("{}", self.spi_mem_flash_wrdi().bit()),
            )
            .field(
                "spi_mem_flash_wren",
                &format_args!("{}", self.spi_mem_flash_wren().bit()),
            )
            .field(
                "spi_mem_flash_read",
                &format_args!("{}", self.spi_mem_flash_read().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 17 - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pe(&mut self) -> SPI_MEM_FLASH_PE_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_PE_W::new(self, 17)
    }
    #[doc = "Bit 18 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr(&mut self) -> SPI_MEM_USR_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_USR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_hpm(&mut self) -> SPI_MEM_FLASH_HPM_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_HPM_W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_res(&mut self) -> SPI_MEM_FLASH_RES_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_RES_W::new(self, 20)
    }
    #[doc = "Bit 21 - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_dp(&mut self) -> SPI_MEM_FLASH_DP_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_DP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_ce(&mut self) -> SPI_MEM_FLASH_CE_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_CE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_be(&mut self) -> SPI_MEM_FLASH_BE_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_BE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_se(&mut self) -> SPI_MEM_FLASH_SE_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_SE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pp(&mut self) -> SPI_MEM_FLASH_PP_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_PP_W::new(self, 25)
    }
    #[doc = "Bit 26 - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_wrsr(&mut self) -> SPI_MEM_FLASH_WRSR_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_WRSR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_rdsr(&mut self) -> SPI_MEM_FLASH_RDSR_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_RDSR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_rdid(&mut self) -> SPI_MEM_FLASH_RDID_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_RDID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_wrdi(&mut self) -> SPI_MEM_FLASH_WRDI_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_WRDI_W::new(self, 29)
    }
    #[doc = "Bit 30 - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_wren(&mut self) -> SPI_MEM_FLASH_WREN_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_WREN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_read(&mut self) -> SPI_MEM_FLASH_READ_W<SPI_MEM_CMD_SPEC> {
        SPI_MEM_FLASH_READ_W::new(self, 31)
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
#[doc = "SPI1 memory command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CMD_SPEC;
impl crate::RegisterSpec for SPI_MEM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_cmd::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_cmd::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CMD to value 0"]
impl crate::Resettable for SPI_MEM_CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
