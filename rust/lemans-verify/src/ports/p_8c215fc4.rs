//! 0x8c215fc4 — word copy `r4`→`r5`, with the count rounded up.
//!
//! ```text
//! cmp/pz r6 ; bt skip ; add #3,r6      ; plain bt: the add runs only for r6 < 0
//! shar r6 ; bra next ; shar r6         ; the bra delay slot shifts again
//! loop: r3 = [r4+] ; [r5] = r3 ; r5 += 4
//! next: cmp/pl r6 ; bt.s loop ; add #-1,r6
//! rts
//! ```
//!
//! Hidden side effects: the first `bt` is undelayed, so the `add #3` runs only
//! when `r6` is negative; both `shar`s execute, halving twice; and `r6` ends
//! at 0 with T from the final `cmp/pl`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let non_negative = (cpu.r[6] as i32) >= 0;
    cpu.set_t(non_negative); // cmp/pz r6
    if !non_negative {
        cpu.r[6] = cpu.r[6].wrapping_add(3); // only when the bt is not taken
    }
    cpu.r[6] = ((cpu.r[6] as i32) >> 1) as u32; // shar r6
    cpu.r[6] = ((cpu.r[6] as i32) >> 1) as u32; // delay slot of `bra`

    // The entry `bra` targets the test, so this is a while loop: test,
    // decrement in the delay slot, then branch back to the body.
    loop {
        let go = (cpu.r[6] as i32) > 0;
        cpu.set_t(go); // cmp/pl r6
        cpu.r[6] = cpu.r[6].wrapping_sub(1); // delay slot of `bt.s`
        if !go {
            break;
        }
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
        cpu.r[5] = cpu.r[5].wrapping_add(4);
    }
}
