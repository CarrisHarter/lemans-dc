//! 0x8c082660 — string trimmer + mode dispatch (156 instructions).
//!
//! CFG (all `bt`/`bf` here are non-delayed; only the four `bra`s and the `rts`
//! have delay slots):
//!   r1 = *(u32*)(r4+0x34)                       0x8c082662
//!   if r1 != 0: strip leading spaces from the global string at 0x8c264ed8
//!     in place, subtracting *(float*)(r4+4) from the global float at
//!     0x8c264ed0 once per space; then strip trailing spaces the same way
//!     (scanning from the NUL backwards) and NUL-terminate.
//!   if r1 == 4: count words (runs of non-NUL/'\n'/' ' chars) in the trimmed
//!     string into r3; if r3 > 1, adjust *F by -(*(r4+4))*(r3-1), and if the
//!     result beats 0x3f2aacda * *(r4+0x60), store
//!     ((*(r4+0x60) - *F) - *(r4+0x38)) / (r3-1) at *r5.
//!   tail (r1 != 2, unsigned test on r1-2): clamp(*(r4+0x60) - *F, min 0),
//!     scaled by 1.0f if r1 == 3, and *(r4+0x38) = *(r4+0x40) + that.
//!
//! Side effects the decompiled C does not show:
//!   * the `add #4,r3` / `add #4,r6` live in the *delay slots of the entry
//!     `bra`s*, so r3/r6 advance exactly once: every stripped space subtracts
//!     the same *(r4+4), never a walk over r4+0, +4, +8, ...
//!   * r6 ends holding the global float pointer 0x8c264ed0 (not a struct
//!     field), r7 ends holding 0x8c264ed0 too (block 2 reloads it as the F
//!     pointer) unless mode==4, where it is the last char scanned;
//!   * r3 ends at r4+0x60 (tail) or r1-2 == 0 (mode 2); r0 ends at r4+0x38;
//!   * r4 itself is incremented by 0x40 on every path except mode 2;
//!   * fpul is loaded with the word count on the mode-4 path;
//!   * r14 is used as the backwards scan pointer but restored by the `rts`
//!     delay slot, so it is invisible; r15 is net unchanged (one push/pop).
//!   * the trailing-space clamp is a real clamp: `bf` at 0x8c082778 is
//!     non-delayed, so `fmov fr0,fr2` at 0x8c08277a runs only when 0 > fr2.

