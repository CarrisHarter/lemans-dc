//! 0x8c05ad90 — init a two-word record and link it to an owner block.
//!
//! ```text
//! push r14, r13, pr
//! r14 = r5 (owner); r13 = r4 (record)
//! call 0x8c056bf8 (no args)          ; stores 0x8c1e8f88 to [r4] as a side effect,
//!                                    ; but r4 still holds `record` at the call,
//!                                    ; so this store is overwritten below
//! [r13]   = 0x8c1e9120 (literal pool @0x8c05adb8)
//! [r13+4] = r14 (owner)
//! r0 = r13
//! [r14+44] = r13 (owner back-link)
//! pop pr, r13, r14; rts
//! ```
//! Contract: returns `record` (r4) in r0; callee-saved r13/r14/pr restored.
//! The callee clobbers r0 with its own store; only visible effect on the port
//! side is its memory write, which the ported callee replays.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // mov.l r13,@-r15
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.r[0] = mem.read_u32(0x8c05adb4).unwrap_or(0); // 0x8c056bf8
    cpu.pr = 0x8c05ada0; // jsr return address (delay slot: nop)
    crate::registry::p_8c056bf8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c05adb8).unwrap_or(0); // 0x8c1e9120
    let _ = mem.write_u32(cpu.r[13], cpu.r[0]); // mov.l r0,@r13
    let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[14]); // mov.l r14,@(4,r13)
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(44), cpu.r[13]); // mov.l r13,@(44,r14)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
