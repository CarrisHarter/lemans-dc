//! 0x8c0753fc — compact a 4-slot index list at param1, then pad with 5s.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15
//! mov #0,r3 ; mov.l r3,@r4            ; *param = 0
//! r0 = *(param+0xe8) ; tst r0,r0 ; bf fill
//! loop (4 iterations max):
//!   r13 = *(param+0x10+i*4)
//!   if *(u8*)(r13+0x10) == 0 → next
//!   if *(u8*)(r13+0x18) == 0 → break to fill
//!   *(param+0xf8+count*4) = i ; count++ ; (*param)++
//! fill: while count < 4: *(param+0xf8+count*4) = 5 ; count++
//! rts (delay: pop r14), pop r13
//! ```
//! Hidden side effects the C does not show: r4 left at param+0xf8 (unless the
//! fill loop was skipped, then param), r3 = final count, r5 = r7 = 4,
//! r6 = 4×iterations (delay slot of `bt.s` runs on the last pass too),
//! r13 = last element pointer (restored from stack, so NOT observable),
//! r1 = 5, r0 = last fill index×4, T from the last `cmp/hi`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let saved_r13 = cpu.r[13]; // pushed, popped at the end: not observable
    let saved_r14 = cpu.r[14];
    cpu.r[3] = 0; // mov #0,r3
    mem.write_u32(base, 0).unwrap(); // mov.l r3,@r4
    let gate_off = mem.read_u32(0x8C075470).unwrap(); // 0xe8
    let gate = mem.read_u32(base.wrapping_add(gate_off)).unwrap();
    cpu.set_t(gate == 0); // tst r0,r0
    if gate == 0 {
        // bf 0x8c075450 is taken when T=0 (gate != 0): the loop only runs when gate == 0
        cpu.r[5] = 0; // mov r3,r5 — loop counter i
        cpu.r[6] = 0; // mov r3,r6 — byte offset
        let g = mem.read_u32(0x8C075474).unwrap(); // 0xf8
        cpu.r[0] = g;
        cpu.r[1] = base.wrapping_add(0x10); // mov r4,r1 ; add #16,r1
        cpu.r[14] = base.wrapping_add(g); // mov r4,r14 ; add r0,r14
        cpu.r[7] = 4; // mov #4,r7
        loop {
            let r13 = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[6])).unwrap();
            cpu.r[13] = r13; // mov.l @(r0,r1),r13
            let b1 = mem.read_u8(r13.wrapping_add(0x10)).unwrap() as u32;
            cpu.r[0] = b1; // mov.b @r0,r0 after add #16
            cpu.set_t(b1 == 0); // tst r0,r0
            if b1 != 0 {
                let b2 = mem.read_u8(r13.wrapping_add(0x18)).unwrap() as u32;
                cpu.r[0] = b2;
                cpu.set_t(b2 == 0); // tst r0,r0
                if b2 == 0 {
                    break; // bt 0x8c075450 — skips the r5/r6 updates
                }
                cpu.r[0] = cpu.r[3] << 2; // mov r3,r0 ; shll2 r0
                mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[5]).unwrap();
                cpu.r[3] = cpu.r[3].wrapping_add(1); // add #1,r3
                cpu.r[0] = mem.read_u32(base).unwrap().wrapping_add(1);
                mem.write_u32(base, cpu.r[0]).unwrap(); // (*param)++
            }
            cpu.r[5] = cpu.r[5].wrapping_add(1); // add #1,r5
            let t = cpu.r[7] > cpu.r[5]; // cmp/gt r5,r7
            cpu.set_t(t);
            cpu.r[6] = cpu.r[6].wrapping_add(4); // bt.s delay slot: always runs
            if !t {
                break;
            }
        }
    }
    // fill loop, join point at 0x8c075450
    cpu.r[5] = 4; // mov #4,r5
    cpu.set_t(4u32 > cpu.r[3]); // cmp/hi r3,r5
    if cpu.r[3] < 4 {
        let g = mem.read_u32(0x8C075474).unwrap(); // 0xf8
        cpu.r[0] = g;
        cpu.r[1] = 5; // mov #5,r1
        cpu.r[4] = base.wrapping_add(g); // add r0,r4 — leftover if loop runs
        loop {
            cpu.r[0] = cpu.r[3] << 2; // mov r3,r0 ; shll2 r0
            cpu.r[3] = cpu.r[3].wrapping_add(1); // add #1,r3
            let t = 4u32 > cpu.r[3]; // cmp/hi r3,r5
            cpu.set_t(t);
            mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1]).unwrap(); // delay-slot store
            if !t {
                break;
            }
        }
    }
    cpu.r[13] = saved_r13; // mov.l @r15+,r13
    cpu.r[14] = saved_r14; // mov.l @r15+,r14 (rts delay slot)
}
