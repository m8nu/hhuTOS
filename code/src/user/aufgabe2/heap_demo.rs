
use crate::devices::cga as cga;  
use crate::devices::cga_print;       
use crate::devices::key as key;     
use crate::devices::keyboard as keyboard;  
use crate::kernel::allocator as allocator;  
use alloc::{boxed::Box, vec::Vec};



// Hilfsfunktion: Auf Return-Taste warten
fn wait_for_return() {
	
	println!("");
  println!("Weiter mit <ENTER>");

   loop {
      let mut key: key::Key = keyboard::key_hit();
        
      if key.valid() == true {
		     if key.get_ascii() == 13 { break; }
      }
   }
}


fn demo() {

    /* Hier muss Code eingefuegt werden */
    println!("Demo 1/4: Allocate 2 struct using Box::new");
    println!("=========================================\n");
    allocator::dump_free_list();

    struct Test {
        a: i32,
        b: i32,
    }

    {
    println!("\nStructs allozieren:");
    let s1 = Box::new(Test { a: 1, b: 2 });
    let s2 = Box::new(Test { a: 3, b: 4 });
    println!("\ts1.a={}, s1.b={}", s1.a, s1.b);
    println!("\ts2.a={}, s2.b={}\n", s2.a, s2.b);


    allocator::dump_free_list();

    wait_for_return();
    cga::clear();
    } //getting out of scope

    println!("Demo 2/4: The 2 structs were deallocated because they went out of scope");
    println!("=========================================================================\n");


    allocator::dump_free_list();

    wait_for_return();
    cga::clear();

    println!("Demo 3/4: Allocate a Vec for storing 3 structs");
    println!("==============================================\n");

    println!("Vec allozieren\n");
    {
    let mut v = Vec::<Test>::new();

    println!("Structs allozieren\n");
    let s1 = Box::new(Test { a: 1, b: 2 });
    let s2 = Box::new(Test { a: 3, b: 4 });
    let s3 = Box::new(Test { a: 5, b: 6 });

    v.push(*s1);
    v.push(*s2);
    v.push(*s3);

    allocator::dump_free_list();
    wait_for_return();
    cga::clear();

    }
    println!("Demo 4/4: Vec will go out of scope");
    println!("==================================\n");

    allocator::dump_free_list();
    println!("Ende der Demo");
    wait_for_return();
    cga::clear();
    // free heap allocated struct before return
}



pub fn run () {

    demo();

    /* Hier muss Code eingefuegt werden */

}
