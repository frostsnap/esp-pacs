#[doc = "Register `DMA_INT_ENA` reader"]
pub type R = crate::R<DMA_INT_ENA_SPEC>;
#[doc = "Register `DMA_INT_ENA` writer"]
pub type W = crate::W<DMA_INT_ENA_SPEC>;
#[doc = "Field `INLINK_DSCR_EMPTY_INT_ENA` reader - The enable bit for lack of enough inlink descriptors. Can be configured in CONF state."]
pub type INLINK_DSCR_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `INLINK_DSCR_EMPTY_INT_ENA` writer - The enable bit for lack of enough inlink descriptors. Can be configured in CONF state."]
pub type INLINK_DSCR_EMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_ENA` reader - The enable bit for outlink descriptor error. Can be configured in CONF state."]
pub type OUTLINK_DSCR_ERROR_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_ENA` writer - The enable bit for outlink descriptor error. Can be configured in CONF state."]
pub type OUTLINK_DSCR_ERROR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_DSCR_ERROR_INT_ENA` reader - The enable bit for inlink descriptor error. Can be configured in CONF state."]
pub type INLINK_DSCR_ERROR_INT_ENA_R = crate::BitReader;
#[doc = "Field `INLINK_DSCR_ERROR_INT_ENA` writer - The enable bit for inlink descriptor error. Can be configured in CONF state."]
pub type INLINK_DSCR_ERROR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DONE_INT_ENA` reader - The enable bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
pub type IN_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_ENA` writer - The enable bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
pub type IN_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_INT_ENA` reader - The enable bit for receiving error. Can be configured in CONF state."]
pub type IN_ERR_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_ENA` writer - The enable bit for receiving error. Can be configured in CONF state."]
pub type IN_ERR_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_INT_ENA` reader - The enable bit for completing receiving all the packets from host. Can be configured in CONF state."]
pub type IN_SUC_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_ENA` writer - The enable bit for completing receiving all the packets from host. Can be configured in CONF state."]
pub type IN_SUC_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DONE_INT_ENA` reader - The enable bit for completing usage of a outlink descriptor . Can be configured in CONF state."]
pub type OUT_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_ENA` writer - The enable bit for completing usage of a outlink descriptor . Can be configured in CONF state."]
pub type OUT_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_ENA` reader - The enable bit for sending a packet to host done. Can be configured in CONF state."]
pub type OUT_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_ENA` writer - The enable bit for sending a packet to host done. Can be configured in CONF state."]
pub type OUT_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` reader - The enable bit for sending all the packets to host done. Can be configured in CONF state."]
pub type OUT_TOTAL_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` writer - The enable bit for sending all the packets to host done. Can be configured in CONF state."]
pub type OUT_TOTAL_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_FULL_ERR_INT_ENA` reader - The enable bit for infifo full error interrupt."]
pub type INFIFO_FULL_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_ERR_INT_ENA` writer - The enable bit for infifo full error interrupt."]
pub type INFIFO_FULL_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_EMPTY_ERR_INT_ENA` reader - The enable bit for outfifo empty error interrupt."]
pub type OUTFIFO_EMPTY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_ERR_INT_ENA` writer - The enable bit for outfifo empty error interrupt."]
pub type OUTFIFO_EMPTY_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD6_INT_ENA` reader - The enable bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD6_INT_ENA` writer - The enable bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD7_INT_ENA` reader - The enable bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD7_INT_ENA` writer - The enable bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD8_INT_ENA` reader - The enable bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD8_INT_ENA` writer - The enable bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD9_INT_ENA` reader - The enable bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD9_INT_ENA` writer - The enable bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMDA_INT_ENA` reader - The enable bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMDA_INT_ENA` writer - The enable bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable bit for lack of enough inlink descriptors. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_ena(&self) -> INLINK_DSCR_EMPTY_INT_ENA_R {
        INLINK_DSCR_EMPTY_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for outlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_ena(&self) -> OUTLINK_DSCR_ERROR_INT_ENA_R {
        OUTLINK_DSCR_ERROR_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for inlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_ena(&self) -> INLINK_DSCR_ERROR_INT_ENA_R {
        INLINK_DSCR_ERROR_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_done_int_ena(&self) -> IN_DONE_INT_ENA_R {
        IN_DONE_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for receiving error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_err_eof_int_ena(&self) -> IN_ERR_EOF_INT_ENA_R {
        IN_ERR_EOF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for completing receiving all the packets from host. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_suc_eof_int_ena(&self) -> IN_SUC_EOF_INT_ENA_R {
        IN_SUC_EOF_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for completing usage of a outlink descriptor . Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_done_int_ena(&self) -> OUT_DONE_INT_ENA_R {
        OUT_DONE_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for sending a packet to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_eof_int_ena(&self) -> OUT_EOF_INT_ENA_R {
        OUT_EOF_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for sending all the packets to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_total_eof_int_ena(&self) -> OUT_TOTAL_EOF_INT_ENA_R {
        OUT_TOTAL_EOF_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for infifo full error interrupt."]
    #[inline(always)]
    pub fn infifo_full_err_int_ena(&self) -> INFIFO_FULL_ERR_INT_ENA_R {
        INFIFO_FULL_ERR_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The enable bit for outfifo empty error interrupt."]
    #[inline(always)]
    pub fn outfifo_empty_err_int_ena(&self) -> OUTFIFO_EMPTY_ERR_INT_ENA_R {
        OUTFIFO_EMPTY_ERR_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The enable bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6_int_ena(&self) -> SLV_CMD6_INT_ENA_R {
        SLV_CMD6_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_ena(&self) -> SLV_CMD7_INT_ENA_R {
        SLV_CMD7_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The enable bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_ena(&self) -> SLV_CMD8_INT_ENA_R {
        SLV_CMD8_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The enable bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_ena(&self) -> SLV_CMD9_INT_ENA_R {
        SLV_CMD9_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The enable bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_ena(&self) -> SLV_CMDA_INT_ENA_R {
        SLV_CMDA_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ENA")
            .field(
                "inlink_dscr_empty_int_ena",
                &format_args!("{}", self.inlink_dscr_empty_int_ena().bit()),
            )
            .field(
                "outlink_dscr_error_int_ena",
                &format_args!("{}", self.outlink_dscr_error_int_ena().bit()),
            )
            .field(
                "inlink_dscr_error_int_ena",
                &format_args!("{}", self.inlink_dscr_error_int_ena().bit()),
            )
            .field(
                "in_done_int_ena",
                &format_args!("{}", self.in_done_int_ena().bit()),
            )
            .field(
                "in_err_eof_int_ena",
                &format_args!("{}", self.in_err_eof_int_ena().bit()),
            )
            .field(
                "in_suc_eof_int_ena",
                &format_args!("{}", self.in_suc_eof_int_ena().bit()),
            )
            .field(
                "out_done_int_ena",
                &format_args!("{}", self.out_done_int_ena().bit()),
            )
            .field(
                "out_eof_int_ena",
                &format_args!("{}", self.out_eof_int_ena().bit()),
            )
            .field(
                "out_total_eof_int_ena",
                &format_args!("{}", self.out_total_eof_int_ena().bit()),
            )
            .field(
                "infifo_full_err_int_ena",
                &format_args!("{}", self.infifo_full_err_int_ena().bit()),
            )
            .field(
                "outfifo_empty_err_int_ena",
                &format_args!("{}", self.outfifo_empty_err_int_ena().bit()),
            )
            .field(
                "slv_cmd6_int_ena",
                &format_args!("{}", self.slv_cmd6_int_ena().bit()),
            )
            .field(
                "slv_cmd7_int_ena",
                &format_args!("{}", self.slv_cmd7_int_ena().bit()),
            )
            .field(
                "slv_cmd8_int_ena",
                &format_args!("{}", self.slv_cmd8_int_ena().bit()),
            )
            .field(
                "slv_cmd9_int_ena",
                &format_args!("{}", self.slv_cmd9_int_ena().bit()),
            )
            .field(
                "slv_cmda_int_ena",
                &format_args!("{}", self.slv_cmda_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for lack of enough inlink descriptors. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_dscr_empty_int_ena(&mut self) -> INLINK_DSCR_EMPTY_INT_ENA_W<DMA_INT_ENA_SPEC> {
        INLINK_DSCR_EMPTY_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable bit for outlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_dscr_error_int_ena(&mut self) -> OUTLINK_DSCR_ERROR_INT_ENA_W<DMA_INT_ENA_SPEC> {
        OUTLINK_DSCR_ERROR_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The enable bit for inlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_dscr_error_int_ena(&mut self) -> INLINK_DSCR_ERROR_INT_ENA_W<DMA_INT_ENA_SPEC> {
        INLINK_DSCR_ERROR_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The enable bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_int_ena(&mut self) -> IN_DONE_INT_ENA_W<DMA_INT_ENA_SPEC> {
        IN_DONE_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for receiving error. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_int_ena(&mut self) -> IN_ERR_EOF_INT_ENA_W<DMA_INT_ENA_SPEC> {
        IN_ERR_EOF_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The enable bit for completing receiving all the packets from host. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_int_ena(&mut self) -> IN_SUC_EOF_INT_ENA_W<DMA_INT_ENA_SPEC> {
        IN_SUC_EOF_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The enable bit for completing usage of a outlink descriptor . Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_int_ena(&mut self) -> OUT_DONE_INT_ENA_W<DMA_INT_ENA_SPEC> {
        OUT_DONE_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The enable bit for sending a packet to host done. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_ena(&mut self) -> OUT_EOF_INT_ENA_W<DMA_INT_ENA_SPEC> {
        OUT_EOF_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The enable bit for sending all the packets to host done. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_int_ena(&mut self) -> OUT_TOTAL_EOF_INT_ENA_W<DMA_INT_ENA_SPEC> {
        OUT_TOTAL_EOF_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The enable bit for infifo full error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_full_err_int_ena(&mut self) -> INFIFO_FULL_ERR_INT_ENA_W<DMA_INT_ENA_SPEC> {
        INFIFO_FULL_ERR_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The enable bit for outfifo empty error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_empty_err_int_ena(&mut self) -> OUTFIFO_EMPTY_ERR_INT_ENA_W<DMA_INT_ENA_SPEC> {
        OUTFIFO_EMPTY_ERR_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The enable bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd6_int_ena(&mut self) -> SLV_CMD6_INT_ENA_W<DMA_INT_ENA_SPEC> {
        SLV_CMD6_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The enable bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd7_int_ena(&mut self) -> SLV_CMD7_INT_ENA_W<DMA_INT_ENA_SPEC> {
        SLV_CMD7_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The enable bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd8_int_ena(&mut self) -> SLV_CMD8_INT_ENA_W<DMA_INT_ENA_SPEC> {
        SLV_CMD8_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The enable bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd9_int_ena(&mut self) -> SLV_CMD9_INT_ENA_W<DMA_INT_ENA_SPEC> {
        SLV_CMD9_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The enable bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmda_int_ena(&mut self) -> SLV_CMDA_INT_ENA_W<DMA_INT_ENA_SPEC> {
        SLV_CMDA_INT_ENA_W::new(self, 15)
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
#[doc = "SPI DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_ENA_SPEC;
impl crate::RegisterSpec for DMA_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_ena::R`](R) reader structure"]
impl crate::Readable for DMA_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_int_ena::W`](W) writer structure"]
impl crate::Writable for DMA_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INT_ENA to value 0"]
impl crate::Resettable for DMA_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
