//! 0x8c239a46 — set or clear bit 3 of the flags word of a per-id object.
//!
//! `r4` (id/param_1) is first tested by 0x8c237780, the fixed-stride table
//! search: a miss returns 0xeeee without touching any object. That search
//! compares `r4` against the *address* of each record (`base+24+idx*0x8c`),
//! not against the word stored there, so a real id is a record address such
//! as 0x8c2c9f2c. On a hit the object pointer lives at `[param_1 + 0x88]`
//! (the field four bytes before the next record) and the object's `+4` flags
//! word gets bit 3 set when `r5` (param_2) is non-zero and cleared when it is
//! zero. Both hit paths return 0.
//!
//! The callee is invoked through its registered port with `pr` set to the
//! instruction after the `jsr` delay slot (0x8c239a54) and the delay slot's
//! `r4 = [r15]` applied first. The two stack arguments are the function's own
//! two-word scratch area, saved after the 8-byte frame is opened.
//!
//! Exit state beyond `r0`: on the miss path T is 1 (the `tst r0,r0` that
//! selects the branch) and the callee's leftovers survive in r1..r7; on the
//! hit path T is `param_2 == 0` (the last flag-setting `tst r3,r3`), and the
//! set path leaves r2 exactly as the callee left it while the clear path
//! overwrites r2 with the masked word and r3 with the -9 mask. `r15` and `pr`
//! return to their entry values.

use sh4_oracle::{Cpu, Memory};

fn call(cpu: &mut Cpu, mem: &mut Memory, target: u32, from: u32) {
    match crate::registry::lookup(target) {
        Some(port) => port(cpu, mem),
        None => panic!("0x{from:08x}: call to unported target 0x{target:08x}"),
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c239a46: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c239a48: add #-8,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // 8c239a4a: mov.l 0x8c239a80,r3
    cpu.r[3] = mem.read_u32(0x8c239a80).unwrap_or(0x8c237780);
    // 8c239a4c: mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    // 8c239a4e: mov.l r5,@(4,r15)
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    // 8c239a50: jsr @r3 ; delay 8c239a52: mov.l @r15,r4
    let target = cpu.r[3];
    cpu.pr = 0x8c239a54;
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    call(cpu, mem, target, 0x8c239a50);
    // 8c239a54: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 8c239a56: bf 0x8c239aa0
    if cpu.r[0] != 0 {
        // 8c239aa0: mov.w 0x8c239ae0,r0
        cpu.r[0] = mem.read_u16(0x8c239ae0).unwrap_or(0x0088) as i16 as i32 as u32;
        // 8c239aa2: mov.l @r15,r4
        cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        // 8c239aa4: mov.l @(4,r15),r3
        cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
        // 8c239aa6: mov.l @(r0,r4),r4
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        // 8c239aa8: tst r3,r3
        cpu.set_t(cpu.r[3] == 0);
        // 8c239aaa: bt 0x8c239ab4
        if cpu.r[3] != 0 {
            // 8c239aac: mov.l @(4,r4),r0
            cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            // 8c239aae: or #8,r0
            cpu.r[0] |= 8;
            // 8c239ab0: bra 0x8c239abc ; delay 8c239ab2: mov.l r0,@(4,r4)
            let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
            // 8c239abc: mov #0,r0
            cpu.r[0] = 0;
        } else {
            // 8c239ab4: mov.l @(4,r4),r2
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            // 8c239ab6: mov #-9,r3
            cpu.r[3] = (-9i32) as u32;
            // 8c239ab8: and r3,r2
            cpu.r[2] &= cpu.r[3];
            // 8c239aba: mov.l r2,@(4,r4)
            let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[2]);
            // 8c239abc: mov #0,r0
            cpu.r[0] = 0;
        }
        // 8c239abe: add #8,r15
        cpu.r[15] = cpu.r[15].wrapping_add(8);
        // 8c239ac0: lds.l @r15+,pr
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    // 8c239a58: mov.l 0x8c239a84,r0
    cpu.r[0] = mem.read_u32(0x8c239a84).unwrap_or(0x0000eeee);
    // 8c239a5a: add #8,r15
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    // 8c239a5c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
