//! 0x8c018edc — null-checked "free if the argument looks positive" wrapper.
//!
//! ```text
//! mov.l r14,@-r15
//! sts.l pr,@-r15
//! mov   r4,r14
//! tst   r14,r14
//! bt    0x8c018ef8        ; null: return 0, the global stays untouched
//! mov.l 0x8c018f00,r3     ; r3 = 0x8c23da88 (flag word, in RAM past the image)
//! mov   #0,r1
//! exts.w r5,r0            ; only the low 16 bits of r5 count
//! cmp/pl r0               ; r0 > 0 ?
//! bf.s  0x8c018ef8
//! mov.l r1,@r3            ; delay slot (always): flag = 0
//! mov.l 0x8c018f04,r0     ; 0x8c01032c: free thunk -> 0x8c159fc0
//! jsr   @r0
//! nop
//! 0x8c018ef8: mov r14,r0
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14
//! ```
//!
//! The `bf.s` delay slot runs on both outcomes, so every non-null call clears
//! the flag even when the size test fails. T ends as whatever the callee left
//! (free's last compare) on the calling path, and as `cmp/pl`'s result on the
//! non-calling path. r4 is passed to free unchanged; the return value is the
//! (non-null) argument.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[3] = mem.read_u32(0x8c018f00).unwrap_or(0x8c23da88); // mov.l 0x8c018f00,r3
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[0] = ((cpu.r[5] as u16 as i16) as i32) as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // delay slot of `bf.s`
        if (cpu.r[0] as i32) > 0 {
            cpu.pr = 0x8c018ef8; // jsr return address; delay slot is a nop
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
