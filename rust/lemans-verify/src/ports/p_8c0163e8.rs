//! 0x8c0163e8 — copy 16-byte records from r1 to r0 while no word has a zero
//! byte (cmp/str against 0), then copy the final word byte-by-byte up to and
//! including its first zero byte. Returns the original r0.
//!
//! Original stack behaviour: pushes r5,r6 but pops r6,r5,r4,r3 at exit, so
//! r3/r4/r5/r6 end as the (zeroed) synthetic-stack values and r15 ends 8 above
//! entry. r0 = original dst (mov r3,r0 before the r3 pop). r1 ends past the
//! terminating zero byte.

use sh4_oracle::{Cpu, Memory};

fn has_zero_byte(v: u32) -> bool {
    v & 0xff == 0 || v & 0xff00 == 0 || v & 0xff0000 == 0 || v & 0xff000000 == 0
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let entry_r15 = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // push r5 ; push r6
    cpu.r[3] = cpu.r[0]; // mov r0,r3
    loop {
        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1+,r4
        cpu.r[1] = cpu.r[1].wrapping_add(4);
        cpu.r[6] = 0;
        let z = has_zero_byte(cpu.r[4]);
        cpu.set_t(z); // cmp/str r4,r6
        if z {
            break;
        }
        cpu.r[5] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.r[1] = cpu.r[1].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
        let z = has_zero_byte(cpu.r[5]);
        cpu.set_t(z);
        if z {
            cpu.r[0] = cpu.r[0].wrapping_add(4); // bra delay
            break;
        }
        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.r[1] = cpu.r[1].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0] + 4, cpu.r[5]);
        let z = has_zero_byte(cpu.r[4]);
        cpu.set_t(z);
        if z {
            cpu.r[0] = cpu.r[0].wrapping_add(8); // bra delay
            break;
        }
        cpu.r[5] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.r[1] = cpu.r[1].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0] + 8, cpu.r[4]);
        let z = has_zero_byte(cpu.r[5]);
        cpu.set_t(z);
        if z {
            cpu.r[0] = cpu.r[0].wrapping_add(12); // falls into tail
            break;
        }
        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.r[1] = cpu.r[1].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0] + 12, cpu.r[5]);
        cpu.r[0] = cpu.r[0].wrapping_add(16); // bra delay
    }
    // byte tail (0x8c016420)
    cpu.r[1] = cpu.r[1].wrapping_sub(4); // add #-4,r1
    cpu.r[6] = cpu.r[0];
    for off in 0..4u32 {
        let b = mem.read_u8(cpu.r[1]).unwrap_or(0); // mov.b @r1+,r0
        cpu.r[1] = cpu.r[1].wrapping_add(1);
        cpu.r[0] = b as u32;
        let _ = mem.write_u8(cpu.r[6] + off, b);
        if off < 3 {
            cpu.set_t(b == 0); // cmp/eq #0,r0 (absent on the 4th byte)
            if b == 0 {
                break;
            }
        }
    }
    // exit (0x8c016440): pop r6,r5,r4 ; r0=r3 ; rts ; delay pop r3
    cpu.r[6] = 0;
    cpu.r[5] = 0;
    cpu.r[4] = 0;
    cpu.r[0] = cpu.r[3];
    cpu.r[3] = 0;
    cpu.r[15] = entry_r15.wrapping_add(8); // 2 pushes, 4 pops
}
