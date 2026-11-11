//! 0x8c02c0e8 — `if (param_4 & 0xFF) out = *p * param_1 + in; else out = 0.0;`
//!
//! ```text
//! extu.b r5,r0
//! tst    r0,r0
//! bt     0x8c02c10c           ; low byte of r5 == 0
//! mov.l  @(4,r4),r1
//! fldi1  fr0                  ; fr0 = 1.0
//! mov.l  0x8c02c118,r0        ; r0 = 0x270
//! add    #12,r4
//! fmov   @r4,fr1              ; fr1 = *(float *)(param_3 + 12)   [r4 advanced]
//! add    r0,r1
//! fmov   @r1,fr2
//! fmul   fr4,fr2
//! fadd   fr1,fr2
//! fmov   fr2,@r4              ; store result
//! fmov   @r4,fr1              ; reload
//! fcmp/gt fr0,fr1             ; T = fr1 > fr0
//! bf     0x8c02c112
//! rts
//! fmov   fr0,@r4              ; delay slot: clamp store of 1.0 when result > 1
//! add    #12,r4               ; (zero path)
//! fldi0  fr0
//! fmov   fr0,@r4
//! rts
//! nop
//! ```
//! Hidden side effects: r0 is first `r5 & 0xFF` and then overwritten with the
//! constant 0x270; r1 keeps `[r4+4] + 0x270`; r4 is advanced by 12 before every
//! float access; the second store (1.0f) happens only on the `result > 1.0`
//! path, and that path is the only one that leaves T = 1 (the zero path leaves
//! T = 1 from `tst`, the low path leaves T = 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5] & 0xFF; // extu.b r5,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        // bt target: zero the slot
        cpu.r[4] = cpu.r[4].wrapping_add(12);
        cpu.fr[0] = 0.0; // fldi0
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
        return; // rts, delay slot nop
    }
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
    cpu.fr[0] = 1.0; // fldi1
    cpu.r[0] = mem.read_u32(0x8C02C118).unwrap(); // 0x270
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.fr[2] = cpu.fr[2] * cpu.fr[4]; // fmul fr4,fr2
    cpu.fr[2] = cpu.fr[2] + cpu.fr[1]; // fadd fr1,fr2
    let _ = mem.write_f32(cpu.r[4], cpu.fr[2]);
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    let over = cpu.fr[1] > cpu.fr[0]; // fcmp/gt fr0,fr1 -> T = fr1 > fr0
    cpu.set_t(over);
    if over {
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // delay slot of `rts`
    }
}
