#[doc = "Register `DT0_CFG` reader"]
pub type R = crate::R<DT0_CFG_SPEC>;
#[doc = "Register `DT0_CFG` writer"]
pub type W = crate::W<DT0_CFG_SPEC>;
#[doc = "Field `DT0_FED_UPMETHOD` reader - "]
pub type DT0_FED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DT0_FED_UPMETHOD` writer - "]
pub type DT0_FED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT0_RED_UPMETHOD` reader - "]
pub type DT0_RED_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `DT0_RED_UPMETHOD` writer - "]
pub type DT0_RED_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT0_DEB_MODE` reader - "]
pub type DT0_DEB_MODE_R = crate::BitReader;
#[doc = "Field `DT0_DEB_MODE` writer - "]
pub type DT0_DEB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_A_OUTSWAP` reader - "]
pub type DT0_A_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DT0_A_OUTSWAP` writer - "]
pub type DT0_A_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_B_OUTSWAP` reader - "]
pub type DT0_B_OUTSWAP_R = crate::BitReader;
#[doc = "Field `DT0_B_OUTSWAP` writer - "]
pub type DT0_B_OUTSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_RED_INSEL` reader - "]
pub type DT0_RED_INSEL_R = crate::BitReader;
#[doc = "Field `DT0_RED_INSEL` writer - "]
pub type DT0_RED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_FED_INSEL` reader - "]
pub type DT0_FED_INSEL_R = crate::BitReader;
#[doc = "Field `DT0_FED_INSEL` writer - "]
pub type DT0_FED_INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_RED_OUTINVERT` reader - "]
pub type DT0_RED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DT0_RED_OUTINVERT` writer - "]
pub type DT0_RED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_FED_OUTINVERT` reader - "]
pub type DT0_FED_OUTINVERT_R = crate::BitReader;
#[doc = "Field `DT0_FED_OUTINVERT` writer - "]
pub type DT0_FED_OUTINVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_A_OUTBYPASS` reader - "]
pub type DT0_A_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DT0_A_OUTBYPASS` writer - "]
pub type DT0_A_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_B_OUTBYPASS` reader - "]
pub type DT0_B_OUTBYPASS_R = crate::BitReader;
#[doc = "Field `DT0_B_OUTBYPASS` writer - "]
pub type DT0_B_OUTBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT0_CLK_SEL` reader - "]
pub type DT0_CLK_SEL_R = crate::BitReader;
#[doc = "Field `DT0_CLK_SEL` writer - "]
pub type DT0_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dt0_fed_upmethod(&self) -> DT0_FED_UPMETHOD_R {
        DT0_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dt0_red_upmethod(&self) -> DT0_RED_UPMETHOD_R {
        DT0_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dt0_deb_mode(&self) -> DT0_DEB_MODE_R {
        DT0_DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dt0_a_outswap(&self) -> DT0_A_OUTSWAP_R {
        DT0_A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dt0_b_outswap(&self) -> DT0_B_OUTSWAP_R {
        DT0_B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dt0_red_insel(&self) -> DT0_RED_INSEL_R {
        DT0_RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dt0_fed_insel(&self) -> DT0_FED_INSEL_R {
        DT0_FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dt0_red_outinvert(&self) -> DT0_RED_OUTINVERT_R {
        DT0_RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dt0_fed_outinvert(&self) -> DT0_FED_OUTINVERT_R {
        DT0_FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dt0_a_outbypass(&self) -> DT0_A_OUTBYPASS_R {
        DT0_A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dt0_b_outbypass(&self) -> DT0_B_OUTBYPASS_R {
        DT0_B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dt0_clk_sel(&self) -> DT0_CLK_SEL_R {
        DT0_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT0_CFG")
            .field(
                "dt0_fed_upmethod",
                &format_args!("{}", self.dt0_fed_upmethod().bits()),
            )
            .field(
                "dt0_red_upmethod",
                &format_args!("{}", self.dt0_red_upmethod().bits()),
            )
            .field(
                "dt0_deb_mode",
                &format_args!("{}", self.dt0_deb_mode().bit()),
            )
            .field(
                "dt0_a_outswap",
                &format_args!("{}", self.dt0_a_outswap().bit()),
            )
            .field(
                "dt0_b_outswap",
                &format_args!("{}", self.dt0_b_outswap().bit()),
            )
            .field(
                "dt0_red_insel",
                &format_args!("{}", self.dt0_red_insel().bit()),
            )
            .field(
                "dt0_fed_insel",
                &format_args!("{}", self.dt0_fed_insel().bit()),
            )
            .field(
                "dt0_red_outinvert",
                &format_args!("{}", self.dt0_red_outinvert().bit()),
            )
            .field(
                "dt0_fed_outinvert",
                &format_args!("{}", self.dt0_fed_outinvert().bit()),
            )
            .field(
                "dt0_a_outbypass",
                &format_args!("{}", self.dt0_a_outbypass().bit()),
            )
            .field(
                "dt0_b_outbypass",
                &format_args!("{}", self.dt0_b_outbypass().bit()),
            )
            .field("dt0_clk_sel", &format_args!("{}", self.dt0_clk_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DT0_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_fed_upmethod(&mut self) -> DT0_FED_UPMETHOD_W<DT0_CFG_SPEC> {
        DT0_FED_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_red_upmethod(&mut self) -> DT0_RED_UPMETHOD_W<DT0_CFG_SPEC> {
        DT0_RED_UPMETHOD_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_deb_mode(&mut self) -> DT0_DEB_MODE_W<DT0_CFG_SPEC> {
        DT0_DEB_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_a_outswap(&mut self) -> DT0_A_OUTSWAP_W<DT0_CFG_SPEC> {
        DT0_A_OUTSWAP_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_b_outswap(&mut self) -> DT0_B_OUTSWAP_W<DT0_CFG_SPEC> {
        DT0_B_OUTSWAP_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_red_insel(&mut self) -> DT0_RED_INSEL_W<DT0_CFG_SPEC> {
        DT0_RED_INSEL_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_fed_insel(&mut self) -> DT0_FED_INSEL_W<DT0_CFG_SPEC> {
        DT0_FED_INSEL_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_red_outinvert(&mut self) -> DT0_RED_OUTINVERT_W<DT0_CFG_SPEC> {
        DT0_RED_OUTINVERT_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_fed_outinvert(&mut self) -> DT0_FED_OUTINVERT_W<DT0_CFG_SPEC> {
        DT0_FED_OUTINVERT_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_a_outbypass(&mut self) -> DT0_A_OUTBYPASS_W<DT0_CFG_SPEC> {
        DT0_A_OUTBYPASS_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_b_outbypass(&mut self) -> DT0_B_OUTBYPASS_W<DT0_CFG_SPEC> {
        DT0_B_OUTBYPASS_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn dt0_clk_sel(&mut self) -> DT0_CLK_SEL_W<DT0_CFG_SPEC> {
        DT0_CLK_SEL_W::new(self, 17)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt0_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt0_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT0_CFG_SPEC;
impl crate::RegisterSpec for DT0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt0_cfg::R`](R) reader structure"]
impl crate::Readable for DT0_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt0_cfg::W`](W) writer structure"]
impl crate::Writable for DT0_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT0_CFG to value 0x0001_8000"]
impl crate::Resettable for DT0_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0001_8000;
}
