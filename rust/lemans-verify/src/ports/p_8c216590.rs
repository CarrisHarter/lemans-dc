//! 0x8c216590 — claim slot on the global context: returns -11 if no context,
//! -9 if already busy, -5 if index out of range; otherwise stores r4/r5 into
//! the context and returns 0.
//!
//! ```text
//! mov.l 0x8c2166dc,r7   ; r7 = 0x8c23d418 (global slot)
//! mov.l @r7,r6          ; r6 = context
//! tst r6,r6 ; bf 0x8c21659c
//! rts ; mov #-11,r0     ; no context
//! mov.l @(28,r6),r2 ; tst r2,r2 ; bt 0x8c2165a6
//! rts ; mov #-9,r0      ; busy
//! cmp/pz r4 ; bf 0x8c2165b0
//! mov.l @(20,r6),r2 ; cmp/ge r2,r4 ; bf 0x8c2165b4
//! rts ; mov #-5,r0      ; r4 < 0 or r4 >= limit
//! mov #1,r2 ; mov.l r2,@(28,r6)
//! mov #2,r2 ; mov.l @r7,r3 ; mov #0,r0 ; mov.l r2,@(32,r3)
//! mov.l @r7,r3 ; mov.l r4,@(40,r3)
//! mov.l @r7,r2 ; mov.l r5,@(48,r2)
//! rts ; nop
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = mem.read_u32(0x8C2166DC).unwrap(); // 0x8c23d418
    cpu.r[6] = mem.read_u32(cpu.r[7]).unwrap();
    cpu.set_t(cpu.r[6] == 0); // tst r6,r6
    if cpu.r[6] == 0 {
        cpu.r[0] = (-11i32) as u32; // rts delay slot
        return;
    }
    cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(28)).unwrap();
    cpu.set_t(cpu.r[2] == 0); // tst r2,r2
    if cpu.r[2] != 0 {
        cpu.r[0] = (-9i32) as u32; // rts delay slot
        return;
    }
    cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz r4
    if (cpu.r[4] as i32) < 0 {
        cpu.r[0] = (-5i32) as u32; // rts delay slot
        return;
    }
    cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(20)).unwrap();
    cpu.set_t((cpu.r[4] as i32) >= (cpu.r[2] as i32)); // cmp/ge r2,r4
    if (cpu.r[4] as i32) >= (cpu.r[2] as i32) {
        cpu.r[0] = (-5i32) as u32; // rts delay slot
        return;
    }
    // success path (0x8c2165b4)
    let _ = mem.write_u32(cpu.r[6].wrapping_add(28), 1); // mov #1,r2 ; mov.l r2,@(28,r6)
    cpu.r[2] = 2;
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap();
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[3].wrapping_add(32), 2);
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap();
    let _ = mem.write_u32(cpu.r[3].wrapping_add(40), cpu.r[4]);
    cpu.r[2] = mem.read_u32(cpu.r[7]).unwrap();
    let _ = mem.write_u32(cpu.r[2].wrapping_add(48), cpu.r[5]);
    // rts ; nop — r0 stays 0
}
