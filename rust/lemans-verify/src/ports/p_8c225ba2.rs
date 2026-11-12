//! 0x8c225ba2 - unpack eight bytes at r5 into a record at r4.
//!
//! Byte 0 goes through 0x8c225ae4 and is multiplied by 100. Byte 1 is added
//! and the sum is stored as a halfword. Bytes 2..6 are stored as the split
//! result. Byte 7's low 3 bits index the table at 0x8c23cca0. `mulu.w`
//! writes macl only.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[4] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[4] &= 0xff; // bsr delay
    cpu.pr = 0x8c225bb4;
    crate::registry::p_8c225ae4::run(cpu, mem);
    cpu.r[2] = 100;
    cpu.macl = (cpu.r[0] & 0xffff).wrapping_mul(cpu.r[2] & 0xffff); // mulu.w r2,r0
    cpu.r[0] = cpu.macl;
    cpu.r[8] = cpu.r[0];
    cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(1)).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[4] = cpu.r[0] & 0xff; // bsr delay
    cpu.pr = 0x8c225bc2;
    crate::registry::p_8c225ae4::run(cpu, mem);
    cpu.r[8] = cpu.r[8].wrapping_add(cpu.r[0]);
    let _ = mem.write_u16(cpu.r[13], cpu.r[8] as u16);

    for (src, dst, ret) in [(2, 2, 0x8c225bcc), (3, 3, 0x8c225bd4), (4, 4, 0x8c225bdc), (5, 5, 0x8c225be4), (6, 6, 0x8c225bec)] {
        cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(src)).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[4] = cpu.r[0] & 0xff;
        cpu.pr = ret;
        crate::registry::p_8c225ae4::run(cpu, mem);
        let _ = mem.write_u8(cpu.r[13].wrapping_add(dst), cpu.r[0] as u8);
    }

    cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(7)).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[1] = mem.read_u32(0x8c225c10).unwrap_or(0x8c23cca0);
    cpu.r[0] &= 0xff;
    cpu.r[0] &= 7;
    cpu.r[0] = mem.read_u8(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0) as i8 as i32 as u32;
    let _ = mem.write_u8(cpu.r[13].wrapping_add(7), cpu.r[0] as u8);

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
