//! 0x8c011344 — strncpy: copy r5 into r4, NUL-padding to n bytes.
//!
//! r1 = 0 is the pad byte (and stays in r1). The `bra` delay slot zeroes
//! r5 (the counter). Loop 1 copies non-NUL src bytes; once src hits NUL,
//! loop 2 pads with r1 while the `bf.s` delay slot advances r7 even on the
//! last store. Final: r0 = r4 (rts delay slot), r5 = n on copy paths,
//! r7 = dst + stores, r3 = last byte loaded, T always set.
//! NOTE: the oracle decodes `mov.b @r0+,r3` (loop 1) as a u32 load with a
//! 1-byte step, while the head load `mov.b @r0,r3` is a true byte load;
//! only the low byte is stored, but r3's final value is the u32.

use sh4_oracle::{Cpu, Memory};

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    let n = cpu.r[6];
    cpu.r[1] = 0; // mov #0,r1
    let mut r0 = src; // mov r5,r0
    let mut r7 = dst; // mov r4,r7
    let mut r5 = 0u32; // delay slot of `bra`: mov r1,r5
    loop {
        // head 0x8c011356
        if r5 >= n {
            cpu.set_t(true); // cmp/hs r6,r5, bt taken
            break;
        }
        let c = sext8(mem, r0); // mov.b @r0,r3 (0x8c01135a), true byte load
        if c != 0 {
            // loop 1, 0x8c01134e
            cpu.r[3] = mem.read_u32(r0).unwrap_or(0); // mov.b @r0+,r3 as decoded (u32, step 1)
            r0 = r0.wrapping_add(1);
            r5 = r5.wrapping_add(1);
            let _ = mem.write_u8(r7, c as u8); // mov.b r3,@r7 stores low byte
            r7 = r7.wrapping_add(1);
            continue;
        }
        cpu.r[3] = c; // head read is the final r3 on the pad path
        // src exhausted
        if r5 >= n {
            cpu.set_t(true); // 0x8c011360, bt taken
            break;
        }
        // loop 2, pad with r1
        loop {
            r5 = r5.wrapping_add(1);
            let _ = mem.write_u8(r7, 0); // mov.b r1,@r7
            r7 = r7.wrapping_add(1);     // delay slot of `bf.s`
            if r5 >= n {
                cpu.set_t(true);
                break;
            }
        }
        break;
    }
    cpu.r[0] = dst; // delay slot of `rts` destroys the src pointer in r0
    cpu.r[5] = r5;
    cpu.r[7] = r7;
}
