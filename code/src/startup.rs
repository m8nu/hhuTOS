
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
#![feature(alloc_error_handler)]


extern crate alloc;
extern crate spin; // we need a mutex in devices::cga_print
extern crate x86;

// insert other modules
#[macro_use] // import macros, too
mod devices;
mod boot;
mod consts;
mod kernel;
mod mylib;
mod user;


use alloc::boxed::Box;
use boot::multiboot;
use core::panic::PanicInfo;

use devices::cga;         // shortcut for cga
use devices::cga_print;
use devices::key;         // used to import code needed by println! 
use devices::keyboard;    // shortcut for keyboard
use devices::pit;         // timer

use kernel::corouts;
use kernel::cpu;
use kernel::interrupts;
use kernel::interrupts::intdispatcher;
use kernel::interrupts::intdispatcher::int_disp;
use kernel::threads::idle_thread;
use kernel::threads::scheduler;
use kernel::threads::thread::Thread;
use kernel::allocator;

use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;

use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;
use user::aufgabe3::keyboard_irq_demo;

use user::aufgabe4::corouts_demo;
use user::aufgabe4::hello_world_thread;
use user::aufgabe4::coop_thread_demo;
use user::aufgabe4::coop_thread_loop;

use user::aufgabe5;

use user::aufgabe6;

use user::aufgabe6::semaphore_demo;
use user::aufgabe7;

use devices::vga;
use mylib::input::getch;

use crate::boot::multiboot::PhysRegion;

// Konstanten im Linker-Skript
extern "C" {
    static ___KERNEL_DATA_START__: u64;
    static ___KERNEL_DATA_END__: u64;
}

// Start- und Endadresse des Kernel-Images ermitteln,
// aufrunden auf das naechste volle MB und zurueckgeben
fn get_kernel_image_region() -> multiboot::PhysRegion {
    let kernel_start: usize;
    let kernel_end: usize;

    unsafe {
        kernel_start = &___KERNEL_DATA_START__ as *const u64 as usize;
        kernel_end = &___KERNEL_DATA_END__ as *const u64 as usize;
    }

    // Kernel-Image auf das naechste MB aufrunden
    let mut kernel_rounded_end = kernel_end & 0xFFFFFFFFFFF00000;
    kernel_rounded_end += 0x100000 - 1; // 1 MB aufaddieren

    PhysRegion {
        start: kernel_start as u64,
        end: kernel_rounded_end as u64,
    }
}

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

fn show_menu(){
    cga::clear();
    println!("1. Textausgabe und Tastatureingabe");
    println!("2. Sound abspielen");
    println!("3. Speicherverwaltung");
    println!("4. Preemptives Multitasking");


        let input = getch();
        if input == '1' as u8{
            aufgabe1();
        } else if input == '2' as u8 {
            aufgabe2();
        } else if input == '3' as u8 {
            heap_demo::run();
        } else if input == '4' as u8 {
            aufgabe6::semaphore_demo::init();
        } else {
            println!("ERR: Unbekannter input! System bitte neustarten");
        }

}

#[no_mangle]
pub extern "C" fn kmain(mbi: u64) {
    kprintln!("kmain");

    let kernel_region = get_kernel_image_region();
    kprintln!("   kernel_region: {:?}", kernel_region);

    // Speicherverwaltung (1 MB) oberhalb des Images initialisieren
    let heap_start = kernel_region.end as usize + 1;
    allocator::init(heap_start, consts::HEAP_SIZE);

    // Multiboot-Infos ausgeben
    multiboot::dump(mbi);

    // Interrupt-Strukturen initialisieren
    interrupts::init();

    // Tastatur-Unterbrechungsroutine 'einstoepseln'
    keyboard::Keyboard::plugin();

    // Zeitgeber-Unterbrechungsroutine 'einstoepseln'
    pit::plugin();

    // Idle-Thread eintragen
    let idle_thread = Thread::new(scheduler::next_thread_id(), idle_thread::idle_thread_entry, true);
    scheduler::Scheduler::ready(idle_thread);

    // HelloWorld-Thread eintragen
    let hello_world_thread = Thread::new(scheduler::next_thread_id(), hello_world_thread::hello_world_thread_entry, true);
    scheduler::Scheduler::ready(hello_world_thread);

    // Scheduler starten & Interrupts erlauben
    scheduler::Scheduler::schedule();
}

/* old startup function
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

    //show_menu();
    //aufgabe1();
    //aufgabe2();
    //aufgabe3();
    //aufgabe4();
    //aufgabe5::preem_thread_demo::init();
    //aufgabe6::semaphore_demo::init();
    //aufgabe7::game_of_life::init();


    scheduler::Scheduler::schedule();

    loop{}
}
*/


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}
