//! 0x8c21741e — set field 20 of the object at r4 to a value derived from r6:
//!   r6==0 -> r5, r6==1 -> [r4+20]+r5, r6==2 -> r5+[r4+16], else return -16.
//! Returns -10 when r4==0 or word[r4+76]==0, -17 when the value is negative
//! or >= [r4+16]; on success stores it to [r4+20] and returns 0.
//!
//! ```text
//! tst r4,r4 ; bt 0x8c21742a
//! mov #76,r0 ; mov.w @(r0,r4),r3 ; tst r3,r3 ; bf 0x8c21742e
//! rts ; mov #-10,r0
//! mov r6,r0 ; cmp/eq #0,r0 ; bt.s 0x8c217442 ; mov.l @(16,r4),r7 (delay, BOTH paths)
//! cmp/eq #1,r0 ; bt 0x8c217446 ; cmp/eq #2,r0 ; bt 0x8c21744c ; bra 0x8c217452 ; nop
//! 0x8c217442: bra 0x8c217456 ; mov r5,r6 (delay)
//! 0x8c217446: mov.l @(20,r4),r6 ; bra 0x8c217456 ; add r5,r6 (delay)
//! 0x8c21744c: mov r5,r6 ; bra 0x8c217456 ; add r7,r6 (delay)
//! 0x8c217452: rts ; mov #-16,r0
//! 0x8c217456: cmp/pz r6 ; bf 0x8c21745e
//!             cmp/gt r7,r6 ; bf 0x8c217462
//! 0x8c21745e: rts ; mov #-17,r0
//! 0x8c217462: mov #0,r0 ; mov.l r6,@(20,r4) ; rts ; nop
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] == 0 {
        cpu.r[0] = (-10i32) as u32; // rts delay
        return;
    }
    cpu.r[0] = 76;
    cpu.r[3] = mem.read_u16(cpu.r[4].wrapping_add(76)).unwrap() as i16 as i32 as u32;
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.r[3] == 0 {
        cpu.r[0] = (-10i32) as u32; // rts delay
        return;
    }
    // 0x8c21742e: mov r6,r0
    cpu.r[0] = cpu.r[6];
    // bt.s delay slot ALWAYS runs: r7 = [r4+16]
    cpu.r[7] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap();
    cpu.set_t(cpu.r[6] == 0); // cmp/eq #0,r0
    if cpu.r[6] == 0 {
        // bra 0x8c217456 ; delay mov r5,r6
        cpu.r[6] = cpu.r[5];
    } else {
        cpu.set_t(cpu.r[6] == 1); // cmp/eq #1,r0
        if cpu.r[6] == 1 {
            cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap();
            // bra 0x8c217456 ; delay add r5,r6
            cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[5]);
        } else {
            cpu.set_t(cpu.r[6] == 2); // cmp/eq #2,r0
            if cpu.r[6] == 2 {
                // mov r5,r6 ; bra 0x8c217456 ; delay add r7,r6
                cpu.r[6] = cpu.r[5].wrapping_add(cpu.r[7]);
            } else {
                // bra 0x8c217452 ; nop ; rts ; mov #-16,r0
                cpu.r[0] = (-16i32) as u32;
                return;
            }
        }
    }
    // 0x8c217456
    cpu.set_t((cpu.r[6] as i32) >= 0); // cmp/pz r6
    if (cpu.r[6] as i32) < 0 {
        cpu.r[0] = (-17i32) as u32; // rts delay
        return;
    }
    cpu.set_t((cpu.r[6] as i32) > (cpu.r[7] as i32)); // cmp/gt r7,r6 -> T = r6 > r7 (Rn > Rm)
    if (cpu.r[6] as i32) > (cpu.r[7] as i32) {
        cpu.r[0] = (-17i32) as u32; // bf not taken -> rts delay at 0x8c21745e
        return;
    }
    // success (0x8c217462)
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[6]);
    // rts ; nop
}
