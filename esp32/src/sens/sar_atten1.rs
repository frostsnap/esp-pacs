#[doc = "Register `SAR_ATTEN1` reader"]
pub type R = crate::R<SAR_ATTEN1_SPEC>;
#[doc = "Register `SAR_ATTEN1` writer"]
pub type W = crate::W<SAR_ATTEN1_SPEC>;
#[doc = "Field `SAR1_ATTEN` reader - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
pub type SAR1_ATTEN_R = crate::FieldReader<u32>;
#[doc = "Field `SAR1_ATTEN` writer - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
pub type SAR1_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
    #[inline(always)]
    pub fn sar1_atten(&self) -> SAR1_ATTEN_R {
        SAR1_ATTEN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_ATTEN1")
            .field("sar1_atten", &format_args!("{}", self.sar1_atten().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_ATTEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_atten(&mut self) -> SAR1_ATTEN_W<SAR_ATTEN1_SPEC> {
        SAR1_ATTEN_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_atten1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_atten1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_ATTEN1_SPEC;
impl crate::RegisterSpec for SAR_ATTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_atten1::R`](R) reader structure"]
impl crate::Readable for SAR_ATTEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_atten1::W`](W) writer structure"]
impl crate::Writable for SAR_ATTEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_ATTEN1 to value 0xffff_ffff"]
impl crate::Resettable for SAR_ATTEN1_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
