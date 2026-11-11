//! 0x8c0e8c64 — expand a display-list record list into u32 entries + a byte each.
//!
//! ```text
//! CFG (delay slots inlined, they run on both paths of .s branches):
//!   prologue: push r14..r8, add #-4,r15; r10=[r15+32] (5th arg), r13=[r4+4],
//!             r1=0, r8=[r4], r3=0, T=(r5>0)
//!   bf.s EPI -> r5<=0 skips everything (delay: r4=r1=0)
//!   r4=0; r11=0; [r15]=0
//!   OUTER (0x8c0e8c88): tst r8,r8
//!     r8==0  -> r6=3
//!     r8==1  -> r6=extu16([r13+2*r3]); r3+=1
//!     else   -> r6 keeps its entry value
//!   L3 (0x8c0e8ca4): T=(r6>0); delay r14=[r15](==0)
//!     r6<=0 -> L4
//!     r6>0  -> r12=r11; INNER (0x8c0e8cac):
//!       r9=extu16([r13+2*(r3+r14)]); r14+=1; T=(r6>r14)
//!       [r10+r12]=r9; r12+=4; r11+=4; delay r1+=1; if T goto INNER
//!   L4 (0x8c0e8cc8): r9=0xfffffffe; r12=r6+r9; r0=r4; r4+=1;
//!       [r7+r0]=(u8)r12; T=(r5>r4); delay r3+=r6; if T goto OUTER
//!   EPI: add #4,r15; pop r8..r13; rts; delay pop r14
//! ```
//! Hidden side effects the C does not show: r14/r15 churn with the [r15] slot
//! (always 0), r0/r1/r3/r4/r6 leftovers, the per-record byte store at
//! [r7+record], the u32 stores at [r10+...], T=0 on exit on every path, and
//! the r6>0 delay-slot load that runs even when the inner loop is skipped.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14..r8,@-r15 ; add #-4,r15
    for reg in [14usize, 13, 12, 11, 10, 9, 8] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);

    cpu.r[10] = mem.read_u32(cpu.r[15].wrapping_add(32)).unwrap(); // mov.l @(32,r15),r10
    cpu.r[13] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap(); // mov.l @(4,r4),r13
    cpu.r[1] = 0; // mov #0,r1
    cpu.r[8] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r8
    cpu.r[3] = cpu.r[1]; // mov r1,r3
    cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
    if (cpu.r[5] as i32) <= 0 {
        cpu.r[4] = cpu.r[1]; // delay slot of bf.s: mov r1,r4
    } else {
        cpu.r[4] = cpu.r[1]; // mov r1,r4
        cpu.r[11] = cpu.r[1]; // mov r1,r11
        let _ = mem.write_u32(cpu.r[15], cpu.r[1]); // mov.l r1,@r15
        loop {
            // OUTER, 0x8c0e8c88
            cpu.set_t(cpu.r[8] & cpu.r[8] == 0); // tst r8,r8
            if cpu.r[8] == 0 {
                cpu.r[6] = 3; // mov #3,r6 (0x8c0e8ca2)
            } else {
                cpu.r[0] = cpu.r[8]; // mov r8,r0
                cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
                if cpu.r[0] == 1 {
                    // 0x8c0e8c96
                    cpu.r[0] = cpu.r[3]; // mov r3,r0
                    cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0
                    cpu.r[0] <<= 1;
                    cpu.r[0] = mem
                        .read_u16(cpu.r[0].wrapping_add(cpu.r[13]))
                        .unwrap() as i16 as i32 as u32; // mov.w @(r0,r13),r0
                    cpu.r[3] = cpu.r[3].wrapping_add(1); // add #1,r3
                    cpu.r[6] = cpu.r[0] & 0xFFFF; // delay slot of bra: extu.w r0,r6
                }
                // else: bra 0x8c0e8ca4 with nop delay; r6 keeps its value
            }
            // L3, 0x8c0e8ca4
            cpu.set_t((cpu.r[6] as i32) > 0); // cmp/pl r6
            cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); // delay slot: mov.l @r15,r14 (both paths)
            if (cpu.r[6] as i32) > 0 {
                cpu.r[12] = cpu.r[11]; // mov r11,r12 (0x8c0e8caa)
                loop {
                    // INNER, 0x8c0e8cac
                    let mut r9 = cpu.r[3].wrapping_add(cpu.r[14]); // mov r3,r9 ; add r14,r9
                    cpu.r[0] = cpu.r[13]; // mov r13,r0
                    cpu.set_t(r9 >> 31 != 0); // shll r9
                    r9 <<= 1;
                    cpu.r[9] = mem
                        .read_u16(cpu.r[0].wrapping_add(r9))
                        .unwrap() as i16 as i32 as u32; // mov.w @(r0,r9),r9
                    cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
                    cpu.set_t((cpu.r[6] as i32) > (cpu.r[14] as i32)); // cmp/gt r14,r6
                    cpu.r[0] = cpu.r[12]; // mov r12,r0
                    cpu.r[9] &= 0xFFFF; // extu.w r9,r9
                    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[10]), cpu.r[9]); // mov.l r9,@(r0,r10)
                    cpu.r[12] = cpu.r[12].wrapping_add(4); // add #4,r12
                    cpu.r[11] = cpu.r[11].wrapping_add(4); // add #4,r11
                    cpu.r[1] = cpu.r[1].wrapping_add(1); // delay slot of bt.s: add #1,r1
                    if !cpu.t() {
                        break;
                    }
                }
            }
            // L4, 0x8c0e8cc8
            let r9 = mem.read_u32(0x8C0E8CEC).unwrap(); // mov.l 0x8c0e8cec,r9 (0xfffffffe)
            cpu.r[9] = r9;
            cpu.r[12] = cpu.r[6]; // mov r6,r12
            cpu.r[0] = cpu.r[4]; // mov r4,r0
            cpu.r[12] = cpu.r[12].wrapping_add(r9); // add r9,r12
            cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
            let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[7]), cpu.r[12] as u8); // mov.b r12,@(r0,r7)
            cpu.set_t((cpu.r[5] as i32) > (cpu.r[4] as i32)); // cmp/gt r4,r5
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]); // delay slot of bt.s: add r6,r3 (both paths)
            if !cpu.t() {
                break;
            }
        }
    }
    // EPI, 0x8c0e8cda: add #4,r15; pop r8..r13; rts; delay pop r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [8usize, 9, 10, 11, 12, 13] {
        cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap();
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); // rts delay slot: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
