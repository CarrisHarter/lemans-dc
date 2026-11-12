//! 0x8c222c3a — clear a 0x148-byte record at `0x8c2c02b8 + 0x148*index`.
//!
//! ```text
//! c3a mov.w 0x8c222cde,r1   r1 = sext16(*(u16*)0x8c222cde) = 0x148
//! c3c mov.l 0x8c222cf8,r3   r3 = *(u32*)0x8c222cf8         = 0x8c2c02b8
//! c3e mul.l r1,r4           MACH:MACL = sext32(r1) * sext32(r4)
//! c40 sts macl,r1           r1 = low half of the product
//! c42 add r3,r1             r1 = record address
//! c44 mov.b @(2,r1),r0      r0 = sext8(record[2])
//! c46 tst r0,r0             T = (record[2] == 0)
//! c48 bf 0x8c222c50         taken when record[2] != 0
//! c4a mov.w 0x8c222ce0,r0   r0 = sext16(0xff01) = 0xffffff01
//! c4c rts
//! c4e nop
//! c50 mov #0,r4             r4 = 0
//! c52 mov r4,r0             r0 = 0
//! c54 mov.b r0,@(9,r1)      record[9] = 0
//! c56 mov.b r0,@(8,r1)      record[8] = 0
//! c58 mov.b r0,@(10,r1)     record[10] = 0
//! c5a mov.b r0,@(11,r1)     record[11] = 0
//! c5c mov.b r0,@(12,r1)     record[12] = 0
//! c5e mov.l r0,@(4,r1)      *(u32*)(record+4) = 0
//! c60 rts
//! c62 nop
//! ```
//!
//! `mul.l` leaves the *signed* 64-bit product in MACH:MACL and the verifier
//! compares both, so the high half is written too even though only MACL is
//! read here. T is set by `tst`: cleared on the clearing path (the `bf` is
//! taken) and left set on the 0xff01 early return.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u16(0x8c222cde).unwrap_or(0) as i16 as i32 as u32; // mov.w 0x8c222cde,r1
    cpu.r[3] = mem.read_u32(0x8c222cf8).unwrap_or(0); // mov.l 0x8c222cf8,r3
    let p = (cpu.r[1] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r1,r4
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    cpu.r[1] = cpu.macl; // sts macl,r1
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // add r3,r1
    cpu.r[0] = mem.read_u8(cpu.r[1].wrapping_add(2)).unwrap_or(0) as i8 as i32 as u32; // mov.b @(2,r1),r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        // `bf` not taken: record[2] == 0, return -255 as a sign-extended word.
        cpu.r[0] = mem.read_u16(0x8c222ce0).unwrap_or(0) as i16 as i32 as u32; // mov.w 0x8c222ce0,r0
        return;
    }
    let base = cpu.r[1];
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let _ = mem.write_u8(base.wrapping_add(9), cpu.r[0] as u8); // mov.b r0,@(9,r1)
    let _ = mem.write_u8(base.wrapping_add(8), cpu.r[0] as u8); // mov.b r0,@(8,r1)
    let _ = mem.write_u8(base.wrapping_add(10), cpu.r[0] as u8); // mov.b r0,@(10,r1)
    let _ = mem.write_u8(base.wrapping_add(11), cpu.r[0] as u8); // mov.b r0,@(11,r1)
    let _ = mem.write_u8(base.wrapping_add(12), cpu.r[0] as u8); // mov.b r0,@(12,r1)
    let _ = mem.write_u32(base.wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r1)
}
