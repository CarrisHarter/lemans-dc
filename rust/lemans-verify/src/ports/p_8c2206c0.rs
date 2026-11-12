//! 0x8c2206c0 — arm four 0x34-byte records through 0x8c220678, then raise a flag.
//!
//! ```text
//! mov.l r14,@-r15 ... sts.l pr,@-r15   ; save r11-r14 and pr
//! r13 = 0x00d0 (208); r12 = &0x8c23d468; r11 = 0x8c220678
//! do {
//!     r4 = [0x8c23d468] + 4;
//!     jsr @r11                         ; 0x8c220678
//!     r4 += r14                        ; delay slot
//!     r14 += 52;
//! } while (r14 < 208);
//! [0x8c2bde5c] = 1;
//! return 0;
//! ```
//!
//! r14 takes 0, 52, 104 and 156, so the callee arms four records. It returns
//! with the entry SR restored; the loop's final `cmp/hs` leaves T set, and no
//! later instruction touches the flags, so the function exits with T = 1.
//! r11-r14 and pr are popped back to their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue pushes.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = 0; // mov #0,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = mem.read_u16(0x8c2206f4).unwrap_or(0) as i16 as i32 as u32; // 0x00d0
    cpu.r[12] = mem.read_u32(0x8c2206fc).unwrap_or(0); // 0x8c23d468
    cpu.r[11] = mem.read_u32(0x8c2206f8).unwrap_or(0); // 0x8c220678
    loop {
        cpu.r[4] = mem.read_u32(cpu.r[12]).unwrap_or(0); // mov.l @r12,r4
        cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
        cpu.pr = 0x8c2206da; // jsr @r11
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[14]); // delay slot: add r14,r4
        crate::registry::p_8c220678::run(cpu, mem);
        cpu.r[14] = cpu.r[14].wrapping_add(52); // add #52,r14
        let t = cpu.r[14] >= cpu.r[13]; // cmp/hs r13,r14
        cpu.set_t(t);
        if !t {
            continue;
        }
        break;
    }
    cpu.r[3] = mem.read_u32(0x8c220700).unwrap_or(0); // 0x8c2bde5c
    cpu.r[2] = 1; // mov #1,r2
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    // Epilogue pops; the final pop sits in the rts delay slot.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
