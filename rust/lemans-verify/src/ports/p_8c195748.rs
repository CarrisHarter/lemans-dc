//! 0x8c195748 — copy one of four 4-float vectors into *[r15].
//!
//! `mov.l @r15,r1` loads the destination. The selector is r5:
//! - r5 == 0            -> vector at 0x8c1d0f28 (pool 0x8c195870)
//! - r5 == 4 and the 0x8c23dab0->+72->+36 chain == 10:
//!     byte[r4+41] != 0: [r4+24] == 4 -> vector at r4+52, else 0x8c1d0f18
//!     byte[r4+41] == 0:               -> 0x8c1d0f98
//! - otherwise: [r4+24] == r5 -> vector at r4+52, else 0x8c1d0f18
//!
//! Every arm ends in the same 4-float copy whose last store sits in the
//! `rts` delay slot, after `add #12,r1`. Final registers: r1 = dest+12,
//! r4 = base+12, r0 = dest+8, r3 = dest+4, fr0 = [base+12]; T is whatever
//! the last `cmp`/`tst` on the path left.

use sh4_oracle::{Cpu, Memory};

fn copy4(cpu: &mut Cpu, mem: &mut Memory, base: u32) {
    let dst = cpu.r[1];
    cpu.r[4] = base;
    cpu.fr[0] = mem.read_f32(base).unwrap();
    cpu.r[3] = base;
    cpu.r[0] = base;
    mem.write_f32(dst, cpu.fr[0]).unwrap();
    cpu.r[3] = base + 4;
    cpu.fr[0] = mem.read_f32(base + 4).unwrap();
    cpu.r[4] = base + 12;
    cpu.r[0] = base + 8;
    cpu.r[3] = dst + 4;
    mem.write_f32(dst + 4, cpu.fr[0]).unwrap();
    cpu.fr[0] = mem.read_f32(base + 8).unwrap();
    cpu.r[0] = dst + 8;
    mem.write_f32(dst + 8, cpu.fr[0]).unwrap();
    cpu.r[1] = dst + 12;
    cpu.fr[0] = mem.read_f32(base + 12).unwrap();
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap(); // rts delay slot
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r1
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        let base = mem.read_u32(0x8C195870).unwrap(); // -> 0x8c1d0f28
        copy4(cpu, mem, base);
        return;
    }
    if cpu.r[5] == 4 {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0 (not taken)
        let mut p = mem.read_u32(0x8C23DAB0).unwrap();
        p = p.wrapping_add(72);
        p = mem.read_u32(p).unwrap();
        cpu.r[0] = mem.read_u32(p + 36).unwrap();
        cpu.set_t(cpu.r[0] == 10); // cmp/eq #10,r0
        if cpu.r[0] == 10 {
            cpu.r[0] = cpu.r[4].wrapping_add(41);
            let b = mem.read_u8(cpu.r[0]).unwrap() as u32; // mov.b + extu.b x2
            cpu.r[0] = b;
            cpu.set_t(b == 0); // tst r0,r0
            if b != 0 {
                cpu.r[0] = mem.read_u32(cpu.r[4] + 24).unwrap();
                cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0
                if cpu.r[0] == 4 {
                    copy4(cpu, mem, cpu.r[4].wrapping_add(52));
                } else {
                    let base = mem.read_u32(0x8C195878).unwrap(); // -> 0x8c1d0f18
                    copy4(cpu, mem, base);
                }
            } else {
                let base = mem.read_u32(0x8C19587C).unwrap(); // -> 0x8c1d0f98
                copy4(cpu, mem, base);
            }
            return;
        }
        // chain != 10 -> fall through to the r5 compare below
    } else {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0 (taken: T=0)
    }
    // 0x8c195818
    cpu.r[0] = mem.read_u32(cpu.r[4] + 24).unwrap();
    cpu.set_t(cpu.r[0] == cpu.r[5]); // cmp/eq r0,r5 -> T = (r5 == r0)
    if cpu.r[0] == cpu.r[5] {
        copy4(cpu, mem, cpu.r[4].wrapping_add(52));
    } else {
        let base = mem.read_u32(0x8C195878).unwrap(); // -> 0x8c1d0f18
        copy4(cpu, mem, base);
    }
}
