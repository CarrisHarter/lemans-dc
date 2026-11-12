//! 0x8c17fca0 - walk until `[r5]` equals the double deref captured up front.
//!
//! 0x8c17fd28 stores `[[r5]]` into a frame slot. 0x8c17fd14 reloads `[r5]`.
//! When they differ, 0x8c17fd1c dereferences a frame slot and the compare
//! repeats. A self-referential pointer takes the equal arm immediately; a
//! pointer that never becomes that captured value does not terminate.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[14] = cpu.r[5];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[0]);
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c17fd08).unwrap_or(0x8c17fd28);
    cpu.r[4] = cpu.r[14]; // jsr delay
    cpu.pr = 0x8c17fcb8;
    crate::registry::p_8c17fd28::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = cpu.r[15].wrapping_add(12);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    loop {
        cpu.r[0] = cpu.r[15];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        cpu.r[0] = mem.read_u32(0x8c17fd0c).unwrap_or(0x8c17fd14);
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c17fcd0;
        crate::registry::p_8c17fd14::run(cpu, mem);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
        if cpu.t() {
            break;
        }
        cpu.r[0] = cpu.r[15].wrapping_add(4);
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        cpu.r[0] = mem.read_u32(0x8c17fd10).unwrap_or(0x8c17fd1c);
        cpu.r[4] = cpu.r[15].wrapping_add(16);
        cpu.r[5] = 0; // jsr delay
        cpu.pr = 0x8c17fcea;
        crate::registry::p_8c17fd1c::run(cpu, mem);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
