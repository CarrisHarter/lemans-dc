//! 0x8c19b7c8 — clamped lerp-ish combine: [r0] = fr2 after the branch.
//!
//! ```text
//! mov.l @r15,r0 ; fmov @r4,fr1 ; fmov @r5,fr2 ; fcmp/gt fr2,fr1  ; T = fr1 > fr2
//! bf 0x8c19b7e4
//! fldi1 fr0 ; fadd fr0,fr2 ; fsub fr1,fr2 ; fmul fr4,fr2 ; fadd fr1,fr2
//! fcmp/gt fr2,fr0   ; T = fr0 > fr2
//! bt 0x8c19b7ea     ; plain bt: delay slot skipped when taken
//! bra 0x8c19b7ea
//! fsub fr0,fr2      ; bra delay slot: runs only when bt was not taken
//! 0x8c19b7e4: fsub fr1,fr2 ; fmul fr4,fr2 ; fadd fr1,fr2
//! 0x8c19b7ea: rts
//! fmov fr2,@r0      ; rts delay slot
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap();
    let dst = cpu.r[0];
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[2] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.set_t(cpu.fr[1] > cpu.fr[2]); // fcmp/gt fr2,fr1 (0xF125): T = fr1 > fr2
    if cpu.fr[1] > cpu.fr[2] {
        cpu.fr[0] = 1.0; // fldi1
        cpu.fr[2] = cpu.fr[2] + cpu.fr[0];
        cpu.fr[2] = cpu.fr[2] - cpu.fr[1];
        cpu.fr[2] = cpu.fr[2] * cpu.fr[4];
        cpu.fr[2] = cpu.fr[2] + cpu.fr[1];
        cpu.set_t(cpu.fr[0] > cpu.fr[2]); // fcmp/gt fr2,fr0 (0xF025): T = fr0 > fr2
        if cpu.fr[0] <= cpu.fr[2] {
            cpu.fr[2] = cpu.fr[2] - cpu.fr[0]; // bra delay slot
        }
    } else {
        cpu.fr[2] = cpu.fr[2] - cpu.fr[1];
        cpu.fr[2] = cpu.fr[2] * cpu.fr[4];
        cpu.fr[2] = cpu.fr[2] + cpu.fr[1];
    }
    mem.write_f32(dst, cpu.fr[2]).unwrap(); // rts delay slot
}
