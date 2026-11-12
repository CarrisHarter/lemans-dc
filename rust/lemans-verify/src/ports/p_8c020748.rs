//! 0x8c020748 — free a block-count descriptor at `r4`.
//!
//! ```text
//! sts.l pr,@-r15
//! r5 = [r4+8] ; tst r5,r5 ; bt out          ; null: nothing to free
//! r0 = [r4+4] ; r1 = r5 ; r0 <<= 4          ; shll2 twice, T untouched
//! r1 += r0 ; cmp/hi r5,r1 ; bf.s out        ; delay slot: r3 = r5
//! loop: r3 += 16 ; cmp/hi r3,r1 ; bt loop
//! out: r0 = 0x8c01032c ; jsr @r0            ; delay slot: r4 = r5
//! lds.l @r15+,pr ; rts ; nop
//! ```
//!
//! The loop's only product is `r3`, and the callee immediately overwrites it,
//! so the call result is entirely the callee's (free at 0x8c159fc0 through the
//! thunk at 0x8c01032c). T at exit is 1 on the null path (from `tst`) and the
//! callee's last comparison (which clears it) otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c020748: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c02074a: mov.l @(8,r4),r5
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // 0x8c02074c: tst r5,r5 ; 0x8c02074e: bt 0x8c02076c
    cpu.set_t(cpu.r[5] == 0);
    if !cpu.t() {
        // 0x8c020750: mov.l @(4,r4),r0
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
        // 0x8c020752: mov r5,r1
        cpu.r[1] = cpu.r[5];
        // 0x8c020754: shll2 r0 ; 0x8c020756: shll2 r0
        cpu.r[0] = cpu.r[0] << 2;
        cpu.r[0] = cpu.r[0] << 2;
        // 0x8c020758: add r0,r1
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        // 0x8c02075a: cmp/hi r5,r1
        cpu.set_t(cpu.r[1] > cpu.r[5]);
        // 0x8c02075c: bf.s 0x8c020766 ; 0x8c02075e: mov r5,r3 (delay slot)
        let mut r3 = cpu.r[5];
        if cpu.t() {
            loop {
                // 0x8c020760: add #16,r3
                r3 = r3.wrapping_add(16);
                // 0x8c020762: cmp/hi r3,r1 ; 0x8c020764: bt 0x8c020760
                cpu.set_t(cpu.r[1] > r3);
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[3] = r3;
        // 0x8c020766: mov.l 0x8c020774,r0 (0x8c01032c)
        cpu.r[0] = mem.read_u32(0x8c020774).unwrap_or(0x8c01032c);
        // 0x8c020768: jsr @r0 ; 0x8c02076a: mov r5,r4 (delay slot)
        cpu.pr = 0x8c02076c;
        cpu.r[4] = cpu.r[5];
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c02076c: lds.l @r15+,pr ; 0x8c02076e: rts ; 0x8c020770: nop
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
