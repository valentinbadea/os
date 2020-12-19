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

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut gpio16 = Gpio::new(16).into_output();
    loop {
        gpio16.set();
        spin_sleep_ms(3000);
        gpio16.clear();
        spin_sleep_ms(3000);
    }
}
