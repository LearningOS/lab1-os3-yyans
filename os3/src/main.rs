#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate log;

extern crate alloc;

#[macro_use]
mod console;
mod config;
mod heap_alloc;
mod lang_items;
mod loader;
mod logging;
mod sbi;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

use core::char::MAX;

use config::{MAX_SYSCALL_NUM, MAX_APP_NUM};
use lazy_static::*;

core::arch::global_asm!(include_str!("entry.asm"));
core::arch::global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    // println!("[kernel] Hello, world!");
    // heap_alloc::init_heap();
    // trap::init();
    // loader::load_apps();
    // trap::enable_timer_interrupt();
    // timer::set_next_trigger();
    // task::run_first_task();

    println!("{}", TEST.arr[0].h1[1]);

    panic!("Unreachable in rust_main!");
}

lazy_static! {
    pub static ref TEST: test2 = {
        test2 {
            arr: [test {
                h1: [1; MAX_SYSCALL_NUM],
            }; MAX_APP_NUM]
        }
    };
}

#[derive(Clone, Copy)]
pub struct test2 {
    pub arr: [test;MAX_APP_NUM],
}

#[derive(Clone, Copy)]
pub struct test {
    pub h1: [u32; MAX_SYSCALL_NUM],
}

