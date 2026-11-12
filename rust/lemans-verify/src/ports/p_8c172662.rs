//! 0x8c172662 — clear the flag word at 0x8c2b5518, then tail-call memset.
//!
//! ```text
//! mov.l 0x8c172694,r1   ; r1 = 0x8c2b5518
//! mov   #0,r2           ; r2 = 0
//! mov.w 0x8c172678,r6   ; r6 = 0x100 (sign-extended)
//! mov.l 0x8c172690,r4   ; r4 = 0x8c2b5418
//! mov.l 0x8c172698,r3   ; r3 = 0x8c010f88 (memset)
//! mov.l r2,@r1          ; [0x8c2b5518] = 0
//! jmp   @r3             ; tail call; pr is NOT touched
//! mov   r2,r5           ; (delay slot) r5 = 0
//! ```
//!
//! The delay slot executes before the jump, so memset runs with
//! r4 = 0x8c2b5418, r5 = 0, r6 = 0x100 and returns to our caller.
//! r1/r2 keep their values; r0, r3, r7 and T are the callee's.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8c172694).unwrap_or(0x8c2b5518); // 0x8c2b5518
    cpu.r[2] = 0;
    cpu.r[6] = mem.read_u16(0x8c172678).unwrap_or(0) as i16 as i32 as u32; // 0x100
    cpu.r[4] = mem.read_u32(0x8c172690).unwrap_or(0x8c2b5418); // 0x8c2b5418
    cpu.r[3] = mem.read_u32(0x8c172698).unwrap_or(0x8c010f88); // 0x8c010f88
    let _ = mem.write_u32(cpu.r[1], cpu.r[2]); // [0x8c2b5518] = 0
    cpu.r[5] = cpu.r[2]; // jmp delay slot: r5 = 0
    crate::registry::p_8c010f88::run(cpu, mem); // tail memset
}
