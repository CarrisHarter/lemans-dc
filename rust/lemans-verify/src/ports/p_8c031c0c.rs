//! 0x8c031c0c — table lookup: `fr0 = lut[i] * lut2[i]` style blend.
//!
//! ```text
//! mov.l  @(4,r4),r5
//! fldi1  fr2                  ; fr2 = 1.0
//! mov.l  0x8c031c84,r0        ; r0 = 0x3a4
//! mov.l  0x8c031c88,r1        ; r1 = 0x274
//! add    r0,r4                ; r4 += 0x3a4
//! fmov   @r4,fr3              ; fr3 = [param + 0x3a4]
//! fcmp/gt fr2,fr3             ; T = fr3 > 1.0
//! bf.s   0x8c031c22
//! add    r1,r5                ; delay slot: r5 += 0x274 on BOTH paths
//! rts
//! fldi0  fr0                  ; delay slot: fr0 = 0.0 (fr3 > 1.0 early-out)
//! ...main path...
//! rts / fmul fr1,fr0 (delay)
//! ```
//!
//! The main path (fr3 <= 1.0) computes `i = (int)(fr3 * 31.0)` via
//! `ftrc`/`sts fpul`/`lds fpul`, then selects `i == 0`, `i == 31`, or a
//! polynomial blend, and finishes with `fr0 *= fr1` in the `rts` delay slot.
//! The oracle in this workspace does not implement `sts fpul,r3` (0x035a) or
//! `lds r3,fpul` (0x435a), so only the `fr3 > 1.0` early-out is verifiable;
//! the main path below mirrors the machine semantics (fpul = fr as i32)
//! for when the oracle grows those opcodes.
//!
//! Hidden side effects: r5 = [r4+4] + 0x274 on every path (delay slot of
//! `bf.s`), r0 = 0x3a4 then 0x8c on the main paths, r4 stays advanced by 0x3a4,
//! fr2 = 1.0 and fr3 = input stay loaded.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[5] = mem.read_u32(base.wrapping_add(4)).unwrap();
    cpu.fr[2] = 1.0; // fldi1
    cpu.r[0] = mem.read_u32(0x8C031C84).unwrap(); // 0x3a4
    cpu.r[1] = mem.read_u32(0x8C031C88).unwrap(); // 0x274
    cpu.r[4] = base.wrapping_add(cpu.r[0]);
    cpu.fr[3] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.set_t(cpu.fr[3] > cpu.fr[2]); // fcmp/gt fr2,fr3 -> T = fr3 > fr2
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]); // delay slot of `bf.s`, both paths
    if cpu.fr[3] > 1.0 {
        cpu.fr[0] = 0.0; // delay slot of `rts`
        return;
    }

    // ---- main path (fr3 <= 1.0) ----
    cpu.r[0] = 0x8C031C8C; // mova
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap(); // 31.0
    cpu.fr[0] = cpu.fr[3]; // fmov fr3,fr0
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // fmul fr1,fr0
    cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc fr0,fpul
    cpu.r[3] = cpu.fpul; // sts fpul,r3
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.r[3] == 0 {
        // bf not taken: index 0, table entry at r5+12
        cpu.r[0] = cpu.r[5].wrapping_add(12);
        cpu.r[3] = cpu.r[3] << 2; // shll2
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // delay slot of `bra`
    } else {
        cpu.r[0] = cpu.r[3]; // mov r3,r0
        cpu.set_t(cpu.r[0] == 31); // cmp/eq #31,r0
        if cpu.r[0] == 31 {
            // index 31, table entry at r5+12+124
            cpu.r[0] = cpu.r[5].wrapping_add(12);
            cpu.r[3] = cpu.r[3] << 2; // shll2
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // delay slot of `bra`
        } else {
            // polynomial blend
            cpu.r[0] = 0x8C031C8C; // mova
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // 31.0
            cpu.fpul = cpu.r[3]; // lds r3,fpul
            cpu.r[1] = cpu.r[5];
            cpu.r[0] = cpu.r[5];
            cpu.r[1] = cpu.r[1].wrapping_add(12);
            cpu.fr[1] = cpu.fpul as i32 as f32; // float fpul,fr1
            cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
            cpu.fr[3] = cpu.fr[3] - cpu.fr[1]; // fsub fr1,fr3
            cpu.r[3] = cpu.r[3] << 2; // shll2
            cpu.r[0] = cpu.r[0].wrapping_add(16);
            cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // fmul fr0,fr3
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
            cpu.fr[2] = cpu.fr[2] - cpu.fr[3]; // fsub fr3,fr2
            cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
            cpu.fr[1] = cpu.fr[1] * cpu.fr[3]; // fmul fr3,fr1
            cpu.fr[0] = cpu.fr[0] * cpu.fr[2]; // fmul fr2,fr0
            cpu.fr[0] = cpu.fr[0] + cpu.fr[1]; // fadd fr1,fr0 (0x8c031c76)
        }
    }

    // ---- converge at 0x8c031c78 ----
    cpu.r[0] = mem.read_u32(0x8C031C90).unwrap(); // 0x8c
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[0]);
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // delay slot of `rts`: fmul fr1,fr0
}
