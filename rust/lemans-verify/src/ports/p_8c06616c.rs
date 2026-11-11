//! 0x8c06616c — the twin of 0x8c066098 with a different register allocation:
//! r5 := r4+8 is the running record pointer (advanced by 48 per inner
//! iteration), r3 := 0 and r7 := r3 (0) at each outer iteration, and the inner
//! body stores only the two floats fr8 at r5+20 and fr6 at r5+44 — no
//! `mov.l r5,@(36,r3)` stores like 0x8c066098.
//!
//! Constants come from 0x8c066220..0x8c066238; fr3 = 1.0 / float([r4+4]).
//!
//! Hidden side effects the decompiled C does not show: fpul := [r4+4];
//! r5 is clobbered from its incoming argument value to r4+8+48*inner_total;
//! r1/r0 end as r5/r5+44 of the last inner store; r3 stays 0; fr12 := fr5 only
//! when the inner loop exits (plain bt skips its delay slot while taken);
//! r15 and fr12..fr15 are restored by the exit-path pops; T ends 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: fmov fr15,fr14,fr13,fr12,@-r15
    for i in [15usize, 14, 13, 12] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_f32(cpu.r[15], cpu.fr[i]);
    }

    cpu.fr[2] = mem.read_f32(0x8c066220).unwrap();
    cpu.r[5] = cpu.r[4];
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
    cpu.fr[1] = mem.read_f32(0x8c066224).unwrap();
    cpu.fpul = cpu.r[1]; // lds r1,fpul
    cpu.fr[0] = mem.read_f32(0x8c066228).unwrap();
    cpu.fr[3] = 1.0; // fldi1 fr3
    cpu.fr[5] = cpu.fpul as i32 as f32; // float fpul,fr5
    cpu.fr[14] = mem.read_f32(0x8c06622c).unwrap();
    cpu.fr[3] = cpu.fr[3] / cpu.fr[5]; // fdiv fr5,fr3
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    cpu.fr[15] = mem.read_f32(0x8c066230).unwrap();
    cpu.fr[12] = 0.0; // fldi0 fr12
    cpu.r[6] = 1;
    cpu.fr[10] = mem.read_f32(0x8c066234).unwrap();
    cpu.r[3] = 0;
    cpu.fr[9] = mem.read_f32(0x8c066238).unwrap(); // bra 0x8c06620e delay slot

    loop {
        // 0x8c06620e: outer loop head
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
        cpu.set_t((cpu.r[0] as i32) >= (cpu.r[6] as i32)); // cmp/ge r6,r0
        if !cpu.t() {
            break;
        }
        // 0x8c0661a6: outer body
        cpu.fr[5] = cpu.fr[12];
        cpu.fr[5] = cpu.fr[5] - cpu.fr[3];
        cpu.fr[11] = cpu.fr[5];
        cpu.fr[12] = -cpu.fr[12];
        cpu.fr[13] = cpu.fr[12];
        cpu.fr[11] = -cpu.fr[11];
        cpu.fr[8] = cpu.fr[11];
        cpu.fr[13] = cpu.fr[13] * cpu.fr[2];
        cpu.fr[8] = cpu.fr[8] * cpu.fr[2];
        cpu.fr[13] = cpu.fr[13] + cpu.fr[1];
        cpu.fr[8] = cpu.fr[8] + cpu.fr[1];
        cpu.fr[7] = cpu.fr[13];
        cpu.fr[6] = cpu.fr[8];
        cpu.fr[7] = cpu.fr[7] * cpu.fr[13];
        cpu.fr[6] = cpu.fr[6] * cpu.fr[8];
        cpu.fr[13] = cpu.fr[13] * cpu.fr[7];
        cpu.fr[8] = cpu.fr[8] * cpu.fr[6];
        cpu.fr[13] = cpu.fr[13] + cpu.fr[7];
        cpu.fr[12] = cpu.fr[12] * cpu.fr[0];
        cpu.fr[8] = cpu.fr[8] + cpu.fr[6];
        cpu.fr[11] = cpu.fr[11] * cpu.fr[0];
        cpu.fr[8] = cpu.fr[8] + cpu.fr[11];
        cpu.fr[13] = cpu.fr[13] + cpu.fr[12];
        cpu.fr[11] = cpu.fr[15];
        cpu.fr[13] = cpu.fr[13] * cpu.fr[14];
        cpu.fr[11] = cpu.fr[11] - cpu.fr[13];
        cpu.fr[11] = cpu.fr[11] * cpu.fr[10];
        cpu.fr[7] = cpu.fr[15];
        cpu.fr[8] = cpu.fr[8] * cpu.fr[14];
        cpu.fr[7] = cpu.fr[7] - cpu.fr[8];
        cpu.fr[7] = cpu.fr[7] * cpu.fr[10];
        cpu.fr[8] = cpu.fr[4];
        cpu.fr[11] = cpu.fr[11] + cpu.fr[9];
        cpu.fr[6] = cpu.fr[4];
        cpu.fr[7] = cpu.fr[7] + cpu.fr[9];
        cpu.r[7] = cpu.r[3];
        cpu.fr[8] = cpu.fr[8] - cpu.fr[11];
        cpu.fr[6] = cpu.fr[6] - cpu.fr[7]; // bra 0x8c066204 delay slot

        loop {
            // 0x8c066204: inner loop head
            cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap();
            cpu.set_t((cpu.r[0] as i32) >= (cpu.r[7] as i32)); // cmp/ge r7,r0
            if !cpu.t() {
                cpu.fr[12] = cpu.fr[5]; // delay slot runs when bt not taken
                break;
            }
            // 0x8c0661f4: inner body
            cpu.r[1] = cpu.r[5].wrapping_add(20);
            cpu.r[0] = cpu.r[5];
            let _ = mem.write_f32(cpu.r[1], cpu.fr[8]);
            cpu.r[0] = cpu.r[0].wrapping_add(44);
            let _ = mem.write_f32(cpu.r[0], cpu.fr[6]);
            cpu.r[5] = cpu.r[5].wrapping_add(48);
            cpu.r[7] = cpu.r[7].wrapping_add(1);
        }
        cpu.r[6] = cpu.r[6].wrapping_add(1); // 0x8c06620c
    }

    // exit path: 0x8c066214..0x8c06621c
    cpu.fr[12] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts; delay slot pops fr15
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
