//! 0x8c217326 — allocate/initialize a 84-byte object at r4.
//! Returns 0 immediately when r4 == 0 or when [ [r4] + 28 ] == r4 (already
//! initialized); otherwise fills the struct and returns r4 in r0.
//!
//! ```text
//! tst r4,r4 ; bt 0x8c217332
//! mov.l @r4,r3 ; mov.l @(28,r3),r2 ; cmp/eq r4,r2 ; bf 0x8c217336
//! rts ; mov #0,r0
//! 0x8c217336:
//! mov.l r5,@(4,r4) ; mov #0,r5 ; mov #68,r0
//! mov.w 0x8c217386,r3   ; r3 = 0x07ff
//! mov r7,r2 ; mov #-11,r1 ; add r3,r2 ; shad r1,r2   ; r2 = (r7+0x7ff) >> 11 (arith)
//! mov.l r6,@(8,r4) ; mov.l r7,@(12,r4) ; mov.l r2,@(16,r4) ; mov #1,r2
//! mov.l r5,@(20,r4) ; mov.l r5,@(24,r4) ; mov.l r5,@(28,r4)
//! mov.l r5,@(32,r4) ; mov.l r5,@(60,r4) ; mov.l r5,@(r0,r4)   ; [r4+68]
//! mov #72,r0 ; mov.w r5,@(r0,r4) ; mov #74,r0 ; mov.w r5,@(r0,r4)
//! mov #76,r0 ; mov.w r2,@(r0,r4) ; mov #80,r0 ; mov.w r5,@(r0,r4)
//! mov #82,r0 ; mov.w r5,@(r0,r4)
//! mov r4,r0 ; rts ; nop
//! ```
//! NOTE: the init path executes `shad r1,r2`, which the sh4-oracle does not
//! implement, so only the two rts paths are oracle-verifiable.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] == 0 {
        cpu.r[0] = 0; // rts delay slot
        return;
    }
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap();
    cpu.r[2] = mem.read_u32(cpu.r[3].wrapping_add(28)).unwrap();
    cpu.set_t(cpu.r[4] == cpu.r[2]); // cmp/eq r4,r2
    if cpu.r[4] == cpu.r[2] {
        cpu.r[0] = 0; // rts delay slot
        return;
    }
    // init path (0x8c217336) — not oracle-verifiable (shad unimplemented)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[5]);
    cpu.r[5] = 0;
    cpu.r[0] = 68;
    cpu.r[3] = mem.read_u16(0x8C217386).unwrap() as i16 as i32 as u32; // 0x7ff
    cpu.r[2] = cpu.r[7];
    cpu.r[1] = (-11i32) as u32;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]); // add r3,r2
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[6]);
    cpu.r[2] = ((cpu.r[2] as i32) >> 11) as u32; // shad r1,r2 (r1 = -11, arithmetic)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[7]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[2]);
    cpu.r[2] = 1;
    let z = cpu.r[5];
    let _ = mem.write_u32(cpu.r[4].wrapping_add(20), z);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(24), z);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(28), z);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(32), z);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(60), z);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(68), z); // mov.l r5,@(r0,r4), r0 = 68
    cpu.r[0] = 72;
    let _ = mem.write_u16(cpu.r[4].wrapping_add(72), z as u16);
    cpu.r[0] = 74;
    let _ = mem.write_u16(cpu.r[4].wrapping_add(74), z as u16);
    cpu.r[0] = 76;
    let _ = mem.write_u16(cpu.r[4].wrapping_add(76), cpu.r[2] as u16);
    cpu.r[0] = 80;
    let _ = mem.write_u16(cpu.r[4].wrapping_add(80), z as u16);
    cpu.r[0] = 82;
    let _ = mem.write_u16(cpu.r[4].wrapping_add(82), z as u16);
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    // rts ; nop
}
