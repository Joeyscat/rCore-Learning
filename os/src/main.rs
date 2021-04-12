#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod sbi;
mod syscall;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    // extern "C" {
    //     fn stext();
    //     fn etext();
    //     fn srodata();
    //     fn erodata();
    //     fn sdata();
    //     fn edata();
    //     fn sbss();
    //     fn ebss();
    //     fn boot_stack();
    //     fn boot_stack_top();
    // }

    clear_bss();
    println!("[kernel] Hello World!");
    trap::init();
    batch::init();
    batch::run_next_app();

    // trace!("Hello World!");
    // trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    // debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    // info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    // warn!(
    //     ".boot_stack [{:#x}, {:#x})",
    //     boot_stack as usize, boot_stack_top as usize
    // );
    // warn!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    // panic!("Shutdown machine!");
}

// 把 [.bss] 段的全局数据清零
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
