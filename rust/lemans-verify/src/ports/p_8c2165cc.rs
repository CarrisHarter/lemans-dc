//! 0x8c2165cc — like 0x8c216590 but marks the slot with 2, zeroes r4 and
//! also clears offset 36; returns 0 on success, -11/-9/-5 on the early exits.
//!
//! ```text
//! mov.l 0x8c2166dc,r7 ; mov.l @r7,r6 ; tst r6,r6 ; bf 0x8c2165d8
//! rts ; mov #-11,r0
//! mov.l @(28,r6),r2 ; tst r2,r2 ; bt 0x8c2165e2
//! rts ; mov #-9,r0
//! cmp/pz r4 ; bf 0x8c2165ec
//! mov.l @(20,r6),r2 ; cmp/ge r2,r4 ; bf 0x8c2165f0
//! rts ; mov #-5,r0
//! mov #2,r2 ; mov.l r2,@(28,r6)
//! mov.l @r7,r3 ; mov.l r2,@(32,r3)
//! mov.l @r7,r3 ; mov.l r4,@(40,r3)
//! mov #0,r4 ; mov.l @r7,r2 ; mov r4,r0 ; mov.l r5,@(48,r2)
//! mov.l @r7,r3 ; mov.l r4,@(36,r3)
//! rts ; nop
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = mem.read_u32(0x8C2166DC).unwrap(); // 0x8c23d418
    cpu.r[6] = mem.read_u32(cpu.r[7]).unwrap();
    cpu.set_t(cpu.r[6] == 0); // tst r6,r6
    if cpu.r[6] == 0 {
        cpu.r[0] = (-11i32) as u32;
        return;
    }
    cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(28)).unwrap();
    cpu.set_t(cpu.r[2] == 0); // tst r2,r2
    if cpu.r[2] != 0 {
        cpu.r[0] = (-9i32) as u32;
        return;
    }
    cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz r4
    if (cpu.r[4] as i32) < 0 {
        cpu.r[0] = (-5i32) as u32;
        return;
    }
    cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(20)).unwrap();
    cpu.set_t((cpu.r[4] as i32) >= (cpu.r[2] as i32)); // cmp/ge r2,r4
    if (cpu.r[4] as i32) >= (cpu.r[2] as i32) {
        cpu.r[0] = (-5i32) as u32;
        return;
    }
    // success path (0x8c2165f0)
    cpu.r[2] = 2;
    let _ = mem.write_u32(cpu.r[6].wrapping_add(28), 2);
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap();
    let _ = mem.write_u32(cpu.r[3].wrapping_add(32), 2);
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap();
    let _ = mem.write_u32(cpu.r[3].wrapping_add(40), cpu.r[4]);
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[2] = mem.read_u32(cpu.r[7]).unwrap();
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let _ = mem.write_u32(cpu.r[2].wrapping_add(48), cpu.r[5]);
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap();
    let _ = mem.write_u32(cpu.r[3].wrapping_add(36), cpu.r[4]);
    // rts ; nop
}
