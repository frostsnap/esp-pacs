#[doc = "Register `HCINTMSK4` reader"]
pub type R = crate::R<HCINTMSK4_SPEC>;
#[doc = "Register `HCINTMSK4` writer"]
pub type W = crate::W<HCINTMSK4_SPEC>;
#[doc = "Field `H_XFERCOMPLMSK4` reader - "]
pub type H_XFERCOMPLMSK4_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK4` writer - "]
pub type H_XFERCOMPLMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHHLTDMSK4` reader - "]
pub type H_CHHLTDMSK4_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK4` writer - "]
pub type H_CHHLTDMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_AHBERRMSK4` reader - "]
pub type H_AHBERRMSK4_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK4` writer - "]
pub type H_AHBERRMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_STALLMSK4` reader - "]
pub type H_STALLMSK4_R = crate::BitReader;
#[doc = "Field `H_STALLMSK4` writer - "]
pub type H_STALLMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NAKMSK4` reader - "]
pub type H_NAKMSK4_R = crate::BitReader;
#[doc = "Field `H_NAKMSK4` writer - "]
pub type H_NAKMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ACKMSK4` reader - "]
pub type H_ACKMSK4_R = crate::BitReader;
#[doc = "Field `H_ACKMSK4` writer - "]
pub type H_ACKMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NYETMSK4` reader - "]
pub type H_NYETMSK4_R = crate::BitReader;
#[doc = "Field `H_NYETMSK4` writer - "]
pub type H_NYETMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XACTERRMSK4` reader - "]
pub type H_XACTERRMSK4_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK4` writer - "]
pub type H_XACTERRMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BBLERRMSK4` reader - "]
pub type H_BBLERRMSK4_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK4` writer - "]
pub type H_BBLERRMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRMOVRUNMSK4` reader - "]
pub type H_FRMOVRUNMSK4_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK4` writer - "]
pub type H_FRMOVRUNMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DATATGLERRMSK4` reader - "]
pub type H_DATATGLERRMSK4_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK4` writer - "]
pub type H_DATATGLERRMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BNAINTRMSK4` reader - "]
pub type H_BNAINTRMSK4_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK4` writer - "]
pub type H_BNAINTRMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK4` reader - "]
pub type H_DESC_LST_ROLLINTRMSK4_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK4` writer - "]
pub type H_DESC_LST_ROLLINTRMSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk4(&self) -> H_XFERCOMPLMSK4_R {
        H_XFERCOMPLMSK4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk4(&self) -> H_CHHLTDMSK4_R {
        H_CHHLTDMSK4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk4(&self) -> H_AHBERRMSK4_R {
        H_AHBERRMSK4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk4(&self) -> H_STALLMSK4_R {
        H_STALLMSK4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk4(&self) -> H_NAKMSK4_R {
        H_NAKMSK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk4(&self) -> H_ACKMSK4_R {
        H_ACKMSK4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk4(&self) -> H_NYETMSK4_R {
        H_NYETMSK4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk4(&self) -> H_XACTERRMSK4_R {
        H_XACTERRMSK4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk4(&self) -> H_BBLERRMSK4_R {
        H_BBLERRMSK4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk4(&self) -> H_FRMOVRUNMSK4_R {
        H_FRMOVRUNMSK4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk4(&self) -> H_DATATGLERRMSK4_R {
        H_DATATGLERRMSK4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk4(&self) -> H_BNAINTRMSK4_R {
        H_BNAINTRMSK4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk4(&self) -> H_DESC_LST_ROLLINTRMSK4_R {
        H_DESC_LST_ROLLINTRMSK4_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK4")
            .field(
                "h_xfercomplmsk4",
                &format_args!("{}", self.h_xfercomplmsk4().bit()),
            )
            .field(
                "h_chhltdmsk4",
                &format_args!("{}", self.h_chhltdmsk4().bit()),
            )
            .field(
                "h_ahberrmsk4",
                &format_args!("{}", self.h_ahberrmsk4().bit()),
            )
            .field("h_stallmsk4", &format_args!("{}", self.h_stallmsk4().bit()))
            .field("h_nakmsk4", &format_args!("{}", self.h_nakmsk4().bit()))
            .field("h_ackmsk4", &format_args!("{}", self.h_ackmsk4().bit()))
            .field("h_nyetmsk4", &format_args!("{}", self.h_nyetmsk4().bit()))
            .field(
                "h_xacterrmsk4",
                &format_args!("{}", self.h_xacterrmsk4().bit()),
            )
            .field(
                "h_bblerrmsk4",
                &format_args!("{}", self.h_bblerrmsk4().bit()),
            )
            .field(
                "h_frmovrunmsk4",
                &format_args!("{}", self.h_frmovrunmsk4().bit()),
            )
            .field(
                "h_datatglerrmsk4",
                &format_args!("{}", self.h_datatglerrmsk4().bit()),
            )
            .field(
                "h_bnaintrmsk4",
                &format_args!("{}", self.h_bnaintrmsk4().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk4",
                &format_args!("{}", self.h_desc_lst_rollintrmsk4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk4(&mut self) -> H_XFERCOMPLMSK4_W<HCINTMSK4_SPEC> {
        H_XFERCOMPLMSK4_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk4(&mut self) -> H_CHHLTDMSK4_W<HCINTMSK4_SPEC> {
        H_CHHLTDMSK4_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk4(&mut self) -> H_AHBERRMSK4_W<HCINTMSK4_SPEC> {
        H_AHBERRMSK4_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk4(&mut self) -> H_STALLMSK4_W<HCINTMSK4_SPEC> {
        H_STALLMSK4_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk4(&mut self) -> H_NAKMSK4_W<HCINTMSK4_SPEC> {
        H_NAKMSK4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk4(&mut self) -> H_ACKMSK4_W<HCINTMSK4_SPEC> {
        H_ACKMSK4_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk4(&mut self) -> H_NYETMSK4_W<HCINTMSK4_SPEC> {
        H_NYETMSK4_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk4(&mut self) -> H_XACTERRMSK4_W<HCINTMSK4_SPEC> {
        H_XACTERRMSK4_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk4(&mut self) -> H_BBLERRMSK4_W<HCINTMSK4_SPEC> {
        H_BBLERRMSK4_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk4(&mut self) -> H_FRMOVRUNMSK4_W<HCINTMSK4_SPEC> {
        H_FRMOVRUNMSK4_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk4(&mut self) -> H_DATATGLERRMSK4_W<HCINTMSK4_SPEC> {
        H_DATATGLERRMSK4_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk4(&mut self) -> H_BNAINTRMSK4_W<HCINTMSK4_SPEC> {
        H_BNAINTRMSK4_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk4(&mut self) -> H_DESC_LST_ROLLINTRMSK4_W<HCINTMSK4_SPEC> {
        H_DESC_LST_ROLLINTRMSK4_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK4_SPEC;
impl crate::RegisterSpec for HCINTMSK4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk4::R`](R) reader structure"]
impl crate::Readable for HCINTMSK4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk4::W`](W) writer structure"]
impl crate::Writable for HCINTMSK4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTMSK4 to value 0"]
impl crate::Resettable for HCINTMSK4_SPEC {
    const RESET_VALUE: u32 = 0;
}
