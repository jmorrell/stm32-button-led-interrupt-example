#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use stm32f4xx_hal as hal;

use crate::hal::{
    gpio::{gpioa::PA0, gpioc::PC13, Edge, Input, Output, PullUp, PushPull},
    interrupt, pac,
    pac::Interrupt,
    prelude::*,
};
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

// Set up global state. It's all mutexed up for concurrency safety.
static BUTTON: Mutex<RefCell<Option<PA0<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<PC13<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    if let (Some(mut dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // set up clocks
        let rcc = dp.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // This is necessary for some reason
        let mut syscfg = dp.SYSCFG.constrain();

        // Set up the IO. We need ports in blocks A and C
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // On the WeAct v3.1 Black Pill the LED is connected to pin PC13.
        let mut board_led = gpioc.pc13.into_push_pull_output();
        board_led.set_high();

        // On the WeAct v3.1 Black Pill the User button is connected to pin PA0
        let mut board_button = gpioa.pa0.into_pull_up_input();
        board_button.make_interrupt_source(&mut syscfg);
        board_button.enable_interrupt(&mut dp.EXTI);
        board_button.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        // Replace the None in the global state with a reference to these IO ports
        free(|cs| {
            BUTTON.borrow(cs).replace(Some(board_button));
            LED.borrow(cs).replace(Some(board_led));
        });

        // Why is this EXTI0 instead of EXTI1 or EXTI9_5?
        // Because we are using port 0, which is connected to line 0
        // See: https://stm32f4-discovery.net/2014/08/stm32f4-external-interrupts-tutorial/
        // NVIC = Nested Vector Interrupt Controller
        // NVIC::unpend clear the interrupts PENDING state
        pac::NVIC::unpend(Interrupt::EXTI0);
        // NVIC::unmask enable the interrupt
        unsafe {
            pac::NVIC::unmask(Interrupt::EXTI0);
        };

        rprintln!("registered interupt!");
    }

    loop {}
}

#[interrupt]
fn EXTI0() {
    // within an interrupt-protected critical section
    free(|cs| {
        // mutably borrow the gobal references to the inputs
        let mut btn_ref = BUTTON.borrow(cs).borrow_mut();
        let mut led_ref = LED.borrow(cs).borrow_mut();
        // try to dereference
        if let (Some(ref mut btn), Some(ref mut led)) = (btn_ref.deref_mut(), led_ref.deref_mut()) {
            // clear so we don't keep getting called
            btn.clear_interrupt_pending_bit();

            // set the LED to match the button
            if btn.is_low() {
                led.set_low();
            } else {
                led.set_high();
            }
        }
    });
}
