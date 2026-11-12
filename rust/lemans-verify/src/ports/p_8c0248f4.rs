//! 0x8c0248f4 — initialize a transform record only if it is still unset.
//!
//! `if (record->flag == 0) 0x8c0b48e8(record);` — the callee is the one-shot
//! initializer for the 0x54-byte record at `r4`, so callers can refresh a
//! record without rebuilding an already-initialized one. `mov.b @r4,r0`
//! sign-extends the flag byte and the two `extu.b`s reduce it to the
//! zero-extended byte; `tst` then sets T for the `bf` that skips the call when
//! the flag is non-zero (`bf` has no delay slot). The `jsr @r0` reads its
//! target from the literal pool at 0x8c02490c (0x8c0b48e8) and its `nop` delay
//! slot runs before the callee, which returns to 0x8c024906. The epilogue
//! reloads the `pr` pushed at entry, so `pr`/`r15` end at their seed values;
//! the only writes are the stack push and whatever the callee stores.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0248f4 sts.l pr,@-r15.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 8c0248f6..8c0248fe — r0 = (u8)*r4 (mov.b sign-extends, then two
    // extu.b clear it); T = (r0 == 0); bf 0x8c024906 when already initialized.
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] == 0 {
        // 8c024900 mov.l 0x8c02490c,r0 ; 8c024902 jsr @r0 ; 8c024904 nop.
        cpu.r[0] = mem.read_u32(0x8c02490c).unwrap_or(0);
        cpu.pr = 0x8c024906;
        crate::registry::p_8c0b48e8::run(cpu, mem);
    }

    // 8c024906 lds.l @r15+,pr ; 8c024908 rts ; 8c02490a nop (delay slot).
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
