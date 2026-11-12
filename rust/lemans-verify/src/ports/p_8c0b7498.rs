//! 0x8c0b7498 — initialise a buffer-span object at `r4`; returns r4.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r6,r14
//! mov.l r12,@-r15 ; mov r4,r13 ; sts.l pr,@-r15 ; mov r5,r12
//! mov.l 0x8c0b74cc,r0 (0x8c0b834c) ; jsr @r0 ; nop
//! r0=[0x8c0b74d0] (0x8c1eae14) ; mov.l r0,@r13
//! r1=[0x8c0b74d4] (0x8c1ead34) ; r0=r13
//! mov.l r12,@(8,r13) ; r3=[r13+8] ; mov.l r3,@(12,r13)
//! r3=[r13+8] ; r3+=r14 ; mov.l r3,@(16,r13)
//! mov.l r1,@r13 ; lds.l @r15+,pr ; mov.l @r15+,r12
//! mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 (delay slot)
//! ```
//!
//! The callee 0x8c0b834c stores three vtable words at `[r4]` and sets the byte
//! at `[r4+4]` to 1; only that byte survives, because this function then writes
//! `[r4]` twice more itself (0x8c1eae14, then 0x8c1ead34 last). Layout: `+8`
//! start, `+12` cursor, `+16` end = start + r6. r5 is stored three times but is
//! never dereferenced here.
//!
//! Hidden side effects: four stack pushes/pops (outside the compared write
//! log), both ordered dword stores at `[r4]`, and `r1 = 0x8c1ead34`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0b7498: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0b749a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0b749c: mov r6,r14
    cpu.r[14] = cpu.r[6];
    // 0x8c0b749e: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0b74a0: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0b74a2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0b74a4: mov r5,r12
    cpu.r[12] = cpu.r[5];
    // 0x8c0b74a6: mov.l 0x8c0b74cc,r0
    cpu.r[0] = mem.read_u32(0x8C0B_74CC).unwrap_or(0); // 0x8c0b834c
    // 0x8c0b74a8: jsr @r0 ; 0x8c0b74aa: nop (delay slot)
    cpu.pr = 0x8C0B_74AC;
    crate::registry::p_8c0b834c::run(cpu, mem);
    // 0x8c0b74ac: mov.l 0x8c0b74d0,r0
    cpu.r[0] = mem.read_u32(0x8C0B_74D0).unwrap_or(0); // 0x8c1eae14
    // 0x8c0b74ae: mov.l r0,@r13
    let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
    // 0x8c0b74b0: mov.l 0x8c0b74d4,r1
    cpu.r[1] = mem.read_u32(0x8C0B_74D4).unwrap_or(0); // 0x8c1ead34
    // 0x8c0b74b2: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0b74b4: mov.l r12,@(8,r13)
    let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[12]);
    // 0x8c0b74b6: mov.l @(8,r13),r3
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
    // 0x8c0b74b8: mov.l r3,@(12,r13)
    let _ = mem.write_u32(cpu.r[13].wrapping_add(12), cpu.r[3]);
    // 0x8c0b74ba: mov.l @(8,r13),r3
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
    // 0x8c0b74bc: add r14,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[14]);
    // 0x8c0b74be: mov.l r3,@(16,r13)
    let _ = mem.write_u32(cpu.r[13].wrapping_add(16), cpu.r[3]);
    // 0x8c0b74c0: mov.l r1,@r13
    let _ = mem.write_u32(cpu.r[13], cpu.r[1]);
    // 0x8c0b74c2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b74c4: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b74c6: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b74c8: rts ; 0x8c0b74ca: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // r0 = r13 from 0x8c0b74b2: the return value is the object pointer.
}
