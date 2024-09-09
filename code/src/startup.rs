
/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: startup                                                         ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Here is the main function called first from the boot code as    ║
   ║         well as the panic handler. All features are set and all modules ║
   ║         are imported.                                                   ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoettner, Univ. Duesseldorf, 5.2.2024                 ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![no_std]
#![feature(const_mut_refs)]
#![allow(dead_code)] // avoid warnings
#![allow(unused_variables)] // avoid warnings
#![allow(unused_imports)]
#![allow(unused_macros)]

extern crate alloc;
extern crate spin; // we need a mutex in devices::cga_print

// insert other modules
#[macro_use]   // import macros, too
mod devices;
mod kernel;
mod user;
mod consts;
pub mod mylib;

use core::panic::PanicInfo;

use devices::cga;         // shortcut for cga
use devices::cga_print;   // used to import code needed by println! 
use devices::keyboard;    // shortcut for keyboard

use kernel::corouts;
use kernel::cpu;

use kernel::interrupts;
use kernel::interrupts::intdispatcher;
use kernel::interrupts::intdispatcher::int_disp;
use kernel::threads::idle_thread;
use kernel::threads::scheduler;
use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;

use kernel::allocator;

use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;
use user::aufgabe3::keyboard_irq_demo;

use user::aufgabe4::corouts_demo;
use user::aufgabe4::hello_world_thread;
use user::aufgabe4::coop_thread_demo;
use user::aufgabe4::coop_thread_loop;

use user::aufgabe5;

use user::aufgabe6;

use devices::vga;



fn aufgabe1() {
   cga::clear();
   text_demo::run();
   keyboard_demo::run();
}

fn aufgabe2() {
   //heap_demo::run();
   sound_demo::run();
}

fn aufgabe3() {
    cga::clear();
    keyboard_irq_demo::run();
}

fn aufgabe4() {
    //corouts_demo::run();
    //hello_world_thread::init();
    //coop_thread_demo::init();
    coop_thread_loop::init();	
}

// Pruefen, ob wir in einem Grafikmodus sind
// Falls ja setzen der Infos in VGA
fn check_graphics_mode(mbi: u64) -> bool {
    unsafe {
        let ptr = mbi;

        let flags = *(mbi as *mut u32);

        // 12 Bit in Flags zeigt an, ob Framebuffer-Infos vorhanden sind
        if flags & 0x1000 == 0 {
            return false;
        }

        let addr = *((mbi + 88) as *mut u64);
        let pitch = *((mbi + 96) as *mut u32);
        let width = *((mbi + 100) as *mut u32);
        let height = *((mbi + 104) as *mut u32);
        let bpp = *((mbi + 108) as *mut u8);
        vga::VGA::init(addr, pitch, width, height, bpp);
    }
    true
}

#[no_mangle]
pub extern "C" fn startup(mbi: u64){
	 

    kprintln!("OS is running ...");

    // Speicherverwaltung initialisieren
    allocator::init();

    // Multiboot-Infos für Grafik auslesen, falls vorhanden
    check_graphics_mode(mbi);

    //idle thread
    idle_thread::init();

    // init interrupts
    interrupts::init();

    // register keyboard ISR

    //plugin keyboard interrupt
    keyboard::Keyboard::plugin();

    //plugin pit interrupt
    devices::pit::plugin();
   
    // CPU enable ints
    cpu::enable_int();

	cga::clear();
    //aufgabe1();
    //aufgabe2();
    //aufgabe3();
    //aufgabe4();

    //aufgabe5::preem_thread_demo::init();

    //aufgabe6::semaphore_demo::init();

    scheduler::Scheduler::schedule();

    loop{}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}
