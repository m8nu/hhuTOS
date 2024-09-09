use spin::mutex;

use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::cga;
use crate::cpu;
use crate::mylib::delay;
use crate::devices::pcspk;
use crate::mylib::mutex::Mutex;
use crate::mylib::spinlock;

//static spinlock
static LOCK: spinlock::Spinlock<i32> = spinlock::Spinlock::new(0);
static MUTEX: Mutex = Mutex::new();

#[no_mangle]
extern "C" fn synced_loop_thread_entry(myself: *mut thread::Thread) {

   /* Hier muss Code eingefuegt werden */
   let mut cnt = 0;
   let my_tid = thread::Thread::get_tid(myself) as u64;

    loop {

        /* Hier muss Code eingefuegt werden */
        //let enabled = cpu::disable_int_nested();
        
        {
            let m = MUTEX.lock();
            cga::setpos(5 + (my_tid-1) * 20, 10);
            delay::delay(10);
            println!("Loop [{}] : {}", my_tid, cnt);
            //cpu::enable_int_nested(enabled);
    
        }
        cnt = cnt + 1;
        delay::delay(100);
    }
}

#[no_mangle]
extern "C" fn music(myself: *mut thread::Thread){
    pcspk::among_us();
    pcspk::aerodynamic();
}

pub fn init() {

   /* Hier muss Code eingefuegt werden */
    let thread1 = thread::Thread::new(scheduler::next_thread_id(), synced_loop_thread_entry);
    let thread2 = thread::Thread::new(scheduler::next_thread_id(), synced_loop_thread_entry);
    let thread3 = thread::Thread::new(scheduler::next_thread_id(), synced_loop_thread_entry);
    let thread4 = thread::Thread::new(scheduler::next_thread_id(), music);

    scheduler::Scheduler::ready(thread1);
    scheduler::Scheduler::ready(thread2);
    scheduler::Scheduler::ready(thread3);
    scheduler::Scheduler::ready(thread4);

}
