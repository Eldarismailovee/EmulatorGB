pub mod bios;
pub mod bus;
pub mod cart;
pub mod cpu;
pub mod dma;
pub mod irq;
pub mod memory;
pub mod ppu;
pub mod scheduler;
pub mod timers;

use cart::Cartridge;
use cpu::Cpu;
use bus::Bus;
use ppu::Ppu;

pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
    ppu: Ppu,
    cart: Option<Cartridge>,
    frame_buffer: Vec<u32>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(),
            ppu: Ppu::new(),
            cart: None,
            frame_buffer: vec![0; 240 * 160],
        }
    }

    pub fn load_rom(&mut self, bytes: Vec<u8>) {
        self.cart = Some(Cartridge::from_bytes(bytes));
    }

    pub fn reset(&mut self) {
        self.frame_buffer.fill(0);
    }

    pub fn frame_buffer(&self) -> &[u32] {
        &self.frame_buffer
    }

    pub fn frame_buffer_mut(&mut self) -> &mut [u32] {
        &mut self.frame_buffer
    }

    pub fn step_frame(&mut self) {
        self.draw_test_pattern();
    }

    fn draw_test_pattern(&mut self) {
        for y in 0..160 {
            for x in 0..240 {
                let i = y * 240 + x;
                let r = (x as u32) & 0xff;
                let g = (y as u32) & 0xff;
                let b = 0x40;
                self.frame_buffer[i] = (r << 16) | (g << 8) | b;
            }
        }
    }
}