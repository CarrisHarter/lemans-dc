//! 0x8c093ba4 — apply a mode-indexed transition to the global flag byte.
//!
//! `param_1` is a state object; the byte slice at `[param_1+0xf4]` is passed to
//! 0x8c074d58 as the table index (twice, with `r6`=4 and 5). The two returned
//! flag bytes plus the mode from 0x8c1cc3b4 decide whether the byte at
//! `*(0x8c2ab460)+0x345` is rewritten and whether 0x8c1cc380 (clear/copy) runs:
//!
//! ```text
//! mode 1: flag4 != 0 -> byte[+0x345] = 0, call 0x8c1cc380; return 0
//! mode 2: flag4 != 0 -> byte[+0x345] = 0, call 0x8c1cc380; return 0
//! mode 3: flag4 != 0 -> byte[+0x345] = 1, call 0x8c1cc380; return 0
//!         flag4 == 0 && flag5 != 0 -> byte[+0x345] = 0, call; return 0
//!         flag4 == 0 && flag5 == 0 -> no store; return 0
//! other : return param_2 unchanged
//! ```
//!
//! Side effects the C hides: `r12`/`r13`/`r14`/`pr` are saved and restored, all
//! calls are leaf routines that keep `pr`; final T is the last `tst`
//! (`flag4 == 0`) on the mode paths, or the last `cmp/eq #3` (false) on the
//! fall-through path. Callees: 0x8c074d58 (flag fetch), 0x8c1cc3b4 (mode),
//! 0x8c1cc380 (clear/copy).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c093ba4-8c093bae: push r14/r13/r12/pr, r13 = param_2, r12 = param_1.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = cpu.r[4];

    // 8c093bb0-8c093bc0: flag4 = 0x8c074d58(*(0x8c264da8), *(r12+0xf4), 4, 0).
    cpu.r[0] = mem.read_u32(0x8c093c70).unwrap_or(0); // 0xf4
    cpu.r[7] = 0;
    cpu.r[1] = mem.read_u32(0x8c093c74).unwrap_or(0); // 0x8c264da8
    cpu.r[6] = 4;
    cpu.r[5] = mem
        .read_u32(cpu.r[0].wrapping_add(cpu.r[12]))
        .unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c093c78).unwrap_or(0); // 0x8c074d58
    cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c074d58::run(cpu, mem);
    cpu.r[14] = cpu.r[0]; // mov r0,r14

    // 8c093bc2-8c093bd4: flag5 = 0x8c074d58(*(0x8c264da8), *(r12+0xf4), 5, 0).
    cpu.r[0] = mem.read_u32(0x8c093c70).unwrap_or(0); // 0xf4
    cpu.r[7] = 0;
    cpu.r[1] = mem.read_u32(0x8c093c74).unwrap_or(0); // 0x8c264da8
    cpu.r[6] = 5;
    cpu.r[5] = mem
        .read_u32(cpu.r[0].wrapping_add(cpu.r[12]))
        .unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c093c78).unwrap_or(0); // 0x8c074d58
    cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c074d58::run(cpu, mem);
    cpu.r[1] = mem.read_u32(0x8c093c7c).unwrap_or(0); // 0x8c2b8728
    cpu.r[12] = cpu.r[0]; // mov r0,r12
    cpu.r[0] = mem.read_u32(0x8c093c80).unwrap_or(0); // 0x8c1cc3b4
    cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c1cc3b4::run(cpu, mem);

    // 8c093bdc-8c093bea: cmp/eq #1,#2,#3 then bra epilogue (T = false there).
    cpu.set_t(cpu.r[0] == 1);
    if cpu.r[0] == 1 {
        // 8c093c46: mode 1 — clear the byte when flag4 is set.
        cpu.r[0] = cpu.r[14] & 0xFF; // extu.b r14,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            store_global_byte(cpu, mem, 0);
            call_clear_and_copy(cpu, mem);
        }
        cpu.r[13] = 0; // 8c093c60: mov #0,r13
    } else {
        cpu.set_t(cpu.r[0] == 2);
        if cpu.r[0] == 2 {
            // 8c093c28: mode 2 — clear the byte when flag4 is set.
            cpu.r[0] = cpu.r[14] & 0xFF; // extu.b r14,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.r[0] != 0 {
                store_global_byte(cpu, mem, 0);
                call_clear_and_copy(cpu, mem);
            }
            cpu.r[13] = 0; // 8c093c42 delay: mov #0,r13
        } else {
            cpu.set_t(cpu.r[0] == 3);
            if cpu.r[0] == 3 {
                // 8c093bec: mode 3 — set on flag4, clear on flag5 only.
                cpu.r[0] = cpu.r[14] & 0xFF; // extu.b r14,r0
                cpu.set_t(cpu.r[0] == 0); // tst r0,r0
                if cpu.r[0] != 0 {
                    store_global_byte(cpu, mem, 1);
                    call_clear_and_copy(cpu, mem);
                } else {
                    cpu.r[0] = cpu.r[12] & 0xFF; // extu.b r12,r0
                    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
                    if cpu.r[0] != 0 {
                        store_global_byte(cpu, mem, 0);
                        call_clear_and_copy(cpu, mem);
                    }
                }
                cpu.r[13] = 0; // 8c093c26/8c093c44 delay: mov #0,r13
            }
            // else: fall-through bra 0x8c093c62, r13 keeps param_2.
        }
    }

    // 8c093c62-8c093c6c: mov r13,r0 ; lds.l pr ; pop r12/r13 ; rts (pop r14).
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

/// 8c093bf2/8c093c10/8c093c2e/8c093c4c: `mov.l &global,r0 ; mov #v,r1 ;
/// mov.l @r0,r3 ; mov.l &0x345,r0 ; mov.b r1,@(r0,r3)`.
fn store_global_byte(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[0] = mem.read_u32(0x8c093c84).unwrap_or(0); // 0x8c2ab460
    cpu.r[1] = value;
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c093c88).unwrap_or(0); // 0x345
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1] as u8);
}

/// The common tail of every store path: load the global pointer into `r4` and
/// call 0x8c1cc380 (clear byte 0x8c215d80, copy 0x8c2b8738 to 0x8c2b8740).
fn call_clear_and_copy(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c093c7c).unwrap_or(0); // 0x8c2b8728
    cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c093c8c).unwrap_or(0); // 0x8c1cc380
    crate::registry::p_8c1cc380::run(cpu, mem);
}
