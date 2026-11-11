//! 0x8c057ec4 — for each of three floats f32[r5+0/4/8], store it into the first
//! of two slots (hi at r4+12/16/20, then lo at r4+0/4/8) whose current value
//! compares against it; r5 += 8 always, r4 += 8 on the last block's else path.
//!
//! ```text
//! per block: r0 = hi slot; fr1 = f32[r5+off]; fr0 = f32[hi]
//!   fcmp/gt fr0,fr1 (T = fr1 > fr0); bf else
//!   bra end ; fmov fr1,@r0        ; hi slot gets fr1
//! else: fr0 = f32[lo]; fcmp/gt fr1,fr0 (T = fr0 > fr1); bf end
//!   fmov fr1,@lo                  ; lo slot gets fr1 when lo > fr1
//! ```
//!
//! Hidden side effects the C does not show: r0/r1 leftovers (r1 = r5+4,
//! r0 = r4+20 finally), r5 is incremented by 8 even on the early rts, r4 is
//! incremented by 8 on the last block's else path *after* r0 = r4+20 was
//! computed from the old r4, fr0/fr1 keep the last block's pair, T is the last
//! fcmp/gt, and the stores go to whichever slot the comparisons picked.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Block 1: fr1 = f32[r5+0], hi = r4+12, lo = r4+0
    cpu.r[0] = cpu.r[4].wrapping_add(12);
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
    if cpu.t() {
        let _ = mem.write_f32(cpu.r[0], cpu.fr[1]); // bra delay: fmov fr1,@r0
    } else {
        cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap(); // fmov @r4,fr0
        cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
        if cpu.t() {
            let _ = mem.write_f32(cpu.r[4], cpu.fr[1]); // fmov fr1,@r4
        }
    }

    // Block 2: fr1 = f32[r5+4], hi = r4+16, lo = r4+4
    cpu.r[1] = cpu.r[5].wrapping_add(4);
    cpu.r[0] = cpu.r[4].wrapping_add(16);
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
    if cpu.t() {
        let _ = mem.write_f32(cpu.r[0], cpu.fr[1]); // bra delay: fmov fr1,@r0
    } else {
        cpu.r[0] = cpu.r[4].wrapping_add(4);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
        if cpu.t() {
            let _ = mem.write_f32(cpu.r[0], cpu.fr[1]); // fmov fr1,@r0
        }
    }

    // Block 3: r5 += 8; fr1 = f32[r5], hi = r4+20, lo = r4+8 (after r4 += 8)
    cpu.r[0] = cpu.r[4];
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    cpu.r[0] = cpu.r[0].wrapping_add(20);
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
    if cpu.t() {
        let _ = mem.write_f32(cpu.r[0], cpu.fr[1]); // rts delay: fmov fr1,@r0
        return;
    }
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
    if cpu.t() {
        let _ = mem.write_f32(cpu.r[4], cpu.fr[1]); // fmov fr1,@r4
    }
    // rts ; nop
}
