/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: cga                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: This module provides functions for doing output on the CGA text ║
   ║         screen. It also supports a text cursor position stored in the   ║
   ║         hardware using ports.                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 6.2.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use crate::kernel::cpu as cpu;
use crate::kernel::cpu::inb as inb;
use crate::kernel::cpu::outb as outb;

use super::kprint;
use super::kprint::kprint;


// make type comparable, printable and enable copy semantics
#[allow(dead_code)]   // avoid warnings for unused colors
#[repr(u8)]           // store each enum variant as an u8
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}


pub const CGA_STD_ATTR: u8       = (Color::Blue as u8) << 4 | (Color::White as u8);

const CGA_BASE_ADDR: u64     = 0xb8000;
const CGA_ROWS   : u64       = 25;
const CGA_COLUMNS: u64       = 80;

const CGA_INDEX_PORT: u16    = 0x3d4;  // select register
const CGA_DATA_PORT: u16     = 0x3d5;  // read/write register
const CGA_HIGH_BYTE_CMD: u8  = 14;     // cursor high byte
const CGA_LOW_BYTE_CMD: u8   = 15;     // cursor high byte


/**
 Description: Clear text screen
*/
pub fn clear() {

   /* Hier muss Code eingefuegt werden */
   for i in 0..CGA_COLUMNS {
       for j in 0..CGA_ROWS {
           show(i, j, ' ', CGA_STD_ATTR);
       }
   }
   setpos(0, 0);

}


/**
 Description: Display the `character` at the given position `x`,`y` with attribute `attrib`
*/
pub fn show (x: u64, y: u64, character: char, attrib: u8) {
    let pos: u64;

    if x>CGA_COLUMNS || y>CGA_ROWS
    {    
		return ; 
    }
    
    pos = (y * CGA_COLUMNS + x) * 2;

    unsafe {
        *((CGA_BASE_ADDR + pos) as *mut u8)     = character as u8;
        *((CGA_BASE_ADDR + pos + 1) as *mut u8) = attrib;
    }
}


/**
 Description: Return cursor position `x`,`y` 
*/
pub fn getpos () -> (u64, u64) {

   /* Hier muss Code eingefuegt werden */
   outb(CGA_INDEX_PORT, CGA_LOW_BYTE_CMD);   //Select low byte
   let low = inb(CGA_DATA_PORT) as u64;      //Read low byte
   outb(CGA_INDEX_PORT, CGA_HIGH_BYTE_CMD);  //Select high byte
   let high = inb(CGA_DATA_PORT) as u64;     //Read high byte
   

   (((low + high * 256) % CGA_COLUMNS) as u64, ((low + high * 256) / CGA_COLUMNS) as u64)
   // Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen 
}


/**
 Description: Set cursor position `x`,`y` 
*/
pub fn setpos (x:u64, y:u64) {

   /* Hier muss Code eingefuegt werden */
   /* Example: (3,7) -> 3*80+7 = 247   */

   outb(CGA_INDEX_PORT, CGA_LOW_BYTE_CMD);                  //Select low byte
   outb(CGA_DATA_PORT, (x + y * CGA_COLUMNS) as u8);         //Write low byte
   outb(CGA_INDEX_PORT, CGA_HIGH_BYTE_CMD);                 //Select high byte
   outb(CGA_DATA_PORT, ((x + y * CGA_COLUMNS) >> 8) as u8); //Write high byte
}

 
/**
 Description: Print byte `b` at actual position cursor position `x`,`y` 
*/
pub fn print_byte (b: u8) {

   /* Hier muss Code eingefuegt werden */
   //check if b is newline
   if b == 13 || b == 10{
      let (x, y) = getpos();

      if y == CGA_ROWS-1{
         scrollup();
         setpos(0, y);
         return;
      }

      setpos(0, y+1);
      return;
   }

   if b == 9{ //Tab
      let (x, y) = getpos();
      let mut i = 0;
      while i < 4{
         show(x+i, y, ' ', CGA_STD_ATTR);
         i += 1;
      }
      setpos(x+4, y);
      return;
   }

   if b == 8{ //Backspace
      let (x, y) = getpos();

      if x > 0{
         setpos(x-1, y);
         show(x-1, y, ' ', CGA_STD_ATTR);
         setpos(x-1, y);
      }

      if y > 0 && x == 0{
         setpos(CGA_COLUMNS-1, y-1);
         show(CGA_COLUMNS-1, y-1, ' ', CGA_STD_ATTR);
         setpos(CGA_COLUMNS-1, y-1);
      }
      return;
   }

   let (x, y) = getpos();
   show(x, y, b as char, CGA_STD_ATTR);
   setpos(x+1, y);

}


/**
 Description: Scroll text lines by one to the top.
*/

pub fn scrollup () {
 
   /* Hier muss Code eingefuegt werden */
   if getpos().1 == 0{ //If cursor is at the top, no need to scroll
      return;
   }

   for i in 0..CGA_COLUMNS * (CGA_ROWS - 1) {
      unsafe { //need to read CGA-Memory 
         *((CGA_BASE_ADDR + i * 2) as *mut u8)     = *((CGA_BASE_ADDR + (i + CGA_COLUMNS) * 2) as *mut u8);
         *((CGA_BASE_ADDR + i * 2 + 1) as *mut u8) = *((CGA_BASE_ADDR + (i + CGA_COLUMNS) * 2 + 1) as *mut u8);
      }
   }
   let (x,y) = getpos();
   if y>0{
      setpos(0, y-1);
   }

   //fix last line
   for i in 0..CGA_COLUMNS {
      show(i, CGA_ROWS-1, ' ', CGA_STD_ATTR);
   }
}
 
 
/**
 Description: Helper function returning an attribute byte for the given 
              parameters `bg`, `fg`, and `blink`
*/
pub fn attribute (bg: Color, fg: Color, blink: bool) -> u8 {

   /* Hier muss Code eingefuegt werden */
   let a = ((bg as u8) << 4 | (fg as u8) | (blink as u8) << 7) as u8;
   
   //Bit 0-3 fg, Bit 4-6 bg, Bit 7 blink
   //Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen 
   a
}