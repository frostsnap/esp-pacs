#[doc = "Register `HCTSIZ3` reader"]
pub type R = crate::R<HCTSIZ3_SPEC>;
#[doc = "Register `HCTSIZ3` writer"]
pub type W = crate::W<HCTSIZ3_SPEC>;
#[doc = "Field `H_XFERSIZE3` reader - "]
pub type H_XFERSIZE3_R = crate::FieldReader<u32>;
#[doc = "Field `H_XFERSIZE3` writer - "]
pub type H_XFERSIZE3_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `H_PKTCNT3` reader - "]
pub type H_PKTCNT3_R = crate::FieldReader<u16>;
#[doc = "Field `H_PKTCNT3` writer - "]
pub type H_PKTCNT3_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `H_PID3` reader - "]
pub type H_PID3_R = crate::FieldReader;
#[doc = "Field `H_PID3` writer - "]
pub type H_PID3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_DOPNG3` reader - "]
pub type H_DOPNG3_R = crate::BitReader;
#[doc = "Field `H_DOPNG3` writer - "]
pub type H_DOPNG3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize3(&self) -> H_XFERSIZE3_R {
        H_XFERSIZE3_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt3(&self) -> H_PKTCNT3_R {
        H_PKTCNT3_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid3(&self) -> H_PID3_R {
        H_PID3_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng3(&self) -> H_DOPNG3_R {
        H_DOPNG3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ3")
            .field(
                "h_xfersize3",
                &format_args!("{}", self.h_xfersize3().bits()),
            )
            .field("h_pktcnt3", &format_args!("{}", self.h_pktcnt3().bits()))
            .field("h_pid3", &format_args!("{}", self.h_pid3().bits()))
            .field("h_dopng3", &format_args!("{}", self.h_dopng3().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCTSIZ3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfersize3(&mut self) -> H_XFERSIZE3_W<HCTSIZ3_SPEC> {
        H_XFERSIZE3_W::new(self, 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_pktcnt3(&mut self) -> H_PKTCNT3_W<HCTSIZ3_SPEC> {
        H_PKTCNT3_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn h_pid3(&mut self) -> H_PID3_W<HCTSIZ3_SPEC> {
        H_PID3_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dopng3(&mut self) -> H_DOPNG3_W<HCTSIZ3_SPEC> {
        H_DOPNG3_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ3_SPEC;
impl crate::RegisterSpec for HCTSIZ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz3::R`](R) reader structure"]
impl crate::Readable for HCTSIZ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz3::W`](W) writer structure"]
impl crate::Writable for HCTSIZ3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ3 to value 0"]
impl crate::Resettable for HCTSIZ3_SPEC {
    const RESET_VALUE: u32 = 0;
}
