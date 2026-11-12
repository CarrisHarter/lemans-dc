//! 0x8c018484 — TAS spinlock acquire: `tas.b @r4` with a `bf` back-edge.
//!
//! ```text
//! 0x8c018484  tas.b @r4        ; T = (*r4 == 0); *r4 |= 0x80
//! 0x8c018486  bf   0x8c018474 ; taken while T == 0, i.e. while the byte was nonzero
//! 0x8c018488  rts
//! 0x8c01848a  nop             ; rts delay slot
//! ```
//!
//! The target of the back-edge is the eight-`nop` pad at 0x8c018474, which the
//! oracle executes before re-testing. Every access is **one byte**: the load is
//! a single byte and the store writes `old | 0x80` to that same byte, so the
//! three neighbouring bytes are untouched. T ends set on the exit path — the
//! terminating read was zero — and no GPR, FPR or other system register moves.
//!
//! The loop-back path is reproduced faithfully (it is the only other control
//! path) but cannot halt under the single-threaded oracle: the loop is its own
//! only writer and it always leaves bit 7 set, so a byte that read nonzero on
//! entry keeps reading nonzero forever. Any spec that halts therefore takes
//! exactly one pass, which is what the hand specs seed.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let addr = cpu.r[4];
    loop {
        // tas.b @r4: byte load, T = (loaded byte == 0), then set bit 7.
        let old = mem.read_u8(addr).unwrap_or(0);
        cpu.set_t(old == 0);
        let _ = mem.write_u8(addr, old | 0x80); // byte store, low 8 bits only
        // bf 0x8c018474: taken when the byte read as nonzero -> nop pad, retry.
        if cpu.t() {
            break;
        }
    }
    // rts ; nop — no register or memory state left to restore.
}
