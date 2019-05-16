// Skeleton code for your Rust projects
// I added several comments and annotations to this file.
// _Please_ read them carefully. They are very important.
// The most important comments are all annotated with "NOTE/WARNING:"

// I will grade your code quality primarily on how "idiomatic" your Rust 
// code is, and how well you implemented the "safe unsafety" guidelines.

extern crate libc;
extern crate time;
extern crate ctrlc;
#[macro_use] extern crate simple_error;
extern crate shuteye;
extern crate mmap;
extern crate nix;
extern crate byteorder;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs::OpenOptions, os::unix::fs::OpenOptionsExt};
use std::error::Error;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::time::Duration;
use shuteye::sleep;
use mmap::{MemoryMap, MapOption};
use std::mem::size_of;
use std::io::{Read, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use std::time::SystemTime;
use std::f64::INFINITY;


#[derive(Copy, Clone)]
struct Pixel {
    r: u16,
    g: u16,
    b: u16,
}

struct GPIO {
    gpio_map_: Option<MemoryMap>,
    output_bits_: u32,
    input_bits_: u32,
    slowdown_: u32,                         // Please refer to the GPIO_SetBits and GPIO_ClearBits functions in the reference implementation to see how this is used.
    gpio_port_: *mut u32,                   // A raw pointer that points to the base of the GPIO register file
    gpio_set_bits_: *mut u32,               // A raw pointer that points to the pin output register (see section 2.1 in the assignment)
    gpio_clr_bits_: *mut u32,               // A raw pointer that points to the pin output clear register (see section 2.1)
    gpio_read_bits_: *mut u32,              // A raw pointer that points to the pin level register (see section 2.1)
    row_mask: u32,                      
    bitplane_timings: [u32; COLOR_DEPTH]    
}

// This is a representation of the "raw" image
struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Pixel>>
}

// This is a representation of the frame we're currently rendering
struct Frame {
    pos: usize,
    pixels: Vec<Vec<Pixel>>
}

// Use this struct to implement high-precision nanosleeps
struct Timer {
    _timemap: Option<MemoryMap>,
    timereg: *mut u32 // a raw pointer to the 1Mhz timer register (see section 2.5 in the assignment)
}

// ============================================================================
// GPIO configuration parameters for the raspberry pi 3
// ============================================================================

const BCM2709_PERI_BASE: u64 = 0x3F000000;
const GPIO_REGISTER_OFFSET: u64 = 0x200000;
const TIMER_REGISTER_OFFSET: u64 = 0x3000;
const REGISTER_BLOCK_SIZE: u64 = 4096;
const COLOR_DEPTH: usize = 8;
const ROWS: u32 = 16;
const SUB_PANELS_ :u32 = 2;

//Tijdelijk
const COLUMNS :u32 = 32;

const PIN_OE  : u64 = 4;
const PIN_CLK : u64 = 17;
const PIN_LAT : u64 = 21;
const PIN_A   : u64 = 22;
const PIN_B   : u64 = 26;
const PIN_C   : u64 = 27;
const PIN_D   : u64 = 20;
const PIN_E   : u64 = 24;
const PIN_R1  : u64 = 5;
const PIN_G1  : u64 = 13;
const PIN_B1  : u64 = 6;
const PIN_R2  : u64 = 12;
const PIN_G2  : u64 = 16;
const PIN_B2  : u64 = 23;

// Convenience macro for creating bitmasks. See comment above "impl GPIO" below
macro_rules! GPIO_BIT {
    ($bit:expr) => {
        1 << $bit
    };
}

// Use this bitmask for sanity checks
const VALID_BITS: u64 = GPIO_BIT!(PIN_OE) | GPIO_BIT!(PIN_CLK) | GPIO_BIT!(PIN_LAT) |
    GPIO_BIT!(PIN_A)  | GPIO_BIT!(PIN_B)  | GPIO_BIT!(PIN_C)   | GPIO_BIT!(PIN_D)   | GPIO_BIT!(PIN_E) |
    GPIO_BIT!(PIN_R1) | GPIO_BIT!(PIN_G1) | GPIO_BIT!(PIN_B1)  |
    GPIO_BIT!(PIN_R2) | GPIO_BIT!(PIN_G2) | GPIO_BIT!(PIN_B2);

