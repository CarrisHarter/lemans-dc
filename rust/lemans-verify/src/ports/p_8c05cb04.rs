//! 0x8c05cb04 — unlink node `param_2` (r5) from the singly-linked list whose
//! head link lives at `param_1 + 0xc`.
//!
//! ```text
//! bra   0x8c05cb1a
//! add   #12,r4            ; delay slot: r4 = &head link, ALWAYS
//! 0x8c05cb08:
//! cmp/eq r5,r0            ; T = (node == r5)
//! bf    0x8c05cb16        ; immediate
//! mov.l @(8,r5),r1        ; only on the match path
//! mov   #0,r0
//! mov.l r1,@r4            ; *link = node->next
//! rts
//! mov.l r0,@(8,r5)        ; delay slot: node->next = 0
//! 0x8c05cb16:
//! add   #8,r0
//! mov   r0,r4
//! 0x8c05cb1a:
//! mov.l @r4,r0
//! tst   r0,r0
//! bf    0x8c05cb08        ; immediate
//! rts
//! nop
//! ```
//! Hidden side effects the C does not show: r4 ends as the *link address* that
//! held the matched node (param_1+0xc on a head match), r0 is 0 on every exit,
//! r1 keeps its incoming value unless a match stores node->next into it, and T
//! ends 1 on every exit (cmp/eq true on match, tst of 0 otherwise).

use sh4_oracle::{Cpu, Memory};

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

#[inline]
fn wr(mem: &mut Memory, addr: u32, v: u32) {
    let _ = mem.write_u32(addr, v);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r5 = cpu.r[5];
    let mut link = cpu.r[4].wrapping_add(12); // bra delay slot: add #12,r4
    cpu.r[4] = link;
    let mut node = rd(mem, link); // 0x8c05cb1a
    cpu.r[0] = node;
    while node != 0 {
        if node == r5 {
            // cmp/eq true -> fall through
            let next = rd(mem, r5.wrapping_add(8)); // 0x8c05cb0c
            cpu.r[1] = next;
            cpu.r[0] = 0; // mov #0,r0
            wr(mem, link, next); // mov.l r1,@r4
            wr(mem, r5.wrapping_add(8), 0); // rts delay slot
            cpu.set_t(true);
            return;
        }
        // bf 0x8c05cb16
        node = node.wrapping_add(8);
        link = node; // mov r0,r4
        cpu.r[4] = link;
        node = rd(mem, link); // 0x8c05cb1a
        cpu.r[0] = node;
    }
    cpu.set_t(true); // tst r0,r0 with r0 == 0; rts delay slot is nop
}
