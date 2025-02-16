#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CH0_TX_END_INT_ENA` reader - The interrupt enable bit for CH0_TX_END_INT."]
pub type CH0_TX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH0_TX_END_INT_ENA` writer - The interrupt enable bit for CH0_TX_END_INT."]
pub type CH0_TX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_END_INT_ENA` reader - The interrupt enable bit for CH1_TX_END_INT."]
pub type CH1_TX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH1_TX_END_INT_ENA` writer - The interrupt enable bit for CH1_TX_END_INT."]
pub type CH1_TX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_END_INT_ENA` reader - The interrupt enable bit for CH2_TX_END_INT."]
pub type CH2_TX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH2_TX_END_INT_ENA` writer - The interrupt enable bit for CH2_TX_END_INT."]
pub type CH2_TX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_END_INT_ENA` reader - The interrupt enable bit for CH3_TX_END_INT."]
pub type CH3_TX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH3_TX_END_INT_ENA` writer - The interrupt enable bit for CH3_TX_END_INT."]
pub type CH3_TX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH0_ERR_INT_ENA` reader - The interrupt enable bit for CH0_ERR_INT."]
pub type TX_CH0_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_CH0_ERR_INT_ENA` writer - The interrupt enable bit for CH0_ERR_INT."]
pub type TX_CH0_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH1_ERR_INT_ENA` reader - The interrupt enable bit for CH1_ERR_INT."]
pub type TX_CH1_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_CH1_ERR_INT_ENA` writer - The interrupt enable bit for CH1_ERR_INT."]
pub type TX_CH1_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH2_ERR_INT_ENA` reader - The interrupt enable bit for CH2_ERR_INT."]
pub type TX_CH2_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_CH2_ERR_INT_ENA` writer - The interrupt enable bit for CH2_ERR_INT."]
pub type TX_CH2_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_ERR_INT_ENA` reader - The interrupt enable bit for CH3_ERR_INT."]
pub type TX_CH3_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_CH3_ERR_INT_ENA` writer - The interrupt enable bit for CH3_ERR_INT."]
pub type TX_CH3_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
pub type CH0_TX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH0_TX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
pub type CH0_TX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
pub type CH1_TX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH1_TX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
pub type CH1_TX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
pub type CH2_TX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH2_TX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
pub type CH2_TX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
pub type CH3_TX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH3_TX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
pub type CH3_TX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_LOOP_INT_ENA` reader - The interrupt enable bit for CH0_TX_LOOP_INT."]
pub type CH0_TX_LOOP_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH0_TX_LOOP_INT_ENA` writer - The interrupt enable bit for CH0_TX_LOOP_INT."]
pub type CH0_TX_LOOP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_LOOP_INT_ENA` reader - The interrupt enable bit for CH1_TX_LOOP_INT."]
pub type CH1_TX_LOOP_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH1_TX_LOOP_INT_ENA` writer - The interrupt enable bit for CH1_TX_LOOP_INT."]
pub type CH1_TX_LOOP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_LOOP_INT_ENA` reader - The interrupt enable bit for CH2_TX_LOOP_INT."]
pub type CH2_TX_LOOP_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH2_TX_LOOP_INT_ENA` writer - The interrupt enable bit for CH2_TX_LOOP_INT."]
pub type CH2_TX_LOOP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_LOOP_INT_ENA` reader - The interrupt enable bit for CH3_TX_LOOP_INT."]
pub type CH3_TX_LOOP_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH3_TX_LOOP_INT_ENA` writer - The interrupt enable bit for CH3_TX_LOOP_INT."]
pub type CH3_TX_LOOP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_END_INT_ENA` reader - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH4_RX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH4_RX_END_INT_ENA` writer - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH4_RX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_END_INT_ENA` reader - The interrupt enable bit for CH5_RX_END_INT."]
pub type CH5_RX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH5_RX_END_INT_ENA` writer - The interrupt enable bit for CH5_RX_END_INT."]
pub type CH5_RX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_END_INT_ENA` reader - The interrupt enable bit for CH6_RX_END_INT."]
pub type CH6_RX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH6_RX_END_INT_ENA` writer - The interrupt enable bit for CH6_RX_END_INT."]
pub type CH6_RX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_END_INT_ENA` reader - The interrupt enable bit for CH7_RX_END_INT."]
pub type CH7_RX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH7_RX_END_INT_ENA` writer - The interrupt enable bit for CH7_RX_END_INT."]
pub type CH7_RX_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_ERR_INT_ENA` reader - The interrupt enable bit for CH4_ERR_INT."]
pub type CH4_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH4_ERR_INT_ENA` writer - The interrupt enable bit for CH4_ERR_INT."]
pub type CH4_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_ERR_INT_ENA` reader - The interrupt enable bit for CH5_ERR_INT."]
pub type CH5_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH5_ERR_INT_ENA` writer - The interrupt enable bit for CH5_ERR_INT."]
pub type CH5_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_ERR_INT_ENA` reader - The interrupt enable bit for CH6_ERR_INT."]
pub type CH6_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH6_ERR_INT_ENA` writer - The interrupt enable bit for CH6_ERR_INT."]
pub type CH6_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_ERR_INT_ENA` reader - The interrupt enable bit for CH7_ERR_INT."]
pub type CH7_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH7_ERR_INT_ENA` writer - The interrupt enable bit for CH7_ERR_INT."]
pub type CH7_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH4_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH4_RX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
pub type CH5_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
pub type CH5_RX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
pub type CH6_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
pub type CH6_RX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
pub type CH7_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
pub type CH7_RX_THR_EVENT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_ENA` reader - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_ENA` writer - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_ENA` reader - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_ENA` writer - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&self) -> CH0_TX_END_INT_ENA_R {
        CH0_TX_END_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&self) -> CH1_TX_END_INT_ENA_R {
        CH1_TX_END_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&self) -> CH2_TX_END_INT_ENA_R {
        CH2_TX_END_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&self) -> CH3_TX_END_INT_ENA_R {
        CH3_TX_END_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch0_err_int_ena(&self) -> TX_CH0_ERR_INT_ENA_R {
        TX_CH0_ERR_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch1_err_int_ena(&self) -> TX_CH1_ERR_INT_ENA_R {
        TX_CH1_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch2_err_int_ena(&self) -> TX_CH2_ERR_INT_ENA_R {
        TX_CH2_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch3_err_int_ena(&self) -> TX_CH3_ERR_INT_ENA_R {
        TX_CH3_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&self) -> CH0_TX_THR_EVENT_INT_ENA_R {
        CH0_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&self) -> CH1_TX_THR_EVENT_INT_ENA_R {
        CH1_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&self) -> CH2_TX_THR_EVENT_INT_ENA_R {
        CH2_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&self) -> CH3_TX_THR_EVENT_INT_ENA_R {
        CH3_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&self) -> CH0_TX_LOOP_INT_ENA_R {
        CH0_TX_LOOP_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&self) -> CH1_TX_LOOP_INT_ENA_R {
        CH1_TX_LOOP_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&self) -> CH2_TX_LOOP_INT_ENA_R {
        CH2_TX_LOOP_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&self) -> CH3_TX_LOOP_INT_ENA_R {
        CH3_TX_LOOP_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end_int_ena(&self) -> CH4_RX_END_INT_ENA_R {
        CH4_RX_END_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH5_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end_int_ena(&self) -> CH5_RX_END_INT_ENA_R {
        CH5_RX_END_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH6_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end_int_ena(&self) -> CH6_RX_END_INT_ENA_R {
        CH6_RX_END_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH7_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end_int_ena(&self) -> CH7_RX_END_INT_ENA_R {
        CH7_RX_END_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_err_int_ena(&self) -> CH4_ERR_INT_ENA_R {
        CH4_ERR_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH5_ERR_INT."]
    #[inline(always)]
    pub fn ch5_err_int_ena(&self) -> CH5_ERR_INT_ENA_R {
        CH5_ERR_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch6_err_int_ena(&self) -> CH6_ERR_INT_ENA_R {
        CH6_ERR_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH7_ERR_INT."]
    #[inline(always)]
    pub fn ch7_err_int_ena(&self) -> CH7_ERR_INT_ENA_R {
        CH7_ERR_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_ena(&self) -> CH4_RX_THR_EVENT_INT_ENA_R {
        CH4_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_ena(&self) -> CH5_RX_THR_EVENT_INT_ENA_R {
        CH5_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_ena(&self) -> CH6_RX_THR_EVENT_INT_ENA_R {
        CH6_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_ena(&self) -> CH7_RX_THR_EVENT_INT_ENA_R {
        CH7_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_ena(&self) -> TX_CH3_DMA_ACCESS_FAIL_INT_ENA_R {
        TX_CH3_DMA_ACCESS_FAIL_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_ena(&self) -> RX_CH7_DMA_ACCESS_FAIL_INT_ENA_R {
        RX_CH7_DMA_ACCESS_FAIL_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "ch0_tx_end_int_ena",
                &format_args!("{}", self.ch0_tx_end_int_ena().bit()),
            )
            .field(
                "ch1_tx_end_int_ena",
                &format_args!("{}", self.ch1_tx_end_int_ena().bit()),
            )
            .field(
                "ch2_tx_end_int_ena",
                &format_args!("{}", self.ch2_tx_end_int_ena().bit()),
            )
            .field(
                "ch3_tx_end_int_ena",
                &format_args!("{}", self.ch3_tx_end_int_ena().bit()),
            )
            .field(
                "tx_ch0_err_int_ena",
                &format_args!("{}", self.tx_ch0_err_int_ena().bit()),
            )
            .field(
                "tx_ch1_err_int_ena",
                &format_args!("{}", self.tx_ch1_err_int_ena().bit()),
            )
            .field(
                "tx_ch2_err_int_ena",
                &format_args!("{}", self.tx_ch2_err_int_ena().bit()),
            )
            .field(
                "tx_ch3_err_int_ena",
                &format_args!("{}", self.tx_ch3_err_int_ena().bit()),
            )
            .field(
                "ch0_tx_thr_event_int_ena",
                &format_args!("{}", self.ch0_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch1_tx_thr_event_int_ena",
                &format_args!("{}", self.ch1_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch2_tx_thr_event_int_ena",
                &format_args!("{}", self.ch2_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch3_tx_thr_event_int_ena",
                &format_args!("{}", self.ch3_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch0_tx_loop_int_ena",
                &format_args!("{}", self.ch0_tx_loop_int_ena().bit()),
            )
            .field(
                "ch1_tx_loop_int_ena",
                &format_args!("{}", self.ch1_tx_loop_int_ena().bit()),
            )
            .field(
                "ch2_tx_loop_int_ena",
                &format_args!("{}", self.ch2_tx_loop_int_ena().bit()),
            )
            .field(
                "ch3_tx_loop_int_ena",
                &format_args!("{}", self.ch3_tx_loop_int_ena().bit()),
            )
            .field(
                "ch4_rx_end_int_ena",
                &format_args!("{}", self.ch4_rx_end_int_ena().bit()),
            )
            .field(
                "ch5_rx_end_int_ena",
                &format_args!("{}", self.ch5_rx_end_int_ena().bit()),
            )
            .field(
                "ch6_rx_end_int_ena",
                &format_args!("{}", self.ch6_rx_end_int_ena().bit()),
            )
            .field(
                "ch7_rx_end_int_ena",
                &format_args!("{}", self.ch7_rx_end_int_ena().bit()),
            )
            .field(
                "ch4_err_int_ena",
                &format_args!("{}", self.ch4_err_int_ena().bit()),
            )
            .field(
                "ch5_err_int_ena",
                &format_args!("{}", self.ch5_err_int_ena().bit()),
            )
            .field(
                "ch6_err_int_ena",
                &format_args!("{}", self.ch6_err_int_ena().bit()),
            )
            .field(
                "ch7_err_int_ena",
                &format_args!("{}", self.ch7_err_int_ena().bit()),
            )
            .field(
                "ch4_rx_thr_event_int_ena",
                &format_args!("{}", self.ch4_rx_thr_event_int_ena().bit()),
            )
            .field(
                "ch5_rx_thr_event_int_ena",
                &format_args!("{}", self.ch5_rx_thr_event_int_ena().bit()),
            )
            .field(
                "ch6_rx_thr_event_int_ena",
                &format_args!("{}", self.ch6_rx_thr_event_int_ena().bit()),
            )
            .field(
                "ch7_rx_thr_event_int_ena",
                &format_args!("{}", self.ch7_rx_thr_event_int_ena().bit()),
            )
            .field(
                "tx_ch3_dma_access_fail_int_ena",
                &format_args!("{}", self.tx_ch3_dma_access_fail_int_ena().bit()),
            )
            .field(
                "rx_ch7_dma_access_fail_int_ena",
                &format_args!("{}", self.rx_ch7_dma_access_fail_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end_int_ena(&mut self) -> CH0_TX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH0_TX_END_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end_int_ena(&mut self) -> CH1_TX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH1_TX_END_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end_int_ena(&mut self) -> CH2_TX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH2_TX_END_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end_int_ena(&mut self) -> CH3_TX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH3_TX_END_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch0_err_int_ena(&mut self) -> TX_CH0_ERR_INT_ENA_W<INT_ENA_SPEC> {
        TX_CH0_ERR_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch1_err_int_ena(&mut self) -> TX_CH1_ERR_INT_ENA_W<INT_ENA_SPEC> {
        TX_CH1_ERR_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch2_err_int_ena(&mut self) -> TX_CH2_ERR_INT_ENA_W<INT_ENA_SPEC> {
        TX_CH2_ERR_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_err_int_ena(&mut self) -> TX_CH3_ERR_INT_ENA_W<INT_ENA_SPEC> {
        TX_CH3_ERR_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event_int_ena(&mut self) -> CH0_TX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH0_TX_THR_EVENT_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event_int_ena(&mut self) -> CH1_TX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH1_TX_THR_EVENT_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event_int_ena(&mut self) -> CH2_TX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH2_TX_THR_EVENT_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event_int_ena(&mut self) -> CH3_TX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH3_TX_THR_EVENT_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop_int_ena(&mut self) -> CH0_TX_LOOP_INT_ENA_W<INT_ENA_SPEC> {
        CH0_TX_LOOP_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop_int_ena(&mut self) -> CH1_TX_LOOP_INT_ENA_W<INT_ENA_SPEC> {
        CH1_TX_LOOP_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop_int_ena(&mut self) -> CH2_TX_LOOP_INT_ENA_W<INT_ENA_SPEC> {
        CH2_TX_LOOP_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop_int_ena(&mut self) -> CH3_TX_LOOP_INT_ENA_W<INT_ENA_SPEC> {
        CH3_TX_LOOP_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end_int_ena(&mut self) -> CH4_RX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH4_RX_END_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH5_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end_int_ena(&mut self) -> CH5_RX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH5_RX_END_INT_ENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH6_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end_int_ena(&mut self) -> CH6_RX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH6_RX_END_INT_ENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH7_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end_int_ena(&mut self) -> CH7_RX_END_INT_ENA_W<INT_ENA_SPEC> {
        CH7_RX_END_INT_ENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_err_int_ena(&mut self) -> CH4_ERR_INT_ENA_W<INT_ENA_SPEC> {
        CH4_ERR_INT_ENA_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH5_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_err_int_ena(&mut self) -> CH5_ERR_INT_ENA_W<INT_ENA_SPEC> {
        CH5_ERR_INT_ENA_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH6_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_err_int_ena(&mut self) -> CH6_ERR_INT_ENA_W<INT_ENA_SPEC> {
        CH6_ERR_INT_ENA_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH7_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_err_int_ena(&mut self) -> CH7_ERR_INT_ENA_W<INT_ENA_SPEC> {
        CH7_ERR_INT_ENA_W::new(self, 23)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event_int_ena(&mut self) -> CH4_RX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH4_RX_THR_EVENT_INT_ENA_W::new(self, 24)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event_int_ena(&mut self) -> CH5_RX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH5_RX_THR_EVENT_INT_ENA_W::new(self, 25)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event_int_ena(&mut self) -> CH6_RX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH6_RX_THR_EVENT_INT_ENA_W::new(self, 26)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event_int_ena(&mut self) -> CH7_RX_THR_EVENT_INT_ENA_W<INT_ENA_SPEC> {
        CH7_RX_THR_EVENT_INT_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail_int_ena(
        &mut self,
    ) -> TX_CH3_DMA_ACCESS_FAIL_INT_ENA_W<INT_ENA_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail_int_ena(
        &mut self,
    ) -> RX_CH7_DMA_ACCESS_FAIL_INT_ENA_W<INT_ENA_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_INT_ENA_W::new(self, 29)
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
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
