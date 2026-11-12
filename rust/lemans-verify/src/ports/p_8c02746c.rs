//! 0x8c02746c — populate four parallel arrays from two lookups and the
//! caller's remaining arguments.
//!
//! ```text
//! r14 = idx (param_1), r12 = param_2, r13 = param_4
//! r1 = FUN_8c0bfb4c(param_2, param_3)                 ; tree lookup
//! [0x178 + idx] = r1
//! r1 = FUN_8c0bfb4c(param_2, param_4)                 ; same tree, other key
//! [0x174 + idx] = r1
//! [0x170 + idx] = (u8)param_6                          ; one byte
//! [0x168 + idx] = param_5
//! ```
//!
//! The two calls dispatch through the literal at 0x8c0274b4 (0x8c0bfb4c);
//! the delay slots reload r4 from r12 (the shared base) and r5 from r6/r13
//! (the two keys). param_5 sits at entry r15, param_6 in the following word
//! (the callee's pushes move r15 down by 16). The `mov.b` load sign-extends
//! and the following `extu.b` keeps the low byte, so the stored byte is
//! param_6 & 0xff.
//!
//! Exit state: r0 = 0x168, r1 = param_5, r3 = 0x170, r14/r13/r12/pr restored,
//! and r6/r7 hold whatever the ported callee left in them.

use sh4_oracle::{Cpu, Memory};

fn call(cpu: &mut Cpu, mem: &mut Memory, target: u32, from: u32) {
    match crate::registry::lookup(target) {
        Some(port) => port(cpu, mem),
        None => panic!("0x{from:08x}: call to unported target 0x{target:08x}"),
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c02746c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 8c02746e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 8c027470: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 8c027472: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 8c027474: mov r7,r13
    cpu.r[13] = cpu.r[7];
    // 8c027476: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c027478: mov r5,r12
    cpu.r[12] = cpu.r[5];
    // 8c02747a: mov.l 0x8c0274b4,r0
    cpu.r[0] = mem.read_u32(0x8c0274b4).unwrap_or(0);
    // 8c02747c: mov r6,r5
    cpu.r[5] = cpu.r[6];
    // 8c02747e: jsr @r0 ; delay 8c027480: mov r12,r4
    let target = cpu.r[0];
    cpu.pr = 0x8c027482;
    cpu.r[4] = cpu.r[12];
    call(cpu, mem, target, 0x8c02747e);
    // 8c027482: mov r0,r1
    cpu.r[1] = cpu.r[0];
    // 8c027484: mov.l 0x8c0274b8,r0
    cpu.r[0] = mem.read_u32(0x8c0274b8).unwrap_or(0);
    // 8c027486: mov r13,r5
    cpu.r[5] = cpu.r[13];
    // 8c027488: mov.l r1,@(r0,r14)
    let addr = cpu.r[0].wrapping_add(cpu.r[14]);
    let _ = mem.write_u32(addr, cpu.r[1]);
    // 8c02748a: mov.l 0x8c0274b4,r0
    cpu.r[0] = mem.read_u32(0x8c0274b4).unwrap_or(0);
    // 8c02748c: jsr @r0 ; delay 8c02748e: mov r12,r4
    let target = cpu.r[0];
    cpu.pr = 0x8c027490;
    cpu.r[4] = cpu.r[12];
    call(cpu, mem, target, 0x8c02748c);
    // 8c027490: mov r0,r1
    cpu.r[1] = cpu.r[0];
    // 8c027492: mov.l 0x8c0274bc,r0
    cpu.r[0] = mem.read_u32(0x8c0274bc).unwrap_or(0);
    // 8c027494: mov.l r1,@(r0,r14)
    let addr = cpu.r[0].wrapping_add(cpu.r[14]);
    let _ = mem.write_u32(addr, cpu.r[1]);
    // 8c027496: mov #20,r0
    cpu.r[0] = 20;
    // 8c027498: mov.b @(r0,r15),r0
    let addr = cpu.r[0].wrapping_add(cpu.r[15]);
    cpu.r[0] = mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32;
    // 8c02749a: mov.l 0x8c0274c0,r3
    cpu.r[3] = mem.read_u32(0x8c0274c0).unwrap_or(0);
    // 8c02749c: mov r0,r1
    cpu.r[1] = cpu.r[0];
    // 8c02749e: extu.b r1,r1
    cpu.r[1] &= 0xff;
    // 8c0274a0: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 8c0274a2: mov.b r1,@(r0,r14)
    let addr = cpu.r[0].wrapping_add(cpu.r[14]);
    let _ = mem.write_u8(addr, cpu.r[1] as u8);
    // 8c0274a4: mov.l @(16,r15),r1
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(16)).unwrap_or(0);
    // 8c0274a6: mov.l 0x8c0274c4,r0
    cpu.r[0] = mem.read_u32(0x8c0274c4).unwrap_or(0);
    // 8c0274a8: mov.l r1,@(r0,r14)
    let addr = cpu.r[0].wrapping_add(cpu.r[14]);
    let _ = mem.write_u32(addr, cpu.r[1]);
    // 8c0274aa: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c0274ac: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c0274ae: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c0274b0: rts ; delay 8c0274b2: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
