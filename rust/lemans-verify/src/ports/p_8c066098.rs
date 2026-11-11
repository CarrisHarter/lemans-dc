//! 0x8c066098 — nested FPU loop over a record array at r4.
//!
//! Outer loop (r6 from 1 while [r4+4] >= r6): compute a polynomial in fr12..fr8
//! from constants at 0x8c066150..0x8c066168 and fr4/fr9; inner loop (r7 from
//! r1=0 while [r4] >= r7) stores fr8 at r3+20, r5 at r3+36, fr6 at r3+44 and
//! r5 at (r3+48)+12, advancing r3 by 48. fr3 = 1.0 / float([r4+4] via fpul).
//!
//! Hidden side effects the decompiled C does not show: fpul := [r4+4];
//! r3 := r4+8+48*inner_total; r1 := 0; r5 is stored verbatim (it is an
//! untouched argument register, not a pointer — unlike the twin 0x8c06616c);
//! r7 ends at [r4]+1; fr12 := fr5 happens only when the inner loop exits (the
//! plain `bt` at 0x8c066138 skips its delay slot while taken); the four
//! prologue pushes are balanced by pops only on the exit path, so r15 and
//! fr12..fr15 are restored exactly; T ends 0 (last cmp/ge at 0x8c066140).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: fmov fr15,fr14,fr13,fr12,@-r15
    for i in [15usize, 14, 13, 12] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_f32(cpu.r[15], cpu.fr[i]);
    }

    cpu.fr[2] = mem.read_f32(0x8c066150).unwrap();
    cpu.r[3] = cpu.r[4];
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
    cpu.fr[1] = mem.read_f32(0x8c066154).unwrap();
    cpu.fpul = cpu.r[1]; // lds r1,fpul
    cpu.fr[0] = mem.read_f32(0x8c066158).unwrap();
    cpu.fr[3] = 1.0; // fldi1 fr3
    cpu.fr[5] = cpu.fpul as i32 as f32; // float fpul,fr5
    cpu.fr[14] = mem.read_f32(0x8c06615c).unwrap();
    cpu.fr[3] = cpu.fr[3] / cpu.fr[5]; // fdiv fr5,fr3
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    cpu.fr[15] = mem.read_f32(0x8c066160).unwrap();
    cpu.fr[12] = 0.0; // fldi0 fr12
    cpu.r[6] = 1;
    cpu.fr[10] = mem.read_f32(0x8c066164).unwrap();
    cpu.r[1] = 0;
    cpu.fr[9] = mem.read_f32(0x8c066168).unwrap(); // bra 0x8c06613e delay slot

    loop {
        // 0x8c06613e: outer loop head
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
        cpu.set_t((cpu.r[0] as i32) >= (cpu.r[6] as i32)); // cmp/ge r6,r0
        if !cpu.t() {
            break; // plain bt not taken: delay slot 0x8c066144 is the next instruction
        }
        // 0x8c0660d2: outer body (bt taken, delay slot skipped)
        cpu.fr[5] = cpu.fr[12];
        cpu.fr[5] = cpu.fr[5] - cpu.fr[3]; // fsub fr3,fr5
        cpu.fr[11] = cpu.fr[5];
        cpu.fr[12] = -cpu.fr[12]; // fneg fr12
        cpu.fr[13] = cpu.fr[12];
        cpu.fr[11] = -cpu.fr[11]; // fneg fr11
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
        cpu.r[7] = cpu.r[1];
        cpu.fr[8] = cpu.fr[8] - cpu.fr[11];
        cpu.fr[6] = cpu.fr[6] - cpu.fr[7]; // bra 0x8c066134 delay slot

        loop {
            // 0x8c066134: inner loop head
            cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap();
            cpu.set_t((cpu.r[0] as i32) >= (cpu.r[7] as i32)); // cmp/ge r7,r0
            if !cpu.t() {
                cpu.fr[12] = cpu.fr[5]; // plain bt not taken: delay slot runs
                break;
            }
            // 0x8c066120: inner body (bt taken, delay slot skipped)
            cpu.r[0] = cpu.r[3].wrapping_add(20);
            let _ = mem.write_f32(cpu.r[0], cpu.fr[8]);
            cpu.r[7] = cpu.r[7].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[3].wrapping_add(36), cpu.r[5]);
            cpu.r[0] = cpu.r[3].wrapping_add(44);
            let _ = mem.write_f32(cpu.r[0], cpu.fr[6]);
            cpu.r[3] = cpu.r[3].wrapping_add(48);
            let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[5]);
        }
        cpu.r[6] = cpu.r[6].wrapping_add(1); // 0x8c06613c
    }

    // exit path: 0x8c066144..0x8c06614c as sequential code after the not-taken bt
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
