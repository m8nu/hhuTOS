
use crate::devices::pcspk;

pub fn run() {
   
   println!("Aufgabe 2: Sound Demo");
   /* Hier muss Code eingefuegt werden */
   println!("Playing Tetris");
   pcspk::tetris();
   println!("Playing Among Us");
   pcspk::among_us();
   println!("Playing Aerodynamic");
   pcspk::aerodynamic();



}

