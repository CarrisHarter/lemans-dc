//! 0x8c16c9b0 — initialise a 104-byte object at r4.
//!
//! Stores: [r4]=pool(0x8c1effa4), [r4+4]=0, byte [r4+8]=0, then the 3-word
//! table at pool(0x8c1d3f68) copied to r4+12/24/40/52/76 (each w0,w1,w2) and
//! r4+16/28/44/56/80, the literal-pool f32 at 0x8c16ca68 (loaded into fr2)
//! stored at r4+64/68/72, 1.0f (fr1) at r4+88 and r4+100, byte 0 at r4+92,
//! 0.0f (fr0) at r4+96. r8..r14 are pushed/popped, so they come back; the
//! leaked registers are r0=r4, r1=r4+100, r3=r4+96, r5=0, r6=r4+76, r7=r4+72,
//! fr0=0.0, fr1=1.0, fr2=[0x8c16ca68] as f32 bits.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let head = mem.read_u32(0x8c16ca60).unwrap(); // 0x8c1effa4
    let table = mem.read_u32(0x8c16ca64).unwrap(); // 0x8c1d3f68
    let w0 = mem.read_u32(table).unwrap();
    let w1 = mem.read_u32(table + 4).unwrap();
    let w2 = mem.read_u32(table + 8).unwrap();

    mem.write_u32(r4, head).unwrap();
    mem.write_u32(r4 + 4, 0).unwrap();
    mem.write_u8(r4 + 8, 0).unwrap();
    for base in [12u32, 24, 40, 52] {
        mem.write_u32(r4 + base, w0).unwrap();
        mem.write_u32(r4 + base + 4, w1).unwrap();
        mem.write_u32(r4 + base + 8, w2).unwrap();
    }
    let f = mem.read_f32(0x8c16ca68).unwrap(); // mova 0x8c16ca68,r0 ; fmov @r0,fr2
    cpu.fr[2] = f;
    mem.write_f32(r4 + 64, f).unwrap();
    mem.write_f32(r4 + 68, f).unwrap();
    mem.write_f32(r4 + 72, f).unwrap();
    mem.write_u32(r4 + 76, w0).unwrap();
    mem.write_u32(r4 + 80, w1).unwrap();
    mem.write_u32(r4 + 84, w2).unwrap();
    cpu.fr[1] = 1.0; // fldi1 fr1
    mem.write_f32(r4 + 88, cpu.fr[1]).unwrap();
    mem.write_u8(r4 + 92, 0).unwrap();
    cpu.fr[0] = 0.0; // fldi0 fr0
    mem.write_f32(r4 + 96, cpu.fr[0]).unwrap();
    mem.write_f32(r4 + 100, cpu.fr[1]).unwrap();

    cpu.r[0] = r4;
    cpu.r[1] = r4 + 100;
    cpu.r[3] = r4 + 96;
    cpu.r[5] = 0;
    cpu.r[6] = r4 + 76;
    cpu.r[7] = w2; // mov.l @(8,r9),r7 at 0x8c16ca3e
}
