#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    conf: CONF,
    unit0_op: UNIT0_OP,
    unit1_op: UNIT1_OP,
    unit0_load_hi: UNIT0_LOAD_HI,
    unit0_load_lo: UNIT0_LOAD_LO,
    unit1_load_hi: UNIT1_LOAD_HI,
    unit1_load_lo: UNIT1_LOAD_LO,
    target0_hi: TARGET0_HI,
    target0_lo: TARGET0_LO,
    target1_hi: TARGET1_HI,
    target1_lo: TARGET1_LO,
    target2_hi: TARGET2_HI,
    target2_lo: TARGET2_LO,
    target0_conf: TARGET0_CONF,
    target1_conf: TARGET1_CONF,
    target2_conf: TARGET2_CONF,
    unit0_value_hi: UNIT0_VALUE_HI,
    unit0_value_lo: UNIT0_VALUE_LO,
    unit1_value_hi: UNIT1_VALUE_HI,
    unit1_value_lo: UNIT1_VALUE_LO,
    comp0_load: COMP0_LOAD,
    comp1_load: COMP1_LOAD,
    comp2_load: COMP2_LOAD,
    unit0_load: UNIT0_LOAD,
    unit1_load: UNIT1_LOAD,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    _reserved29: [u8; 0x88],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Configure system timer clock"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04 - system timer unit0 value update register"]
    #[inline(always)]
    pub const fn unit0_op(&self) -> &UNIT0_OP {
        &self.unit0_op
    }
    #[doc = "0x08 - system timer unit1 value update register"]
    #[inline(always)]
    pub const fn unit1_op(&self) -> &UNIT1_OP {
        &self.unit1_op
    }
    #[doc = "0x0c - system timer unit0 value high load register"]
    #[inline(always)]
    pub const fn unit0_load_hi(&self) -> &UNIT0_LOAD_HI {
        &self.unit0_load_hi
    }
    #[doc = "0x10 - system timer unit0 value low load register"]
    #[inline(always)]
    pub const fn unit0_load_lo(&self) -> &UNIT0_LOAD_LO {
        &self.unit0_load_lo
    }
    #[doc = "0x14 - system timer unit1 value high load register"]
    #[inline(always)]
    pub const fn unit1_load_hi(&self) -> &UNIT1_LOAD_HI {
        &self.unit1_load_hi
    }
    #[doc = "0x18 - system timer unit1 value low load register"]
    #[inline(always)]
    pub const fn unit1_load_lo(&self) -> &UNIT1_LOAD_LO {
        &self.unit1_load_lo
    }
    #[doc = "0x1c - system timer comp0 value high register"]
    #[inline(always)]
    pub const fn target0_hi(&self) -> &TARGET0_HI {
        &self.target0_hi
    }
    #[doc = "0x20 - system timer comp0 value low register"]
    #[inline(always)]
    pub const fn target0_lo(&self) -> &TARGET0_LO {
        &self.target0_lo
    }
    #[doc = "0x24 - system timer comp1 value high register"]
    #[inline(always)]
    pub const fn target1_hi(&self) -> &TARGET1_HI {
        &self.target1_hi
    }
    #[doc = "0x28 - system timer comp1 value low register"]
    #[inline(always)]
    pub const fn target1_lo(&self) -> &TARGET1_LO {
        &self.target1_lo
    }
    #[doc = "0x2c - system timer comp2 value high register"]
    #[inline(always)]
    pub const fn target2_hi(&self) -> &TARGET2_HI {
        &self.target2_hi
    }
    #[doc = "0x30 - system timer comp2 value low register"]
    #[inline(always)]
    pub const fn target2_lo(&self) -> &TARGET2_LO {
        &self.target2_lo
    }
    #[doc = "0x34 - system timer comp0 target mode register"]
    #[inline(always)]
    pub const fn target0_conf(&self) -> &TARGET0_CONF {
        &self.target0_conf
    }
    #[doc = "0x38 - system timer comp1 target mode register"]
    #[inline(always)]
    pub const fn target1_conf(&self) -> &TARGET1_CONF {
        &self.target1_conf
    }
    #[doc = "0x3c - system timer comp2 target mode register"]
    #[inline(always)]
    pub const fn target2_conf(&self) -> &TARGET2_CONF {
        &self.target2_conf
    }
    #[doc = "0x40 - system timer unit0 value high register"]
    #[inline(always)]
    pub const fn unit0_value_hi(&self) -> &UNIT0_VALUE_HI {
        &self.unit0_value_hi
    }
    #[doc = "0x44 - system timer unit0 value low register"]
    #[inline(always)]
    pub const fn unit0_value_lo(&self) -> &UNIT0_VALUE_LO {
        &self.unit0_value_lo
    }
    #[doc = "0x48 - system timer unit1 value high register"]
    #[inline(always)]
    pub const fn unit1_value_hi(&self) -> &UNIT1_VALUE_HI {
        &self.unit1_value_hi
    }
    #[doc = "0x4c - system timer unit1 value low register"]
    #[inline(always)]
    pub const fn unit1_value_lo(&self) -> &UNIT1_VALUE_LO {
        &self.unit1_value_lo
    }
    #[doc = "0x50 - system timer comp0 conf sync register"]
    #[inline(always)]
    pub const fn comp0_load(&self) -> &COMP0_LOAD {
        &self.comp0_load
    }
    #[doc = "0x54 - system timer comp1 conf sync register"]
    #[inline(always)]
    pub const fn comp1_load(&self) -> &COMP1_LOAD {
        &self.comp1_load
    }
    #[doc = "0x58 - system timer comp2 conf sync register"]
    #[inline(always)]
    pub const fn comp2_load(&self) -> &COMP2_LOAD {
        &self.comp2_load
    }
    #[doc = "0x5c - system timer unit0 conf sync register"]
    #[inline(always)]
    pub const fn unit0_load(&self) -> &UNIT0_LOAD {
        &self.unit0_load
    }
    #[doc = "0x60 - system timer unit1 conf sync register"]
    #[inline(always)]
    pub const fn unit1_load(&self) -> &UNIT1_LOAD {
        &self.unit1_load
    }
    #[doc = "0x64 - systimer interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x68 - systimer interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x6c - systimer interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x70 - systimer interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xfc - system timer version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF (rw) register accessor: Configure system timer clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configure system timer clock"]
