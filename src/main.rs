// I figured Rust would be a good language for writing a CHIP-8 Emulator since
//  it's not a super complicated system and I don't have a super complicated
//  brain to handle programming something like that. No clue if this oversized
//  comment makes any sense.

extern crate sdl2;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::thread;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::keyboard::Scancode;
use sdl2::pixels;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::gfx::primitives;
use sdl2::rect;

// i swear to god if rustc doesnt inline these
fn get_nnn(instruction :&u16) -> usize {
    return (instruction&0x0FFF).into();
}
fn get_nn(instruction :&u16) -> usize {
    return (instruction&0x00FF).into();
}
fn get_n(instruction :&u16) -> usize {
    return (instruction&0x000F).into();
}
fn get_x(instruction :&u16) -> usize {
    return ((instruction&0x0F00)>>8).into();
}
fn get_y(instruction :&u16) -> usize {
    return ((instruction&0x00F0)>>4).into();
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("CHIP Oxide", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    // Now that we've stolen the demo code with minor tweaks, let's get down to business
    // get rom file
    let args: Vec<String> = env::args().collect();
    // Memory seems important, Wikipedia says 4k is a nice size, let's do it.
    let mut memory: [u8; 0x1000] = [0; 0x1000];
    // shove sprites in memory
    let mut i: usize = 0;
    let chr_rom = [
        0b11110000 as u8,//0
        0b10010000 as u8,
        0b10010000 as u8,
        0b10010000 as u8,
        0b11110000 as u8,
        0b11100000 as u8,//1
        0b00100000 as u8,
        0b00100000 as u8,
        0b00100000 as u8,
        0b11110000 as u8,
        0b01100000 as u8,//2
        0b10010000 as u8,
        0b00110000 as u8,
        0b01100000 as u8,
        0b11110000 as u8,
        0b11110000 as u8,//3
        0b00010000 as u8,
        0b01110000 as u8,
        0b00010000 as u8,
        0b11110000 as u8,
        0b10010000 as u8,//4
        0b10010000 as u8,
        0b11110000 as u8,
        0b00010000 as u8,
        0b00010000 as u8,
        0b11110000 as u8,//5
        0b10000000 as u8,
        0b11110000 as u8,
        0b00010000 as u8,
        0b11110000 as u8,
        0b01110000 as u8,//6
        0b10000000 as u8,
        0b11100000 as u8,
        0b10010000 as u8,
        0b01100000 as u8,
        0b11110000 as u8,//7
        0b00010000 as u8,
        0b00010000 as u8,
        0b00100000 as u8,
        0b00100000 as u8,
        0b01100000 as u8,//8
        0b10010000 as u8,
        0b01100000 as u8,
        0b10010000 as u8,
        0b01100000 as u8,
        0b01110000 as u8,//9
        0b10010000 as u8,
        0b01110000 as u8,
        0b00010000 as u8,
        0b00010000 as u8,
        0b01100000 as u8,//A
        0b10010000 as u8,
        0b11110000 as u8,
        0b10010000 as u8,
        0b10010000 as u8,
        0b11100000 as u8,//B
        0b10010000 as u8,
        0b11100000 as u8,
        0b10010000 as u8,
        0b11100000 as u8,
        0b01110000 as u8,//C
        0b10000000 as u8,
        0b10000000 as u8,
        0b10000000 as u8,
        0b01110000 as u8,
        0b11100000 as u8,//D
        0b10010000 as u8,
        0b10010000 as u8,
        0b10010000 as u8,
        0b11100000 as u8,
        0b11110000 as u8,//E
        0b10000000 as u8,
        0b11100000 as u8,
        0b10000000 as u8,
        0b11110000 as u8,
        0b11110000 as u8,//F
        0b10000000 as u8,
        0b11100000 as u8,
        0b10000000 as u8,
        0b10000000 as u8,
    ];
    for elem in chr_rom.iter() {
        memory[i] = *elem;
        i+=1;
    }
    // throw in the rom while we're at it
    // time to make thing work: 2 days from 2020/01/05 (and counting?)
    let mut rom: Vec<u8> = Vec::new();
    {
        let mut rom_file = File::open(&args[1]).unwrap();
        let _ = rom_file.read_to_end(&mut rom);
    }
    i = 0x200;
    // putting it in memory...
    for byte in &rom {
        memory[i] = *byte;
        i=i+1;
    }
    println!("\nfirst");
    let rom_size = i;
    println!("rom end: {}", rom_size);
    println!("rom len: {}", rom.len());
    // but neither just that or i am good enough to make endianness proper from
    // the start so do it now ig
    i=0x200;
    /*let mut temp: u8;
    //basically combing through and swapping bytes
    while i < rom.len()+0x200 {
        temp=memory[i];
        memory[i]=memory[i+1];
        memory[i+1]=temp;
        i = i+2;
    }
    println!("second");*/
    
    println!("");
    // V0-VF registers, the easy way
    let mut registers: [u8; 16] = [0; 16];
    // time registers
    let mut dt: u8 = 0; // delay
    let mut st: u8 = 0; // sound
                        // speaking of time stuff, let's make a thread for those
    let timer_thread = thread::spawn(move || {
        thread::sleep(Duration::new(1 / 60, 0));
        if st > 0 {
            st=st-1;
        }
    });
    // pointers but not really (plus a bit more)
    let mut stack: [usize;16] = [0;16];
    let mut sp: usize = 0;
    let mut pc: usize = 0x200;
    let mut program_i: u16 = 0;

    // behind the scenes stuff
    //
    let mut rp: u16 = 0;
    let mut dumptruck: usize = 0;

    // screen is nice
    let mut screen: [bool;64*32] = [false;64*32];
    // input stuff
    let mut keys_pressed: [bool;16];
    let mut instruction: u16;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        i = 0;
        for elem in screen.iter() {
            
        }
        canvas.present();
        while dt > 0 {
            thread::sleep(Duration::new(1 / 60, 0));
            dt-=1;
        }
        
        //instruction = ((memory[pc] as u16)<< 8)|memory[pc+2] as u16;
        // ^ old code that turned FF60 into FF15 somehow and now nothing works

        instruction = (memory[pc] as u16) << 8;
        instruction|=memory[pc+1] as u16;
        print!("Execution {:04x}: ",instruction);
        match (instruction&0xF000)>>12 {
            0x0 => {
                if get_nnn(&instruction) == 0x0E0 {
                    println!("CLS");
                    screen = [false;64*32];
                } else if get_nnn(&instruction) == 0x0EE {
                    println!("RET");
                    pc = stack[sp];
                    sp-=1;
                } else {
                    println!("FATAL: Invalid instruction {:X}",instruction);
                    break 'running;
                }
            },
            0x1 => {
                println!("JP {:#X}", get_nnn(&instruction));
                pc = get_nnn(&instruction);
            },
            0x2 => {
                println!("CALL {:#X}", get_nnn(&instruction));
                sp+=1;
                stack[sp] = pc;

                pc = get_nnn(&instruction);
            },
            0x3 => {
                println!("SE V{:X}, {:#02X}",get_x(&instruction),get_nn(&instruction));
                
            },
            0x4 => {
                
            },
            0x5 => {
                
            },
            0x6 => {
                println!("LD V{:X}, {:#02X}",get_x(&instruction),get_nn(&instruction));
                registers[get_x(&instruction)] = get_nn(&instruction) as u8;
            },
            0x7 => {
                
            },
            0x8 => {
                
            },
            0x9 => {
                
            },
            0xA => {
                
            },
            0xB => {
                
            },
            0xC => {
                
            },
            0xD => {
                println!("DRW V{:X}, V{:X}, {:X}", get_x(&instruction), get_y(&instruction), get_n(&instruction));

            },
            0xE => {
                
            },
            0xF => {
                match get_nn(&instruction) {
                    0x15 => {
                        println!("LD DT, V{:x}",get_x(&instruction));
                        dt = registers[get_x(&instruction)];
                    },
                    0x18 => {
                        println!("LD ST, V{:x}",get_x(&instruction));
                        st = registers[get_x(&instruction)];
                    },
                    0x1E => {
                        println!("ADD I, V{:x}",get_x(&instruction));
                        program_i += registers[get_x(&instruction)] as u16;
                    },
                    0x29 => {
                        println!("LD I, V{:x}", get_x(&instruction));
                        program_i = (registers[get_x(&instruction)]*5) as u16;
                    }
                    _ => {
                        println!("FATAL: Unknown instruction {:X}",instruction);
                        break 'running;
                    }
                }
            },
            _ => {
                println!("FATAL: Unknown instruction {:X}",instruction);
                break 'running;
            }
        }
        pc += 2;
    }
    let _ = timer_thread.join();
    Ok(())
}
