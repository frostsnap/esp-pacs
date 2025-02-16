#[doc = "Register `HCCHAR5` reader"]
pub type R = crate::R<HCCHAR5_SPEC>;
#[doc = "Register `HCCHAR5` writer"]
pub type W = crate::W<HCCHAR5_SPEC>;
#[doc = "Field `H_MPS5` reader - "]
pub type H_MPS5_R = crate::FieldReader<u16>;
#[doc = "Field `H_MPS5` writer - "]
pub type H_MPS5_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `H_EPNUM5` reader - "]
pub type H_EPNUM5_R = crate::FieldReader;
#[doc = "Field `H_EPNUM5` writer - "]
pub type H_EPNUM5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H_EPDIR5` reader - "]
pub type H_EPDIR5_R = crate::BitReader;
#[doc = "Field `H_EPDIR5` writer - "]
pub type H_EPDIR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_LSPDDEV5` reader - "]
pub type H_LSPDDEV5_R = crate::BitReader;
#[doc = "Field `H_LSPDDEV5` writer - "]
pub type H_LSPDDEV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_EPTYPE5` reader - "]
pub type H_EPTYPE5_R = crate::FieldReader;
#[doc = "Field `H_EPTYPE5` writer - "]
pub type H_EPTYPE5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_EC5` reader - "]
pub type H_EC5_R = crate::BitReader;
#[doc = "Field `H_EC5` writer - "]
pub type H_EC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DEVADDR5` reader - "]
pub type H_DEVADDR5_R = crate::FieldReader;
#[doc = "Field `H_DEVADDR5` writer - "]
pub type H_DEVADDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `H_ODDFRM5` reader - "]
pub type H_ODDFRM5_R = crate::BitReader;
#[doc = "Field `H_ODDFRM5` writer - "]
pub type H_ODDFRM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHDIS5` reader - "]
pub type H_CHDIS5_R = crate::BitReader;
#[doc = "Field `H_CHDIS5` writer - "]
pub type H_CHDIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHENA5` reader - "]
pub type H_CHENA5_R = crate::BitReader;
#[doc = "Field `H_CHENA5` writer - "]
pub type H_CHENA5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps5(&self) -> H_MPS5_R {
        H_MPS5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum5(&self) -> H_EPNUM5_R {
        H_EPNUM5_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir5(&self) -> H_EPDIR5_R {
        H_EPDIR5_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev5(&self) -> H_LSPDDEV5_R {
        H_LSPDDEV5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype5(&self) -> H_EPTYPE5_R {
        H_EPTYPE5_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec5(&self) -> H_EC5_R {
        H_EC5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr5(&self) -> H_DEVADDR5_R {
        H_DEVADDR5_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm5(&self) -> H_ODDFRM5_R {
        H_ODDFRM5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis5(&self) -> H_CHDIS5_R {
        H_CHDIS5_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena5(&self) -> H_CHENA5_R {
        H_CHENA5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR5")
            .field("h_mps5", &format_args!("{}", self.h_mps5().bits()))
            .field("h_epnum5", &format_args!("{}", self.h_epnum5().bits()))
            .field("h_epdir5", &format_args!("{}", self.h_epdir5().bit()))
            .field("h_lspddev5", &format_args!("{}", self.h_lspddev5().bit()))
            .field("h_eptype5", &format_args!("{}", self.h_eptype5().bits()))
            .field("h_ec5", &format_args!("{}", self.h_ec5().bit()))
            .field("h_devaddr5", &format_args!("{}", self.h_devaddr5().bits()))
            .field("h_oddfrm5", &format_args!("{}", self.h_oddfrm5().bit()))
            .field("h_chdis5", &format_args!("{}", self.h_chdis5().bit()))
            .field("h_chena5", &format_args!("{}", self.h_chena5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCCHAR5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps5(&mut self) -> H_MPS5_W<HCCHAR5_SPEC> {
        H_MPS5_W::new(self, 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum5(&mut self) -> H_EPNUM5_W<HCCHAR5_SPEC> {
        H_EPNUM5_W::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir5(&mut self) -> H_EPDIR5_W<HCCHAR5_SPEC> {
        H_EPDIR5_W::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev5(&mut self) -> H_LSPDDEV5_W<HCCHAR5_SPEC> {
        H_LSPDDEV5_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype5(&mut self) -> H_EPTYPE5_W<HCCHAR5_SPEC> {
        H_EPTYPE5_W::new(self, 18)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec5(&mut self) -> H_EC5_W<HCCHAR5_SPEC> {
        H_EC5_W::new(self, 21)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr5(&mut self) -> H_DEVADDR5_W<HCCHAR5_SPEC> {
        H_DEVADDR5_W::new(self, 22)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm5(&mut self) -> H_ODDFRM5_W<HCCHAR5_SPEC> {
        H_ODDFRM5_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis5(&mut self) -> H_CHDIS5_W<HCCHAR5_SPEC> {
        H_CHDIS5_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena5(&mut self) -> H_CHENA5_W<HCCHAR5_SPEC> {
        H_CHENA5_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR5_SPEC;
impl crate::RegisterSpec for HCCHAR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar5::R`](R) reader structure"]
impl crate::Readable for HCCHAR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar5::W`](W) writer structure"]
impl crate::Writable for HCCHAR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCHAR5 to value 0"]
impl crate::Resettable for HCCHAR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
