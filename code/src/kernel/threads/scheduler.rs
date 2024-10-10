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
use core::any::Any;
use core::ptr;
use core::sync::atomic::AtomicUsize;
use spin::Mutex;

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::mylib::queue;

static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn set_initialized() {
    SCHEDULER.lock().initialized = true;
}

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
    initialized: bool,
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
            initialized: false,
        }
    }

    /**
        Description: Check if we can switch from the current running thread to another one. \
                     If doable prepare everything and return raw pointers to current and next thread. \
                     The switching of threads is done from within the ISR of the PIT, in order to \
                     release the lock of the scheduler. 

        Return: \
               `(current,next)` current thread, next thread (to switch to)
    */
    pub fn prepare_preempt(&mut self) -> (*mut thread::Thread, *mut thread::Thread) {
        // If the scheduler is not initialized, we abort
        if self.initialized == false {
            return (ptr::null_mut(), ptr::null_mut());
        }
      /* Hier muss Code eingefuegt werden */
      let cur = self.active;
      unsafe {
        self.ready_queue.enqueue(Box::from_raw(cur));
      }
      let next = self.ready_queue.dequeue();
      if let Some(that) = next {
        self.active = Box::into_raw(that);
        return (cur, self.active);
      } else {
        return (ptr::null_mut(), ptr::null_mut());
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
        let next_thread = SCHEDULER.lock().ready_queue.dequeue();

        if let Some(mut that) = next_thread {
            let current_active = SCHEDULER.lock().active;
            SCHEDULER.lock().ready_queue.enqueue(unsafe { Box::from_raw(current_active) });
            SCHEDULER.lock().active = that.as_mut();
            thread::Thread::switch(current_active, Box::into_raw(that));
        } else {
            return;
        }
    }

    /**
        Description: Kill thread with given thread id. The thread will just be
                     removed from the ready queue.

        Parameters: \
               `tokill_tid` id of the thread to be killed. Calling thread cannot kill itself.
    */
    pub fn kill(tokill_tid: usize) {

        /* Hier muss Code eingefuegt werden */
        if tokill_tid == get_active_tid(){
            return;
        }
        //SCHEDULER.lock().ready_queue.remove(thread::Thread::new(tokill_tid,thread::kickoff_kernel_thread, true));
    }

    
    /**
        Description: Check if we can switch from the current running thread to another one. \
                     If doable prepare everything and return raw pointers to current and next thread. \
                     The switching of threads is done later by calling 'Thread::switch'. \
                     This function is very similar to `prepare_preempt` except the \
                     current thread is not inserted in the `ready_queue` but returned. \
                     The next thread is removed from the `ready_queue` and `active` is set.

        Return: \
               `(current,next)` current thread, next thread (to switch to)
    */
    pub fn prepare_block(&mut self) -> (*mut thread::Thread, *mut thread::Thread) {
        // If the scheduler is not initialized, we abort
        if self.initialized == false {
            return (ptr::null_mut(), ptr::null_mut());
        }
        /* Hier muss Code eingefuegt werden */
        let cur = self.active;
        let next = self.ready_queue.dequeue();
        if let Some(that) = next {
            self.active = Box::into_raw(that);
        return (cur, self.active);
        } else {
        panic!("No thread to switch to");
        }
    }
}


/**
 Description: Prepare the blocking of the calling thread (which is the active thread)
*/
pub fn prepare_block() -> (*mut thread::Thread, *mut thread::Thread) {
    SCHEDULER.lock().prepare_block()
}

/**
 Description: Deblock thread `that`. This will result in putting
              `that` into the ready-queue but no thread switching.
*/
pub fn deblock(that: *mut thread::Thread) {
    unsafe {
        SCHEDULER.lock().ready_queue.enqueue(Box::from_raw(that));
    }
}



