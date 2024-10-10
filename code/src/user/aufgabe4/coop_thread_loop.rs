use crate::devices::cga;
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;

#[no_mangle]
extern "C" fn coop_thread_loop_entry() {

   /* Hier muss Code eingefuegt werden */
   let mut counter = 0;
   loop {
      cga::setpos(17 * scheduler::get_active_tid() as u64 - 10, 20);
      println!("Loop [{}] : {}  ",scheduler::get_active_tid() as u64, counter);
      counter += 1;
      scheduler::Scheduler::yield_cpu();
   }

}

pub fn init() {

   /* Hier muss Code eingefuegt werden */
   let tid = scheduler::next_thread_id();
   let coop_thread_loop = thread::Thread::new(tid, coop_thread_loop_entry, false);
   scheduler::Scheduler::ready(coop_thread_loop);
   tid as usize;

}
