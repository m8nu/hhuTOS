/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: Thread                                                          ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Functions for creating, starting, switching and ending Threads. ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::{boxed::Box, rc::Rc};
use core::ffi::c_void;
use core::ptr;

use crate::consts;
use crate::devices::cga;
use crate::kernel::{cpu, stack};

// Description: Assembly functions in 'Thread.asm'
extern "C" {
    fn _Thread_start(stack_ptr: usize);
    fn _Thread_switch(now_stack_ptr: *mut usize, then_stack: usize);
}

/**
   Description: Meta data for a Thread
*/
#[repr(C)]
pub struct Thread {
    tid: usize,
    stack_ptr: usize,    // stack pointer to saved context
    stack: stack::Stack, // memory for stack
    entry: extern "C" fn(*mut Thread),
}

impl Thread {
    /**
       Description: Create new Thread
    */
    pub fn new(my_tid: usize, my_entry: extern "C" fn(*mut Thread)) -> Box<Thread> {
        let my_stack = stack::Stack::new(4096);
        let my_stack_ptr = my_stack.end_of_stack();

        let mut thread = Box::new(Thread {
            tid: my_tid,
            stack_ptr: my_stack_ptr,
            stack: my_stack,
            entry: my_entry,
        });

        thread.thread_prepare_stack();
        thread
    }

    /**
       Description: Start Thread `thr`
    */
    pub fn start(thr: *mut Thread) {
        /* Hier muss Code eingefuegt werden */
        unsafe {
            _Thread_start((*thr).stack_ptr);
        }
    }

    /**
       Description: Switch from Thread `now` to Thread `then`
    */
    pub fn switch(now: *mut Thread, then: *mut Thread) {

        /* Hier muss Code eingefuegt werden */
        unsafe {
            _Thread_switch(&mut (*now).stack_ptr, (*then).stack_ptr);
        }
    }

    /**
       Description: Return raw pointer to self
    */
    pub fn get_raw_pointer(&mut self) -> *mut Thread {
        self
    }

    /**
       Description: Return Thread id of `thr_object`
    */
    pub fn get_tid(thr_object: *const Thread) -> usize {
        unsafe { (*thr_object).tid }
    }

    /**
      Description: Prepare the stack of a newly created Thread. It is used to \
                   switch the stack and return to the 'kickoff' function.  \
                   The prepared stack is used in '_Thread_start' to start the first Thread.\
                   Starting all other Threads is done in '_Thread_switch' where the \
                   prepared stack is used to kickoff a Thread.
    */
    fn thread_prepare_stack(&mut self) {
        let faddr = kickoff_thread as *const ();
        let object: *const Thread = self;
        let sp: *mut u64 = self.stack_ptr as *mut u64;

        // The stack should look like a function of a Thread was called with one
        // parameter "object" (raw pointer to the Thread struct)
        unsafe {
            *sp = 0x131155 as u64; // dummy return address

            *sp.offset(-1) = faddr as u64; // address of 'kickoff'

            // save all registers on stack
            *sp.offset(-2) = 0; // r8
            *sp.offset(-3) = 0; // r9
            *sp.offset(-4) = 0; // r10
            *sp.offset(-5) = 0; // r11
            *sp.offset(-6) = 0; // r12
            *sp.offset(-7) = 0; // r13
            *sp.offset(-8) = 0; // r14
            *sp.offset(-9) = 0; // r15

            *sp.offset(-10) = 0; // rax
            *sp.offset(-11) = 0; // rbx
            *sp.offset(-12) = 0; // rcx
            *sp.offset(-13) = 0; // rdx

            *sp.offset(-14) = 0; // rsi
            *sp.offset(-15) = object as u64; // rdi -> 1. param. fuer 'kickoff'
            *sp.offset(-16) = 0; // rbp
            *sp.offset(-17) = 0x2; // rflags (IE = 0); interrupts disabled

            // Zum Schluss speichern wir den Zeiger auf den zuletzt belegten
            // Eintrag auf dem Stack in 'context'. Daruber gelangen wir in
            // Thread_start an die noetigen Register
            self.stack_ptr = self.stack_ptr - (consts::STACK_ENTRY_SIZE * 17);
        }

        /*
              println!("Prepared Stack: top-address = {:x}", self.stack.get_data() as u64);
              unsafe {
                 println!("  {:x}: {:x}  // dummy raddr", sp as u64, *(sp) as u64);
                 println!("  {:x}: {:x}  // *object", sp.offset(-15) as u64, *(sp.offset(-15)) as u64);
                 println!("  {:x}: {:x}  // kickoff", sp.offset(-1) as u64, *(sp.offset(-1)) as u64);
                 println!("  {:x}: last used ", sp.offset(-17) as u64);
                 println!("");
                 println!("  self.context = {:x}  // context", self.context);
              }
              loop {}
        */
    }
}

/**
   Description: Called indirectly by using the prepared stack in '_Thread_start' and '_Thread_switch'
*/
#[no_mangle]
pub extern "C" fn kickoff_thread(object: *mut Thread) {
    //kprintln!("kickoff");
    cpu::enable_int(); // interrupts are disabled during Thread start
    unsafe {
        ((*object).entry)(object);
    }
    loop {}
}

/**
   Description: Necessary for implementing the ready queue in the scheduler
*/
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.tid == other.tid
    }
}