pub mod conf;
#[doc = "UNIT0_OP (rw) register accessor: system timer unit0 value update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit0_op::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit0_op::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_op`] module"]
pub type UNIT0_OP = crate::Reg<unit0_op::UNIT0_OP_SPEC>;
#[doc = "system timer unit0 value update register"]
pub mod unit0_op;
#[doc = "UNIT1_OP (rw) register accessor: system timer unit1 value update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_op::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit1_op::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_op`] module"]
pub type UNIT1_OP = crate::Reg<unit1_op::UNIT1_OP_SPEC>;
#[doc = "system timer unit1 value update register"]
pub mod unit1_op;
#[doc = "UNIT0_LOAD_HI (rw) register accessor: system timer unit0 value high load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit0_load_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit0_load_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_load_hi`] module"]
pub type UNIT0_LOAD_HI = crate::Reg<unit0_load_hi::UNIT0_LOAD_HI_SPEC>;
#[doc = "system timer unit0 value high load register"]
pub mod unit0_load_hi;
#[doc = "UNIT0_LOAD_LO (rw) register accessor: system timer unit0 value low load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit0_load_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit0_load_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_load_lo`] module"]
pub type UNIT0_LOAD_LO = crate::Reg<unit0_load_lo::UNIT0_LOAD_LO_SPEC>;
#[doc = "system timer unit0 value low load register"]
pub mod unit0_load_lo;
#[doc = "UNIT1_LOAD_HI (rw) register accessor: system timer unit1 value high load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_load_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit1_load_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_load_hi`] module"]
pub type UNIT1_LOAD_HI = crate::Reg<unit1_load_hi::UNIT1_LOAD_HI_SPEC>;
#[doc = "system timer unit1 value high load register"]
pub mod unit1_load_hi;
#[doc = "UNIT1_LOAD_LO (rw) register accessor: system timer unit1 value low load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_load_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit1_load_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_load_lo`] module"]
pub type UNIT1_LOAD_LO = crate::Reg<unit1_load_lo::UNIT1_LOAD_LO_SPEC>;
#[doc = "system timer unit1 value low load register"]
pub mod unit1_load_lo;
#[doc = "TARGET0_HI (rw) register accessor: system timer comp0 value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target0_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target0_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target0_hi`] module"]
pub type TARGET0_HI = crate::Reg<target0_hi::TARGET0_HI_SPEC>;
#[doc = "system timer comp0 value high register"]
pub mod target0_hi;
#[doc = "TARGET0_LO (rw) register accessor: system timer comp0 value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target0_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target0_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target0_lo`] module"]
pub type TARGET0_LO = crate::Reg<target0_lo::TARGET0_LO_SPEC>;
#[doc = "system timer comp0 value low register"]
pub mod target0_lo;
#[doc = "TARGET1_HI (rw) register accessor: system timer comp1 value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target1_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target1_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target1_hi`] module"]
pub type TARGET1_HI = crate::Reg<target1_hi::TARGET1_HI_SPEC>;
#[doc = "system timer comp1 value high register"]
pub mod target1_hi;
#[doc = "TARGET1_LO (rw) register accessor: system timer comp1 value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target1_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target1_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target1_lo`] module"]
pub type TARGET1_LO = crate::Reg<target1_lo::TARGET1_LO_SPEC>;
#[doc = "system timer comp1 value low register"]
pub mod target1_lo;
#[doc = "TARGET2_HI (rw) register accessor: system timer comp2 value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target2_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target2_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target2_hi`] module"]
pub type TARGET2_HI = crate::Reg<target2_hi::TARGET2_HI_SPEC>;
#[doc = "system timer comp2 value high register"]
pub mod target2_hi;
#[doc = "TARGET2_LO (rw) register accessor: system timer comp2 value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target2_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target2_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target2_lo`] module"]
pub type TARGET2_LO = crate::Reg<target2_lo::TARGET2_LO_SPEC>;
#[doc = "system timer comp2 value low register"]
pub mod target2_lo;
#[doc = "TARGET0_CONF (rw) register accessor: system timer comp0 target mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target0_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target0_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target0_conf`] module"]
pub type TARGET0_CONF = crate::Reg<target0_conf::TARGET0_CONF_SPEC>;
#[doc = "system timer comp0 target mode register"]
pub mod target0_conf;
#[doc = "TARGET1_CONF (rw) register accessor: system timer comp1 target mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target1_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target1_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target1_conf`] module"]
pub type TARGET1_CONF = crate::Reg<target1_conf::TARGET1_CONF_SPEC>;
#[doc = "system timer comp1 target mode register"]
pub mod target1_conf;
#[doc = "TARGET2_CONF (rw) register accessor: system timer comp2 target mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target2_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target2_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target2_conf`] module"]
pub type TARGET2_CONF = crate::Reg<target2_conf::TARGET2_CONF_SPEC>;
#[doc = "system timer comp2 target mode register"]
pub mod target2_conf;
#[doc = "UNIT0_VALUE_HI (r) register accessor: system timer unit0 value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit0_value_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_value_hi`] module"]
pub type UNIT0_VALUE_HI = crate::Reg<unit0_value_hi::UNIT0_VALUE_HI_SPEC>;
#[doc = "system timer unit0 value high register"]
pub mod unit0_value_hi;
#[doc = "UNIT0_VALUE_LO (r) register accessor: system timer unit0 value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit0_value_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_value_lo`] module"]
pub type UNIT0_VALUE_LO = crate::Reg<unit0_value_lo::UNIT0_VALUE_LO_SPEC>;
#[doc = "system timer unit0 value low register"]
pub mod unit0_value_lo;
#[doc = "UNIT1_VALUE_HI (r) register accessor: system timer unit1 value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_value_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_value_hi`] module"]
pub type UNIT1_VALUE_HI = crate::Reg<unit1_value_hi::UNIT1_VALUE_HI_SPEC>;
#[doc = "system timer unit1 value high register"]
pub mod unit1_value_hi;
#[doc = "UNIT1_VALUE_LO (r) register accessor: system timer unit1 value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_value_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_value_lo`] module"]
pub type UNIT1_VALUE_LO = crate::Reg<unit1_value_lo::UNIT1_VALUE_LO_SPEC>;
#[doc = "system timer unit1 value low register"]
pub mod unit1_value_lo;
#[doc = "COMP0_LOAD (w) register accessor: system timer comp0 conf sync register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_load`] module"]
pub type COMP0_LOAD = crate::Reg<comp0_load::COMP0_LOAD_SPEC>;
#[doc = "system timer comp0 conf sync register"]
pub mod comp0_load;
#[doc = "COMP1_LOAD (w) register accessor: system timer comp1 conf sync register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_load`] module"]
pub type COMP1_LOAD = crate::Reg<comp1_load::COMP1_LOAD_SPEC>;
#[doc = "system timer comp1 conf sync register"]
pub mod comp1_load;
#[doc = "COMP2_LOAD (w) register accessor: system timer comp2 conf sync register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_load`] module"]
pub type COMP2_LOAD = crate::Reg<comp2_load::COMP2_LOAD_SPEC>;
#[doc = "system timer comp2 conf sync register"]
pub mod comp2_load;
#[doc = "UNIT0_LOAD (w) register accessor: system timer unit0 conf sync register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit0_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_load`] module"]
pub type UNIT0_LOAD = crate::Reg<unit0_load::UNIT0_LOAD_SPEC>;
#[doc = "system timer unit0 conf sync register"]
pub mod unit0_load;
#[doc = "UNIT1_LOAD (w) register accessor: system timer unit1 conf sync register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit1_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_load`] module"]
pub type UNIT1_LOAD = crate::Reg<unit1_load::UNIT1_LOAD_SPEC>;
#[doc = "system timer unit1 conf sync register"]
pub mod unit1_load;
#[doc = "INT_ENA (rw) register accessor: systimer interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "systimer interrupt enable register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: systimer interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "systimer interrupt raw register"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: systimer interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "systimer interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: systimer interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "systimer interrupt status register"]
pub mod int_st;
#[doc = "DATE (rw) register accessor: system timer version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "system timer version control register"]
pub mod date;
