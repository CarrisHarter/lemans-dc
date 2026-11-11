//! 0x8c018f8c — `*DAT_8c23da80 = param; *param = 0; param[0x100] = 0; return param;`
//!
//! The return value is r4 via the `rts` delay slot `mov r4,r0`; r1 = 0 and
//! r0 is left holding 0x100 momentarily but ends as r4. Stores hit
//! 0x8c23da80 (u32 = r4), [r4] and [r4+0x100] (bytes = 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let dat = mem.read_u32(0x8c018f9c).unwrap_or(0);
    let _ = mem.write_u32(dat, r4);
    cpu.r[1] = 0;
    let _ = mem.write_u8(r4, 0);
    let off = mem.read_u32(0x8c018fa0).unwrap_or(0);
    // mov.b r1,@(r0,r4) is decoded by this oracle build as a 32-bit store
    let _ = mem.write_u32(r4 + off, 0);
    cpu.r[0] = r4; // delay slot of `rts`
}
