
use crate::kernel::threads::{scheduler::{self, Scheduler}, thread};

use super::coop_thread_loop;


#[no_mangle]
extern "C" fn coop_demo_thread_entry(myself: *mut thread::Thread) {

   /* Hier muss Code eingefuegt werden */
   coop_thread_loop::init();
   coop_thread_loop::init();
   coop_thread_loop::init();

   for _ in 0..10000 {
      scheduler::Scheduler::yield_cpu();
   }
   scheduler::Scheduler::exit();

}


/**
 Return thread id of created thread
*/
pub fn init() -> usize {

   /* Hier muss Code eingefuegt werden */
   let tid = scheduler::next_thread_id();
   let coop_demo_thread = thread::Thread::new(tid, coop_demo_thread_entry);
   scheduler::Scheduler::ready(coop_demo_thread);
   return tid as usize;
}
