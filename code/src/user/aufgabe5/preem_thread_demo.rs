use crate::devices::cga;
use crate::devices::pcspk;
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;

#[no_mangle]
extern "C" fn thread_loop_entry(myself: *mut thread::Thread) {

   /* Hier muss Code eingefuegt werden */
   let mut counter = 0;
   loop {
      cga::setpos(17 * Thread::get_tid(myself) as u64 - 10, 20);
      println!("Loop [{}] : {}  ",Thread::get_tid(myself) as u64, counter);
      counter += 1;
   }

}

#[no_mangle]
extern "C" fn thread_tetris(myself: *mut thread::Thread) {
   pcspk::tetris();
}

pub fn init() {

   /* Hier muss Code eingefuegt werden */
   let tid = scheduler::next_thread_id();
   let thread_loop = thread::Thread::new(tid, thread_loop_entry);
   scheduler::Scheduler::ready(thread_loop);
   
   let tid2 = scheduler::next_thread_id();
   let thread_loop2 = thread::Thread::new(tid2, thread_tetris);
   scheduler::Scheduler::ready(thread_loop2);
}