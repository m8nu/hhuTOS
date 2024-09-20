/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: mutex                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Mutex with wait_queue. It will block threads calling 'lock', if ║
   ║         the lock is already held by another thread. When the lock is    ║
   ║         freed a waiting thread is deblocked (put into ready queue).     ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 13.6.2024                 ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use alloc::boxed::Box;
use core::ptr::{self, null};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::kernel::{cpu, interrupts};
use crate::kernel::threads::scheduler::{self, prepare_block, Scheduler};
use crate::kernel::threads::thread::Thread;
use crate::mylib::queue::Queue;
use crate::mylib::spinlock::Spinlock;


/**
 Description: Mutex
*/pub struct Mutex {
    lock: AtomicBool,
    wait_queue: Spinlock<Queue<Box<Thread>>>, // blockierte Threads
}

// Gleiche unsafe Implementierung wie in 'std::sync::Mutex'
unsafe impl Sync for Mutex {}
unsafe impl Send for Mutex {}

impl Mutex {
    pub const fn new() -> Mutex {
			
	/* Hier muss Code eingefuegt werden */
    return Mutex {
        lock: AtomicBool::new(false),
        wait_queue: Spinlock::new(Queue::new()),
    };

    }

    /**
     Description: Get the mutex.
    */
    pub fn lock(&self) -> MutexGuard {
		/* Hier muss Code eingefuegt werden */
        let l = self.lock.load(Ordering::SeqCst);
        if l {
            let was_enabled = cpu::disable_int_nested();
            let (curr, next) = prepare_block();
            if curr == ptr::null_mut() || next == ptr::null_mut() || curr == next {
                panic!("No threads to switch to");
            } else {
                unsafe {self.wait_queue.lock().enqueue(Box::from_raw(curr)); }
                Thread::switch(curr, next);
           }
           cpu::enable_int_nested(was_enabled);
        } else {
            self.lock.store(true, Ordering::SeqCst);
        }
        MutexGuard { lock: self }
    }

    /**
     Description: Free the mutex. Called from `drop` in the `MutexGuard`
    */
    fn unlock(&self) {
		/* Hier muss Code eingefuegt werden */
        if let Some(q) = self.wait_queue.lock().dequeue(){
            scheduler::deblock(Box::into_raw(q));
        } else {
            self.lock.store(false, Ordering::SeqCst);
        }


    }

}

/**
Description: Mutex guard used by Mutex to automatically call `unlock`
             for the mutex in case the guard is dropped.
*/
pub struct MutexGuard<'a> {
    lock: &'a Mutex,
}


/**
Description: Implementation for `drop()` which will call `unlock` on the mutex
*/
impl<'a> Drop for MutexGuard<'a> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}