// ============================================================================
// mmap_bcm_register - convenience function used to map the GPIO register block
// ============================================================================

static mut row_mask: u32 = 0;


fn mmap_bcm_register(register_offset: usize) -> Option<MemoryMap> {

    let mem_file = 
        match OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_SYNC)
        .open("/dev/mem") {
            Err(why) => panic!("couldn't open /dev/mem: {}", why.description()),
            Ok(file) => file
        };

    let mmap_options = &[
        MapOption::MapNonStandardFlags(libc::MAP_SHARED),
        MapOption::MapReadable,
        MapOption::MapWritable,
        MapOption::MapFd(mem_file.as_raw_fd()),
        MapOption::MapOffset(BCM2709_PERI_BASE as usize + register_offset as usize)       
    ];    

    let result = MemoryMap::new(REGISTER_BLOCK_SIZE as usize, mmap_options).unwrap();

    return match result.data().is_null() {
        true => {
            eprintln!("mmap error: {}", std::io::Error::last_os_error());
            eprintln!("Pi3: MMapping from base 0x{:X}, offset 0x{:X}", BCM2709_PERI_BASE, register_offset);
            None
        },
        false => Some(result)
    };
    
    // NOTE/WARNING: When a MemoryMap struct is dropped, the mapped 
    // memory region is automatically unmapped!
}

//
// NOTE/WARNING: In many cases, particularly those where you need to set or clear 
// multiple bits at once, it is convenient to store multiple pin numbers in one bit 
// mask value. If you want to simultaneously set PIN_A and PIN_C to high, for example, 
// you should probably create a bit mask with the positions of PIN_A and PIN_C set to 1, 
// and all other positions set to 0. You can do this using the GPIO_BIT! macro.
//
// In this example, you would do something like:
//     let pin_mask = GPIO_BIT!(PIN_A) | GPIO_BIT!(PIN_C);
//     io.set_bits(pin_mask);
//
impl GPIO {

    //
    // configures pin number @pin_num as an output pin by writing to the 
    // appropriate Function Select register (see section 2.1).
    // 
    // NOTE/WARNING: This method configures one pin at a time. The @pin_num argument 
    // that is expected here is really a pin number and not a bitmask!
    //
    // Doing something like:
    //     io.configure_output_pin(VALID_BITS);
    // Would be WRONG! This call would make the program crash.
    //
    // Doing something like:
    //     if GPIO_BIT!(PIN_A) & VALID_BITS {
    //         io.configure_output_pin(PIN_A);
    //     }
    // Would be OK!
    //
    fn configure_output_pin(self: &mut GPIO, pin_num: u64) {
        let register_num = (pin_num / 10) as isize;
        let register_ref = unsafe { self.gpio_port_.offset(register_num) };
        // NOTE/WARNING: When reading from or writing to MMIO memory regions, you MUST 
        // use the std::ptr::read_volatile and std::ptr::write_volatile functions
        let current_val = unsafe { std::ptr::read_volatile(register_ref) };
        // the bit range within the register is [(pin_num % 10) * 3 .. (pin_num % 10) * 3 + 2]
        // we need to set these bits to 001
        let new_val = (current_val & !(7 << ((pin_num % 10)*3))) | (1 << ((pin_num % 10)*3));
        // NOTE/WARNING: When reading from or writing to MMIO memory regions, you MUST 
        // use the std::ptr::read_volatile and std::ptr::write_volatile functions
        unsafe { std::ptr::write_volatile(register_ref, new_val) };
    }

