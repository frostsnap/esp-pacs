#[doc = "Register `INTR_ENA` reader"]
pub type R = crate::R<INTR_ENA_SPEC>;
#[doc = "Register `INTR_ENA` writer"]
pub type W = crate::W<INTR_ENA_SPEC>;
#[doc = "Field `FIFO_OVERFLOW_INTR_ENA` reader - Set 1 enable fifo_overflow interrupt"]
pub type FIFO_OVERFLOW_INTR_ENA_R = crate::BitReader;
#[doc = "Field `FIFO_OVERFLOW_INTR_ENA` writer - Set 1 enable fifo_overflow interrupt"]
pub type FIFO_OVERFLOW_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FULL_INTR_ENA` reader - Set 1 enable mem_full interrupt"]
pub type MEM_FULL_INTR_ENA_R = crate::BitReader;
#[doc = "Field `MEM_FULL_INTR_ENA` writer - Set 1 enable mem_full interrupt"]
pub type MEM_FULL_INTR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 enable fifo_overflow interrupt"]
    #[inline(always)]
    pub fn fifo_overflow_intr_ena(&self) -> FIFO_OVERFLOW_INTR_ENA_R {
        FIFO_OVERFLOW_INTR_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 enable mem_full interrupt"]
    #[inline(always)]
    pub fn mem_full_intr_ena(&self) -> MEM_FULL_INTR_ENA_R {
        MEM_FULL_INTR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_ENA")
            .field(
                "fifo_overflow_intr_ena",
                &format_args!("{}", self.fifo_overflow_intr_ena().bit()),
            )
            .field(
                "mem_full_intr_ena",
                &format_args!("{}", self.mem_full_intr_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 enable fifo_overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow_intr_ena(&mut self) -> FIFO_OVERFLOW_INTR_ENA_W<INTR_ENA_SPEC> {
        FIFO_OVERFLOW_INTR_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 enable mem_full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mem_full_intr_ena(&mut self) -> MEM_FULL_INTR_ENA_W<INTR_ENA_SPEC> {
        MEM_FULL_INTR_ENA_W::new(self, 1)
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
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_ENA_SPEC;
impl crate::RegisterSpec for INTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_ena::R`](R) reader structure"]
impl crate::Readable for INTR_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_ena::W`](W) writer structure"]
impl crate::Writable for INTR_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_ENA to value 0"]
impl crate::Resettable for INTR_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
