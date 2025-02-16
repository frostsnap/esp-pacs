#[doc = "Register `IN_POP_CH%s` reader"]
pub type R = crate::R<IN_POP_CH_SPEC>;
#[doc = "Register `IN_POP_CH%s` writer"]
pub type W = crate::W<IN_POP_CH_SPEC>;
#[doc = "Field `INFIFO_RDATA_CH` reader - This register stores the data popping from AXI_DMA FIFO."]
pub type INFIFO_RDATA_CH_R = crate::FieldReader<u16>;
#[doc = "Field `INFIFO_POP_CH` writer - Set this bit to pop data from AXI_DMA FIFO."]
pub type INFIFO_POP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - This register stores the data popping from AXI_DMA FIFO."]
    #[inline(always)]
    pub fn infifo_rdata_ch(&self) -> INFIFO_RDATA_CH_R {
        INFIFO_RDATA_CH_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_POP_CH")
            .field(
                "infifo_rdata_ch",
                &format_args!("{}", self.infifo_rdata_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_POP_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to pop data from AXI_DMA FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_pop_ch(&mut self) -> INFIFO_POP_CH_W<IN_POP_CH_SPEC> {
        INFIFO_POP_CH_W::new(self, 12)
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
#[doc = "Pop control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_POP_CH_SPEC;
impl crate::RegisterSpec for IN_POP_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pop_ch::R`](R) reader structure"]
impl crate::Readable for IN_POP_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_pop_ch::W`](W) writer structure"]
impl crate::Writable for IN_POP_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_POP_CH%s to value 0x0800"]
impl crate::Resettable for IN_POP_CH_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
