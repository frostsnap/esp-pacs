#[doc = "Register `HCINT4` reader"]
pub type R = crate::R<HCINT4_SPEC>;
#[doc = "Register `HCINT4` writer"]
pub type W = crate::W<HCINT4_SPEC>;
#[doc = "Field `H_XFERCOMPL4` reader - "]
pub type H_XFERCOMPL4_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL4` writer - "]
pub type H_XFERCOMPL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHHLTD4` reader - "]
pub type H_CHHLTD4_R = crate::BitReader;
#[doc = "Field `H_CHHLTD4` writer - "]
pub type H_CHHLTD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_AHBERR4` reader - "]
pub type H_AHBERR4_R = crate::BitReader;
#[doc = "Field `H_AHBERR4` writer - "]
pub type H_AHBERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_STALL4` reader - "]
pub type H_STALL4_R = crate::BitReader;
#[doc = "Field `H_STALL4` writer - "]
pub type H_STALL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NACK4` reader - "]
pub type H_NACK4_R = crate::BitReader;
#[doc = "Field `H_NACK4` writer - "]
pub type H_NACK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ACK4` reader - "]
pub type H_ACK4_R = crate::BitReader;
#[doc = "Field `H_ACK4` writer - "]
pub type H_ACK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NYET4` reader - "]
pub type H_NYET4_R = crate::BitReader;
#[doc = "Field `H_NYET4` writer - "]
pub type H_NYET4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XACTERR4` reader - "]
pub type H_XACTERR4_R = crate::BitReader;
#[doc = "Field `H_XACTERR4` writer - "]
pub type H_XACTERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BBLERR4` reader - "]
pub type H_BBLERR4_R = crate::BitReader;
#[doc = "Field `H_BBLERR4` writer - "]
pub type H_BBLERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRMOVRUN4` reader - "]
pub type H_FRMOVRUN4_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN4` writer - "]
pub type H_FRMOVRUN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DATATGLERR4` reader - "]
pub type H_DATATGLERR4_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR4` writer - "]
pub type H_DATATGLERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BNAINTR4` reader - "]
pub type H_BNAINTR4_R = crate::BitReader;
#[doc = "Field `H_BNAINTR4` writer - "]
pub type H_BNAINTR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XCS_XACT_ERR4` reader - "]
pub type H_XCS_XACT_ERR4_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR4` writer - "]
pub type H_XCS_XACT_ERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESC_LST_ROLLINTR4` reader - "]
pub type H_DESC_LST_ROLLINTR4_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR4` writer - "]
pub type H_DESC_LST_ROLLINTR4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl4(&self) -> H_XFERCOMPL4_R {
        H_XFERCOMPL4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd4(&self) -> H_CHHLTD4_R {
        H_CHHLTD4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr4(&self) -> H_AHBERR4_R {
        H_AHBERR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall4(&self) -> H_STALL4_R {
        H_STALL4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack4(&self) -> H_NACK4_R {
        H_NACK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack4(&self) -> H_ACK4_R {
        H_ACK4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet4(&self) -> H_NYET4_R {
        H_NYET4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr4(&self) -> H_XACTERR4_R {
        H_XACTERR4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr4(&self) -> H_BBLERR4_R {
        H_BBLERR4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun4(&self) -> H_FRMOVRUN4_R {
        H_FRMOVRUN4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr4(&self) -> H_DATATGLERR4_R {
        H_DATATGLERR4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr4(&self) -> H_BNAINTR4_R {
        H_BNAINTR4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err4(&self) -> H_XCS_XACT_ERR4_R {
        H_XCS_XACT_ERR4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr4(&self) -> H_DESC_LST_ROLLINTR4_R {
        H_DESC_LST_ROLLINTR4_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT4")
            .field(
                "h_xfercompl4",
                &format_args!("{}", self.h_xfercompl4().bit()),
            )
            .field("h_chhltd4", &format_args!("{}", self.h_chhltd4().bit()))
            .field("h_ahberr4", &format_args!("{}", self.h_ahberr4().bit()))
            .field("h_stall4", &format_args!("{}", self.h_stall4().bit()))
            .field("h_nack4", &format_args!("{}", self.h_nack4().bit()))
            .field("h_ack4", &format_args!("{}", self.h_ack4().bit()))
            .field("h_nyet4", &format_args!("{}", self.h_nyet4().bit()))
            .field("h_xacterr4", &format_args!("{}", self.h_xacterr4().bit()))
            .field("h_bblerr4", &format_args!("{}", self.h_bblerr4().bit()))
            .field("h_frmovrun4", &format_args!("{}", self.h_frmovrun4().bit()))
            .field(
                "h_datatglerr4",
                &format_args!("{}", self.h_datatglerr4().bit()),
            )
            .field("h_bnaintr4", &format_args!("{}", self.h_bnaintr4().bit()))
            .field(
                "h_xcs_xact_err4",
                &format_args!("{}", self.h_xcs_xact_err4().bit()),
            )
            .field(
                "h_desc_lst_rollintr4",
                &format_args!("{}", self.h_desc_lst_rollintr4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl4(&mut self) -> H_XFERCOMPL4_W<HCINT4_SPEC> {
        H_XFERCOMPL4_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd4(&mut self) -> H_CHHLTD4_W<HCINT4_SPEC> {
        H_CHHLTD4_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr4(&mut self) -> H_AHBERR4_W<HCINT4_SPEC> {
        H_AHBERR4_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall4(&mut self) -> H_STALL4_W<HCINT4_SPEC> {
        H_STALL4_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack4(&mut self) -> H_NACK4_W<HCINT4_SPEC> {
        H_NACK4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack4(&mut self) -> H_ACK4_W<HCINT4_SPEC> {
        H_ACK4_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet4(&mut self) -> H_NYET4_W<HCINT4_SPEC> {
        H_NYET4_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr4(&mut self) -> H_XACTERR4_W<HCINT4_SPEC> {
        H_XACTERR4_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr4(&mut self) -> H_BBLERR4_W<HCINT4_SPEC> {
        H_BBLERR4_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun4(&mut self) -> H_FRMOVRUN4_W<HCINT4_SPEC> {
        H_FRMOVRUN4_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr4(&mut self) -> H_DATATGLERR4_W<HCINT4_SPEC> {
        H_DATATGLERR4_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr4(&mut self) -> H_BNAINTR4_W<HCINT4_SPEC> {
        H_BNAINTR4_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err4(&mut self) -> H_XCS_XACT_ERR4_W<HCINT4_SPEC> {
        H_XCS_XACT_ERR4_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr4(&mut self) -> H_DESC_LST_ROLLINTR4_W<HCINT4_SPEC> {
        H_DESC_LST_ROLLINTR4_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT4_SPEC;
impl crate::RegisterSpec for HCINT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint4::R`](R) reader structure"]
impl crate::Readable for HCINT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcint4::W`](W) writer structure"]
impl crate::Writable for HCINT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINT4 to value 0"]
impl crate::Resettable for HCINT4_SPEC {
    const RESET_VALUE: u32 = 0;
}
