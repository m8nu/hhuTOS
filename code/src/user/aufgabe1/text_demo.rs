use crate::devices::cga::{self, getpos};        use crate::devices::cga::setpos;
// shortcut for cga
use crate::devices::cga_print;  // used to import code needed by println! 


pub fn run () {

   /* Hier muss Code eingefuegt werden */
   //println!("pos: {}", cga::getpos());

   let title = "Test der Zahlenausgabefunktion:";
   println!("{}\n", title);



   println!("  dec | hex  | bin");
   println!(" ------------------");
   for i in 0..16 {
      if i > 9 {
         println!("  {}  | {:#x}  | {:b}", i, i, i);
      } else {
         println!("  {}   | {:#x}  | {:b}", i, i, i);
      }
   }

   println!("  {}  | {:#x} | {:b}\n", 16, 16, 16);

   let temp = getpos();


   let arr = [
              " _     _          _____ ___  ____ ",
              "| |__ | |__  _   |_   _/ _ \\/ ___|",
              "| '_ \\| '_ \\| | | || || | | \\___ \\",
              "| | | | | | | |_| || || |_| |___) |",
              "|_| |_|_| |_|\\__,_||_| \\___/|____/ "
              ];

   for i in 0..arr.len() {
      setpos(35, 10 + (i as u64));
      println!("{}", arr[i]);
   }

   /*
    _     _          _____ ___  ____  
   | |__ | |__  _   |_   _/ _ \/ ___| 
   | '_ \| '_ \| | | || || | | \___ \ 
   | | | | | | | |_| || || |_| |___) |
   |_| |_|_| |_|\__,_||_| \___/|____/ 
    */

   setpos(temp.0, temp.1);
}