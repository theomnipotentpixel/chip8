use macroquad::prelude::*;
use std::fs;
use clap::Parser;
use macroquad::rand::rand;

struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index_register: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub display: [[bool; 64]; 32],
    opcode: u16,
    keypad: [bool; 16],
    font_start: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let font_set =
            [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
            ];
        let mut memory = [0; 4096];
        for i in 0..80 {
            memory[i + 0x50] = font_set[i];
        }
        Self {
            registers: [0; 16],
            memory,
            index_register: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; 64]; 32],
            opcode: 0,
            keypad: [false; 16],
            font_start: 0x50,
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        self.load_rom_offset(rom_path, 0x200);
    }

    pub fn load_rom_offset(&mut self, rom_path: &str, start_addr: u16) {
        let data = fs::read(rom_path).unwrap();
        if data.len() + start_addr as usize > 4096 {
            panic!("ROM file is too large!");
        }
        for i in 0..data.len() {
            self.memory[start_addr as usize + i] = data[i];
        }
    }

    pub fn step(&mut self){
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16);
        self.pc += 2;
        if self.opcode == 0x00e0 {
            self.op_00e0();
        } else if self.opcode == 0x00ee {
            self.op_00ee();
        } else if self.opcode & 0xf000  == 0x1000 {
            self.op_1nnn()
        } else if self.opcode & 0xf000  == 0x2000 {
            self.op_2nnn()
        } else if self.opcode & 0xf000  == 0x3000 {
            self.op_3xkk()
        } else if self.opcode & 0xf000  == 0x4000 {
            self.op_4xkk()
        } else if self.opcode & 0xf00f  == 0x5000 {
            self.op_5xy0()
        } else if self.opcode & 0xf000  == 0x6000 {
            self.op_6xkk()
        } else if self.opcode & 0xf000  == 0x7000 {
            self.op_7xkk()
        } else if self.opcode & 0xf00f  == 0x8000 {
            self.op_8xy0()
        } else if self.opcode & 0xf00f  == 0x8001 {
            self.op_8xy1()
        } else if self.opcode & 0xf00f  == 0x8002 {
            self.op_8xy2()
        } else if self.opcode & 0xf00f  == 0x8003 {
            self.op_8xy3()
        } else if self.opcode & 0xf00f  == 0x8004 {
            self.op_8xy4()
        } else if self.opcode & 0xf00f  == 0x8005 {
            self.op_8xy5()
        } else if self.opcode & 0xf00f  == 0x8006 {
            self.op_8xy6()
        } else if self.opcode & 0xf00f  == 0x8007 {
            self.op_8xy7()
        } else if self.opcode & 0xf00f  == 0x800e {
            self.op_8xye()
        } else if self.opcode & 0xf00f  == 0x9000 {
            self.op_9xy0()
        } else if self.opcode & 0xf000  == 0xa000 {
            self.op_annn()
        } else if self.opcode & 0xf000  == 0xb000 {
            self.op_bnnn()
        } else if self.opcode & 0xf000  == 0xc000 {
            self.op_cxkk()
        } else if self.opcode & 0xf000  == 0xd000 {
            self.op_dxyn()
        } else if self.opcode & 0xf0ff  == 0xe09e {
            self.op_ex9e()
        } else if self.opcode & 0xf0ff  == 0xe0a1 {
            self.op_exa1()
        } else if self.opcode & 0xf0ff  == 0xf007 {
            self.op_fx07()
        } else if self.opcode & 0xf0ff  == 0xf00a {
            self.op_fx0a()
        } else if self.opcode & 0xf0ff  == 0xf015 {
            self.op_fx15()
        } else if self.opcode & 0xf0ff  == 0xf018 {
            self.op_fx18()
        } else if self.opcode & 0xf0ff  == 0xf01e {
            self.op_fx1e()
        } else if self.opcode & 0xf0ff  == 0xf029 {
            self.op_fx29()
        } else if self.opcode & 0xf0ff  == 0xf033 {
            self.op_fx33()
        } else if self.opcode & 0xf0ff  == 0xf055 {
            self.op_fx55()
        } else if self.opcode & 0xf0ff  == 0xf065 {
            self.op_fx65()
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn op_00e0(&mut self) {
        self.display = [[false; 64]; 32];
    }

    pub fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    pub fn op_1nnn(&mut self) {
        self.pc = self.opcode & 0xfff;
    }

    pub fn op_2nnn(&mut self) {
        let addr = self.opcode & 0xfff;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.sp %= 16;
        self.pc = addr;
    }

    pub fn op_3xkk(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let byte = (self.opcode & 0xff) as u8;
        if self.registers[vx as usize] == byte {
            self.pc += 2;
        }
    }

    pub fn op_4xkk(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let byte = (self.opcode & 0xff) as u8;
        if self.registers[vx as usize] != byte {
            self.pc += 2;
        }
    }

    pub fn op_5xy0(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    pub fn op_6xkk(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let byte = (self.opcode & 0xff) as u8;
        self.registers[vx as usize] = byte;
    }

    pub fn op_7xkk(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let byte = (self.opcode & 0xff) as u8;
        self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(byte);
    }

    pub fn op_8xy0(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        self.registers[vx as usize] = self.registers[vy as usize];
    }

    pub fn op_8xy1(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        self.registers[vx as usize] |= self.registers[vy as usize];
    }

    pub fn op_8xy2(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        self.registers[vx as usize] &= self.registers[vy as usize];
    }

    pub fn op_8xy3(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        self.registers[vx as usize] ^= self.registers[vy as usize];
    }

    pub fn op_8xy4(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        let sum = self.registers[vx as usize] as u16 + self.registers[vy as usize] as u16;
        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[vx as usize] = (sum & 0xff) as u8;
    }

    pub fn op_8xy5(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        if self.registers[vx as usize] > self.registers[vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[vx as usize] = self.registers[vx as usize].wrapping_sub(self.registers[vy as usize]);
    }

    pub fn op_8xy6(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        self.registers[0xF] = self.registers[vx as usize] & 0x1;

        self.registers[vx as usize] >>= 1;
    }

    pub fn op_8xy7(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        if self.registers[vy as usize] > self.registers[vx as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[vx as usize] = self.registers[vy as usize].wrapping_sub(self.registers[vx as usize]);
    }

    pub fn op_8xye(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        self.registers[0xF] = (self.registers[vx as usize] & 0x80) >> 7;
        self.registers[vx as usize] <<= 1;
    }

    pub fn op_9xy0(&mut self){
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    pub fn op_annn(&mut self){
        self.index_register = (self.opcode & 0x0fff);
    }

    pub fn op_bnnn(&mut self){
        self.pc = self.registers[0] as u16 + (self.opcode & 0x0fff);
    }

    pub fn op_cxkk(&mut self){
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let byte = (self.opcode & 0xff) as u8;
        self.registers[vx as usize] = (rand() as u8) & byte;
    }

    pub fn op_dxyn(&mut self){
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let vy = ((self.opcode & 0x0f0) >> 4) as u8;
        let height = (self.opcode & 0x0f) as u8;

        let x_pos = self.registers[vx as usize] % 64;
        let y_pos = self.registers[vy as usize] % 32;

        self.registers[0xF] = 0;

        for row in 0..height {
            let spr_byte = self.memory[(self.index_register + row as u16) as usize];
            for col in 0..8 {
                let spr_pixel = spr_byte & (0x80 >> col);
                let screen_pixel = self.display[((y_pos+row) as usize)%32][((x_pos+col) as usize)%64];

                if spr_pixel != 0 {
                    if screen_pixel {
                        self.registers[0xF] = 1;
                    }
                    self.display[((y_pos+row) as usize)%32][((x_pos+col) as usize)%64] = !self.display[((y_pos+row) as usize)%32][((x_pos+col) as usize)%64];
                }
            }
        }
    }

    pub fn op_ex9e(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        if self.keypad[self.registers[vx as usize] as usize] {
            self.pc += 2;
        }
    }

    pub fn op_exa1(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        if !self.keypad[self.registers[vx as usize] as usize] {
            self.pc += 2;
        }
    }

    pub fn op_fx07(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        self.registers[vx as usize] = self.delay_timer;
    }

    pub fn op_fx0a(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as usize;

        if self.keypad[0]
        {
            self.registers[vx] = 0;
        }
        else if self.keypad[1]
        {
            self.registers[vx] = 1;
        }
        else if self.keypad[2]
        {
            self.registers[vx] = 2;
        }
        else if self.keypad[3]
        {
            self.registers[vx] = 3;
        }
        else if self.keypad[4]
        {
            self.registers[vx] = 4;
        }
        else if self.keypad[5]
        {
            self.registers[vx] = 5;
        }
        else if self.keypad[6]
        {
            self.registers[vx] = 6;
        }
        else if self.keypad[7]
        {
            self.registers[vx] = 7;
        }
        else if self.keypad[8]
        {
            self.registers[vx] = 8;
        }
        else if self.keypad[9]
        {
            self.registers[vx] = 9;
        }
        else if self.keypad[10]
        {
            self.registers[vx] = 10;
        }
        else if self.keypad[11]
        {
            self.registers[vx] = 11;
        }
        else if self.keypad[12]
        {
            self.registers[vx] = 12;
        }
        else if self.keypad[13]
        {
            self.registers[vx] = 13;
        }
        else if self.keypad[14]
        {
            self.registers[vx] = 14;
        }
        else if self.keypad[15]
        {
            self.registers[vx] = 15;
        }
        else
        {
            self.pc -= 2;
        }
    }

    pub fn op_fx15(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        self.delay_timer = self.registers[vx as usize];
    }

    pub fn op_fx18(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        self.sound_timer = self.registers[vx as usize];
    }

    pub fn op_fx1e(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        self.index_register += self.registers[vx as usize] as u16;
    }

    pub fn op_fx29(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let digit = self.registers[vx as usize];
        self.index_register = self.font_start + ((5 * digit) as u16);
    }

    pub fn op_fx33(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        let mut val = self.registers[vx as usize];

        self.memory[(self.index_register + 2) as usize] = val % 10;
        val /= 10;
        self.memory[(self.index_register + 1) as usize] = val % 10;
        val /= 10;
        self.memory[(self.index_register) as usize] = val % 10;
    }

    pub fn op_fx55(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        for i in 0..=vx {
            self.memory[self.index_register as usize + i as usize] = self.registers[i as usize];
        }
    }

    pub fn op_fx65(&mut self) {
        let vx = ((self.opcode & 0xf00) >> 8) as u8;
        for i in 0..=vx {
            self.registers[i as usize] = self.memory[self.index_register as usize + i as usize];
        }
    }

    pub fn update_keys(&mut self) {
        let keys = [
            KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
            KeyCode::Q, KeyCode::W, KeyCode::E, KeyCode::R,
            KeyCode::A, KeyCode::S, KeyCode::D, KeyCode::F,
            KeyCode::Z, KeyCode::X, KeyCode::C, KeyCode::V
        ];
        for i in 0..keys.len() {
            if is_key_down(keys[i]) {
                self.keypad[i] = true;
            } else {
                self.keypad[i] = false;
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the rom
    #[arg(long)]
    rom: Option<String>,

    /// Milliseconds per step
    #[arg(short, long, default_value_t = 3)]
    delay: u16,
}

#[macroquad::main("Chip 8 Emulator")]
async fn main() {
    let args = Args::parse();

    let screen_scale = 16;
    let mut state: Chip8 = Chip8::new();
    let mut rom = args.rom;
    state.load_rom(&*rom.unwrap_or("res/test_opcode.ch8".parse().unwrap()));
    request_new_screen_size((64*screen_scale) as f32, (32*screen_scale) as f32);
    next_frame().await;
    clear_background(WHITE);
    let mut last_cycle = std::time::Instant::now();
    let mut last_frame = std::time::Instant::now();

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if last_cycle.elapsed().as_millis() > args.delay as u128 {
            last_cycle = std::time::Instant::now();

            state.update_keys();
            state.step();
        }
        if last_frame.elapsed().as_secs_f32() > 1.0 / 60.0 {
            last_frame = std::time::Instant::now();
            next_frame().await;
            clear_background(WHITE);
            for x in 0..64 {
                for y in 0..32 {
                    if state.display[y][x] {
                        draw_rectangle((x * screen_scale) as f32, (y * screen_scale) as f32, screen_scale as f32, screen_scale as f32, WHITE);
                    } else {
                        draw_rectangle((x * screen_scale) as f32, (y * screen_scale) as f32, screen_scale as f32, screen_scale as f32, BLACK);
                    }
                }
            }
        }
    }
}
