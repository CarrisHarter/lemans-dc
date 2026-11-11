//! 0x8c15039c — init a 44-byte header at r4: two global pointer stores to
//! `*r4` (the second wins), then 4 words copied twice from the struct at
//! 0x8c1d3f78 into r4+4..r4+32, then two zero floats at r4+36/r4+40.
//!
//! Original stores `@r4` twice (`mov.l r0,@r4` at 0x8c1503a4 and 0x8c1503ae);
//! both writes are recorded. r14 is pushed and restored by the rts delay slot,
//! so it is net unchanged. `fldi0 fr0` + `fmov fr0,@r3`/`@r1` write 0.0f bits.
//! r4 is reloaded from the source struct mid-function, so the final r4 is
//! `mem[0x8c1d3f78+12]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let g1 = mem.read_u32(0x8c1503e4).unwrap_or(0); // 0x8c1ef3a8
    let g2 = mem.read_u32(0x8c1503e8).unwrap_or(0); // 0x8c1ef39c
    let r7 = mem.read_u32(0x8c1503ec).unwrap_or(0); // 0x8c1d3f78
    let _ = mem.write_u32(r4, g1);
    let _ = mem.write_u32(r4, g2);
    let a0 = mem.read_u32(r7).unwrap_or(0);
    let a1 = mem.read_u32(r7 + 4).unwrap_or(0);
    let a2 = mem.read_u32(r7 + 8).unwrap_or(0);
    let a3 = mem.read_u32(r7 + 12).unwrap_or(0);
    let _ = mem.write_u32(r4 + 4, a0);
    let _ = mem.write_u32(r4 + 8, a1);
    let _ = mem.write_u32(r4 + 12, a2);
    let _ = mem.write_u32(r4 + 16, a3);
    let _ = mem.write_u32(r4 + 20, a0);
    let _ = mem.write_u32(r4 + 24, a1);
    let _ = mem.write_u32(r4 + 28, a2);
    let _ = mem.write_u32(r4 + 32, a3);
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_f32(r4 + 36, cpu.fr[0]); // fmov fr0,@r3
    let _ = mem.write_f32(r4 + 40, cpu.fr[0]); // fmov fr0,@r1
    cpu.r[0] = r4;
    cpu.r[1] = r4 + 40;
    cpu.r[3] = r4 + 36;
    cpu.r[4] = a3;
    cpu.r[5] = a2;
    cpu.r[6] = r4 + 20;
    cpu.r[7] = r7;
    // r14 restored by the rts delay slot pop: net unchanged
}
