//! 0x8c104544 — publish a validated render-state record into four globals.
//!
//! ```text
//! sts.l pr,@-r15
//! r0 = r4 + 0x44 ; r5 = [r0]
//! r7 = FUN_8c1044f4(r4, r5)            ; jsr through 0x8c104590
//! tst r7,r7 ; bt out                   ; no record: return 0
//! r6 = [r7+0x14] ; r0 = [r6]
//! cmp/eq #1,r0 ; bf out                ; header word must be 1
//! r6 += 4
//! r5 = 0x8c2ab350
//! r0 = [r6]     ; r4 = [r6+4]          ; [0x8c2ab350] = r0
//! r1 = 0x8c203da0 ; r3 = 0x8c203d98
//! [0x8c2ab354] = r4
//! r0 = r7 ; r4 = [r6+8] ; [0x8c2ab358] = r4
//! r6 += 12 ; fr0 = [r6]                ; the record's f32 at +16
//! r5 += 12 ; [r5] = fr0                ; [0x8c2ab35c] = fr0
//! fr1 = [0x8c203da0] ; fr0 = [r5]
//! fr1 = fr1 / fr0
//! [0x8c203d98] = fr1                   ; delay slot of the `bra` to out
//! out: lds.l @r15+,pr ; rts
//! ```
//!
//! The callee is looked up from the literal pool at 0x8c104590 (0x8c1044f4),
//! so it is ported by calling its registered port with `pr` set past the `jsr`
//! and its (nop) delay slot already done. The pushed `pr`, the callee's own
//! frame and the `lds.l @r15+,pr` all touch the synthetic stack, which the
//! verifier filters; `r15` itself ends at its entry value.
//!
//! Both failing arms leave `r0` = 0 and `r7` = callee result; the `bt` arm's
//! last flag write is `tst r7,r7` (T set), while the `bf` arm's is
//! `cmp/eq #1,r0` (T clear). Adds, movs and `fdiv` do not touch T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c104544: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c104546: mov r4,r0 ; 0x8c104548: add #68,r0
    cpu.r[0] = cpu.r[4].wrapping_add(68);
    // 0x8c10454a: mov.l @r0,r5
    cpu.r[5] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 0x8c10454c: mov.l 0x8c104590,r0 (0x8c1044f4)
    cpu.r[0] = mem.read_u32(0x8c104590).unwrap_or(0x8c1044f4);
    // 0x8c10454e: jsr @r0 ; 0x8c104550: nop (delay slot)
    cpu.pr = 0x8c104552;
    crate::registry::p_8c1044f4::run(cpu, mem);
    // 0x8c104552: mov r0,r7
    cpu.r[7] = cpu.r[0];
    // 0x8c104554: tst r7,r7
    cpu.set_t(cpu.r[7] == 0);
    if cpu.t() {
        // 0x8c104556: bt 0x8c104588 ; 0x8c104588: mov #0,r0
        cpu.r[0] = 0;
    } else {
        // 0x8c104558: mov.l @(20,r7),r6
        cpu.r[6] = mem.read_u32(cpu.r[7].wrapping_add(20)).unwrap_or(0);
        // 0x8c10455a: mov.l @r6,r0
        cpu.r[0] = mem.read_u32(cpu.r[6]).unwrap_or(0);
        // 0x8c10455c: cmp/eq #1,r0
        cpu.set_t(cpu.r[0] == 1);
        // 0x8c10455e: bf 0x8c104588
        if cpu.t() {
            // 0x8c104560: add #4,r6
            cpu.r[6] = cpu.r[6].wrapping_add(4);
            // 0x8c104562: mov.l @r6,r0
            cpu.r[0] = mem.read_u32(cpu.r[6]).unwrap_or(0);
            // 0x8c104564: mov.l 0x8c104594,r5 (0x8c2ab350)
            cpu.r[5] = mem.read_u32(0x8c104594).unwrap_or(0x8c2ab350);
            // 0x8c104566: mov.l @(4,r6),r4
            cpu.r[4] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
            // 0x8c104568: mov.l 0x8c104598,r1 (0x8c203da0)
            cpu.r[1] = mem.read_u32(0x8c104598).unwrap_or(0x8c203da0);
            // 0x8c10456a: mov.l 0x8c10459c,r3 (0x8c203d98)
            cpu.r[3] = mem.read_u32(0x8c10459c).unwrap_or(0x8c203d98);
            // 0x8c10456c: mov.l r0,@r5
            let _ = mem.write_u32(cpu.r[5], cpu.r[0]);
            // 0x8c10456e: mov.l r4,@(4,r5)
            let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[4]);
            // 0x8c104570: mov r7,r0
            cpu.r[0] = cpu.r[7];
            // 0x8c104572: mov.l @(8,r6),r4
            cpu.r[4] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
            // 0x8c104574: mov.l r4,@(8,r5)
            let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[4]);
            // 0x8c104576: add #12,r6
            cpu.r[6] = cpu.r[6].wrapping_add(12);
            // 0x8c104578: fmov @r6,fr0
            cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
            // 0x8c10457a: add #12,r5
            cpu.r[5] = cpu.r[5].wrapping_add(12);
            // 0x8c10457c: fmov fr0,@r5
            let _ = mem.write_f32(cpu.r[5], cpu.fr[0]);
            // 0x8c10457e: fmov @r1,fr1
            cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
            // 0x8c104580: fmov @r5,fr0
            cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
            // 0x8c104582: fdiv fr0,fr1
            cpu.fr[1] /= cpu.fr[0];
            // 0x8c104584: bra 0x8c10458a ; 0x8c104586: fmov fr1,@r3 (delay slot)
            let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
        } else {
            // 0x8c104588: mov #0,r0
            cpu.r[0] = 0;
        }
    }
    // 0x8c10458a: lds.l @r15+,pr ; 0x8c10458c: rts ; 0x8c10458e: nop
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
