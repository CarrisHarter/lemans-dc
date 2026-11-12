//! 0x8c216e64 - unpack a record when its first byte is at least 34.
//!
//! Copies two 4-byte fields, one flag byte and a counted string, then stores
//! the string's checksum at `dst+9`. A first byte below 34 returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_r14 = cpu.r[14];
    let saved_r13 = cpu.r[13];
    let saved_r12 = cpu.r[12];
    let saved_pr = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[2] = 34;
    cpu.r[13] = cpu.r[4];
    cpu.r[3] = mem.read_u8(cpu.r[13]).unwrap_or(0) as u32;
    cpu.set_t((cpu.r[3] as i32) >= (cpu.r[2] as i32)); // cmp/ge r2,r3
    cpu.r[14] = cpu.r[5]; // bt.s delay
    if !cpu.t() {
        cpu.r[0] = 0; // bra delay
    } else {
        cpu.r[4] = cpu.r[13].wrapping_add(2); // bsr delay
        cpu.r[5] = cpu.r[14];
        cpu.r[6] = 4;
        cpu.pr = 0x8c216e88;
        crate::registry::p_8c216d58::run(cpu, mem);
        cpu.r[4] = cpu.r[13].wrapping_add(10); // bsr delay
        cpu.r[5] = cpu.r[14].wrapping_add(4);
        cpu.r[6] = 4;
        cpu.pr = 0x8c216e94;
        crate::registry::p_8c216d58::run(cpu, mem);
        cpu.r[0] = 25;
        cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0) as i8 as i32 as u32;
        let _ = mem.write_u8(cpu.r[14].wrapping_add(8), cpu.r[0] as u8);
        cpu.r[0] = 32;
        cpu.r[12] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[0] &= 0xff;
        cpu.set_t(cpu.r[0] & 2 == 0); // tst #2,r0
        cpu.r[12] &= 0xff; // bf.s delay: extu.b r12,r12
        if cpu.t() {
            cpu.r[12] = cpu.r[12].wrapping_sub(2);
        }
        cpu.r[4] = cpu.r[13].wrapping_add(33); // bsr delay
        cpu.r[5] = cpu.r[14].wrapping_add(10);
        cpu.r[6] = cpu.r[12];
        cpu.pr = 0x8c216eb6;
        crate::registry::p_8c216d58::run(cpu, mem);
        cpu.r[3] = cpu.r[14].wrapping_add(10);
        cpu.r[4] = 0;
        cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[3]);
        let _ = mem.write_u8(cpu.r[12], 0);
        cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[0] &= 0xff;
        cpu.set_t(cpu.r[0] & 2 == 0); // tst #2,r0
        if !cpu.t() {
            cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(10)).unwrap_or(0) as i8 as i32 as u32;
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            cpu.r[5] = 46; // bf.s delay
            if cpu.t() {
                cpu.r[0] = cpu.r[5];
                let _ = mem.write_u8(cpu.r[14].wrapping_add(10), cpu.r[0] as u8); // bra delay
            } else {
                cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
                if cpu.t() {
                    cpu.r[0] = cpu.r[5];
                    let _ = mem.write_u8(cpu.r[14].wrapping_add(10), cpu.r[0] as u8);
                    let _ = mem.write_u8(cpu.r[14].wrapping_add(11), cpu.r[0] as u8);
                    cpu.r[0] = cpu.r[4];
                    let _ = mem.write_u8(cpu.r[14].wrapping_add(12), cpu.r[0] as u8);
                }
            }
        }
        cpu.r[4] = cpu.r[14].wrapping_add(10); // bsr delay
        cpu.pr = 0x8c216eec;
        crate::registry::p_8c216e3e::run(cpu, mem);
        let _ = mem.write_u8(cpu.r[14].wrapping_add(9), cpu.r[0] as u8);
        cpu.r[0] = mem.read_u8(cpu.r[13]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[0] &= 0xff;
    }
    cpu.pr = saved_pr;
    cpu.r[12] = saved_r12;
    cpu.r[13] = saved_r13;
    cpu.r[14] = saved_r14;
    cpu.r[15] = cpu.r[15].wrapping_add(16);
}
