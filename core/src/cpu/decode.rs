use crate::cpu::register_file::Reg16::{AF, BC, DE, HL, SP};
use crate::cpu::register_file::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::{Addr, Cond, Cpu, Immediate8, Step};
use crate::hardware::Bus;

impl Cpu {
  pub fn decode_exec_fetch<B: Bus>(&mut self, bus: &mut B, op: u8) -> Step {
    match op {
      // --- 8-bit operations
      // 8-bit loads
      0x7f => self.load(bus, A, A),
      0x78 => self.load(bus, A, B),
      0x79 => self.load(bus, A, C),
      0x7a => self.load(bus, A, D),
      0x7b => self.load(bus, A, E),
      0x7c => self.load(bus, A, H),
      0x7d => self.load(bus, A, L),
      0x7e => self.load(bus, A, Addr::HL),
      0x47 => self.load(bus, B, A),
      0x40 => self.ld_b_b(bus),
      0x41 => self.load(bus, B, C),
      0x42 => self.load(bus, B, D),
      0x43 => self.load(bus, B, E),
      0x44 => self.load(bus, B, H),
      0x45 => self.load(bus, B, L),
      0x46 => self.load(bus, B, Addr::HL),
      0x4f => self.load(bus, C, A),
      0x48 => self.load(bus, C, B),
      0x49 => self.load(bus, C, C),
      0x4a => self.load(bus, C, D),
      0x4b => self.load(bus, C, E),
      0x4c => self.load(bus, C, H),
      0x4d => self.load(bus, C, L),
      0x4e => self.load(bus, C, Addr::HL),
      0x57 => self.load(bus, D, A),
      0x50 => self.load(bus, D, B),
      0x51 => self.load(bus, D, C),
      0x52 => self.load(bus, D, D),
      0x53 => self.load(bus, D, E),
      0x54 => self.load(bus, D, H),
      0x55 => self.load(bus, D, L),
      0x56 => self.load(bus, D, Addr::HL),
      0x5f => self.load(bus, E, A),
      0x58 => self.load(bus, E, B),
      0x59 => self.load(bus, E, C),
      0x5a => self.load(bus, E, D),
      0x5b => self.load(bus, E, E),
      0x5c => self.load(bus, E, H),
      0x5d => self.load(bus, E, L),
      0x5e => self.load(bus, E, Addr::HL),
      0x67 => self.load(bus, H, A),
      0x60 => self.load(bus, H, B),
      0x61 => self.load(bus, H, C),
      0x62 => self.load(bus, H, D),
      0x63 => self.load(bus, H, E),
      0x64 => self.load(bus, H, H),
      0x65 => self.load(bus, H, L),
      0x66 => self.load(bus, H, Addr::HL),
      0x6f => self.load(bus, L, A),
      0x68 => self.load(bus, L, B),
      0x69 => self.load(bus, L, C),
      0x6a => self.load(bus, L, D),
      0x6b => self.load(bus, L, E),
      0x6c => self.load(bus, L, H),
      0x6d => self.load(bus, L, L),
      0x6e => self.load(bus, L, Addr::HL),
      0x3e => self.load(bus, A, Immediate8),
      0x06 => self.load(bus, B, Immediate8),
      0x0e => self.load(bus, C, Immediate8),
      0x16 => self.load(bus, D, Immediate8),
      0x1e => self.load(bus, E, Immediate8),
      0x26 => self.load(bus, H, Immediate8),
      0x2e => self.load(bus, L, Immediate8),
      0x36 => self.load(bus, Addr::HL, Immediate8),
      0x77 => self.load(bus, Addr::HL, A),
      0x70 => self.load(bus, Addr::HL, B),
      0x71 => self.load(bus, Addr::HL, C),
      0x72 => self.load(bus, Addr::HL, D),
      0x73 => self.load(bus, Addr::HL, E),
      0x74 => self.load(bus, Addr::HL, H),
      0x75 => self.load(bus, Addr::HL, L),
      0x0a => self.load(bus, A, Addr::BC),
      0x02 => self.load(bus, Addr::BC, A),
      0x1a => self.load(bus, A, Addr::DE),
      0x12 => self.load(bus, Addr::DE, A),
      0xfa => self.load(bus, A, Addr::Direct),
      0xea => self.load(bus, Addr::Direct, A),
      0x3a => self.load(bus, A, Addr::HLD),
      0x32 => self.load(bus, Addr::HLD, A),
      0x2a => self.load(bus, A, Addr::HLI),
      0x22 => self.load(bus, Addr::HLI, A),
      0xf2 => self.load(bus, A, Addr::ZeroPageC),
      0xe2 => self.load(bus, Addr::ZeroPageC, A),
      0xf0 => self.load(bus, A, Addr::ZeroPage),
      0xe0 => self.load(bus, Addr::ZeroPage, A),
      // 8-bit arithmetic
      0x87 => self.add(bus, A),
      0x80 => self.add(bus, B),
      0x81 => self.add(bus, C),
      0x82 => self.add(bus, D),
      0x83 => self.add(bus, E),
      0x84 => self.add(bus, H),
      0x85 => self.add(bus, L),
      0x86 => self.add(bus, Addr::HL),
      0xc6 => self.add(bus, Immediate8),
      0x8f => self.adc(bus, A),
      0x88 => self.adc(bus, B),
      0x89 => self.adc(bus, C),
      0x8a => self.adc(bus, D),
      0x8b => self.adc(bus, E),
      0x8c => self.adc(bus, H),
      0x8d => self.adc(bus, L),
      0x8e => self.adc(bus, Addr::HL),
      0xce => self.adc(bus, Immediate8),
      0x97 => self.sub(bus, A),
      0x90 => self.sub(bus, B),
      0x91 => self.sub(bus, C),
      0x92 => self.sub(bus, D),
      0x93 => self.sub(bus, E),
      0x94 => self.sub(bus, H),
      0x95 => self.sub(bus, L),
      0x96 => self.sub(bus, Addr::HL),
      0xd6 => self.sub(bus, Immediate8),
      0x9f => self.sbc(bus, A),
      0x98 => self.sbc(bus, B),
      0x99 => self.sbc(bus, C),
      0x9a => self.sbc(bus, D),
      0x9b => self.sbc(bus, E),
      0x9c => self.sbc(bus, H),
      0x9d => self.sbc(bus, L),
      0x9e => self.sbc(bus, Addr::HL),
      0xde => self.sbc(bus, Immediate8),
      0xbf => self.cp(bus, A),
      0xb8 => self.cp(bus, B),
      0xb9 => self.cp(bus, C),
      0xba => self.cp(bus, D),
      0xbb => self.cp(bus, E),
      0xbc => self.cp(bus, H),
      0xbd => self.cp(bus, L),
      0xbe => self.cp(bus, Addr::HL),
      0xfe => self.cp(bus, Immediate8),
      0xa7 => self.and(bus, A),
      0xa0 => self.and(bus, B),
      0xa1 => self.and(bus, C),
      0xa2 => self.and(bus, D),
      0xa3 => self.and(bus, E),
      0xa4 => self.and(bus, H),
      0xa5 => self.and(bus, L),
      0xa6 => self.and(bus, Addr::HL),
      0xe6 => self.and(bus, Immediate8),
      0xb7 => self.or(bus, A),
      0xb0 => self.or(bus, B),
      0xb1 => self.or(bus, C),
      0xb2 => self.or(bus, D),
      0xb3 => self.or(bus, E),
      0xb4 => self.or(bus, H),
      0xb5 => self.or(bus, L),
      0xb6 => self.or(bus, Addr::HL),
      0xf6 => self.or(bus, Immediate8),
      0xaf => self.xor(bus, A),
      0xa8 => self.xor(bus, B),
      0xa9 => self.xor(bus, C),
      0xaa => self.xor(bus, D),
      0xab => self.xor(bus, E),
      0xac => self.xor(bus, H),
      0xad => self.xor(bus, L),
      0xae => self.xor(bus, Addr::HL),
      0xee => self.xor(bus, Immediate8),
      0x3c => self.inc(bus, A),
      0x04 => self.inc(bus, B),
      0x0c => self.inc(bus, C),
      0x14 => self.inc(bus, D),
      0x1c => self.inc(bus, E),
      0x24 => self.inc(bus, H),
      0x2c => self.inc(bus, L),
      0x34 => self.inc(bus, Addr::HL),
      0x3d => self.dec(bus, A),
      0x05 => self.dec(bus, B),
      0x0d => self.dec(bus, C),
      0x15 => self.dec(bus, D),
      0x1d => self.dec(bus, E),
      0x25 => self.dec(bus, H),
      0x2d => self.dec(bus, L),
      0x35 => self.dec(bus, Addr::HL),
      0x07 => self.rlca(bus),
      0x17 => self.rla(bus),
      0x0f => self.rrca(bus),
      0x1f => self.rra(bus),
      // --- Control
      0xc3 => self.jp(bus),
      0xe9 => self.jp_hl(bus),
      0x18 => self.jr(bus),
      0xcd => self.call(bus),
      0xc9 => self.ret(bus),
      0xd9 => self.reti(bus),
      0xc2 => self.jp_cc(bus, Cond::NZ),
      0xca => self.jp_cc(bus, Cond::Z),
      0xd2 => self.jp_cc(bus, Cond::NC),
      0xda => self.jp_cc(bus, Cond::C),
      0x20 => self.jr_cc(bus, Cond::NZ),
      0x28 => self.jr_cc(bus, Cond::Z),
      0x30 => self.jr_cc(bus, Cond::NC),
      0x38 => self.jr_cc(bus, Cond::C),
      0xc4 => self.call_cc(bus, Cond::NZ),
      0xcc => self.call_cc(bus, Cond::Z),
      0xd4 => self.call_cc(bus, Cond::NC),
      0xdc => self.call_cc(bus, Cond::C),
      0xc0 => self.ret_cc(bus, Cond::NZ),
      0xc8 => self.ret_cc(bus, Cond::Z),
      0xd0 => self.ret_cc(bus, Cond::NC),
      0xd8 => self.ret_cc(bus, Cond::C),
      0xc7 => self.rst(bus, 0x00),
      0xcf => self.rst(bus, 0x08),
      0xd7 => self.rst(bus, 0x10),
      0xdf => self.rst(bus, 0x18),
      0xe7 => self.rst(bus, 0x20),
      0xef => self.rst(bus, 0x28),
      0xf7 => self.rst(bus, 0x30),
      0xff => self.rst(bus, 0x38),
      // --- Miscellaneous
      0x76 => self.halt(bus),
      0x10 => self.stop(bus),
      0xf3 => self.di(bus),
      0xfb => self.ei(bus),
      0x3f => self.ccf(bus),
      0x37 => self.scf(bus),
      0x00 => self.nop(bus),
      0x27 => self.daa(bus),
      0x2f => self.cpl(bus),
      // --- 16-bit operations
      // 16-bit loads
      0x01 => self.load16_imm(bus, BC),
      0x11 => self.load16_imm(bus, DE),
      0x21 => self.load16_imm(bus, HL),
      0x31 => self.load16_imm(bus, SP),
      0x08 => self.load16_nn_sp(bus),
      0xf9 => self.load16_sp_hl(bus),
      0xf8 => self.load16_hl_sp_e(bus),
      0xc5 => self.push16(bus, BC),
      0xd5 => self.push16(bus, DE),
      0xe5 => self.push16(bus, HL),
      0xf5 => self.push16(bus, AF),
      0xc1 => self.pop16(bus, BC),
      0xd1 => self.pop16(bus, DE),
      0xe1 => self.pop16(bus, HL),
      0xf1 => self.pop16(bus, AF),
      // 16-bit arithmetic
      0x09 => self.add16(bus, BC),
      0x19 => self.add16(bus, DE),
      0x29 => self.add16(bus, HL),
      0x39 => self.add16(bus, SP),
      0xe8 => self.add16_sp_e(bus),
      0x03 => self.inc16(bus, BC),
      0x13 => self.inc16(bus, DE),
      0x23 => self.inc16(bus, HL),
      0x33 => self.inc16(bus, SP),
      0x0b => self.dec16(bus, BC),
      0x1b => self.dec16(bus, DE),
      0x2b => self.dec16(bus, HL),
      0x3b => self.dec16(bus, SP),
      0xcb => self.cb_prefix(bus),
      0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd => {
        self.undefined(bus, op)
      }
    }
  }

