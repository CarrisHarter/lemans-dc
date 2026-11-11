//! 0x8c0ffac8 — find byte (char)r5 in base[r6..count); return offset or -1.
//!
//! ```text
//! mov.l @r4,r0 ; mov.l @r0,r1     ; r1 = header->count
//! cmp/hs r1,r6 ; bf 0x8c0ffad4    ; index >= count -> ...
//! rts ; mov #-1,r0                ; delay slot: return -1
//! 0x8c0ffad4: mov.l @(12,r0),r7   ; r7 = base
//! mov r7,r0 ; mov r7,r4 ; add r6,r0 ; add r1,r4  ; r0 = base+idx, r4 = base+count
//! cmp/eq r4,r0 ; bt 0x8c0ffaf6    ; idx == count -> rts -1
//! exts.b r5,r1                    ; needle, sign-extended, computed ONCE
//! 0x8c0ffae4: mov.b @r0,r3 ; exts.b r3,r3 ; cmp/eq r1,r3
//! bf 0x8c0ffaf0                   ; not equal -> advance (no delay slot)
//! rts ; sub r7,r0                 ; delay slot: return r0 - base
//! 0x8c0ffaf0: add #1,r0 ; cmp/eq r4,r0 ; bf 0x8c0ffae4
//! 0x8c0ffaf6: rts ; mov #-1,r0     ; delay slot: return -1
//! ```
//! Hidden side effects: r4 = base+count, r7 = base, r3 = last byte scanned
//! (sign-extended), r1 = sign-extended needle, T = 1 on both -1 paths and on
//! a hit, 0 never survives (every exit goes through a true cmp or cmp/hs).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap(); // mov.l @r0,r1 (count)
    cpu.set_t(cpu.r[6] >= cpu.r[1]); // cmp/hs r1,r6
    if cpu.r[6] >= cpu.r[1] {
        cpu.r[0] = 0xFFFF_FFFF; // rts delay slot: mov #-1,r0
        return;
    }
    cpu.r[7] = mem.read_u32(cpu.r[0] + 12).unwrap(); // mov.l @(12,r0),r7
    cpu.r[0] = cpu.r[7];
    cpu.r[4] = cpu.r[7];
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[6]); // add r6,r0
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // add r1,r4
    cpu.set_t(cpu.r[4] == cpu.r[0]); // cmp/eq r4,r0
    if cpu.r[4] == cpu.r[0] {
        cpu.r[0] = 0xFFFF_FFFF; // bt -> rts delay slot: mov #-1,r0
        return;
    }
    cpu.r[1] = cpu.r[5] as u8 as i8 as i32 as u32; // exts.b r5,r1
    loop {
        cpu.r[3] = mem.read_u8(cpu.r[0]).unwrap() as i8 as i32 as u32; // mov.b @r0,r3
        cpu.r[3] = cpu.r[3] as u8 as i8 as i32 as u32; // exts.b r3,r3
        cpu.set_t(cpu.r[1] == cpu.r[3]); // cmp/eq r1,r3
        if cpu.r[1] == cpu.r[3] {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[7]); // rts delay slot: sub r7,r0
            return;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(1); // add #1,r0
        cpu.set_t(cpu.r[4] == cpu.r[0]); // cmp/eq r4,r0
        if cpu.r[4] == cpu.r[0] {
            cpu.r[0] = 0xFFFF_FFFF; // fall through to rts at 0x8c0ffaf6, delay mov #-1,r0
            return;
        }
    }
}
