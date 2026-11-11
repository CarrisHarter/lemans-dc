//! 0x8c1ac904 — 64-iteration blend loop over two 0x8c1e5c00-based buffers.
//!
//! Prologue: fr0 = fr1 = 0, then a switch on r5 (1/2/4 with r4 sub-select)
//! loads r3 from a literal table and computes
//!   fr0 = f32[r3+56] + f32[0x8c1aca50]
//!   fr1 = f32[r3+68] + f32[0x8c1aca54]
//! (the second add sits in the `bra` delay slot). Any other r5 leaves
//! fr0 = fr1 = 0.
//!
//! Then r4 = (r4 << 8) << 1 (`shll8` then `shll`, the latter setting T once),
//! and the loop runs 64 times with r13 = 0,8,...,504:
//!   [r6+r13] = -f32[r3'+r13] * fr5 + fr0        (r6 = r4' + 0x8c2b7a30, r3' = 0x8c1e5c04)
//!   [r7+r13] = fr1 + f32[0x8c1e5c00+r13] * fr3  (r7 = 0x8c2b7a34 + r4')
//! with fr5 = f32[0x8c1aca70], fr3 = f32[0x8c1aca78]. The `bt.s` delay slot
//! `add #8,r13` runs on every iteration including the last. r13/r14 are
//! restored by the epilogue pops, so they end at their entry values; final
//! T = 0 (last cmp/gt: 64 > 64 is false). r4 is clobbered to r6+r13.

use sh4_oracle::{Cpu, Memory};

fn arm(cpu: &mut Cpu, mem: &mut Memory) {
    // r3 already holds the table base; pattern per switch arm.
    let a = mem.read_f32(0x8C1ACA50).unwrap();
    cpu.r[3] = cpu.r[3].wrapping_add(0); // base
    let base = cpu.r[3];
    cpu.r[1] = base.wrapping_add(56);
    cpu.fr[0] = mem.read_f32(base + 56).unwrap();
    cpu.r[3] = base.wrapping_add(68);
    cpu.fr[0] = cpu.fr[0] + a;
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.fr[2] = mem.read_f32(0x8C1ACA54).unwrap();
    cpu.fr[1] = cpu.fr[1] + cpu.fr[2]; // bra delay slot
    cpu.r[0] = 0x8C1ACA54;
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved13 = cpu.r[13];
    let saved14 = cpu.r[14];

    cpu.fr[0] = 0.0; // fldi0
    cpu.fr[1] = cpu.fr[0]; // fmov fr0,fr1

    let r5 = cpu.r[5];
    if r5 == 1 {
        cpu.r[3] = mem.read_u32(0x8C1ACA4C).unwrap(); // 0x8c1e5e00
        arm(cpu, mem);
    } else if r5 == 2 {
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] == 0 {
            cpu.r[3] = mem.read_u32(0x8C1ACA58).unwrap(); // 0x8c1e5ee8
            arm(cpu, mem);
        } else {
            cpu.r[0] = cpu.r[4];
            cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
            if cpu.r[4] == 1 {
                cpu.r[3] = mem.read_u32(0x8C1ACA5C).unwrap(); // 0x8c1e5fd0
                arm(cpu, mem);
            }
        }
    } else if r5 == 4 {
        cpu.set_t(cpu.r[4] == 0);
        let pool = match cpu.r[4] {
            0 => Some(0x8C1ACA60u32), // 0x8c1e60b8
            1 => Some(0x8C1ACA64),    // 0x8c1e61a0
            2 => Some(0x8C1ACA68),    // 0x8c1e6288
            3 => Some(0x8C1ACA6C),    // 0x8c1e6370
            _ => None,
        };
        if let Some(p) = pool {
            cpu.r[3] = mem.read_u32(p).unwrap();
            arm(cpu, mem);
        }
    }

    // 0x8c1ac9f6
    cpu.r[4] = cpu.r[4] << 8; // shll8 (T untouched)
    cpu.r[14] = 0;
    cpu.r[13] = cpu.r[14];
    cpu.set_t(cpu.r[4] & 0x8000_0000 != 0); // shll: T from bit shifted out
    cpu.r[4] = cpu.r[4] << 1;
    let r4s = cpu.r[4];
    cpu.fr[5] = mem.read_f32(0x8C1ACA70).unwrap();
    let r6 = r4s.wrapping_add(0x8C2B7A30);
    let r7 = 0x8C2B7A30u32.wrapping_add(4).wrapping_add(r4s);
    cpu.fr[3] = mem.read_f32(0x8C1ACA78).unwrap();
    cpu.r[5] = 0x8C1E5C00;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[3] = 0x8C1E5C04;
    cpu.r[0] = 64;

    let fr5 = cpu.fr[5];
    let fr3 = cpu.fr[3];
    loop {
        cpu.r[1] = cpu.r[3].wrapping_add(cpu.r[13]);
        cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap();
        cpu.r[4] = r6.wrapping_add(cpu.r[13]);
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.fr[2] = -cpu.fr[2]; // fneg
        cpu.fr[2] = cpu.fr[2] * fr5;
        cpu.fr[2] = cpu.fr[2] + cpu.fr[0];
        cpu.r[1] = cpu.r[13];
        mem.write_f32(cpu.r[4], cpu.fr[2]).unwrap();
        cpu.r[1] = cpu.r[13].wrapping_add(cpu.r[5]);
        cpu.fr[4] = mem.read_f32(cpu.r[1]).unwrap();
        cpu.fr[2] = cpu.fr[1];
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32)); // cmp/gt r14,r0
        cpu.r[1] = r7.wrapping_add(cpu.r[13]);
        cpu.fr[4] = cpu.fr[4] * fr3;
        cpu.fr[2] = cpu.fr[2] + cpu.fr[4];
        mem.write_f32(cpu.r[1], cpu.fr[2]).unwrap();
        cpu.r[13] = cpu.r[13].wrapping_add(8); // bt.s delay slot, always
        if !((cpu.r[0] as i32) > (cpu.r[14] as i32)) {
            break;
        }
    }

    cpu.r[13] = saved13; // epilogue pops
    cpu.r[14] = saved14;
}
