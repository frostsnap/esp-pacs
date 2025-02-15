#[doc = "Register `CK_BG_HIGH` reader"]
pub type R = crate::R<CK_BG_HIGH_SPEC>;
#[doc = "Register `CK_BG_HIGH` writer"]
pub type W = crate::W<CK_BG_HIGH_SPEC>;
#[doc = "Field `COLORKEY_BG_B_HIGH` reader - color key higher threshold of background b channel"]
pub type COLORKEY_BG_B_HIGH_R = crate::FieldReader;
#[doc = "Field `COLORKEY_BG_B_HIGH` writer - color key higher threshold of background b channel"]
pub type COLORKEY_BG_B_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_BG_G_HIGH` reader - color key higher threshold of background g channel"]
pub type COLORKEY_BG_G_HIGH_R = crate::FieldReader;
#[doc = "Field `COLORKEY_BG_G_HIGH` writer - color key higher threshold of background g channel"]
pub type COLORKEY_BG_G_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_BG_R_HIGH` reader - color key higher threshold of background r channel"]
pub type COLORKEY_BG_R_HIGH_R = crate::FieldReader;
#[doc = "Field `COLORKEY_BG_R_HIGH` writer - color key higher threshold of background r channel"]
pub type COLORKEY_BG_R_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - color key higher threshold of background b channel"]
    #[inline(always)]
    pub fn colorkey_bg_b_high(&self) -> COLORKEY_BG_B_HIGH_R {
        COLORKEY_BG_B_HIGH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - color key higher threshold of background g channel"]
    #[inline(always)]
    pub fn colorkey_bg_g_high(&self) -> COLORKEY_BG_G_HIGH_R {
        COLORKEY_BG_G_HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - color key higher threshold of background r channel"]
    #[inline(always)]
    pub fn colorkey_bg_r_high(&self) -> COLORKEY_BG_R_HIGH_R {
        COLORKEY_BG_R_HIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK_BG_HIGH")
            .field(
                "colorkey_bg_b_high",
                &format_args!("{}", self.colorkey_bg_b_high().bits()),
            )
            .field(
                "colorkey_bg_g_high",
                &format_args!("{}", self.colorkey_bg_g_high().bits()),
            )
            .field(
                "colorkey_bg_r_high",
                &format_args!("{}", self.colorkey_bg_r_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CK_BG_HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - color key higher threshold of background b channel"]
    #[inline(always)]
    #[must_use]
    pub fn colorkey_bg_b_high(&mut self) -> COLORKEY_BG_B_HIGH_W<CK_BG_HIGH_SPEC> {
        COLORKEY_BG_B_HIGH_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - color key higher threshold of background g channel"]
    #[inline(always)]
    #[must_use]
    pub fn colorkey_bg_g_high(&mut self) -> COLORKEY_BG_G_HIGH_W<CK_BG_HIGH_SPEC> {
        COLORKEY_BG_G_HIGH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - color key higher threshold of background r channel"]
    #[inline(always)]
    #[must_use]
    pub fn colorkey_bg_r_high(&mut self) -> COLORKEY_BG_R_HIGH_W<CK_BG_HIGH_SPEC> {
        COLORKEY_BG_R_HIGH_W::new(self, 16)
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
#[doc = "background color key higher threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ck_bg_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_bg_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CK_BG_HIGH_SPEC;
impl crate::RegisterSpec for CK_BG_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck_bg_high::R`](R) reader structure"]
impl crate::Readable for CK_BG_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ck_bg_high::W`](W) writer structure"]
impl crate::Writable for CK_BG_HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CK_BG_HIGH to value 0"]
impl crate::Resettable for CK_BG_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
