/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: intdispatcher                                                   ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Interrupt dispatching in Rust. The main function is 'int_disp'  ║
   ║         which is called for any interrupt and calls a registered ISR    ║
   ║         of device driver, e.g. the keyboard.                            ║
   ║                                                                         ║
   ║         'int_disp' is called from 'interrupts.asm' where all the x86    ║
   ║         low-level stuff is handled.                                     ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 7.3.2022                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
extern crate spin;

use core::borrow::Borrow;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::kernel::cpu;
use crate::kernel::interrupts::isr;
use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;

pub const INT_VEC_TIMER: usize = 32;
pub const INT_VEC_KEYBOARD: usize = 33;

/**
 Description:
	@@ -33,75 +11,48 @@ pub const INT_VEC_KEYBOARD: usize = 33;
*/
#[no_mangle]
pub extern "C" fn int_disp(vector: u32) {
    if is_initialized() == false {
        panic!("int_disp called but INT_VECTORS not initialized.");
    }
    if report(vector as usize) == false {
        kprint!("Panic: unexpected interrupt nr = {}", vector);
        kprint!(" - processor halted.");
        cpu::halt();
    }

    //kprint!("int_disp: vector = {}\n", vector);
}

const MAX_VEC_NUM: usize = 256;

static mut INT_VECTORS: Option<IntVectors> = None;
static INT_VECTORS_INITIALIZED: AtomicUsize = AtomicUsize::new(0);

//static INT_VECTORS: Mutex<IntVectors> = Mutex::new(IntVectors { map: Vec::new() });

// Interrupt vector map
struct IntVectors {
    map: Vec<Box<dyn isr::ISR>>,
}

// used in 'int_disp' to check if interrupt dispatching tables has been initialized
fn is_initialized() -> bool {
    let v = INT_VECTORS_INITIALIZED.load(Ordering::SeqCst);
    if v == 0 {
        return false;
    }
    return true;
}

// required by the compiler for gloabl 'INT_DISPATCHER'
unsafe impl Send for IntVectors {}
unsafe impl Sync for IntVectors {}

/**
 Description:
    Initializing the ISR map with MAX_VEC_NUM default ISRs.
    Specific ISRs can be overwritten by calling `assign`.
*/
pub fn init() {

    kprintln!("INT_VECTORS: init");
    unsafe {
        INT_VECTORS = Some(IntVectors { map: Vec::new() });

        for _ in 0..MAX_VEC_NUM {
            INT_VECTORS
                .as_mut()
                .unwrap()
                .map
                .push(Box::new(isr::Default));
        }
    }
    INT_VECTORS_INITIALIZED.store(1, Ordering::SeqCst);
}

/**
 Description:
    Register an ISR.
 Parameters: \
    `vector` vector number of interrupt
    `isr` the isr to be registered
*/
pub fn register(vector: usize, isr: Box<dyn isr::ISR>) -> bool {

    /* Hier muss Code eingefuegt werden */
    if vector >= MAX_VEC_NUM {
       return false;
    }
    let was_enabled = cpu::disable_int_nested();
 
    unsafe {
       INT_VECTORS.as_mut().unwrap().map[vector] = isr;
    }
 
    cpu::enable_int_nested(was_enabled);
    return true;
 
 }

/**
Description:
   Check if an ISR is registered for `vector`. If so, call it.
Parameters: \
   `vector` vector of the interrupt which was fired.
*/
pub fn report(vector: usize) -> bool {
    /* Hier muss Code eingefuegt werden */
    if vector >= MAX_VEC_NUM {
       return false;
    }
    unsafe {
       let vectors = INT_VECTORS.as_mut().unwrap();
       let ref isr = vectors.map[vector];
       if  !isr.is_default_isr(){
          isr.trigger();
          return true;
       } else {
          return false;
       }
    }

 }