    fn init_outputs(self: &mut GPIO, mut outputs: u32) -> u32 {

        // alle gebruikte bits controleren of valid indien ze valid zijn zal in de output hun waarde op 1 staan
        outputs &= VALID_BITS as u32;

        // indien er output en input bits gekend zijn dan gaan we deze uit de geldige output bits filteren dus enkel al de ongebruikte bits gaan dan
        // is dit de eerste keer dat init_outputs opgeroepen wordt dan zal ouputbits en inputbits op 0 staan
        outputs &= !(self.output_bits_ | self.input_bits_ );

        // alle 28 bits overlopen en controleren of deze op 1 staan indien zo dan worden ze geconfigureerd als output bits
        for b in 0..28 {
            if outputs & (1<<b) !=0 {
                self.configure_output_pin(b as u64);
            }
        }

        self.output_bits_ |= outputs;
        outputs
        // TODO: Implement this yourself. Note: this function expects 
        // a bitmask as the @outputs argument
    }

    fn set_bits(self: &mut GPIO, value: u32) {
        // TODO: Implement this yourself. Remember to take the slowdown_ value into account!
        // This function expects a bitmask as the @value argument

        unsafe { std::ptr::write_volatile(self.gpio_set_bits_, value) };
        for i in 0..self.slowdown_ {

            unsafe { std::ptr::write_volatile(self.gpio_set_bits_, value) };
        }
    }

    fn clear_bits(self: &mut GPIO, value: u32) {        
        // TODO: Implement this yourself. Remember to take the slowdown_ value into account!
        // This function expects a bitmask as the @value argument
        unsafe { std::ptr::write_volatile(self.gpio_clr_bits_, value) };

        for i in 0..self.slowdown_ {
            unsafe { std::ptr::write_volatile(self.gpio_clr_bits_, value) };
        }
    }

    // Write all the bits of @value that also appear in @mask. Leave the rest untouched.
    // @value and @mask are bitmasks
    fn write_masked_bits( 
        self: &mut GPIO,
        value: u32,
        mask: u32
    ) {

        // TODO: Implement this yourself.

        self.clear_bits(!value & mask);
        self.set_bits(value & mask);

    }

    fn new(slowdown: u32) -> GPIO {

        // Map the GPIO register file. See section 2.1 in the assignment for details
        let map = mmap_bcm_register(GPIO_REGISTER_OFFSET as usize);
        
        // Initialize the GPIO struct with default values
        let mut io: GPIO = GPIO {
            gpio_map_: None,
            output_bits_: 0,
            input_bits_: 0,
            slowdown_: slowdown,
            gpio_port_: 0 as *mut u32,
            gpio_set_bits_: 0 as *mut u32,
            gpio_clr_bits_: 0 as *mut u32,
            gpio_read_bits_: 0 as *mut u32,
            row_mask: 0,
            bitplane_timings: [0; COLOR_DEPTH]  
        };

        match &map {
            Some(m) => {
                unsafe {
                    io.gpio_port_ = m.data() as *mut u32;
                    // TODO: Calculate the correct values of the other raw pointers here.
                    // You should use the offset() method on the gpio_port_ pointer.
                    // Keep in mind that Rust raw pointer arithmetic works exactly like
                    // C pointer arithmetic. See the course slides for details

                    io.gpio_set_bits_ = io.gpio_port_.offset(7);
                    io.gpio_clr_bits_ = io.gpio_port_.offset(10);
                    io.gpio_read_bits_ = io.gpio_port_.offset(13);

                    let mut all_used_bits : u32 = 0;
                    all_used_bits |= GPIO_BIT!(PIN_OE) | GPIO_BIT!(PIN_CLK) | GPIO_BIT!(PIN_LAT) |
                        GPIO_BIT!(PIN_R1) | GPIO_BIT!(PIN_G1) | GPIO_BIT!(PIN_B1)  |
                        GPIO_BIT!(PIN_R2) | GPIO_BIT!(PIN_G2) | GPIO_BIT!(PIN_B2);

                    row_mask = GPIO_BIT!(PIN_A);
                    let mut d = ROWS /SUB_PANELS_; 
                    if d > 2 {row_mask |= GPIO_BIT!(PIN_B);}
                    if d > 4 {row_mask |= GPIO_BIT!(PIN_C);}
                    if d > 8 {row_mask |= GPIO_BIT!(PIN_D);}
                    if d > 16 {row_mask |= GPIO_BIT!(PIN_E);}
                         
                    all_used_bits |= row_mask;
		    io.row_mask = row_mask;
                    let result = io.init_outputs(all_used_bits);
                    assert!(result == all_used_bits);
                }
                // TODO: Implement this yourself.


                let mut timing_ns: u32 = 200;
                for  b in 0..COLOR_DEPTH {
                    io.bitplane_timings[b] = timing_ns;
                    timing_ns *= 2;
                }
            },
            None => {}
        }


        io.gpio_map_ = map;
        io
    }
    
