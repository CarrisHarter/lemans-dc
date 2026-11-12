//! 0x8c1779fe - map a small negative selector to a 32-bit constant.
//!
//! A null `r5` is an error: 0x8c174564(101, str@0x8c1e224c) runs and the
//! function returns 0. Otherwise `r4` selects the constant written to `*r5`:
//! `0 -> 0xac44`, `-1..-8 -> 0x5622, 0x2b11, 0x1394, 0x0808, 0x04e5, 0x0272,
//! 0x0139, 0x009c`, and anything else is an error through 0x8c174564 with
//! `str@0x8c1e2278` and code 0x323. The success paths return 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t((cpu.r[5] & cpu.r[5]) == 0); // tst r5,r5
    let mut result;
    if cpu.t() {
        cpu.r[5] = mem.read_u32(0x8c177aa0).unwrap_or(0x8c1e224c); // mov.l 0x8c177aa0,r5
        cpu.r[4] = 101; // bra delay: mov #101,r4
        error(cpu, mem);
        result = 0; // rts delay at 0x8c177a7a
    } else {
        cpu.set_t((cpu.r[4] & cpu.r[4]) == 0); // tst r4,r4
        if cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c177aa4).unwrap_or(0xac44); // mov.l 0x8c177aa4,r2
            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bra delay: mov.l r2,@r5
            result = 1;
        } else {
            cpu.r[0] = cpu.r[4]; // mov r4,r0
            cpu.set_t(cpu.r[0] == 0xffff_ffff); // cmp/eq #-1,r0
            if cpu.t() {
                cpu.r[3] = mem.read_u16(0x8c177a86).unwrap_or(0x5622) as i16 as i32 as u32; // mov.w
                let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bra delay: mov.l r3,@r5
                result = 1;
            } else {
                cpu.set_t(cpu.r[0] == 0xffff_fffe); // cmp/eq #-2,r0
                if cpu.t() {
                    cpu.r[2] = mem.read_u16(0x8c177a88).unwrap_or(0x2b11) as i16 as i32 as u32; // mov.w
                    let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bra delay: mov.l r2,@r5
                    result = 1;
                } else {
                    cpu.r[0] = cpu.r[4]; // mov r4,r0
                    cpu.set_t(cpu.r[0] == 0xffff_fffd); // cmp/eq #-3,r0
                    if cpu.t() {
                        cpu.r[3] = mem.read_u16(0x8c177a8a).unwrap_or(0x1394) as i16 as i32 as u32; // mov.w
                        let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bra delay: mov.l r3,@r5
                        result = 1;
                    } else {
                        cpu.set_t(cpu.r[0] == 0xffff_fffc); // cmp/eq #-4,r0
                        if cpu.t() {
                            cpu.r[2] = mem.read_u16(0x8c177a8c).unwrap_or(0x0808) as i16 as i32 as u32; // mov.w
                            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bra delay: mov.l r2,@r5
                            result = 1;
                        } else {
                            cpu.r[0] = cpu.r[4]; // mov r4,r0
                            cpu.set_t(cpu.r[0] == 0xffff_fffb); // cmp/eq #-5,r0
                            if cpu.t() {
                                cpu.r[3] = mem.read_u16(0x8c177a8e).unwrap_or(0x04e5) as i16 as i32 as u32; // mov.w
                                let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bra delay: mov.l r3,@r5
                                result = 1;
                            } else {
                                cpu.set_t(cpu.r[0] == 0xffff_fffa); // cmp/eq #-6,r0
                                if cpu.t() {
                                    cpu.r[2] = mem.read_u16(0x8c177a90).unwrap_or(0x0272) as i16 as i32 as u32; // mov.w
                                    let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bra delay: mov.l r2,@r5
                                    result = 1;
                                } else {
                                    cpu.r[0] = cpu.r[4]; // mov r4,r0
                                    cpu.set_t(cpu.r[0] == 0xffff_fff9); // cmp/eq #-7,r0
                                    if cpu.t() {
                                        cpu.r[3] = mem.read_u16(0x8c177a92).unwrap_or(0x0139) as i16 as i32 as u32; // mov.w
                                        let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bra delay: mov.l r3,@r5
                                        result = 1;
                                    } else {
                                        cpu.set_t(cpu.r[0] == 0xffff_fff8); // cmp/eq #-8,r0
                                        if cpu.t() {
                                            cpu.r[2] = mem.read_u16(0x8c177a94).unwrap_or(0x009c) as i16 as i32 as u32; // mov.w
                                            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bra delay: mov.l r2,@r5
                                            result = 1;
                                        } else {
                                            cpu.r[4] = mem.read_u16(0x8c177a96).unwrap_or(0x0323) as i16 as i32 as u32; // mov.w
                                            cpu.r[5] = mem.read_u32(0x8c177aa8).unwrap_or(0x8c1e2278); // mov.l 0x8c177aa8,r5
                                            error(cpu, mem);
                                            result = 0; // rts delay at 0x8c177a7a
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cpu.r[0] = result;
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn error(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c177a9c).unwrap_or(0x8c174564); // mov.l 0x8c177a9c,r2
    cpu.pr = 0x8c177a76; // jsr delay is nop
    crate::registry::p_8c174564::run(cpu, mem);
}
