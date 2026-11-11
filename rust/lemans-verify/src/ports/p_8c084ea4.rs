//! 0x8c084ea4 — copy 32-byte records from r4 to r6 until r4 == r5 (stride 32).
//!
//! Original side effects the C does not show: on return r0 = r6 (rts delay
//! slot), r1 = dst+24, r3 = dst+20, r7 = dst+16 and fr0 = the float loaded
//! from src+24 (all from the last iteration that had r6 != 0), r4 = r5 and
//! T = 1 (final cmp/eq). The `add #32,r6` sits in the bf.s delay slot, so r6
//! advances on every iteration including the last.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let end = cpu.r[5];
    cpu.set_t(cpu.r[4] == end); // cmp/eq r5,r4
    while cpu.r[4] != end {
        let src = cpu.r[4];
        let dst = cpu.r[6];
        cpu.set_t(dst == 0); // tst r6,r6
        if dst != 0 {
            let b = mem.read_u8(src).unwrap_or(0);
            cpu.r[3] = b as u32; // mov.b @r4,r3 ; extu.b r3,r3
            cpu.r[1] = dst; // mov r6,r1
            let _ = mem.write_u8(dst, b); // mov.b r3,@r6
            cpu.fr[0] = mem.read_f32(src + 4).unwrap_or(0.0);
            cpu.r[1] = dst + 4;
            cpu.r[7] = dst; // mov r6,r7
            let _ = mem.write_f32(dst + 4, cpu.fr[0]);
            cpu.fr[0] = mem.read_f32(src + 8).unwrap_or(0.0);
            cpu.r[1] = dst + 8;
            let _ = mem.write_f32(dst + 8, cpu.fr[0]);
            cpu.fr[0] = mem.read_f32(src + 12).unwrap_or(0.0);
            cpu.r[1] = dst + 12;
            let _ = mem.write_f32(dst + 12, cpu.fr[0]);
            cpu.fr[0] = mem.read_f32(src + 16).unwrap_or(0.0);
            cpu.r[7] = dst + 16; // add #16,r7
            cpu.r[3] = dst; // mov r6,r3
            let _ = mem.write_f32(dst + 16, cpu.fr[0]); // fmov fr0,@r7
            cpu.fr[0] = mem.read_f32(src + 20).unwrap_or(0.0);
            cpu.r[3] = dst + 20; // add #20,r3
            cpu.r[1] = dst;
            let _ = mem.write_f32(dst + 20, cpu.fr[0]); // fmov fr0,@r3
            cpu.fr[0] = mem.read_f32(src + 24).unwrap_or(0.0);
            cpu.r[1] = dst + 24; // add #24,r1
            let _ = mem.write_f32(dst + 24, cpu.fr[0]);
            let hw = mem.read_u16(src + 28).unwrap_or(0);
            let _ = mem.write_u16(dst + 28, hw);
        }
        cpu.r[4] = cpu.r[4].wrapping_add(32); // add #32,r4
        cpu.set_t(cpu.r[4] == end); // cmp/eq r5,r4
        cpu.r[6] = cpu.r[6].wrapping_add(32); // bf.s delay slot
    }
    cpu.r[0] = cpu.r[6]; // rts delay slot: mov r6,r0
}
