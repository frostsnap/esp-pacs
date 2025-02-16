#[doc = "Register `CACHE_TAG_CONTENT` reader"]
pub type R = crate::R<CACHE_TAG_CONTENT_SPEC>;
#[doc = "Register `CACHE_TAG_CONTENT` writer"]
pub type W = crate::W<CACHE_TAG_CONTENT_SPEC>;
#[doc = "Field `CACHE_TAG_CONTENT` reader - This is a constant place where we can write data to or read data from the tag memory on the specified cache."]
pub type CACHE_TAG_CONTENT_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_TAG_CONTENT` writer - This is a constant place where we can write data to or read data from the tag memory on the specified cache."]
pub type CACHE_TAG_CONTENT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is a constant place where we can write data to or read data from the tag memory on the specified cache."]
    #[inline(always)]
    pub fn cache_tag_content(&self) -> CACHE_TAG_CONTENT_R {
        CACHE_TAG_CONTENT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_TAG_CONTENT")
            .field(
                "cache_tag_content",
                &format_args!("{}", self.cache_tag_content().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_TAG_CONTENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is a constant place where we can write data to or read data from the tag memory on the specified cache."]
    #[inline(always)]
    #[must_use]
    pub fn cache_tag_content(&mut self) -> CACHE_TAG_CONTENT_W<CACHE_TAG_CONTENT_SPEC> {
        CACHE_TAG_CONTENT_W::new(self, 0)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_content::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_content::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_TAG_CONTENT_SPEC;
impl crate::RegisterSpec for CACHE_TAG_CONTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_tag_content::R`](R) reader structure"]
impl crate::Readable for CACHE_TAG_CONTENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_tag_content::W`](W) writer structure"]
impl crate::Writable for CACHE_TAG_CONTENT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_TAG_CONTENT to value 0"]
impl crate::Resettable for CACHE_TAG_CONTENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
