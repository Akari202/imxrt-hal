// TODO: write general documentation and examples
// TODO: check about testing

use crate::ral;

/// A quadrature encoder counter
pub struct Qdc<const N: u8> {
    /// Registers for this QDC instance
    qdc: ral::enc::Instance<N>,
}

/// QDC1 alias
pub type Qdc1 = Qdc<1>;
/// QDC2 alias
pub type Qdc2 = Qdc<2>;
/// QDC3 alias
pub type Qdc3 = Qdc<3>;
/// QDC4 alias
pub type Qdc4 = Qdc<4>;

impl<const N: u8> Qdc<N> {
    /// Create a QDC counter from the RAL's ENC instance.
    ///
    /// When `new` returns, the QDC is reset
    pub fn new(qdc: ral::enc::Instance<N>) -> Self {
        Self { qdc }
    }

    /// Enable or disable the watchdog that monitors if motion is being recorded
    /// 2 successive counts indicate proper operation and will reset the timer
    pub fn set_watchdog_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, WDE: (enable as u16));
    }

    /// Indicates if the watchdog is enabled
    pub fn is_watchdog_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, WDE == 1)
    }

    /// Enable or disable interrupts when watchdog timesout
    pub fn set_watchdog_interrupt_on_timeout_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, DIE: (enable as u16));
    }

    /// Indicates if the watchdog timeout interrupt is enabled
    pub fn is_watchdog_interrupt_on_timeout_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, DIE == 1)
    }

    /// Clears the watchdog timeout flag
    pub fn clear_watchdog_timeout(&mut self) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, DIRQ: 1);
    }

    /// Indicates if the watchdog timeout flag is set
    /// flag will remain set until manually cleared or the watchdog is disabled
    pub fn is_watchdog_timeout(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, DIRQ == 1)
    }

    /// Sets the number of clock cycles before the watchdog timer triggers
    /// 2 successive counts will reset the timer
    pub fn set_watchdog_timeout_cycles(&mut self, cycles: u16) {
        ral::modify_reg!(ral::enc, self.qdc, WTR, WDOG: cycles);
    }

    /// Returns the total number of clock cycles before the watchdog timer triggers
    pub fn watchdog_timeout_cycles(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, WTR, WDOG)
    }

    /// Enable or disable counting in the reverse direction
    pub fn set_reverse_counting_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, REV: (enable as u16));
    }

    /// Indicates if reverse counting is enabled
    pub fn is_reverse_counting_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, REV == 1)
    }

    /// Enable or disable single phase counting mode
    /// when disabled the PHASEA and PHASEB channels will be treated as a two phase quadrature input
    /// when enabled a positive transition on channel a will trigger a count signal
    /// if reverse counting matches PHASEB it will count up, otherwise count down
    pub fn set_single_phase_counting_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, PH1: (enable as u16));
    }

    /// Indicates if single phase counting is diasbled
    pub fn is_single_phase_counting_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, PH1 == 1)
    }

    /// Enable or disable interrupts on HOME signal
    pub fn set_home_signal_interrupt_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, HIE: (enable as u16));
    }

    /// Indicates if the HOME signal interrupt is enabled
    pub fn is_home_signal_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, HIE == 1)
    }

    /// Clears the HOME signal interrupt flag
    pub fn clear_home_signal_interrupt(&mut self) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, HIRQ: 1);
    }

    /// Indicates if the HOME signal interrupt flag is set
    /// flag will remain set until manually cleared
    /// will not trigger without HOME signal interrupts enabled
    pub fn is_home_signal_interrupt_set(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, HIRQ == 1)
    }

    /// Enable or disable triggering on the negative-going edge of the HOME signal
    /// one or the other, cannot trigger on both
    pub fn set_home_signal_negative_edge_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, HNE: (enable as u16));
    }

    /// Indicates if the HOME signal will trigger on negative-going edge
    pub fn is_home_signal_negative_edge_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, HNE == 1)
    }

    /// Enable or disable using HOME to initialize position counters UPOS and LPOS
    /// (can also be done with INDEX)
    pub fn set_home_initialize_position_counter_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, HIP: (enable as u16));
    }

    /// Indicates if the HOME signal will initialize the position counter
    pub fn is_home_initialize_position_counter_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, HIP == 1)
    }

    /// Initialize the position counter to a set value
    /// by first setting the initialization registers
    pub fn initialize_position_counter_to_value(&mut self, value: u32) {
        self.set_position_initialization_value(value);
        ral::modify_reg!(ral::enc, self.qdc, CTRL, SWIP: 1);
    }

    /// Set the initialization registers for the position counter
    /// without initializing the counter
    pub fn set_position_initialization_value(&mut self, value: u32) {
        ral::modify_reg!(ral::enc, self.qdc, UINIT, INIT: ((value >> 16) as u16));
        ral::modify_reg!(ral::enc, self.qdc, LINIT, INIT: (value as u16));
    }

    /// Get the position initialization value
    pub fn position_initialization_value(&mut self) -> u32 {
        let upper_value: u16 = ral::read_reg!(ral::enc, self.qdc, UINIT, INIT);
        let lower_value: u16 = ral::read_reg!(ral::enc, self.qdc, LINIT, INIT);
        (upper_value as u32) << 16 | (lower_value as u32)
    }

    /// Enable or disable interrupts on INDEX signal
    pub fn set_index_signal_interrupt_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, XIE: (enable as u16));
    }

    /// Indicates if the INDEX signal interrupt is enabled
    pub fn is_index_signal_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, XIE == 1)
    }

    /// Clears the INDEX signal interrupt flag
    pub fn clear_index_signal_interrupt(&mut self) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, XIRQ: 1);
    }

    /// Indicates if the INDEX signal interrupt flag is set
    /// flag will remain set until manually cleared
    /// will not trigger without INDEX signal interrupts enabled
    pub fn is_index_signal_interrupt_set(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, XIRQ == 1)
    }

    /// Enable or disable triggering on the negative-going edge of the INDEX signal
    /// one or the other, cannot trigger on both
    /// INDEX is used to count revolutions
    pub fn set_index_signal_negative_edge_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, XNE: (enable as u16));
    }

    /// Indicates if the INDEX signal will trigger on negative-going edge
    pub fn is_index_signal_negative_edge_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, XNE == 1)
    }

    /// Enable or disable using INDEX to initialize position counters UPOS and LPOS
    /// (can also be done with HOME)
    pub fn set_index_initialize_position_counter_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, XIP: (enable as u16));
    }

    /// Indicates if the INDEX signal will initialize the position counter
    pub fn is_index_initialize_position_counter_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, XIP == 1)
    }

    /// Enable or disable interrupts on compare
    pub fn set_compare_interrupt_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, CMPIE: (enable as u16));
    }

    /// Indicates if the compare interrupt is enabled
    pub fn is_compare_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, CMPIE == 1)
    }

    /// Clears the compare interrupt flag
    pub fn clear_compare_interrupt(&mut self) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL, CMPIRQ: 1);
    }

    /// Indicates if the compare interrupt flag is set
    /// flag will remain set until manually cleared
    pub fn is_compare_interrupt_set(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL, CMPIRQ == 1)
    }

    /// Set the registers for compare
    pub fn set_compare_value(&mut self, value: u32) {
        ral::modify_reg!(ral::enc, self.qdc, UCOMP, COMP: ((value >> 16) as u16));
        ral::modify_reg!(ral::enc, self.qdc, LCOMP, COMP: (value as u16));
    }

    /// Get the value used for compare
    pub fn compare_value(&mut self) -> u32 {
        let upper_value: u16 = ral::read_reg!(ral::enc, self.qdc, UCOMP, COMP);
        let lower_value: u16 = ral::read_reg!(ral::enc, self.qdc, LCOMP, COMP);
        (upper_value as u32) << 16 | (lower_value as u32)
    }

    /// Returns the clock prescaler value
    /// the clock is divided by 2^prescaler
    pub fn prescaler(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, FILT, FILT_PRSC)
    }

    /// Set the prescaler value
    /// the prescaler value is clamped between 1 and 128
    /// the clock is divided by 2^prescaler
    pub fn set_prescaler(&mut self, prescaler: u16) {
        let clamped_prescaler: u16 = prescaler.clamp(1, 128);
        ral::modify_reg!(ral::enc, self.qdc, FILT, FILT_PRSC: clamped_prescaler);
    }

    /// Returns the input filter sample count
    pub fn input_filter_count(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, FILT, FILT_CNT) + 3
    }

    /// Set the input filter count
    /// the filter value is clamped between 3 and 10
    /// the number of consecutive samples that must agree, before the input filter accepts an input transition
    /// effects input latency
    pub fn set_input_filter_count(&mut self, value: u16) {
        let clamped_value: u16 = value.clamp(3, 10) - 3;
        ral::modify_reg!(ral::enc, self.qdc, FILT, FILT_CNT: clamped_value);
    }

    /// Disable the input filter
    /// this will clear any value set in the input filter sample period
    pub fn disable_input_filter(&mut self) {
        ral::modify_reg!(ral::enc, self.qdc, FILT, FILT_PER: 0);
    }

    /// Gets the sampling period of the input filter in clock cycles
    /// 0 means the input filter is disabled
    pub fn input_sampling_period(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, FILT, FILT_PER)
    }

    /// Set the input sampling period in clock cycles
    /// effects input latency
    pub fn set_input_sampling_period(&mut self, value: u8) {
        if ral::read_reg!(ral::enc, self.qdc, FILT, FILT_PER != 0) {
            ral::modify_reg!(ral::enc, self.qdc, FILT, FILT_PER: 0);
        }
        ral::modify_reg!(ral::enc, self.qdc, FILT, FILT_PER: (value as u16));
    }

    /// Gets the position difference
    /// when the position, revolution, or difference counter is read
    /// this value is copied to the hold register and cleared
    pub fn position_difference(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, POSD, POSD)
    }

    /// Gets the previous position difference
    /// contains a snapshot of the last time the diffrence was read
    /// usefull for calculating velocity
    pub fn previous_position_difference(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, POSDH, POSDH)
    }

    /// Gets the revolution count
    /// revolutions are triggered with INDEX
    /// when the position, revolution, or difference counter is read
    /// this value is copied to the hold register and cleared
    pub fn revolution_count(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, REV, REV)
    }

    /// Gets the previous revolution count
    /// contains a snapshot of the last time the revolution count was read
    pub fn previous_revolution_count(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, REVH, REVH)
    }

    /// Gets the position count
    /// when the position, revolution, or difference counter is read
    /// the count is copied to the hold register and cleared
    pub fn position_count(&mut self) -> u32 {
        let upper_value: u16 = ral::read_reg!(ral::enc, self.qdc, UPOS, POS);
        let lower_value: u16 = ral::read_reg!(ral::enc, self.qdc, LPOS, POS);
        (upper_value as u32) << 16 | (lower_value as u32)
    }

    /// Gets the previous revolution count
    /// contains a snapshot of the last time the revolution count was read
    pub fn previous_position_count(&mut self) -> u32 {
        let upper_value: u16 = ral::read_reg!(ral::enc, self.qdc, UPOSH, POSH);
        let lower_value: u16 = ral::read_reg!(ral::enc, self.qdc, LPOSH, POSH);
        (upper_value as u32) << 16 | (lower_value as u32)
    }

    /// Gets the direction of the most recent count
    /// returns true if most recent count was in the up direction
    pub fn count_direction(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL2, DIR == 1)
    }

    /// Gets the raw and filtered input values
    /// values are given as a u8 whose binary is ordered as such
    /// filtered PHASEA, filtered PHASEB, filtered INDEX, filtered HOME, raw PHASEA, raw PHASEB, raw INDEX, raw HOME
    pub fn input_monitor(&self) -> u8 {
        ral::read_reg!(ral::enc, self.qdc, IMR) as u8
    }

    /// Enable or disable clearing position, revolution, and difference values
    /// on rising endge of TRIGGER
    pub fn set_trigger_clear_primary_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL2, UPDPOS: (enable as u16));
    }

    /// Indicates if clearing clearing position, revolution, and difference values
    /// on TRIGGER is enabled
    pub fn is_trigger_clear_primary_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL2, UPDPOS == 1)
    }

    /// Enable or disable updating position, revolution, and difference previous values
    /// on rising endge of TRIGGER
    pub fn set_trigger_update_previous_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL2, UPDHLD: (enable as u16));
    }

    /// Indicates if updating clearing position, revolution, and difference previous values
    /// on TRIGGER is enabled
    pub fn is_trigger_update_previous_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL2, UPDHLD == 1)
    }

    /// Enable or disable test mode
    /// test mode can generate a quadrature signal sent directly to the counter
    pub fn set_test_mode_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, TST, TEN: (enable as u16));
    }

    /// Indicates if test mode is enabled
    pub fn is_test_mode_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, TST, TEN == 1)
    }

    /// Enable or disable test mode counter
    /// the test module wont send signals until this is enabled
    pub fn set_test_counter_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, TST, TCE: (enable as u16));
    }

    /// Indicates if the test mode counter is enabled
    pub fn is_test_counter_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, TST, TCE == 1)
    }

    /// Enable or disable test mode counting in the negative direction
    pub fn set_test_reverse_mode_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, TST, QDN: (enable as u16));
    }

    /// Indicates if the test mode counter counts in the negative direction
    pub fn is_test_reverse_mode_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, TST, QDN == 1)
    }

    /// Returns the number of qudrature pulses the test module will generate when the counter enabled
    pub fn test_pulse_count(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, TST, TEST_COUNT)
    }

    /// Set the number of qudrature pulses the test module will generate when the counter enabled
    /// the value is clamped between 0 and 255
    pub fn set_test_pulse_count(&mut self, value: u16) {
        let clamped_value: u16 = value.clamp(0, 255);
        ral::modify_reg!(ral::enc, self.qdc, TST, TEST_COUNT: clamped_value);
    }

    /// Returns the period in clock cycles of the test modules qudrature output
    pub fn test_pulse_period(&self) -> u16 {
        ral::read_reg!(ral::enc, self.qdc, TST, TEST_PERIOD)
    }

    /// Set the period in clock cycles of the test modules qudrature output
    /// the value is clamped between 0 and 32
    pub fn set_test_pulse_period(&mut self, value: u16) {
        let clamped_value: u16 = value.clamp(0, 32);
        ral::modify_reg!(ral::enc, self.qdc, TST, TEST_PERIOD: clamped_value);
    }

    /// Enable or disable interrupts on modulus rollunder
    /// TODO: interrupt will trigger when
    pub fn set_modulus_rollunder_interrupt_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL2, RUIE: (enable as u16));
    }

    /// Indicates if the modulus rollunder interrupt is enabled
    pub fn is_modulus_rollunder_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL2, RUIE == 1)
    }

    /// Clears the modulus rollunder interrupt flag
    pub fn clear_modulus_rollunder_interrupt(&mut self) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL2, RUIRQ: 1);
    }

    /// Indicates if the modulus rollunder interrupt flag is set
    /// flag will remain set until manually cleared
    pub fn is_modulus_rollunder_interrupt_set(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL2, RUIRQ == 1)
    }

    /// Enable or disable interrupts on modulus rollover
    /// TODO: interrupt will trigger when
    pub fn set_modulus_rollover_interrupt_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL2, ROIE: (enable as u16));
    }

    /// Indicates if the modulus rollover interrupt is enabled
    pub fn is_modulus_rollover_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL2, ROIE == 1)
    }

    /// Clears the modulus rollover interrupt flag
    pub fn clear_modulus_rollover_interrupt(&mut self) {
        ral::modify_reg!(ral::enc, self.qdc, CTRL2, ROIRQ: 1);
    }

    /// Indicates if the modulus rollover interrupt flag is set
    /// flag will remain set until manually cleared
    pub fn is_modulus_rollover_interrupt_set(&self) -> bool {
        ral::read_reg!(ral::enc, self.qdc, CTRL2, ROIRQ == 1)
    }

    // TODO: Add configuration for CTRL2 and TST

    /// Release the peripheral instance.
    ///
    /// This does not change any peripheral state; it simply releases
    /// the instance as-is. If you need to return the registers in a known,
    /// good state, consider calling [`reset()`](Self::reset) and
    /// [`disable()`](Self::disable) before this call.
    pub fn release(self) -> ral::enc::Instance<N> {
        self.qdc
    }
}
