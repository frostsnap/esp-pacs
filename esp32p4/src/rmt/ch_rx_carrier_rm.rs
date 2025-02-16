#[doc = "Register `CH%s_RX_CARRIER_RM` reader"]
pub type R = crate::R<CH_RX_CARRIER_RM_SPEC>;
#[doc = "Register `CH%s_RX_CARRIER_RM` writer"]
pub type W = crate::W<CH_RX_CARRIER_RM_SPEC>;
#[doc = "Field `CARRIER_LOW_THRES_CH` reader - The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s."]
pub type CARRIER_LOW_THRES_CH_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_LOW_THRES_CH` writer - The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s."]
pub type CARRIER_LOW_THRES_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CARRIER_HIGH_THRES_CH` reader - The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s."]
pub type CARRIER_HIGH_THRES_CH_R = crate::FieldReader<u16>;
#[doc = "Field `CARRIER_HIGH_THRES_CH` writer - The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s."]
pub type CARRIER_HIGH_THRES_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s."]
    #[inline(always)]
    pub fn carrier_low_thres_ch(&self) -> CARRIER_LOW_THRES_CH_R {
        CARRIER_LOW_THRES_CH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s."]
    #[inline(always)]
    pub fn carrier_high_thres_ch(&self) -> CARRIER_HIGH_THRES_CH_R {
        CARRIER_HIGH_THRES_CH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CARRIER_RM")
            .field(
                "carrier_low_thres_ch",
                &format_args!("{}", self.carrier_low_thres_ch().bits()),
            )
            .field(
                "carrier_high_thres_ch",
                &format_args!("{}", self.carrier_high_thres_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_RX_CARRIER_RM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The low level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_LOW_THRES_CH%s + 1) for channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_low_thres_ch(&mut self) -> CARRIER_LOW_THRES_CH_W<CH_RX_CARRIER_RM_SPEC> {
        CARRIER_LOW_THRES_CH_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - The high level period in a carrier modulation mode is (REG_RMT_REG_CARRIER_HIGH_THRES_CH%s + 1) for channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_high_thres_ch(&mut self) -> CARRIER_HIGH_THRES_CH_W<CH_RX_CARRIER_RM_SPEC> {
        CARRIER_HIGH_THRES_CH_W::new(self, 16)
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
#[doc = "Channel %s carrier remove register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_carrier_rm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_carrier_rm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_CARRIER_RM_SPEC;
impl crate::RegisterSpec for CH_RX_CARRIER_RM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_carrier_rm::R`](R) reader structure"]
impl crate::Readable for CH_RX_CARRIER_RM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_carrier_rm::W`](W) writer structure"]
impl crate::Writable for CH_RX_CARRIER_RM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_RX_CARRIER_RM to value 0"]
impl crate::Resettable for CH_RX_CARRIER_RM_SPEC {
    const RESET_VALUE: u32 = 0;
}
