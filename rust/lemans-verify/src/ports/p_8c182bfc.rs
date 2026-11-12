//! 0x8c182bfc — copy stack byte [r15+1] to [r15], init struct via 0x8c1837dc, store saved r5 at [r13+16], return r13.
//!
//! ```text
//! mov.l r14,@-r15      ; push r14
//! mov.l r13,@-r15      ; push r13
//! mov   r5,r14         ; save value arg
//! sts.l pr,@-r15       ; push pr
//! add   #-4,r15        ; scratch word
//! mov   r4,r13         ; save object arg
//! mov   r15,r0
//! add   #1,r0
//! mov.b @r0,r0         ; r0 = sext(byte [r15+1])
//! mov   r15,r5
//! mov   r15,r6
//! mov.b r0,@r5         ; [r15] = that byte
//! mov.l 0x8c182c28,r0  ; r0 = 0x8c1837dc
//! jsr   @r0 ; add #2,r6 (delay: r6 = r15+2, r5 = r15, r4 passthrough)
//! mov.l r14,@(16,r13)  ; [obj+16] = saved value
//! mov   r13,r0         ; return obj
//! add   #4,r15
//! lds.l @r15+,pr
//! mov.l @r15+,r13
//! rts ; mov.l @r15+,r14 (delay)
//! ```
//!
//! The jsr return pr is 0x8c182c1a; the callee is inlined via the registry so
//! its stores to [r4..r4+12] and its clobbers (r0,r1,r3,r4,r6) match exactly.
//! No instruction here touches T, so it passes through unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c182bfc: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c182bfe: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c182c00: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c182c02: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c182c04: add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c182c06: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c182c08..0x8c182c0c: r0 = sext(byte [r15+1])
    cpu.r[0] = cpu.r[15];
    cpu.r[0] = cpu.r[0].wrapping_add(1);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
    // 0x8c182c0e..0x8c182c12: r5 = r15; r6 = r15; [r15] = byte
    cpu.r[5] = cpu.r[15];
    cpu.r[6] = cpu.r[15];
    let _ = mem.write_u8(cpu.r[5], cpu.r[0] as u8);
    // 0x8c182c14: mov.l 0x8c182c28,r0
    cpu.r[0] = mem.read_u32(0x8c182c28).unwrap_or(0x8c1837dc);
    // 0x8c182c16: jsr @r0 ; 0x8c182c18: add #2,r6 (delay slot)
    cpu.r[6] = cpu.r[6].wrapping_add(2);
    cpu.pr = 0x8c182c1a;
    crate::registry::p_8c1837dc::run(cpu, mem);
    // 0x8c182c1a: mov.l r14,@(16,r13)
    let _ = mem.write_u32(cpu.r[13].wrapping_add(16), cpu.r[14]);
    // 0x8c182c1c: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c182c1e: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c182c20: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c182c22: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c182c24: rts ; 0x8c182c26: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
