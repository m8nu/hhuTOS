
use crate::kernel::threads::{scheduler::{self, Scheduler}, thread};

use super::coop_thread_loop;

use crate::devices::pcspk;


#[no_mangle]
extern "C" fn demo_thread_entry(myself: *mut thread::Thread) {

   /* Hier muss Code eingefuegt werden */
   coop_thread_loop::init();
   coop_thread_loop::init();
   coop_thread_loop::init();
   scheduler::Scheduler::exit();

}


/**
 Return thread id of created thread
*/
pub fn init() -> usize {

   /* Hier muss Code eingefuegt werden */
   let tid = scheduler::next_thread_id();
   let coop_demo_thread = thread::Thread::new(tid, demo_thread_entry);
   scheduler::Scheduler::ready(coop_demo_thread);
   return tid as usize;
}
