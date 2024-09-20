/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pit                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Programmable Interval Timer.                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 15.6.2023                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![allow(dead_code)]

use alloc::boxed::Box;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::interrupts::intdispatcher;
use crate::kernel::interrupts::isr;
use crate::kernel::interrupts::pic;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::scheduler::SCHEDULER;
use crate::kernel::threads::thread;

use super::cga::setpos;
use super::kprint;

// read systime
pub fn get_systime() -> u64 {
    SYS_TIME.load(Ordering::SeqCst)
}

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;

// system time ticks (each 10ms one incremented)
static SYS_TIME: AtomicU64 = AtomicU64::new(0);

// index for displaying spinner
static SYS_TIME_DISPLAY: AtomicUsize = AtomicUsize::new(0);

/**
  Description: Configure pit to fire an interrupt after `x` microseconds. \

*/
pub fn interval(x: u32) {

    /* Hier muss Code eingefuegt werden */
    cpu::outb(PORT_CTRL, 0b00110110); //Counter 0 Mod 3

    cpu::outb(PORT_DATA0, ((x*1193) & 0xff) as u8);
    cpu::outb(PORT_DATA0, (((x*1193) >> 8) & 0xff) as u8);

}

/**
 Description: Configure pit using `interval` to fire an interrupt each 10ms.  \
              Then register `trigger` in interrupt dispatcher and allow the \
              timer IRQ in the PIC.

 Parameters: \
            `f` frequency of musical note \
            `d` duration in ms
*/
pub fn plugin() {

   /* Hier muss Code eingefuegt werden */

    // Register the ISR in the interrupt dispatcher
    let pit_isr = Box::new(PitISR {});
    
    intdispatcher::register(0x20, pit_isr);

    // Allow the timer IRQ in the PIC
    pic::allow(pic::IRQ_TIMER);

    // Start the pit
    interval(10);
}

struct PitISR;

impl isr::ISR for PitISR {
    /**
     Description: ISR of the pit.
    */
    fn trigger(&self) {
           
        // progress system time by one tick
        let time = SYS_TIME.fetch_add(1, Ordering::SeqCst);

        /* Hier muss Code eingefuegt werden */

        // Rotate the spinner each 100 ticks. One tick is 10ms, so the spinner
        // rotates 360 degress in about 1s
        let s: [char; 4] = ['|', '/', '-', '\\'];
        if time % 100 == 0 {
            let time_display = SYS_TIME_DISPLAY.fetch_add(1, Ordering::SeqCst);
            let (x,y) = cga::getpos();
            cga::setpos(79, 0);
            cga::print_byte(s[time_display % 4] as u8);
            cga::setpos(x, y);
        
        }
        /* Hier muss Code eingefuegt werden */

        // We try to switch to the next thread 
        let opt= scheduler::SCHEDULER.try_lock();
        let (cur, next);
        if let Some(mut s) = opt {
            (cur, next) = s.prepare_preempt();
            if cur.is_null() || next.is_null() || cur == next {
                return;

            }
        } else {
            return;
        }
        thread::Thread::switch(cur, next);       
    }
}