  pub fn cb_decode_exec_fetch<B: Bus>(&mut self, bus: &mut B, op: u8) -> Step {
    match op {
      // --- 8-bit operations
      // 8-bit arithmetic
      0x07 => self.rlc(bus, A),
      0x00 => self.rlc(bus, B),
      0x01 => self.rlc(bus, C),
      0x02 => self.rlc(bus, D),
      0x03 => self.rlc(bus, E),
      0x04 => self.rlc(bus, H),
      0x05 => self.rlc(bus, L),
      0x06 => self.rlc(bus, Addr::HL),
      0x17 => self.rl(bus, A),
      0x10 => self.rl(bus, B),
      0x11 => self.rl(bus, C),
      0x12 => self.rl(bus, D),
      0x13 => self.rl(bus, E),
      0x14 => self.rl(bus, H),
      0x15 => self.rl(bus, L),
      0x16 => self.rl(bus, Addr::HL),
      0x0f => self.rrc(bus, A),
      0x08 => self.rrc(bus, B),
      0x09 => self.rrc(bus, C),
      0x0a => self.rrc(bus, D),
      0x0b => self.rrc(bus, E),
      0x0c => self.rrc(bus, H),
      0x0d => self.rrc(bus, L),
      0x0e => self.rrc(bus, Addr::HL),
      0x1f => self.rr(bus, A),
      0x18 => self.rr(bus, B),
      0x19 => self.rr(bus, C),
      0x1a => self.rr(bus, D),
      0x1b => self.rr(bus, E),
      0x1c => self.rr(bus, H),
      0x1d => self.rr(bus, L),
      0x1e => self.rr(bus, Addr::HL),
      0x27 => self.sla(bus, A),
      0x20 => self.sla(bus, B),
      0x21 => self.sla(bus, C),
      0x22 => self.sla(bus, D),
      0x23 => self.sla(bus, E),
      0x24 => self.sla(bus, H),
      0x25 => self.sla(bus, L),
      0x26 => self.sla(bus, Addr::HL),
      0x2f => self.sra(bus, A),
      0x28 => self.sra(bus, B),
      0x29 => self.sra(bus, C),
      0x2a => self.sra(bus, D),
      0x2b => self.sra(bus, E),
      0x2c => self.sra(bus, H),
      0x2d => self.sra(bus, L),
      0x2e => self.sra(bus, Addr::HL),
      0x3f => self.srl(bus, A),
      0x38 => self.srl(bus, B),
      0x39 => self.srl(bus, C),
      0x3a => self.srl(bus, D),
      0x3b => self.srl(bus, E),
      0x3c => self.srl(bus, H),
      0x3d => self.srl(bus, L),
      0x3e => self.srl(bus, Addr::HL),
      0x37 => self.swap(bus, A),
      0x30 => self.swap(bus, B),
      0x31 => self.swap(bus, C),
      0x32 => self.swap(bus, D),
      0x33 => self.swap(bus, E),
      0x34 => self.swap(bus, H),
      0x35 => self.swap(bus, L),
      0x36 => self.swap(bus, Addr::HL),
      0x47 => self.bit(bus, 0, A),
      0x4f => self.bit(bus, 1, A),
      0x57 => self.bit(bus, 2, A),
      0x5f => self.bit(bus, 3, A),
      0x67 => self.bit(bus, 4, A),
      0x6f => self.bit(bus, 5, A),
      0x77 => self.bit(bus, 6, A),
      0x7f => self.bit(bus, 7, A),
      0x40 => self.bit(bus, 0, B),
      0x48 => self.bit(bus, 1, B),
      0x50 => self.bit(bus, 2, B),
      0x58 => self.bit(bus, 3, B),
      0x60 => self.bit(bus, 4, B),
      0x68 => self.bit(bus, 5, B),
      0x70 => self.bit(bus, 6, B),
      0x78 => self.bit(bus, 7, B),
      0x41 => self.bit(bus, 0, C),
      0x49 => self.bit(bus, 1, C),
      0x51 => self.bit(bus, 2, C),
      0x59 => self.bit(bus, 3, C),
      0x61 => self.bit(bus, 4, C),
      0x69 => self.bit(bus, 5, C),
      0x71 => self.bit(bus, 6, C),
      0x79 => self.bit(bus, 7, C),
      0x42 => self.bit(bus, 0, D),
      0x4a => self.bit(bus, 1, D),
      0x52 => self.bit(bus, 2, D),
      0x5a => self.bit(bus, 3, D),
      0x62 => self.bit(bus, 4, D),
      0x6a => self.bit(bus, 5, D),
      0x72 => self.bit(bus, 6, D),
      0x7a => self.bit(bus, 7, D),
      0x43 => self.bit(bus, 0, E),
      0x4b => self.bit(bus, 1, E),
      0x53 => self.bit(bus, 2, E),
      0x5b => self.bit(bus, 3, E),
      0x63 => self.bit(bus, 4, E),
      0x6b => self.bit(bus, 5, E),
      0x73 => self.bit(bus, 6, E),
      0x7b => self.bit(bus, 7, E),
      0x44 => self.bit(bus, 0, H),
      0x4c => self.bit(bus, 1, H),
      0x54 => self.bit(bus, 2, H),
      0x5c => self.bit(bus, 3, H),
      0x64 => self.bit(bus, 4, H),
      0x6c => self.bit(bus, 5, H),
      0x74 => self.bit(bus, 6, H),
      0x7c => self.bit(bus, 7, H),
      0x45 => self.bit(bus, 0, L),
      0x4d => self.bit(bus, 1, L),
      0x55 => self.bit(bus, 2, L),
      0x5d => self.bit(bus, 3, L),
      0x65 => self.bit(bus, 4, L),
      0x6d => self.bit(bus, 5, L),
      0x75 => self.bit(bus, 6, L),
      0x7d => self.bit(bus, 7, L),
      0x46 => self.bit(bus, 0, Addr::HL),
      0x4e => self.bit(bus, 1, Addr::HL),
      0x56 => self.bit(bus, 2, Addr::HL),
      0x5e => self.bit(bus, 3, Addr::HL),
      0x66 => self.bit(bus, 4, Addr::HL),
      0x6e => self.bit(bus, 5, Addr::HL),
      0x76 => self.bit(bus, 6, Addr::HL),
      0x7e => self.bit(bus, 7, Addr::HL),
      0xc7 => self.set(bus, 0, A),
      0xcf => self.set(bus, 1, A),
      0xd7 => self.set(bus, 2, A),
      0xdf => self.set(bus, 3, A),
      0xe7 => self.set(bus, 4, A),
      0xef => self.set(bus, 5, A),
      0xf7 => self.set(bus, 6, A),
      0xff => self.set(bus, 7, A),
      0xc0 => self.set(bus, 0, B),
      0xc8 => self.set(bus, 1, B),
      0xd0 => self.set(bus, 2, B),
      0xd8 => self.set(bus, 3, B),
      0xe0 => self.set(bus, 4, B),
      0xe8 => self.set(bus, 5, B),
      0xf0 => self.set(bus, 6, B),
      0xf8 => self.set(bus, 7, B),
      0xc1 => self.set(bus, 0, C),
      0xc9 => self.set(bus, 1, C),
      0xd1 => self.set(bus, 2, C),
      0xd9 => self.set(bus, 3, C),
      0xe1 => self.set(bus, 4, C),
      0xe9 => self.set(bus, 5, C),
      0xf1 => self.set(bus, 6, C),
      0xf9 => self.set(bus, 7, C),
      0xc2 => self.set(bus, 0, D),
      0xca => self.set(bus, 1, D),
      0xd2 => self.set(bus, 2, D),
      0xda => self.set(bus, 3, D),
      0xe2 => self.set(bus, 4, D),
      0xea => self.set(bus, 5, D),
      0xf2 => self.set(bus, 6, D),
      0xfa => self.set(bus, 7, D),
      0xc3 => self.set(bus, 0, E),
      0xcb => self.set(bus, 1, E),
      0xd3 => self.set(bus, 2, E),
      0xdb => self.set(bus, 3, E),
      0xe3 => self.set(bus, 4, E),
      0xeb => self.set(bus, 5, E),
      0xf3 => self.set(bus, 6, E),
      0xfb => self.set(bus, 7, E),
      0xc4 => self.set(bus, 0, H),
      0xcc => self.set(bus, 1, H),
      0xd4 => self.set(bus, 2, H),
      0xdc => self.set(bus, 3, H),
      0xe4 => self.set(bus, 4, H),
      0xec => self.set(bus, 5, H),
      0xf4 => self.set(bus, 6, H),
      0xfc => self.set(bus, 7, H),
      0xc5 => self.set(bus, 0, L),
      0xcd => self.set(bus, 1, L),
      0xd5 => self.set(bus, 2, L),
      0xdd => self.set(bus, 3, L),
      0xe5 => self.set(bus, 4, L),
      0xed => self.set(bus, 5, L),
      0xf5 => self.set(bus, 6, L),
      0xfd => self.set(bus, 7, L),
      0xc6 => self.set(bus, 0, Addr::HL),
      0xce => self.set(bus, 1, Addr::HL),
      0xd6 => self.set(bus, 2, Addr::HL),
      0xde => self.set(bus, 3, Addr::HL),
      0xe6 => self.set(bus, 4, Addr::HL),
      0xee => self.set(bus, 5, Addr::HL),
      0xf6 => self.set(bus, 6, Addr::HL),
      0xfe => self.set(bus, 7, Addr::HL),
      0x87 => self.res(bus, 0, A),
      0x8f => self.res(bus, 1, A),
      0x97 => self.res(bus, 2, A),
      0x9f => self.res(bus, 3, A),
      0xa7 => self.res(bus, 4, A),
      0xaf => self.res(bus, 5, A),
      0xb7 => self.res(bus, 6, A),
      0xbf => self.res(bus, 7, A),
      0x80 => self.res(bus, 0, B),
      0x88 => self.res(bus, 1, B),
      0x90 => self.res(bus, 2, B),
      0x98 => self.res(bus, 3, B),
      0xa0 => self.res(bus, 4, B),
      0xa8 => self.res(bus, 5, B),
      0xb0 => self.res(bus, 6, B),
      0xb8 => self.res(bus, 7, B),
      0x81 => self.res(bus, 0, C),
      0x89 => self.res(bus, 1, C),
      0x91 => self.res(bus, 2, C),
      0x99 => self.res(bus, 3, C),
      0xa1 => self.res(bus, 4, C),
      0xa9 => self.res(bus, 5, C),
      0xb1 => self.res(bus, 6, C),
      0xb9 => self.res(bus, 7, C),
      0x82 => self.res(bus, 0, D),
      0x8a => self.res(bus, 1, D),
      0x92 => self.res(bus, 2, D),
      0x9a => self.res(bus, 3, D),
      0xa2 => self.res(bus, 4, D),
      0xaa => self.res(bus, 5, D),
      0xb2 => self.res(bus, 6, D),
      0xba => self.res(bus, 7, D),
      0x83 => self.res(bus, 0, E),
      0x8b => self.res(bus, 1, E),
      0x93 => self.res(bus, 2, E),
      0x9b => self.res(bus, 3, E),
      0xa3 => self.res(bus, 4, E),
      0xab => self.res(bus, 5, E),
      0xb3 => self.res(bus, 6, E),
      0xbb => self.res(bus, 7, E),
      0x84 => self.res(bus, 0, H),
      0x8c => self.res(bus, 1, H),
      0x94 => self.res(bus, 2, H),
      0x9c => self.res(bus, 3, H),
      0xa4 => self.res(bus, 4, H),
      0xac => self.res(bus, 5, H),
      0xb4 => self.res(bus, 6, H),
      0xbc => self.res(bus, 7, H),
      0x85 => self.res(bus, 0, L),
      0x8d => self.res(bus, 1, L),
      0x95 => self.res(bus, 2, L),
      0x9d => self.res(bus, 3, L),
      0xa5 => self.res(bus, 4, L),
      0xad => self.res(bus, 5, L),
      0xb5 => self.res(bus, 6, L),
      0xbd => self.res(bus, 7, L),
      0x86 => self.res(bus, 0, Addr::HL),
      0x8e => self.res(bus, 1, Addr::HL),
      0x96 => self.res(bus, 2, Addr::HL),
      0x9e => self.res(bus, 3, Addr::HL),
      0xa6 => self.res(bus, 4, Addr::HL),
      0xae => self.res(bus, 5, Addr::HL),
      0xb6 => self.res(bus, 6, Addr::HL),
      0xbe => self.res(bus, 7, Addr::HL),
    }
  }
}
