//! 0x8c0b5e94 — clamp `r5` to >= 1 and publish it through three globals.
//!
//! ```text
//! mov.l 0x8c0b5eb0,r0   ; r0 = &G1 (0x8c26d010)
//! cmp/pl r5             ; T = (r5 > 0)
//! bt.s  0x8c0b5e9e
//! mov.l r4,@r0          ; delay slot: *G1 = r4, runs on BOTH paths
//! mov   #1,r5           ; only when r5 <= 0
//! mov.l 0x8c0b5eb4,r0   ; r0 = &G2
//! mov.l 0x8c0b5eb8,r3   ; r3 = &G3
//! mov.l @r0,r0          ; r0 = *G2
//! mov.l 0x8c0b5ebc,r1   ; r1 = &G4
//! add   r5,r0
//! mov.l r5,@r3          ; *G3 = r5
//! rts
//! mov.l r0,@r1          ; delay slot: *G4 = r0
//! ```
//! Hidden side effects: the delay-slot store `*G1 = r4` fires even when the
//! branch is taken, `r5` is clamped to 1, and r0/r1/r3 end as the loaded
//! global pointers / sum.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g1 = mem.read_u32(0x8C0B5EB0).unwrap();
    cpu.r[0] = g1;
    let t = (cpu.r[5] as i32) > 0; // cmp/pl r5
    cpu.set_t(t);
    let _ = mem.write_u32(g1, cpu.r[4]); // delay slot of bt.s — always runs
    if !t {
        cpu.r[5] = 1; // mov #1,r5
    }
    cpu.r[0] = mem.read_u32(0x8C0B5EB4).unwrap(); // &G2
    cpu.r[3] = mem.read_u32(0x8C0B5EB8).unwrap(); // &G3
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // *G2
    cpu.r[1] = mem.read_u32(0x8C0B5EBC).unwrap(); // &G4
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]); // add r5,r0
    let _ = mem.write_u32(cpu.r[3], cpu.r[5]); // mov.l r5,@r3
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // rts delay slot: mov.l r0,@r1
}
