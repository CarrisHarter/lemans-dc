//! 0x8c05d378 — initialise an object and install its vtable pointer.
//!
//! ```text
//! push r14; push pr
//! r14 = r4                       ; keep the object pointer
//! call 0x8c056bf8                ; base initialiser: *r4 = 0x8c1e8f88
//! [r14+0]  = 0x8c1e9148          ; vtable pointer, overwrites the callee's store
//! [r14+4]  = 0                    ; fields 1..3
//! [r14+8]  = 0
//! [r14+12] = 0
//! [r14+16] = 0xf2                 ; field 4
//! [r14+20..52] = 0                ; fields 5..13
//! return r14 (r0 = r14)
//! ```
//!
//! Side effects the C does not show: the callee's store to `[r4]` is recorded in
//! the write log even though this function immediately replaces it, and `r0`
//! ends as the object pointer, not the vtable literal.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c05d3b0).unwrap_or(0); // call target
    cpu.pr = 0x8c05d384; // return address of the jsr
    crate::registry::p_8c056bf8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c05d3b4).unwrap_or(0); // vtable pointer
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[3]);
    cpu.r[0] = cpu.r[14];
    cpu.r[1] = mem.read_u32(0x8c05d3b8).unwrap_or(0); // 0xf2
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(24), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(28), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(32), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(36), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(40), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(44), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(48), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(52), cpu.r[3]);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
