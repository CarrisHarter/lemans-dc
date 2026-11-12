//! 0x8c161a98 — 3-element sort network over `[[r4]]`, `[[r5]]`, `[[r6]]`.
//!
//! `r11`/`r12`/`r13` hold the three slot pointers; each stage calls the
//! comparator at 0x8c10f2d4 (`r15+20`, `[slot]`, `[slot]`), which returns 1
//! when `load(r6) > load(r5)` (signed). The first compare swaps slots 0/1,
//! the second exits early when slots 1/2 are ordered (skipping the third
//! call), and the third call — whose `jsr` delay slot loads `r6=[slot0]` —
//! decides between a full rotation and placing the saved element at slot 1.
//! Callee-saved `r11`-`r14` and `pr` are restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c161a98..8c161aa6: push r14,r13,r12 ; r13=r6 ; push r11 ; r12=r5 ; push pr ; r11=r4
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[11] = cpu.r[4];
    // C1: 8c161aa8: r6=[r11] ; aa: r4=r15 ; ac: r5=[r12] ; ae: r0=[0x8c161b04]
    cpu.r[6] = mem.read_u32(cpu.r[11]).unwrap_or(0);
    cpu.r[4] = cpu.r[15];
    cpu.r[5] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c161b04).unwrap_or(0);
    // 8c161ab0: jsr @r0 ; ab2: r4+=20 (delay)
    cpu.pr = 0x8c161ab4;
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    crate::registry::p_8c10f2d4::run(cpu, mem);
    // 8c161ab4: extu.b r0,r0 ; ab6: tst r0,r0 ; ab8: bt noswap1
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // 8c161aba: r1=[r11] ; abc: r0=[r12] ; abe: [r11]=r0 ; ac0: [r12]=r1
        cpu.r[1] = mem.read_u32(cpu.r[11]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
        let _ = mem.write_u32(cpu.r[12], cpu.r[1]);
    }
    // C2: 8c161ac2: r6=[r12] ; ac4: r4=r15 ; ac6: r5=[r13] ; ac8: r0=[pool]
    cpu.r[6] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    cpu.r[4] = cpu.r[15];
    cpu.r[5] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c161b04).unwrap_or(0);
    // 8c161aca: jsr @r0 ; acc: r4+=20 (delay)
    cpu.pr = 0x8c161ace;
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    crate::registry::p_8c10f2d4::run(cpu, mem);
    // 8c161ace: extu.b r0,r0 ; ad0: tst r0,r0 ; ad2: bt done
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // 8c161ad4: r14=[r13] ; ad6: r4=r15 ; ad8: r1=[r12] ; ada: r4+=20
        cpu.r[14] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[4] = cpu.r[15];
        cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(20);
        // adc: r0=[pool] ; ade: r5=r14 ; ae0: [r13]=r1
        cpu.r[0] = mem.read_u32(0x8c161b04).unwrap_or(0);
        cpu.r[5] = cpu.r[14];
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]);
        // 8c161ae2: jsr @r0 ; ae4: r6=[r11] (delay)
        cpu.pr = 0x8c161ae6;
        cpu.r[6] = mem.read_u32(cpu.r[11]).unwrap_or(0);
        crate::registry::p_8c10f2d4::run(cpu, mem);
        // 8c161ae6: extu.b r0,r0 ; ae8: tst r0,r0 ; aea: bt place12
        cpu.r[0] &= 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            // aec: r0=[r11] ; aee: [r12]=r0 ; af0: bra done2 ; af2: [r11]=r14 (delay)
            cpu.r[0] = mem.read_u32(cpu.r[11]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[12], cpu.r[0]);
            let _ = mem.write_u32(cpu.r[11], cpu.r[14]);
        } else {
            // af4: [r12]=r14
            let _ = mem.write_u32(cpu.r[12], cpu.r[14]);
        }
    }
    // done: 8c161af6: pop pr,r11,r12,r13 ; afe: rts ; b00: pop r14 (delay)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
