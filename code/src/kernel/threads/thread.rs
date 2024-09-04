

impl Thread {
    /**
       Description: Start thread `now`
    */
    pub fn start(now: *mut Thread) {

       /* Hier muss Code eingefuegt werden */
       
    }

    /**
       Description: Switch from thread `now` to thread `then`
    */
    pub fn switch(now: *mut Thread, then: *mut Thread) {

       /* Hier muss Code eingefuegt werden */

    }


/**
   Description: Necessary for implementing the ready queue in the scheduler
*/
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.tid == other.tid
    }
}

