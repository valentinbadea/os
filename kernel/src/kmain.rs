#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(auto_traits)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(restricted_std)]
#![feature(negative_impls)]
extern crate pi;
extern crate stack_vec;

pub mod console;
pub mod lang_items;
pub mod mutex;
pub mod shell;

use pi::gpio::Gpio;
use pi::timer::spin_sleep_ms;
use pi::uart::MiniUart;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut uart = MiniUart::new();
    loop {
        let x = uart.read_byte();
        uart.write_byte(x);
    }
}
