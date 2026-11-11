//! 0x8c014da4 — store `*(u32 *)r5` big-endian into the 4 bytes at r4.
//!
//! Hidden side effects the C does not show: r0 and r6 are left holding r4+4,
//! r7 ends at -8 (the delay-slot `add #-8,r7` runs on the final iteration
//! too), r3 ends at 0, and r2 keeps the unshifted word because the last
//! `shld` amount is 0. T is left set (final `cmp/hs` succeeds).

use sh4_oracle::{Cpu, Memory};

fn shld(amount: i32, v: u32) -> u32 {
    if amount >= 0 {
        v << (amount as u32 & 31)
    } else {
        v >> ((-amount) as u32 & 31)
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(4); // mov r4,r0 ; add #4,r0
    cpu.r[7] = 24; // mov #24,r7
    cpu.r[6] = cpu.r[4]; // mov r4,r6
    loop {
        cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap(); // mov.l @r5,r2
        cpu.r[3] = (cpu.r[7] as i32).wrapping_neg() as u32; // neg r7,r3
        cpu.r[2] = shld(cpu.r[3] as i32, cpu.r[2]); // shld r3,r2
        let _ = mem.write_u8(cpu.r[6], cpu.r[2] as u8); // mov.b r2,@r6
        cpu.r[6] = cpu.r[6].wrapping_add(1); // add #1,r6
        let t = cpu.r[6] >= cpu.r[0]; // cmp/hs r0,r6
        cpu.r[7] = cpu.r[7].wrapping_add((-8i32) as u32); // bf.s delay slot: add #-8,r7
        cpu.set_t(t);
        if t {
            break; // bf.s not taken
        }
    }
    // rts ; nop
}
