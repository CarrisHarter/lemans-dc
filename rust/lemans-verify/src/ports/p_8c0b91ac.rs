//! 0x8c0b91ac — split a path into a node chain and store a payload on the tail.
//!
//! `r5` is the path, `r4` the list object, `r7` and `r6` the two words written
//! to the final node's `+4` and `+8`. A path whose last byte is `'/'` returns
//! after `strlen` and touches nothing else. Otherwise each component is looked
//! up at `[node+12]` among the siblings at `[node]`, descending through
//! `[node+8]`. A miss allocates `strlen+13` bytes, links the node, and copies
//! the name to `node+12`. An empty component stops the walk and stores the
//! payload. `r8`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[11] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[10] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[9] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(128);
    cpu.r[15] = cpu.r[15].wrapping_sub(128);
    cpu.r[15] = cpu.r[15].wrapping_sub(128);
    cpu.r[15] = cpu.r[15].wrapping_sub(128);
    cpu.r[8] = cpu.r[7];

    cpu.r[0] = rd(mem, 0x8c0b92dc); // strlen
    cpu.pr = 0x8c0b91d2;
    cpu.r[4] = cpu.r[11]; // jsr delay
    crate::registry::p_8c0112f4::run(cpu, mem);
    cpu.r[1] = cpu.r[11].wrapping_sub(1);
    cpu.r[0] = sext8(mem, cpu.r[0].wrapping_add(cpu.r[1])); // last byte
    cpu.set_t(cpu.r[0] == 47); // cmp/eq #47
    if !cpu.t() {
        cpu.r[4] = rd(mem, 0x8c0b92e0); // 0x100
        cpu.r[5] = cpu.r[11];
        cpu.r[0] = rd(mem, 0x8c0b92e4); // strcpy
        cpu.pr = 0x8c0b91e8;
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[15]); // jsr delay: buffer at sp+0x100
        crate::registry::p_8c017f48::run(cpu, mem);
        cpu.r[11] = rd(mem, cpu.r[9].wrapping_add(4));
        cpu.r[14] = 0;
        cpu.r[12] = (-1i32) as u32; // bra delay
        loop {
            cpu.r[4] = rd(mem, 0x8c0b92e0); // 0x100
            cpu.r[0] = rd(mem, 0x8c0b92dc); // strlen
            cpu.pr = 0x8c0b9272;
            cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[15]);
            crate::registry::p_8c0112f4::run(cpu, mem);
            cpu.r[1] = rd(mem, 0x8c0b92e0);
            cpu.r[3] = cpu.r[0];
            cpu.r[13] = 0;
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[15]); // bra delay: r1 = sp+0x100
            loop {
                cpu.set_t((cpu.r[3] as i32) > (cpu.r[13] as i32)); // cmp/gt r13, r3
                if !cpu.t() {
                    break;
                }
                cpu.r[0] = cpu.r[1];
                cpu.r[0] = sext8(mem, cpu.r[0].wrapping_add(cpu.r[13]));
                cpu.set_t(cpu.r[0] == 47);
                if cpu.t() {
                    break;
                }
                cpu.r[13] = cpu.r[13].wrapping_add(1);
            }
            cpu.r[5] = rd(mem, 0x8c0b92e0); // 0x100
            cpu.r[6] = cpu.r[13];
            cpu.r[0] = rd(mem, 0x8c0b92ec); // strncpy
            cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[15]);
            cpu.pr = 0x8c0b9298;
            cpu.r[4] = cpu.r[15]; // jsr delay
            crate::registry::p_8c011344::run(cpu, mem);
            cpu.r[0] = cpu.r[15];
            cpu.r[1] = 0;
            let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[13]), 0);
            cpu.r[0] = rd(mem, 0x8c0b92e0);
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[15]);
            cpu.r[0] = sext8(mem, cpu.r[0].wrapping_add(cpu.r[13]));
            cpu.set_t(cpu.r[0] == 47);
            if cpu.t() {
                cpu.r[13] = cpu.r[13].wrapping_add(1);
            }
            cpu.r[4] = rd(mem, 0x8c0b92e0);
            cpu.r[0] = rd(mem, 0x8c0b92e4); // strcpy
            cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[15]);
            cpu.r[13] = cpu.r[4].wrapping_add(cpu.r[13]);
            cpu.pr = 0x8c0b92b8;
            cpu.r[5] = cpu.r[13]; // jsr delay
            crate::registry::p_8c017f48::run(cpu, mem);
            cpu.r[0] = sext8(mem, cpu.r[15]);
            cpu.set_t(cpu.r[0] == 0); // tst
            if cpu.t() {
                break;
            }
            cpu.set_t(cpu.r[11] == 0);
            if !cpu.t() {
                cpu.r[1] = cpu.r[15];
                cpu.r[0] = 0;
                loop {
                    cpu.r[6] = cpu.r[11];
                    cpu.r[5] = cpu.r[1];
                    cpu.r[6] = cpu.r[6].wrapping_add(12); // bra delay
                    let mut at_body = true;
                    let matched = loop {
                        if !at_body {
                            cpu.r[3] = cpu.r[4] & 0xff; // extu.b r4, r3
                            cpu.set_t(cpu.r[3] == 0);
                            if cpu.t() {
                                cpu.r[3] = cpu.r[0]; // bra delay
                                break true;
                            }
                        }
                        at_body = false;
                        cpu.r[4] = sext8(mem, cpu.r[5]);
                        cpu.r[7] = sext8(mem, cpu.r[6]);
                        cpu.r[4] &= 0xff;
                        cpu.r[3] = cpu.r[4] & 0xff;
                        cpu.r[7] &= 0xff;
                        cpu.r[7] &= 0xff;
                        cpu.r[5] = cpu.r[5].wrapping_add(1);
                        cpu.set_t(cpu.r[3] == cpu.r[7]);
                        cpu.r[6] = cpu.r[6].wrapping_add(1); // bt.s delay, always
                        if !cpu.t() {
                            cpu.r[3] = cpu.r[3].wrapping_sub(cpu.r[7]);
                            break false;
                        }
                    };
                    cpu.set_t(cpu.r[3] == 0);
                    if matched || cpu.t() {
                        break;
                    }
                    cpu.r[11] = rd(mem, cpu.r[11]);
                    cpu.set_t(cpu.r[11] == 0);
                    if cpu.t() {
                        break;
                    }
                }
            }
            cpu.set_t(cpu.r[11] == 0);
            if cpu.t() {
                cpu.r[0] = rd(mem, 0x8c0b92dc); // strlen
                cpu.pr = 0x8c0b9238;
                cpu.r[4] = cpu.r[15];
                crate::registry::p_8c0112f4::run(cpu, mem);
                cpu.r[0] = cpu.r[0].wrapping_add(13);
                cpu.r[4] = cpu.r[0];
                cpu.r[0] = rd(mem, 0x8c0b92e8); // malloc
                cpu.pr = 0x8c0b9242;
                crate::registry::p_8c15a0fc::run(cpu, mem);
                cpu.r[11] = cpu.r[0];
                let _ = mem.write_u32(cpu.r[11].wrapping_add(4), cpu.r[12]);
                cpu.r[0] = 0;
                let _ = mem.write_u32(cpu.r[11].wrapping_add(8), cpu.r[0]);
                cpu.r[4] = cpu.r[11];
                cpu.r[0] = rd(mem, 0x8c0b92e4); // strcpy
                cpu.r[5] = cpu.r[15];
                cpu.pr = 0x8c0b9254;
                cpu.r[4] = cpu.r[4].wrapping_add(12); // jsr delay
                crate::registry::p_8c017f48::run(cpu, mem);
                cpu.set_t(cpu.r[14] == 0);
                if cpu.t() {
                    cpu.r[0] = rd(mem, cpu.r[9].wrapping_add(4));
                    let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                    let _ = mem.write_u32(cpu.r[9].wrapping_add(4), cpu.r[11]);
                } else {
                    cpu.r[0] = rd(mem, cpu.r[14].wrapping_add(8));
                    let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[11]); // bra delay
                }
            }
            cpu.r[14] = cpu.r[11];
            cpu.r[11] = rd(mem, cpu.r[11].wrapping_add(8));
        }
        let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[8]);
        let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[10]);
    }

    cpu.r[2] = rd(mem, 0x8c0b92f0); // 0x200
    cpu.r[15] = cpu.r[15].wrapping_add(cpu.r[2]);
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