    // Calculates the pins we must activate to push the address of the specified double_row
    fn get_row_bits(self: &GPIO, double_row: u8) -> u32 {
        // TODO: Implement this yourself.
        let mut row_address: u32;
        match (double_row & 1)!=0 {
            true=> row_address = GPIO_BIT!(PIN_A),
            false=> row_address = 0,
        };

        match (double_row & 2)!=0 {
            true=> row_address |= GPIO_BIT!(PIN_B),
            false=> row_address |= 0,
	};

        match (double_row & 4)!=0 {
            true=> row_address |= GPIO_BIT!(PIN_C),
            false=> row_address |= 0,
        };

        match (double_row & 8)!=0 {
            true=> row_address |= GPIO_BIT!(PIN_D),
            false=> row_address |= 0,
        };

        match (double_row & 10)!=0 {
            true=> row_address |= GPIO_BIT!(PIN_E),
            false=> row_address |= 0,
        };
        unsafe {
            return row_address as u32 & self.row_mask;
        }
    }
}

impl Timer {
    // Reads from the 1Mhz timer register (see Section 2.5 in the assignment)
    unsafe fn read(self: &Timer) -> u32 {
        // TODO: Implement this yourself.
        unsafe { std::ptr::read_volatile(self.timereg) }

    }

    fn new() -> Timer {
        // TODO: Implement this yourself.

        let map = mmap_bcm_register(TIMER_REGISTER_OFFSET as usize);

        let mut timer: Timer = Timer {
            _timemap: None,
            timereg: 0 as *mut u32
        };

        match &map {
            Some(map) =>{
                unsafe {
                    timer.timereg = map.data() as * mut u32;
                    timer.timereg.offset(1);
                }
            },
            None => {}

        };

        timer


    }

    // High-precision sleep function (see section 2.5 in the assignment)
    // NOTE/WARNING: Since the raspberry pi's timer frequency is only 1Mhz, 
    // you cannot reach full nanosecond precision here. You will have to think
    // about how you can approximate the desired precision. Obviously, there is
    // no perfect solution here.
    fn nanosleep(self: &Timer, mut nanos: u32) {
        // TODO: Implement this yourself.
        let mut kJitterAllowanceNanos: u32 = 60*150;
        if nanos > kJitterAllowanceNanos + 5000 {
            let before = unsafe {self.read() };
            let difference = nanos - kJitterAllowanceNanos;
            match sleep(std::time::Duration::new(0, difference)){
                Some(reamin) => {
                    let after = unsafe {self.read()} ;
                    let nanoseconds_passed: u32 = 1000* (after-before) as u32;
                    nanos -= nanoseconds_passed;
                }
                None => {
                    return
                }
            }
        }
        if nanos <20 {return }
        let mut nanoseconds_left = ((nanos - 20) * 100 / 110) as i64;
        for x in nanoseconds_left..0 {
            unsafe{self.read()};
        }
    }
}

// TODO: Implement your frame calculation/updating logic here.
// The Frame should contain the pixels that are currently shown
// on the LED board. In most cases, the Frame will have less pixels
// than the input Image!
impl Frame {


