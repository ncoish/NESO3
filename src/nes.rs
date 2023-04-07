use cpu::CPU;
use ppu::PPU;
use parser;

pub struct NES {
    cpu: CPU,
    ppu: PPU,
}