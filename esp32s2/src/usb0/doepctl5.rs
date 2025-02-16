#[doc = "Register `DOEPCTL5` reader"]
pub type R = crate::R<DOEPCTL5_SPEC>;
#[doc = "Register `DOEPCTL5` writer"]
pub type W = crate::W<DOEPCTL5_SPEC>;
#[doc = "Field `MPS5` reader - "]
pub type MPS5_R = crate::FieldReader<u16>;
#[doc = "Field `USBACTEP5` reader - "]
pub type USBACTEP5_R = crate::BitReader;
#[doc = "Field `NAKSTS5` reader - "]
pub type NAKSTS5_R = crate::BitReader;
#[doc = "Field `EPTYPE5` reader - "]
pub type EPTYPE5_R = crate::FieldReader;
#[doc = "Field `SNP5` reader - "]
pub type SNP5_R = crate::BitReader;
#[doc = "Field `SNP5` writer - "]
pub type SNP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL5` reader - "]
pub type STALL5_R = crate::BitReader;
#[doc = "Field `STALL5` writer - "]
pub type STALL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK5` writer - "]
pub type CNAK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_SNAK5` writer - "]
pub type DO_SNAK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_SETD0PID5` writer - "]
pub type DO_SETD0PID5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_SETD1PID5` writer - "]
pub type DO_SETD1PID5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS5` reader - "]
pub type EPDIS5_R = crate::BitReader;
#[doc = "Field `EPENA5` reader - "]
pub type EPENA5_R = crate::BitReader;
#[doc = "Field `EPENA5` writer - "]
pub type EPENA5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps5(&self) -> MPS5_R {
        MPS5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep5(&self) -> USBACTEP5_R {
        USBACTEP5_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts5(&self) -> NAKSTS5_R {
        NAKSTS5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype5(&self) -> EPTYPE5_R {
        EPTYPE5_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp5(&self) -> SNP5_R {
        SNP5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall5(&self) -> STALL5_R {
        STALL5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis5(&self) -> EPDIS5_R {
        EPDIS5_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena5(&self) -> EPENA5_R {
        EPENA5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL5")
            .field("mps5", &format_args!("{}", self.mps5().bits()))
            .field("usbactep5", &format_args!("{}", self.usbactep5().bit()))
            .field("naksts5", &format_args!("{}", self.naksts5().bit()))
            .field("eptype5", &format_args!("{}", self.eptype5().bits()))
            .field("snp5", &format_args!("{}", self.snp5().bit()))
            .field("stall5", &format_args!("{}", self.stall5().bit()))
            .field("epdis5", &format_args!("{}", self.epdis5().bit()))
            .field("epena5", &format_args!("{}", self.epena5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp5(&mut self) -> SNP5_W<DOEPCTL5_SPEC> {
        SNP5_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall5(&mut self) -> STALL5_W<DOEPCTL5_SPEC> {
        STALL5_W::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak5(&mut self) -> CNAK5_W<DOEPCTL5_SPEC> {
        CNAK5_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak5(&mut self) -> DO_SNAK5_W<DOEPCTL5_SPEC> {
        DO_SNAK5_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd0pid5(&mut self) -> DO_SETD0PID5_W<DOEPCTL5_SPEC> {
        DO_SETD0PID5_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd1pid5(&mut self) -> DO_SETD1PID5_W<DOEPCTL5_SPEC> {
        DO_SETD1PID5_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena5(&mut self) -> EPENA5_W<DOEPCTL5_SPEC> {
        EPENA5_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL5_SPEC;
impl crate::RegisterSpec for DOEPCTL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl5::R`](R) reader structure"]
impl crate::Readable for DOEPCTL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl5::W`](W) writer structure"]
impl crate::Writable for DOEPCTL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL5 to value 0x8000"]
impl crate::Resettable for DOEPCTL5_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