    fn nextFrame(mut pos: usize, image: &Image) -> Frame {

        let mut v: Vec<Vec<Pixel>> = vec![];
        for row in 0..ROWS {
            let mut kolom: Vec<Pixel> = vec![];

                for col in 0 .. COLUMNS {
                    let position = (pos as u32 + col)% image.width as u32 ;
                    kolom.push(image.pixels[(ROWS -1 - row) as usize][position as usize]);

                        /*
                    struct Pixel*pix = &Frame[row][col];

                     // select the image column to show in this position
                    int pos = (current_position + col) % image_width;
                    struct PPMPixel*raw = &image[pos + row * image_width];

                    pix -> R = RawColorToFullColor(raw -> R);
                    pix -> G = RawColorToFullColor(raw -> G);
                    pix -> B = RawColorToFullColor(raw -> B);*/
                }
            v.push(kolom);
        }
        let mut pos2 = pos+1;
        if pos2 >= image.width{
            pos2 = 0;
        }

        let mut frame: Frame = Frame{
            pos: pos2,
            pixels: v
        };

        frame



/*
        if ( + + current_position >= image_width)
        current_position = 0;*/
    }

    fn render_water_frame(pos: u32, image: &Image) -> Frame{
        let mut v: Vec<Vec<Pixel>> = vec![];
        let sign = 13-pos;

        for row in 0..ROWS {
            let mut kolom: Vec<Pixel> = vec![];

            for col in 0 .. COLUMNS {
                let position = (pos as u32 + col)% image.width as u32 ;
                if col >=8 && col <sign && row == 7{
                    kolom.push(Pixel{r:0,g:0,b:0});
                } else{
                    kolom.push(image.pixels[(ROWS -1 - row) as usize][position as usize]);

                }


            }
            v.push(kolom);
        }
        let mut pos2 = (pos+1) as usize;
        if pos2 >= image.width{
            pos2 = 0;
        }

        let mut frame: Frame = Frame{
            pos: pos2,
            pixels: v
        };

        frame
    }
}

fn render_water(gpio:&mut GPIO, timer:&mut Timer,image:&mut Image){
    for x in 0..13{
        while (dur < 0.5){
            let mut frame = Frame::render_water_frame(x, image);
            let mut color_clk_mask: u32 = 0;
            color_clk_mask = GPIO_BIT!(PIN_R1) | GPIO_BIT!(PIN_G1) | GPIO_BIT!(PIN_B1) | GPIO_BIT!(PIN_R2) | GPIO_BIT!(PIN_G2) | GPIO_BIT!(PIN_B2) | GPIO_BIT!(PIN_CLK);
            for row_loop in 0..(ROWS / 2) {
                for b in 0..COLOR_DEPTH {
                    for col in 0..32 {
                        let mut top: Pixel = frame.pixels[row_loop as usize][col as usize];
                        let mut bot: Pixel = frame.pixels[(ROWS / 2 + row_loop) as usize][col as usize];
                        //println!("row: {} col: {} top.r: {} top.g: {} top.b: {} bot.r: {} bot.g: {} bot.b{}",row_loop,col,top.r,top.g,top.b,bot.r,bot.g,bot.b);
                        gpio.write_masked_bits(getPlaneBits(top, bot, b as u8), color_clk_mask);
                        //println!("{:#034b}",getPlaneBits(top, bot,b as u8));
                        gpio.set_bits(GPIO_BIT!(PIN_CLK));
                    };
                    gpio.clear_bits(color_clk_mask);

                    unsafe {
                        let row_bits = gpio.get_row_bits(row_loop as u8);
                        //println!("row number: {:#034b}",row_loop);
                        //println!("row bits: {:#034b}",row_bits);
                        //println!("row mask: {:#034b}",row_mask);
                        gpio.write_masked_bits(row_bits, row_mask);
                    };
                    gpio.set_bits(GPIO_BIT!(PIN_LAT));
                    gpio.clear_bits(GPIO_BIT!(PIN_LAT));
                    gpio.clear_bits(GPIO_BIT!(PIN_OE));
                    timer.nanosleep(gpio.bitplane_timings[b]);
                    gpio.set_bits(GPIO_BIT!(PIN_OE));
                };
                //gpio.set_bits(GPIO_BIT!(PIN_OE));
            };
            let mut done = match current_time.duration_since(starttime) {
                Ok(done) => done,
                Err(why) => panic!("Woops time did not elapse well: {}", why.description()),
            };
            dur = done.as_secs() as f64;
        }
    }
}


