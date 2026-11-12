//! 0x8c239060 — advance a registered object's rolling offsets, or report
//! 0xeeee when the object is not in the allocator's table.
//!
//! ```text
//! hit = FUN_8c237780(p)                    ; table membership test
//! if (!hit) return 0xeeee
//! u    = arg3 + p[0x30] - 1                ; wrapping; 0xffff sign-extends
//! step = arg3 & ~31
//! if (arg2 == 0) {
//!     if (p[0x40] < u) step = -step        ; unsigned cmp/hi
//!     else { step = 0; ret = 0xeeee }
//! } else {
//!     if (p[0x28] <= u) { step = 0; ret = 0xeeee }   ; unsigned cmp/hs
//! }
//! p[0x30] += step
//! p[0x2c]  = p[0x30] - 1
//! return ret                               ; r13: 0xeeee or 0
//! ```
//!
//! The prologue pushes r14/r13/pr and a 12-byte frame holding the three
//! arguments; the literal at 0x8c2390c4 supplies the callee address. r4 is
//! param_1 on the early exit and param_1+24 afterwards, so the two stores land
//! at +0x30/+0x2c through it. The jmp-free exit restores r14 in the rts delay
//! slot. T at exit comes from the last tst/cmp the taken path executed.

use sh4_oracle::{Cpu, Memory};

fn call(cpu: &mut Cpu, mem: &mut Memory, target: u32, from: u32) {
    match crate::registry::lookup(target) {
        Some(port) => port(cpu, mem),
        None => panic!("0x{from:08x}: call to unported target 0x{target:08x}"),
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c239060: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 8c239062: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 8c239064: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c239066: add #-12,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    // 8c239068: mov #0,r14
    cpu.r[14] = 0;
    // 8c23906a: mov.l 0x8c2390c4,r3
    cpu.r[3] = mem.read_u32(0x8c2390c4).unwrap_or(0x8c237780);
    // 8c23906c: mov r14,r13
    cpu.r[13] = cpu.r[14];
    // 8c23906e: mov.l r4,@(4,r15)
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[4]);
    // 8c239070: mov.l r5,@(8,r15)
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[5]);
    // 8c239072: mov.l r6,@r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[6]);
    // 8c239074: jsr @r3 ; delay 8c239076: mov.l @(4,r15),r4
    let target = cpu.r[3];
    cpu.pr = 0x8c239078;
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    call(cpu, mem, target, 0x8c239074);
    // 8c239078: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 8c23907a: bf 0x8c239082
    if cpu.t() {
        // 8c23907c: mov.l 0x8c2390c8,r0
        cpu.r[0] = mem.read_u32(0x8c2390c8).unwrap_or(0x0000_eeee);
        // 8c23907e: bra 0x8c2390f2 ; delay 8c239080: nop
    } else {
        // 8c239082: mov.l @(4,r15),r4
        cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
        // 8c239084: mov #-32,r2
        cpu.r[2] = 0xffff_ffe0;
        // 8c239086: mov.l @r15,r5
        cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        // 8c239088: add #24,r4
        cpu.r[4] = cpu.r[4].wrapping_add(24);
        // 8c23908a: mov.l @(8,r15),r1
        cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
        // 8c23908c: mov.l @(24,r4),r6
        cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
        // 8c23908e: mov.w 0x8c2390c0,r3
        cpu.r[3] = mem.read_u16(0x8c2390c0).unwrap_or(0xffff) as u16 as i16 as i32 as u32;
        // 8c239090: tst r1,r1
        cpu.set_t(cpu.r[1] == 0);
        // 8c239092: mov.l @r15,r7
        cpu.r[7] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        // 8c239094: add r6,r5
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
        // 8c239096: add r3,r5
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[3]);
        // 8c239098: bt.s 0x8c2390ac ; delay 8c23909a: and r2,r7
        if cpu.t() {
            cpu.r[7] &= cpu.r[2];
            // 8c2390ac: mov.l @(40,r4),r1
            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0);
            // 8c2390ae: cmp/hi r1,r5
            cpu.set_t(cpu.r[5] > cpu.r[1]);
            // 8c2390b0: bf 0x8c2390e0
            if cpu.t() {
                // 8c2390b2: bra 0x8c2390e4 ; delay 8c2390b4: neg r7,r5
                cpu.r[5] = 0u32.wrapping_sub(cpu.r[7]);
            } else {
                // 8c2390e0: mov.l 0x8c239104,r13
                cpu.r[13] = mem.read_u32(0x8c239104).unwrap_or(0x0000_eeee);
                // 8c2390e2: mov r14,r5
                cpu.r[5] = cpu.r[14];
            }
        } else {
            cpu.r[7] &= cpu.r[2];
            // 8c23909c: mov.l @(16,r4),r1
            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
            // 8c23909e: cmp/hs r1,r5
            cpu.set_t(cpu.r[5] >= cpu.r[1]);
            // 8c2390a0: bt 0x8c2390a6
            if cpu.t() {
                // 8c2390a6: mov.l 0x8c2390c8,r13
                cpu.r[13] = mem.read_u32(0x8c2390c8).unwrap_or(0x0000_eeee);
                // 8c2390a8: bra 0x8c2390e4 ; delay 8c2390aa: mov r14,r5
                cpu.r[5] = cpu.r[14];
            } else {
                // 8c2390a2: bra 0x8c2390e4 ; delay 8c2390a4: mov r7,r5
                cpu.r[5] = cpu.r[7];
            }
        }
        // 8c2390e4: mov.w 0x8c239100,r3
        cpu.r[3] = mem.read_u16(0x8c239100).unwrap_or(0xffff) as u16 as i16 as i32 as u32;
        // 8c2390e6: add r5,r6
        cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[5]);
        // 8c2390e8: mov r6,r2
        cpu.r[2] = cpu.r[6];
        // 8c2390ea: mov r13,r0
        cpu.r[0] = cpu.r[13];
        // 8c2390ec: add r3,r2
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
        // 8c2390ee: mov.l r6,@(24,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[6]);
        // 8c2390f0: mov.l r2,@(20,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[2]);
    }
    // 8c2390f2: add #12,r15
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    // 8c2390f4: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c2390f6: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c2390f8: rts ; delay 8c2390fa: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
