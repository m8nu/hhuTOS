use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::devices::cga; 
use crate::devices::cga_print; 
use crate::devices::fonts::font_8x8;
use crate::devices::keyboard;
use crate::devices::keyboard::get_lastkey;
use crate::devices::pit::get_systime;
use crate::devices::vga;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread;
use crate::mylib::input::getch;

#[derive(Copy, Clone)]
enum BlockSize {
    Small = 10,
    Medium = 20,
    Large = 25,
}

struct Board {
    field: Vec<Vec<bool>>,
    size: BlockSize,
}

impl Board{

    pub fn draw_board(&self) {
        let (xres, yres) = vga::get_res();
        let size = self.size as u32;

        for y in 0..yres {
            for x in 0..xres {
                vga::draw_pixel(x, y, vga::rgb_24(255, 255, 255));
                if x % size == 0 || y % size == 0 {
                    vga::draw_pixel(x, y, vga::rgb_24(0,0,0));
                }
                if self.field[(y / size) as usize][(x / size) as usize] == true {
                    for i in 0..size{
                        for j in 0..size{
                            vga::draw_pixel(x + i, y + j, vga::rgb_24(0, 0, 0));
                        }
                    }
                }
            }
        }

        vga::draw_string(0, yres-10, vga::rgb_24(0, 0, 255), "(1) Toggle Draw, (3) Start/Stop, (4) Set Blocksize, (Enter) Next Generation");
    }

    pub fn change_board_size(&mut self, size: BlockSize) {
        let (xres, yres) = vga::get_res();
    
        self.field = vec![vec![false; (xres / size as u32) as usize]; (yres / size as u32) as usize];
        self.size = size;
        let size = size as u32;
    
        for y in 0..yres {
            for x in 0..xres {
                if x % size == 0 || y % size == 0 {
                    vga::draw_pixel(x, y, vga::rgb_24(0,0,0));
                }
                if self.field[(y / size) as usize][(x / size) as usize] == true {
                    for i in 0..size{
                        for j in 0..size{
                            vga::draw_pixel(x + i, y + j, vga::rgb_24(0, 0, 0));
                        }
                    }
                }
            }
        }
    }

    pub fn update_field(&mut self){
        let count = self.count_neigbours();
        let mut new_field = vec![vec![false; self.field[0].len()]; self.field.len()];

        for y in 0..self.field.len(){
            for x in 0..self.field[0].len(){
                if self.field[y][x] == true{
                    if count[y][x] < 2 || count[y][x] > 3{
                        new_field[y][x] = false;
                    }else{
                        new_field[y][x] = true;
                    }
                }else{
                    if count[y][x] == 3{
                        new_field[y][x] = true;
                    }
                }
            }

        }
        self.field = new_field;
    }

    pub fn count_neigbours(&mut self) -> Vec<Vec<i32>>{
        let mut count = vec![vec![0; self.field[0].len()]; self.field.len()];

        for y in 0..self.field.len(){
            for x in 0..self.field[0].len(){
                for i in -1..2{
                    for j in -1..2{
                        if i == 0 && j == 0{
                            continue;
                        }
                        if y as i32 + i < 0 || y as i32 + i >= self.field.len() as i32 || x as i32 + j < 0 || x as i32 + j >= self.field[0].len() as i32{
                            continue;
                        }
                        if self.field[(y as i32 + i) as usize][(x as i32 + j) as usize] == true{
                            count[y][x] += 1;
                        }
                    }
                }
            }
        }
        return count;
    }

    pub fn draw_courser(&mut self){
        let (xres, yres) = vga::get_res();
        let mut courser_pos = (self.field[0].len() as u32 / 2 * self.size as u32, self.field.len() as u32 / 2 * self.size as u32);

        loop{

            for i in 0..self.size as u32{
                for j in 0..self.size as u32{
                    vga::draw_pixel(courser_pos.0 + i, courser_pos.1 + j, vga::rgb_24(255, 0, 0));
                }
            }
            let key = getch();
            //if W pressed
            if key == 119{
                if courser_pos.1 == 0{
                    courser_pos.1 = yres - self.size as u32;
                } else {
                    courser_pos.1 -= self.size as u32;
                }
            }
            //if S pressed
            if key == 115{
                if courser_pos.1 == yres - self.size as u32{
                    courser_pos.1 = 0;
                } else {
                    courser_pos.1 += self.size as u32;
                }
            }
            //if A pressed
            if key == 97{
                if courser_pos.0 == 0{
                    courser_pos.0 = xres - self.size as u32;
                } else {
                    courser_pos.0 -= self.size as u32;
                }
            }
            //if D pressed
            if key == 100{
                if courser_pos.0 == xres - self.size as u32{
                    courser_pos.0 = 0;
                } else {
                    courser_pos.0 += self.size as u32;
                }
            }

            //exit draw mode if 1
            if key == 49{
                break;
            }

            //if enter or space pressed toggle field 
            if key == 13 || key == 32{
                self.field[(courser_pos.1 / self.size as u32) as usize][(courser_pos.0 / self.size as u32) as usize] = !self.field[(courser_pos.1 / self.size as u32) as usize][(courser_pos.0 / self.size as u32) as usize];
            }

            self.draw_board();
            
        }
            
    }
}

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn game_run(myself: *mut thread::Thread) {
    let (xres, yres) = vga::get_res();

    let mut board = Board {
        field: vec![vec![false; (xres / BlockSize::Medium as u32) as usize]; (yres / BlockSize::Medium as u32) as usize],
        size: BlockSize::Medium,
    };


    //Glider
    board.field[1][2] = true;
    board.field[2][3] = true;
    board.field[3][1] = true;
    board.field[3][2] = true;
    board.field[3][3] = true;



    board.draw_board();

    loop {
        let key = getch();
        //check if key is enter
        if key == 13 {
            board.update_field();
            board.draw_board();
        }
        //check if key is 1 for Draw
        if key == 49{
            board.draw_courser();
            board.draw_board();
        }

        //check if key is 3 for Start/Stop
        let mut play = false;
        if key == 51{
            play = !play;
            while play{
                board.update_field();
                board.draw_board();
                let key = get_lastkey();
                if key == 51{
                    play = !play
                }
            }
        }

        //check if key is 4
        if key == 52{
            vga::draw_string(xres/4, yres/2, vga::rgb_24(0, 0, 0), "Small (1), Medium (2), Large (3)");
            let key = getch();
            if key == 49{
                board.change_board_size(BlockSize::Small);
            }
            if key == 50{
                board.change_board_size(BlockSize::Medium);
            }
            if key == 51{
                board.change_board_size(BlockSize::Large);
            }
            board.draw_board();
        }
    }

}


/**
 Description: Create and add the graphic demo thread
*/
pub fn init() { 
    let game_thread = thread::Thread::new(scheduler::next_thread_id(), game_run);
    scheduler::Scheduler::ready(game_thread);
}
