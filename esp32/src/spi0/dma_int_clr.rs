#[doc = "Register `DMA_INT_CLR` reader"]
pub type R = crate::R<DMA_INT_CLR_SPEC>;
#[doc = "Register `DMA_INT_CLR` writer"]
pub type W = crate::W<DMA_INT_CLR_SPEC>;
#[doc = "Field `INLINK_DSCR_EMPTY_INT_CLR` reader - The clear bit for lack of enough inlink descriptors."]
pub type INLINK_DSCR_EMPTY_INT_CLR_R = crate::BitReader;
#[doc = "Field `INLINK_DSCR_EMPTY_INT_CLR` writer - The clear bit for lack of enough inlink descriptors."]
pub type INLINK_DSCR_EMPTY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_CLR` reader - The clear bit for outlink descriptor error."]
pub type OUTLINK_DSCR_ERROR_INT_CLR_R = crate::BitReader;
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_CLR` writer - The clear bit for outlink descriptor error."]
pub type OUTLINK_DSCR_ERROR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_DSCR_ERROR_INT_CLR` reader - The clear bit for inlink descriptor error."]
pub type INLINK_DSCR_ERROR_INT_CLR_R = crate::BitReader;
#[doc = "Field `INLINK_DSCR_ERROR_INT_CLR` writer - The clear bit for inlink descriptor error."]
pub type INLINK_DSCR_ERROR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DONE_INT_CLR` reader - The clear bit for completing usage of a inlink descriptor."]
pub type IN_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_CLR` writer - The clear bit for completing usage of a inlink descriptor."]
pub type IN_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_INT_CLR` reader - The clear bit for receiving error."]
pub type IN_ERR_EOF_INT_CLR_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_CLR` writer - The clear bit for receiving error."]
pub type IN_ERR_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_INT_CLR` reader - The clear bit for completing receiving all the packets from host."]
pub type IN_SUC_EOF_INT_CLR_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_CLR` writer - The clear bit for completing receiving all the packets from host."]
pub type IN_SUC_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DONE_INT_CLR` reader - The clear bit for completing usage of a outlink descriptor."]
pub type OUT_DONE_INT_CLR_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_CLR` writer - The clear bit for completing usage of a outlink descriptor."]
pub type OUT_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_CLR` reader - The clear bit for sending a packet to host done."]
pub type OUT_EOF_INT_CLR_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_CLR` writer - The clear bit for sending a packet to host done."]
pub type OUT_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` reader - The clear bit for sending all the packets to host done."]
pub type OUT_TOTAL_EOF_INT_CLR_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` writer - The clear bit for sending all the packets to host done."]
pub type OUT_TOTAL_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The clear bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_clr(&self) -> INLINK_DSCR_EMPTY_INT_CLR_R {
        INLINK_DSCR_EMPTY_INT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The clear bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_clr(&self) -> OUTLINK_DSCR_ERROR_INT_CLR_R {
        OUTLINK_DSCR_ERROR_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The clear bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_clr(&self) -> INLINK_DSCR_ERROR_INT_CLR_R {
        INLINK_DSCR_ERROR_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The clear bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_clr(&self) -> IN_DONE_INT_CLR_R {
        IN_DONE_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The clear bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&self) -> IN_ERR_EOF_INT_CLR_R {
        IN_ERR_EOF_INT_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The clear bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&self) -> IN_SUC_EOF_INT_CLR_R {
        IN_SUC_EOF_INT_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The clear bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_clr(&self) -> OUT_DONE_INT_CLR_R {
        OUT_DONE_INT_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The clear bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_clr(&self) -> OUT_EOF_INT_CLR_R {
        OUT_EOF_INT_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The clear bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&self) -> OUT_TOTAL_EOF_INT_CLR_R {
        OUT_TOTAL_EOF_INT_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_CLR")
            .field(
                "inlink_dscr_empty_int_clr",
                &format_args!("{}", self.inlink_dscr_empty_int_clr().bit()),
            )
            .field(
                "outlink_dscr_error_int_clr",
                &format_args!("{}", self.outlink_dscr_error_int_clr().bit()),
            )
            .field(
                "inlink_dscr_error_int_clr",
                &format_args!("{}", self.inlink_dscr_error_int_clr().bit()),
            )
            .field(
                "in_done_int_clr",
                &format_args!("{}", self.in_done_int_clr().bit()),
            )
            .field(
                "in_err_eof_int_clr",
                &format_args!("{}", self.in_err_eof_int_clr().bit()),
            )
            .field(
                "in_suc_eof_int_clr",
                &format_args!("{}", self.in_suc_eof_int_clr().bit()),
            )
            .field(
                "out_done_int_clr",
                &format_args!("{}", self.out_done_int_clr().bit()),
            )
            .field(
                "out_eof_int_clr",
                &format_args!("{}", self.out_eof_int_clr().bit()),
            )
            .field(
                "out_total_eof_int_clr",
                &format_args!("{}", self.out_total_eof_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for lack of enough inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_dscr_empty_int_clr(&mut self) -> INLINK_DSCR_EMPTY_INT_CLR_W<DMA_INT_CLR_SPEC> {
        INLINK_DSCR_EMPTY_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The clear bit for outlink descriptor error."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_dscr_error_int_clr(&mut self) -> OUTLINK_DSCR_ERROR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        OUTLINK_DSCR_ERROR_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The clear bit for inlink descriptor error."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_dscr_error_int_clr(&mut self) -> INLINK_DSCR_ERROR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        INLINK_DSCR_ERROR_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_int_clr(&mut self) -> IN_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        IN_DONE_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for receiving error."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_int_clr(&mut self) -> IN_ERR_EOF_INT_CLR_W<DMA_INT_CLR_SPEC> {
        IN_ERR_EOF_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The clear bit for completing receiving all the packets from host."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_int_clr(&mut self) -> IN_SUC_EOF_INT_CLR_W<DMA_INT_CLR_SPEC> {
        IN_SUC_EOF_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The clear bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_int_clr(&mut self) -> OUT_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        OUT_DONE_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The clear bit for sending a packet to host done."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W<DMA_INT_CLR_SPEC> {
        OUT_EOF_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The clear bit for sending all the packets to host done."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_int_clr(&mut self) -> OUT_TOTAL_EOF_INT_CLR_W<DMA_INT_CLR_SPEC> {
        OUT_TOTAL_EOF_INT_CLR_W::new(self, 8)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_clr::R`](R) reader structure"]
impl crate::Readable for DMA_INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_int_clr::W`](W) writer structure"]
impl crate::Writable for DMA_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INT_CLR to value 0"]
impl crate::Resettable for DMA_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