// TODO: Add your PPM parser here
// NOTE/WARNING: Please make sure that your implementation can handle comments in the PPM file
// You do not need to add support for any formats other than P6
// You may assume that the max_color value is always 255, but you should add sanity checks
// to safely reject files with other max_color values
impl Image {

    fn RawColorToFullColor(raw: u8) -> u16 {
        let number: u16 = raw as u16 ;
        number
    }

    fn read_pixel(cursor: &mut Cursor<Vec<u8>>) -> Result<Pixel, Box<std::error::Error>>{

        /*
        let mut re:u16 = cursor.read_u16::<LittleEndian>()?;
        let mut gr:u16 = cursor.read_u16::<LittleEndian>()?;
        let mut bl:u16 = cursor.read_u16::<LittleEndian>()?;*/

        let mut re:u8 = cursor.read_u8()?;
        let mut gr:u8 = cursor.read_u8()?;
        let mut bl:u8 = cursor.read_u8()?;
        let pixel = Pixel{r:Image::RawColorToFullColor(re),g:Image::RawColorToFullColor(gr),b:Image::RawColorToFullColor(bl)};


        Ok(pixel)

    }

    fn read_num(cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Box<std::error::Error>> {
        let mut v: Vec<u8> = vec![];
        let mut c:[u8; 1] = [0];




        //consume whitespace
        loop{
            cursor.read(&mut c)?;
            match &c {
                b" " | b"\t" | b"\n" => {},
                _ => { cursor.seek(std::io::SeekFrom::Current(-1)); break; }
            };
        };


        //read number
        loop{
            cursor.read(&mut c)?;
            match c[0] {
                b'0' ... b'9' => { v.push(c[0]);},
                b' '| b'\t' | b'\n' => { cursor.seek(std::io::SeekFrom::Current(-1)); break;}
                _ => {bail!("Parse error");}
            };
        };



        let num_str = std::str::from_utf8(&v)?;
        let num = num_str.parse::<usize>()?;



        Ok(num)

    }

    fn decode_ppm_image(cursor: &mut Cursor<Vec<u8>>) -> Result<Image, Box<std::error::Error>> {
        let mut image = Image {
            width: 0,
            height: 0,
            pixels: vec![]
        };
        //let mut buf2 = vec![];

        let mut header: [u8;2] = [0,2];
        cursor.read(&mut header);


        match &header {
            b"P6" => {println!("Header match"); },
            _ => {bail!("header mismatch"); }
        }


        let mut c:[u8; 1] = [0];
        loop{
            cursor.read(&mut c)?;
            match &c {
                b"#" => {loop{
                    cursor.read(&mut c)?;
                    match &c {
                        b"\n" => {break;},
                        _ => {}
                    };
                };},
                b" " | b"\t" | b"\n" => {},
                _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
            };
        };

        let w = Image::read_num(cursor)?;


        loop{
            cursor.read(&mut c)?;
            match &c {
                b"#" => {loop{
                    cursor.read(&mut c)?;
                    match &c {
                        b"\n" => {break;},
                        _ => {}
                    };
                };},
                b" " | b"\t" | b"\n" => {},
                _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
            };
        };


        let h = Image::read_num(cursor)?;

        loop{
            cursor.read(&mut c)?;
            match &c {
                b"#" => {loop{
                    cursor.read(&mut c)?;
                    match &c {
                        b"\n" => {break;},
                        _ => {}
                    };
                };},
                b" " | b"\t" | b"\n" => {},
                _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
            };
        };

        let max = Image::read_num(cursor)?;

        if max != 255{
            panic!("Max Value too damn high!");
        }

        loop{
            cursor.read(&mut c)?;
            match &c {
                b"#" => {loop{
                    cursor.read(&mut c)?;
                    match &c {
                        b"\n" => {break;},
                        _ => {}
                    };
                };},
                b" " | b"\t" | b"\n" => {},
                _ => {cursor.seek(std::io::SeekFrom::Current(-1)); break; }
            };
        };

        println!("{}", h);
        println!("{}", w);
        println!("{}",max);

        let mut allePix: Vec<Vec<Pixel>> = vec![];

        loop{
            cursor.read(&mut c)?;
            match &c {
                b" " | b"\t" | b"\n" => {},
                _ => { cursor.seek(std::io::SeekFrom::Current(-1)); break; }
            };
        };
        for x in 0..h {
            let mut hoogte_pix: Vec<Pixel> = vec![];

            for y in 0..w {
                let pixel = Image::read_pixel(cursor)?;
                hoogte_pix.push(pixel);
                //println!("y is : {}",y);
            }
            allePix.insert(0, hoogte_pix)

        }
        image.width=w;
        image.height=h;
        image.pixels=allePix;

        Ok(image)

    }



}

fn getPlaneBits(top: Pixel, bot: Pixel, plane: u8) ->  u32{
    let mut out: u32 = 0;
    if top.r & (1 << plane) !=0 {
        out |= GPIO_BIT!(PIN_R1);
    };
    if top.g & (1 << plane) !=0 {
        out |= GPIO_BIT!(PIN_G1);
    };
    if top.b & (1 << plane) !=0 {
        out |= GPIO_BIT!(PIN_B1);
    };
    if bot.r & (1 << plane) !=0 {
        out |= GPIO_BIT!(PIN_R2);
    };
    if bot.g & (1 << plane) !=0 {
        out |= GPIO_BIT!(PIN_G2);
    };
    if bot.b & (1 << plane) !=0 {
        out |= GPIO_BIT!(PIN_B2);
    };
    out
}
fn scroll_for(gpio:&mut GPIO, timer:&mut Timer, image:&mut Image, mut duration: f64, slowfactor: u64,scrollable: bool){

    let interrupt_received = Arc::new(AtomicBool::new(false));


    let mut frame: Frame = Frame::nextFrame(0, &image);

    println!("frame made");
    // This code sets up a CTRL-C handler that writes "true" to the
    // interrupt_received bool.

    if(duration == -1){
        duration = INFINITY;
    }

    let mut prev_time = SystemTime::now();
    let mut starttime= SystemTime::now();

    let mut dur =  0 as f64;

    let int_recv = interrupt_received.clone();
    ctrlc::set_handler(move || {
        int_recv.store(true, Ordering::SeqCst);
    }).unwrap();

    while (interrupt_received.load(Ordering::SeqCst) == false) && (dur < duration) {

        let mut color_clk_mask:u32 = 0;
        color_clk_mask = GPIO_BIT!(PIN_R1) | GPIO_BIT!(PIN_G1) | GPIO_BIT!(PIN_B1) | GPIO_BIT!(PIN_R2) | GPIO_BIT!(PIN_G2) | GPIO_BIT!(PIN_B2) | GPIO_BIT!(PIN_CLK);

        for row_loop in 0 .. (ROWS/2){
            for b in 0..COLOR_DEPTH{
                for col in 0 .. 32 {
                    let mut top:Pixel = frame.pixels[row_loop as usize][col as usize];
                    let mut bot:Pixel = frame.pixels[(ROWS/2 + row_loop )as usize][col as usize];
                    //println!("row: {} col: {} top.r: {} top.g: {} top.b: {} bot.r: {} bot.g: {} bot.b{}",row_loop,col,top.r,top.g,top.b,bot.r,bot.g,bot.b);
                    gpio.write_masked_bits(getPlaneBits(top, bot, b as u8), color_clk_mask);
                    //println!("{:#034b}",getPlaneBits(top, bot,b as u8));
                    gpio.set_bits(GPIO_BIT!(PIN_CLK));

                }
                gpio.clear_bits(color_clk_mask);

                unsafe {
                    let row_bits = gpio.get_row_bits(row_loop as u8);
                    //println!("row number: {:#034b}",row_loop);
                    //println!("row bits: {:#034b}",row_bits);
                    //println!("row mask: {:#034b}",row_mask);
                    gpio.write_masked_bits(row_bits, row_mask);
                };
                gpio.set_bits(GPIO_BIT!(PIN_LAT));
                gpio.clear_bits(GPIO_BIT!(PIN_LAT));
                gpio.clear_bits(GPIO_BIT!(PIN_OE));
                timer.nanosleep(gpio.bitplane_timings[b]);
                gpio.set_bits(GPIO_BIT!(PIN_OE));

            }
            //gpio.set_bits(GPIO_BIT!(PIN_OE));
        }


        let mut current_time = SystemTime::now();

        if(scrollable){
            //NEXT FRAME LOGIC


            let mut elap = match current_time.duration_since(prev_time) {
                Ok(elap) => elap,
                Err(why) => panic!("Woops time did not elapse well: {}", why.description()),
            };


            let mut sec =  elap.as_secs();
            let mut usec =  elap.as_micros();

            let mut usec_since_prev_frame = (sec) * 1000 * 1000 +(usec) as u64;

            if usec_since_prev_frame >= (40000*slowfactor) {
                prev_time = current_time;


                frame = Frame::nextFrame(frame.pos,&image);
            }
        }

        let mut done = match current_time.duration_since(starttime) {
            Ok(done) => done,
            Err(why) => panic!("Woops time did not elapse well: {}", why.description()),
        };
        dur = done.as_secs() as f64;
    }

}

pub fn main() {
    let args : Vec<String> = std::env::args().collect();
    let interrupt_received = Arc::new(AtomicBool::new(false));

    // sanity checks
    if nix::unistd::Uid::current().is_root() == false {
        eprintln!("Must run as root to be able to access /dev/mem\nPrepend \'sudo\' to the command");
        std::process::exit(1);
    } else if args.len() < 2 {
        eprintln!("Syntax: {:?} [image]", args[0]);
        std::process::exit(1);
    }

    // TODO: Read the PPM file here. You can find its name in args[1]
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path)    {
        Err(why) => panic!("Could not open file: {} (Reason: {})",
                           display, why.description()),
        Ok(file) => file
    };

    // read the full file into memory. panic on failure
    let mut raw_file = Vec::new();
    file.read_to_end(&mut raw_file).unwrap();

    // construct a cursor so we can seek in the raw buffer
    let mut cursor = Cursor::new(raw_file);
    let mut image = match Image::decode_ppm_image(&mut cursor) {
        Ok(img) => img,
        Err(why) => panic!("Could not parse PPM file - Desc: {}", why.description()),
    };
    // TODO: Initialize the GPIO struct and the Timer struct

    let mut gpio =  GPIO::new(1);
    println!("gpio made");

    let mut timer = Timer::new();
    println!("timer made");

    render_water(&mut gpio, &mut timer, &mut image);

    //scroll_for(&mut gpio,&mut timer,&mut image, -1 as f64,1,true);

    //scroll_for(&mut gpio,&mut timer,&mut image, -1 as f64,1,false);


    for index in 1..18 {

    };


    //gpio.set_bits(GPIO_BIT!(PIN_OE));
    println!("Exiting.");
    if interrupt_received.load(Ordering::SeqCst) == true {
        println!("Received CTRL-C");
    } else {
        println!("Timeout reached");
    }
    
    // TODO: You may want to reset the board here (i.e., disable all LEDs)
}
