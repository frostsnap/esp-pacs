#[doc = "Register `CCM_COEF0` reader"]
pub type R = crate::R<CCM_COEF0_SPEC>;
#[doc = "Register `CCM_COEF0` writer"]
pub type W = crate::W<CCM_COEF0_SPEC>;
#[doc = "Field `CCM_RR` reader - this field configures the color correction matrix coefficient"]
pub type CCM_RR_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_RR` writer - this field configures the color correction matrix coefficient"]
pub type CCM_RR_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CCM_RG` reader - this field configures the color correction matrix coefficient"]
pub type CCM_RG_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_RG` writer - this field configures the color correction matrix coefficient"]
pub type CCM_RG_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_rr(&self) -> CCM_RR_R {
        CCM_RR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_rg(&self) -> CCM_RG_R {
        CCM_RG_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_COEF0")
            .field("ccm_rr", &format_args!("{}", self.ccm_rr().bits()))
            .field("ccm_rg", &format_args!("{}", self.ccm_rg().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CCM_COEF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_rr(&mut self) -> CCM_RR_W<CCM_COEF0_SPEC> {
        CCM_RR_W::new(self, 0)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_rg(&mut self) -> CCM_RG_W<CCM_COEF0_SPEC> {
        CCM_RG_W::new(self, 13)
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
#[doc = "ccm coef register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCM_COEF0_SPEC;
impl crate::RegisterSpec for CCM_COEF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm_coef0::R`](R) reader structure"]
impl crate::Readable for CCM_COEF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccm_coef0::W`](W) writer structure"]
impl crate::Writable for CCM_COEF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCM_COEF0 to value 0x0250_0740"]
impl crate::Resettable for CCM_COEF0_SPEC {
    const RESET_VALUE: u32 = 0x0250_0740;
}
