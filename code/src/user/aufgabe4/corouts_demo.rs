use crate::{devices::cga, kernel::corouts::coroutine::{self, Coroutine}};


extern "C" fn coroutine_loop_entry(myself: *mut coroutine::Coroutine) {

   /* Hier muss Code eingefuegt werden */
   let mut counter = 0;

   loop {
      let id = coroutine::Coroutine::get_cid(myself) as u64;
      cga::setpos(17 * id - 10, 20);
      println!("Loop [{}] : {}  ", id, counter);
      counter += 1;

      coroutine::Coroutine::switch2next(myself);
   }

}

pub fn run() {

   /* Hier muss Code eingefuegt werden */

    // Anlegen aller Koroutinen
    let mut coroutine1 = coroutine::Coroutine::new(1, coroutine_loop_entry);
    let mut coroutine2 = coroutine::Coroutine::new(2, coroutine_loop_entry);
    let mut coroutine3 = coroutine::Coroutine::new(3, coroutine_loop_entry);
    
    // Zyklisches Verketten aller Koroutinen
    coroutine1.set_next(coroutine2.get_raw_pointer());
    coroutine2.set_next(coroutine3.get_raw_pointer());
    coroutine3.set_next(coroutine1.get_raw_pointer());

    // Start der ersten Koroutine
    Coroutine::start(coroutine1.get_raw_pointer());
}