use sh4_oracle::{Cpu, Memory};

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Literals in the image's data pool.
    let str_glob = mem.read_u32(0x8c082798).unwrap_or(0); // 0x8c264ed8
    let flt_glob = mem.read_u32(0x8c08279c).unwrap_or(0); // 0x8c264ed0

    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(0x34)).unwrap_or(0);
    cpu.set_t(cpu.r[1] == 0); // tst r1,r1 (0x8c082664)
    if cpu.r[1] != 0 {
        // ---- block 1: strip leading spaces, then copy the rest down ----
        cpu.r[7] = str_glob; // 0x8c082668
        cpu.r[3] = cpu.r[4]; // 0x8c08266a
        cpu.r[6] = flt_glob; // 0x8c08266c
        cpu.r[3] = cpu.r[3].wrapping_add(4); // bra delay slot 0x8c082670
        loop {
            // 0x8c08267c
            let c = sext8(mem, cpu.r[7]);
            cpu.set_t(c == 0); // tst r0,r0
            if c == 0 {
                break; // bt 0x8c082688
            }
            cpu.set_t(c == 32); // cmp/eq #32,r0
            if c != 32 {
                break; // fall through to 0x8c082688
            }
            // 0x8c082672 body
            cpu.fr[1] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
            cpu.r[7] = cpu.r[7].wrapping_add(1);
            cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
            cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
            let _ = mem.write_f32(cpu.r[6], cpu.fr[1]);
        }
        // 0x8c082688: copy the remainder of the string onto its start
        cpu.r[3] = str_glob;
        loop {
            // 0x8c082698
            let c = sext8(mem, cpu.r[7]);
            cpu.set_t(c == 0); // tst r0,r0
            if c != 0 {
                // 0x8c08268e body
                let b = mem.read_u8(cpu.r[7]).unwrap_or(0);
                let dst = cpu.r[3];
                cpu.r[3] = cpu.r[3].wrapping_add(1);
                let _ = mem.write_u8(dst, b);
                cpu.r[7] = cpu.r[7].wrapping_add(1);
                continue;
            }
            break;
        }
        cpu.r[0] = 0; // 0x8c0826a0
        let p = cpu.r[3];
        let _ = mem.write_u8(p, 0); // 0x8c0826a2

        // ---- block 2: strip trailing spaces, scanning backwards ----
        let mut r14 = str_glob; // scratch; the original's r14 is restored by rts
        loop {
            // 0x8c0826b0
            let c = sext8(mem, r14);
            cpu.set_t(c == 0);
            if c != 0 {
                r14 = r14.wrapping_add(1); // 0x8c0826ae
                continue;
            }
            break;
        }
        cpu.r[7] = flt_glob; // 0x8c0826b8 (fall-through of the strlen loop)
        cpu.r[6] = cpu.r[4]; // 0x8c0826ba
        cpu.r[3] = str_glob; // 0x8c0826bc
        r14 = r14.wrapping_sub(1); // 0x8c0826be
        cpu.r[6] = cpu.r[6].wrapping_add(4); // bra delay slot 0x8c0826c2
        loop {
            // 0x8c0826ce
            cpu.set_t(r14 >= cpu.r[3]); // cmp/hs r3,r14
            if r14 < cpu.r[3] {
                break; // bf 0x8c0826da
            }
            let c = sext8(mem, r14);
            cpu.set_t(c == 32); // cmp/eq #32,r0
            if c != 32 {
                break; // fall through to 0x8c0826da
            }
            // 0x8c0826c4 body
            cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
            r14 = r14.wrapping_sub(1);
            cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
            cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
            let _ = mem.write_f32(cpu.r[7], cpu.fr[1]);
        }
        cpu.r[0] = 0; // 0x8c0826da
        let _ = mem.write_u8(r14.wrapping_add(1), 0); // 0x8c0826dc
    }

    // ---- dispatch on mode ----
    cpu.r[0] = cpu.r[1]; // 0x8c0826de
    cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0
    if cpu.r[1] == 4 {
        cpu.r[6] = str_glob; // 0x8c0826e4
        cpu.r[3] = 0; // bra delay slot 0x8c0826e8
        'words: loop {
            // 0x8c082714
            cpu.r[7] = mem.read_u8(cpu.r[6]).unwrap_or(0) as u32;
            let mut c = cpu.r[7] as u8 as i8 as i32 as u32; // exts.b r7,r0
            cpu.set_t(c == 0); // tst r0,r0
            if c == 0 {
                break 'words; // fall through to 0x8c08271c
            }
            // 0x8c0826ea
            if c == 10 || c == 32 {
                cpu.r[6] = cpu.r[6].wrapping_add(1); // bt 0x8c082712
                continue 'words;
            }
            cpu.r[3] = cpu.r[3].wrapping_add(1); // 0x8c0826f8
            loop {
                // 0x8c0826fa inner skip
                cpu.r[6] = cpu.r[6].wrapping_add(1);
                cpu.r[7] = mem.read_u8(cpu.r[6]).unwrap_or(0) as u32;
                c = cpu.r[7] as u8 as i8 as i32 as u32;
                if c == 0 || c == 10 || c == 32 {
                    break; // bt 0x8c08270c / fall-through of bf
                }
            }
            // 0x8c08270c
            cpu.set_t(c == 0);
            if c == 0 {
                break 'words; // bt 0x8c08271c
            }
            cpu.r[6] = cpu.r[6].wrapping_add(1); // 0x8c082712
        }
        // 0x8c08271c
        cpu.r[0] = 1;
        cpu.set_t(cpu.r[3] > cpu.r[0]); // cmp/gt r0,r3
        if cpu.r[3] > 1 {
            cpu.r[0] = cpu.r[4].wrapping_add(4); // 0x8c082722
            cpu.r[6] = flt_glob; // 0x8c082726
            cpu.fpul = cpu.r[3]; // lds r3,fpul (0x8c082728)
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0x8c08272a
            cpu.fr[0] = 1.0; // fldi1 fr0 (0x8c08272c)
            cpu.fr[3] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // 0x8c08272e
            cpu.fr[4] = cpu.fpul as i32 as f32; // float fpul,fr4 (0x8c082730)
            cpu.fr[4] = cpu.fr[4] - cpu.fr[0]; // 0x8c082732
            cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0 (0x8c082734)
            cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // fmul fr1,fr0 (0x8c082736)
            cpu.fr[3] = cpu.fr[3] - cpu.fr[0]; // 0x8c082738
            cpu.r[3] = cpu.r[4]; // 0x8c08273a
            let _ = mem.write_f32(cpu.r[6], cpu.fr[3]); // 0x8c08273c
            cpu.r[3] = cpu.r[4].wrapping_add(0x60); // 0x8c08273e
            cpu.r[0] = (0x8c082740u32 & !3).wrapping_add(4).wrapping_add(0x17 * 4); // mova (0x8c0827a0)
            cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // 0x8c082742
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0x8c082744
            cpu.fr[0] = cpu.fr[2]; // 0x8c082746
            cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // 0x8c082748
            cpu.set_t(cpu.fr[3] > cpu.fr[0]); // fcmp/gt (0x8c08274a): fr3 > fr0
            if cpu.fr[3] > cpu.fr[0] {
                cpu.r[0] = cpu.r[4].wrapping_add(0x38); // 0x8c08274e
                cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0x8c082752
                cpu.fr[2] = cpu.fr[2] - cpu.fr[3]; // 0x8c082754
                cpu.fr[2] = cpu.fr[2] - cpu.fr[0]; // 0x8c082756
                cpu.fr[2] = cpu.fr[2] / cpu.fr[4]; // fdiv (0x8c082758)
                let r5 = cpu.r[5];
                let _ = mem.write_f32(r5, cpu.fr[2]); // 0x8c08275a
            }
        }
    }

    // ---- tail: runs only when (r1-2) <u 2, i.e. modes 2 and 3 ----
    let neg2 = mem.read_u32(0x8c0827a4).unwrap_or(0); // 0xfffffffe
    cpu.r[3] = cpu.r[1].wrapping_add(neg2); // 0x8c08275e
    cpu.r[0] = 1; // 0x8c082762
    cpu.set_t(cpu.r[0] >= cpu.r[3]); // cmp/hs r3,r0 (0x8c082764): T = r0 >=u r3
    if cpu.r[0] >= cpu.r[3] {
        cpu.r[0] = flt_glob; // 0x8c082768
        cpu.r[3] = cpu.r[4].wrapping_add(0x60); // 0x8c08276a
        cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // 0x8c08276e
        cpu.fr[0] = 0.0; // fldi0 fr0 (0x8c082770)
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0x8c082772
        cpu.fr[2] = cpu.fr[2] - cpu.fr[1]; // 0x8c082774
        cpu.set_t(cpu.fr[0] > cpu.fr[2]); // fcmp/gt (0x8c082776): fr0 > fr2
        if cpu.fr[0] > cpu.fr[2] {
            cpu.fr[2] = cpu.fr[0]; // 0x8c08277a (non-delayed bf: clamp only when 0 > fr2)
        }
        cpu.r[0] = cpu.r[1]; // 0x8c08277c
        cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0 (0x8c08277e)
        if cpu.r[1] == 3 {
            cpu.r[0] = (0x8c082782u32 & !3).wrapping_add(4).wrapping_add(0x09 * 4); // mova (0x8c0827a8)
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0x8c082784
            cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c082786
        }
        cpu.r[0] = cpu.r[4]; // 0x8c082788
        cpu.r[4] = cpu.r[4].wrapping_add(0x40); // 0x8c08278a
        cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // 0x8c08278c
        cpu.r[0] = cpu.r[0].wrapping_add(0x38); // 0x8c08278e
        cpu.fr[0] = cpu.fr[0] + cpu.fr[2]; // fadd (0x8c082790)
        let r0 = cpu.r[0];
        let _ = mem.write_f32(r0, cpu.fr[0]); // 0x8c082792
    }
    // rts (0x8c082794): delay slot restores r14; r15 net unchanged.
}
