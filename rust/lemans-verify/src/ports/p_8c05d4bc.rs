//! 0x8c05d4bc — free every node of a ring list, then close the head.
//!
//! `r4` points to a word holding the head node. Starting at `*head`, each node
//! is handed to the free thunk at 0x8c01032c until the walk reaches the head
//! again; the head's first two words are then set to the head itself.
//!
//! The loop's `mov.l @r14,r14` is the `jsr` delay slot: the next pointer is
//! read from the node *before* free runs, so a freed node is never dereferenced
//! again. `r0` is reloaded from `[r13]` before every compare and once before
//! the second closing store, so free's effect on the head word is honoured.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.r[14] = mem.read_u32(cpu.r[0]).unwrap_or(0); // bra delay slot
    loop {
        // 0x8c05d4d2: compare the current node with the head word.
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        let done = cpu.r[0] == cpu.r[14];
        cpu.set_t(done);
        if done {
            break;
        }
        cpu.r[4] = cpu.r[14];
        cpu.r[0] = mem.read_u32(0x8c05d4e8).unwrap_or(0x8c01032c);
        cpu.r[14] = mem.read_u32(cpu.r[14]).unwrap_or(0); // jsr delay slot
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c05d4d8: `*head = head; head[1] = head`. The first store reuses the
    // compare's r0; only the second one reloads the head word.
    let _ = mem.write_u32(cpu.r[0], cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[0]);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
