//! 0x8c173084 — range-checked 3-byte copy, else the bounded strcpy dispatch.
//!
//! ```text
//! push r14,pr; r15-=8; tst r4,r4; bf L9a (plain: no delay slot)
//! r2=[0x8c173148] (0x8c1725fc); r5=[0x8c173144] (str @0x8c1df544)
//! jsr @r2; delay: r4=101; bra Lca; delay nop
//! L9a: r2=[0x8c17314c] (0x8c2b5264); r14=r4; r3=[r2] (lo)
//! cmp/hs r3,r14 (T = r14>=lo unsigned); [r15+4]=r3; bf Lb6 (plain)
//! r0=[0x8c173150] (0x8c2b5268); r1=[r15+4] (lo); r3=[r0] (len); r1+=r3 (hi)
//! cmp/hs r1,r14 (T = r14>=hi unsigned); bt Lb6 (plain); bra Lb8; delay r2=1
//! Lb6: r2=0
//! Lb8: r1=r2; tst r1,r1; bt.s Ld4; delay: [r15]=r2
//! r3=[0x8c173148] (0x8c1725fc); r4=[0x8c173142].w sign-extended (0x1fb)
//! r5=[0x8c173154] (str @0x8c1df570); jsr @r3; delay nop
//! Lca: r0=0; r15+=8; pop pr; rts; delay pop r14
//! Ld4: r4+=32; [r5]=[r4]; [r6]=[r4+1]; [r7]=[r4+2]; r0=1
//! r15+=8; pop pr; rts; delay pop r14
//! ```
//!
//! The `bf` tests the `tst r4,r4` result: r4==0 falls through to
//! `0x8c1725fc(101, "acSystemGetDriverRevision - driver is NULL")` and returns
//! 0; r4!=0 takes the branch to the range probe. Both comparisons are unsigned:
//! r14 < lo clears T at the first `bf` (r2=0), r14 >= lo+len sets T at the
//! `bt` (r2=0); only lo <= r14 < lo+len yields r2=1. The `bt.s` tests
//! `tst r1,r1`: r2==1 falls through to `0x8c1725fc(0x1fb, "…must be in SH")`
//! and returns 0; r2==0 branches to the copy path and returns 1. So the range
//! [lo, lo+len) is the *rejected* window (the AICA-memory address range the
//! revision pointer must avoid): in-range values emit the error, out-of-range
//! values take the copy.
//! The frame slots at (r15) and (r15+4) sit inside the stack window, so their
//! stores are comparison-excluded — but the reads must still see them, hence
//! the port writes them for real. r14 restores the caller's value on the jsr
//! paths (the delayed pop) while the entry r4 survives in the callee's r13;
//! the copy path clobbers r4 with r4+32 and leaves r14 intact via its own pop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // push r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.t() {
        // r4==0: 0x8c1725fc(101, str @0x8c1df544), then r0=0.
        cpu.r[2] = mem.read_u32(0x8c173148).unwrap_or(0x8c1725fc);
        cpu.r[5] = mem.read_u32(0x8c173144).unwrap_or(0x8c1df544);
        cpu.pr = 0x8c173096; // jsr @r2
        cpu.r[4] = 101; // delay slot: mov #101,r4
        crate::registry::p_8c1725fc::run(cpu, mem);
        // bra Lca; delay nop
        // Lca: shared jsr-path epilogue.
        cpu.r[0] = 0; // mov #0,r0
        cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // rts; delay pop r14
        cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r14
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    } else {
        // L9a: probe [0x8c2b5264] <= r4 < [0x8c2b5264]+[0x8c2b5268].
        cpu.r[2] = mem.read_u32(0x8c17314c).unwrap_or(0x8c2b5264);
        cpu.r[14] = cpu.r[4]; // mov r4,r14
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2,r3 (lo)
        cpu.set_t(cpu.r[14] >= cpu.r[3]); // cmp/hs r3,r14
        let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[3]); // mov.l r3,@(4,r15)
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c173150).unwrap_or(0x8c2b5268);
            cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r1
            cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r3 (len)
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // add r3,r1 (hi)
            cpu.set_t(cpu.r[14] >= cpu.r[1]); // cmp/hs r1,r14
            if cpu.t() {
                // Lb6 (bt taken: r14 >= hi).
                cpu.r[2] = 0; // mov #0,r2
            } else {
                // bra Lb8; delay r2=1
                cpu.r[2] = 1; // mov #1,r2
            }
        } else {
            // Lb6 (bf taken: r14 < lo).
            cpu.r[2] = 0; // mov #0,r2
        }
        // Lb8: r2==0 (out-of-range) takes bt.s to the copy path; r2==1
        // (in-range) falls through to the emit path.
        cpu.r[1] = cpu.r[2]; // mov r2,r1
        cpu.set_t(cpu.r[1] == 0); // tst r1,r1
        // bt.s Ld4 — the delay-slot store runs whether it branches or not.
        let to_copy = cpu.t();
        let _ = mem.write_u32(cpu.r[15], cpu.r[2]); // mov.l r2,@r15
        if to_copy {
            // Ld4: copy [r4+32..34] to [r5],[r6],[r7]; r0=1.
            cpu.r[4] = cpu.r[4].wrapping_add(32); // add #32,r4
            cpu.r[2] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r4,r2
            let _ = mem.write_u8(cpu.r[5], cpu.r[2] as u8); // mov.b r2,@r5
            cpu.r[0] = mem
                .read_u8(cpu.r[4].wrapping_add(1))
                .unwrap_or(0) as i8 as i32 as u32; // mov.b @(1,r4),r0
            let _ = mem.write_u8(cpu.r[6], cpu.r[0] as u8); // mov.b r0,@r6
            cpu.r[0] = mem
                .read_u8(cpu.r[4].wrapping_add(2))
                .unwrap_or(0) as i8 as i32 as u32; // mov.b @(2,r4),r0
            let _ = mem.write_u8(cpu.r[7], cpu.r[0] as u8); // mov.b r0,@r7
            cpu.r[0] = 1; // mov #1,r0
            cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            // rts; delay pop r14
            cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r14
            cpu.r[15] = cpu.r[15].wrapping_add(4);
        } else {
            // Emit path: 0x8c1725fc(0x1fb, str @0x8c1df570), then r0=0.
            cpu.r[3] = mem.read_u32(0x8c173148).unwrap_or(0x8c1725fc);
            cpu.r[4] = mem.read_u16(0x8c173142).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[5] = mem.read_u32(0x8c173154).unwrap_or(0x8c1df570);
            cpu.pr = 0x8c1730ca; // jsr @r3
            // delay slot: nop
            crate::registry::p_8c1725fc::run(cpu, mem);
            // Lca: shared jsr-path epilogue.
            cpu.r[0] = 0; // mov #0,r0
            cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            // rts; delay pop r14
            cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r14
            cpu.r[15] = cpu.r[15].wrapping_add(4);
        }
    }
}
