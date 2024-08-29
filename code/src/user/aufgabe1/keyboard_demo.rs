use crate::devices::cga as cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::devices::key as key;      // shortcut for key
use crate::devices::keyboard as keyboard;  // shortcut for keyboard


pub fn run() {

   /* Hier muss Code einfgeÃ¼gt werden */ 
        
   // 'key_hit' aufrufen und Zeichen ausgeben

   println!("Tastatur mit Eingaben bitte testen:");

   loop {
      let key = keyboard::key_hit();
      if key.asc != 0{
         cga::print_byte(key.asc);
         //print!("{}", key.asc as char);
      }
   }

}