#[doc = "Register `DIEPCTL6` reader"]
pub type R = crate::R<DIEPCTL6_SPEC>;
#[doc = "Register `DIEPCTL6` writer"]
pub type W = crate::W<DIEPCTL6_SPEC>;
#[doc = "Field `D_MPS6` reader - "]
pub type D_MPS6_R = crate::FieldReader;
#[doc = "Field `D_MPS6` writer - "]
pub type D_MPS6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D_USBACTEP6` reader - "]
pub type D_USBACTEP6_R = crate::BitReader;
#[doc = "Field `D_NAKSTS6` reader - "]
pub type D_NAKSTS6_R = crate::BitReader;
#[doc = "Field `D_EPTYPE6` reader - "]
pub type D_EPTYPE6_R = crate::FieldReader;
#[doc = "Field `D_STALL6` reader - "]
pub type D_STALL6_R = crate::BitReader;
#[doc = "Field `D_STALL6` writer - "]
pub type D_STALL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_TXFNUM6` reader - "]
pub type D_TXFNUM6_R = crate::FieldReader;
#[doc = "Field `D_TXFNUM6` writer - "]
pub type D_TXFNUM6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D_CNAK6` writer - "]
pub type D_CNAK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI_SNAK6` writer - "]
pub type DI_SNAK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI_SETD0PID6` writer - "]
pub type DI_SETD0PID6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI_SETD1PID6` writer - "]
pub type DI_SETD1PID6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_EPDIS6` reader - "]
pub type D_EPDIS6_R = crate::BitReader;
#[doc = "Field `D_EPDIS6` writer - "]
pub type D_EPDIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_EPENA6` reader - "]
pub type D_EPENA6_R = crate::BitReader;
#[doc = "Field `D_EPENA6` writer - "]
pub type D_EPENA6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn d_mps6(&self) -> D_MPS6_R {
        D_MPS6_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn d_usbactep6(&self) -> D_USBACTEP6_R {
        D_USBACTEP6_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn d_naksts6(&self) -> D_NAKSTS6_R {
        D_NAKSTS6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn d_eptype6(&self) -> D_EPTYPE6_R {
        D_EPTYPE6_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn d_stall6(&self) -> D_STALL6_R {
        D_STALL6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn d_txfnum6(&self) -> D_TXFNUM6_R {
        D_TXFNUM6_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn d_epdis6(&self) -> D_EPDIS6_R {
        D_EPDIS6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn d_epena6(&self) -> D_EPENA6_R {
        D_EPENA6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL6")
            .field("d_mps6", &format_args!("{}", self.d_mps6().bits()))
            .field("d_usbactep6", &format_args!("{}", self.d_usbactep6().bit()))
            .field("d_naksts6", &format_args!("{}", self.d_naksts6().bit()))
            .field("d_eptype6", &format_args!("{}", self.d_eptype6().bits()))
            .field("d_stall6", &format_args!("{}", self.d_stall6().bit()))
            .field("d_txfnum6", &format_args!("{}", self.d_txfnum6().bits()))
            .field("d_epdis6", &format_args!("{}", self.d_epdis6().bit()))
            .field("d_epena6", &format_args!("{}", self.d_epena6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn d_mps6(&mut self) -> D_MPS6_W<DIEPCTL6_SPEC> {
        D_MPS6_W::new(self, 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn d_stall6(&mut self) -> D_STALL6_W<DIEPCTL6_SPEC> {
        D_STALL6_W::new(self, 21)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfnum6(&mut self) -> D_TXFNUM6_W<DIEPCTL6_SPEC> {
        D_TXFNUM6_W::new(self, 22)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn d_cnak6(&mut self) -> D_CNAK6_W<DIEPCTL6_SPEC> {
        D_CNAK6_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak6(&mut self) -> DI_SNAK6_W<DIEPCTL6_SPEC> {
        DI_SNAK6_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd0pid6(&mut self) -> DI_SETD0PID6_W<DIEPCTL6_SPEC> {
        DI_SETD0PID6_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd1pid6(&mut self) -> DI_SETD1PID6_W<DIEPCTL6_SPEC> {
        DI_SETD1PID6_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdis6(&mut self) -> D_EPDIS6_W<DIEPCTL6_SPEC> {
        D_EPDIS6_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn d_epena6(&mut self) -> D_EPENA6_W<DIEPCTL6_SPEC> {
        D_EPENA6_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL6_SPEC;
impl crate::RegisterSpec for DIEPCTL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl6::R`](R) reader structure"]
impl crate::Readable for DIEPCTL6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl6::W`](W) writer structure"]
impl crate::Writable for DIEPCTL6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPCTL6 to value 0x8000"]
impl crate::Resettable for DIEPCTL6_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
