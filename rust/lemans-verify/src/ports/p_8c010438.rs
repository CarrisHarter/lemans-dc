//! 0x8c010438 — `*DAT_8c01052c = (param_1 == 0) ? PTR_PTR_8c010530 : param_1; return old;`
//!
//! ```text
//! mov.l 0x8c01052c,r5   ; r5 = slot pointer
//! tst   r4,r4           ; T = (r4 == 0)
//! bf.s  0x8c010446
//! mov.l @r5,r6          ; delay slot: old value, BOTH paths
//! mov.l 0x8c010530,r2   ; (r4 == 0 path) r2 = default pointer
//! bra   0x8c010448
//! mov.l r2,@r5          ; delay slot: store default
//! mov.l r4,@r5          ; (r4 != 0 path) store r4
//! rts
//! mov   r6,r0           ; delay slot: return old value
//! ```
//! Hidden side effects: r5/r6/r2 leftovers, the store through the pointer,
//! the delay-slot load that runs even when the branch is not taken, and a
//! T bit that differs per path (0 when r4 != 0, 1 when r4 == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let slot = mem.read_u32(0x8C01052C).unwrap();
    cpu.r[5] = slot;
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    cpu.r[6] = mem.read_u32(slot).unwrap(); // delay slot of `bf.s` runs on both paths
    if cpu.r[4] == 0 {
        let default = mem.read_u32(0x8C010530).unwrap();
        cpu.r[2] = default;
        let _ = mem.write_u32(slot, default); // delay slot of `bra`
    } else {
        let _ = mem.write_u32(slot, cpu.r[4]);
    }
    cpu.r[0] = cpu.r[6]; // delay slot of `rts`
}
