/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: scheduler                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: A basic round-robin scheduler for cooperative threads.          ║
   ║         No priorties supported.                                         ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::boxed::Box;
use core::ptr;
use core::sync::atomic::AtomicUsize;
use spin::Mutex;

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::mylib::queue;

static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_thread_id() -> usize {
    THREAD_ID_COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst)
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

/**
 Description: Return callers thread ID
*/
pub fn get_active_tid() -> usize {
    thread::Thread::get_tid(SCHEDULER.lock().active)
}

pub struct Scheduler {
    active: *mut thread::Thread,
    ready_queue: queue::Queue<Box<thread::Thread>>, // auf die CPU wartende Threads
}

unsafe impl Send for Scheduler {}

impl Scheduler {
    /**
     Description: Create the scheduler
    */
    pub const fn new() -> Self {
        Scheduler {
            active: ptr::null_mut(),
            ready_queue: queue::Queue::new(),
        }
    }

    /**
     Description: Start the scheduler. Called only once from 'startup'
    */
    pub fn schedule() {
        let next_thread = SCHEDULER.lock().ready_queue.dequeue();
        if let Some(that) = next_thread {
            // convert 'next_thread' into raw pointer.
            // Prevents Rust from deleting it too early but we need to manually call 'drop' later
            let raw = Box::into_raw(that);

            // set active reference in SCHEDULER
            SCHEDULER.lock().active = raw;

            // and start this thread
            thread::Thread::start(raw);
        } else {
            panic!("Panic: no thread, cannot start scheduler");
        }
    }

    /**
        Description: Register new thread in ready queue

        Parameters: \
               `that` thread to be registered
    */
    pub fn ready(that: Box<thread::Thread>) {
        SCHEDULER.lock().ready_queue.enqueue(that);
    }

    /**
        Description: Calling thread terminates. Scheduler switches to next thread.
                     (The thread terminating is not in the ready queue.)
    */
    pub fn exit() {
        // Get next thread from ready queue
        let next = SCHEDULER.lock().ready_queue.dequeue();
        if next.is_none() {
            panic!("Cannot exit thread as there is no other thread to run!");
        }

        // Start next thread
        if let Some(nx) = next {
            let raw = Box::into_raw(nx);
            SCHEDULER.lock().active = raw;
            thread::Thread::start(raw);
        }
    }

    /**
        Description: Yield cpu and switch to next thread
    */
    pub fn yield_cpu() {

       /* Hier muss Code eingefuegt werden */

    }

    /**
        Description: Kill thread with given thread id. The thread will just be
                     removed from the ready queue.

        Parameters: \
               `tokill_tid` id of the thread to be killed. Calling thread cannot kill itself.
    */
    pub fn kill(tokill_tid: usize) {

       /* Hier muss Code eingefuegt werden */

    }
}
