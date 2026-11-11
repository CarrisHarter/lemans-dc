//! 0x8c070aa4 — in-place tolower over the string at r4 (stops at NUL).
//!
//! Original side effects: r5 walks the string, r6 = last byte (extu.b),
//! r7 = -32, r1 = 122, r3 = 97, r0 = r4 (rts delay slot), T = (last byte == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = cpu.r[4];
    cpu.r[7] = (-32i32) as u32;
    cpu.r[1] = 122;
    cpu.r[3] = 97;
    loop {
        let b = mem.read_u8(cpu.r[5]).unwrap_or(0);
        cpu.r[0] = b as u32;
        cpu.r[6] = b as u32; // extu.b r0,r6
        cpu.set_t(cpu.r[6] >= cpu.r[3]); // cmp/ge r3,r6
        if cpu.r[6] >= cpu.r[3] {
            cpu.set_t(cpu.r[1] >= cpu.r[6]); // cmp/ge r6,r1
            if cpu.r[1] >= cpu.r[6] {
                cpu.r[0] = cpu.r[6].wrapping_add(cpu.r[7]); // add r7,r0 (-32)
                let _ = mem.write_u8(cpu.r[5], cpu.r[0] as u8);
            }
        }
        cpu.set_t(cpu.r[6] == 0); // tst r6,r6 (0x8c070abe)
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bf.s delay slot
        if cpu.r[6] != 0 {
            continue;
        }
        break;
    }
    cpu.r[0] = cpu.r[4]; // rts delay slot
}
