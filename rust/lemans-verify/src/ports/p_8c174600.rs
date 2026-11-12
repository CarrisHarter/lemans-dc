//! 0x8c174600 — tail-call thunk to `memset` (0x8c010f88).
//!
//! `mov.l @(0x8c1746dc,pc),r3 ; mov.l @(0x8c1746d8,pc),r4 ;
//! mov.w @(0x8c1746ce,pc),r6 ; jmp @r3 ; mov #0,r5` (delay).
//! Zeroes 0xfb8 bytes at 0x8c2b5630. `jmp` leaves `pr` alone, so the
//! callee's `rts` returns to our caller; memset's whole register
//! residue (r0=dst, r3=0, r7=n, T set) passes through unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c1746dc).unwrap_or(0); // 0x8c010f88 (memset)
    cpu.r[4] = mem.read_u32(0x8c1746d8).unwrap_or(0); // 0x8c2b5630
    cpu.r[6] = mem.read_u16(0x8c1746ce).unwrap_or(0) as i16 as i32 as u32; // 0xfb8
    cpu.r[5] = 0; // delay slot of `jmp`: mov #0,r5
    crate::registry::p_8c010f88::run(cpu, mem); // jmp @r3
}
