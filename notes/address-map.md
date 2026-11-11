# Address map

## Verified ports

| address | name | evidence |
| --- | --- | --- |
| `0x8c175c72` | `am_sound_is_playing` | `notes/verifications/0x8c175c72.json` — 9 oracle steps |
| `0x8c18b9c8` | `array_lookup` | `notes/verifications/0x8c18b9c8*.json` — both branches (12 and 7 oracle steps) |
| `0x8c1552e4` | `store_through_two_levels` | `notes/verifications/0x8c1552e4.json` — 4 oracle steps, stores match |
| `0x8c010080` | `FUN_8c010080` | `notes/verifications/0x8c010080.json` — 2 oracle steps; none |
| `0x8c010124` | `FUN_8c010124` | `notes/verifications/0x8c010124.json` — 2 oracle steps; none |
| `0x8c010128` | `FUN_8c010128` | `notes/verifications/0x8c010128.json` — 2 oracle steps; r0=0 |
| `0x8c01012c` | `FUN_8c01012c` | `notes/verifications/0x8c01012c.json` — 2 oracle steps; r0=0 |
| `0x8c010130` | `FUN_8c010130` | `notes/verifications/0x8c010130.json` — 2 oracle steps; r0=0 |
| `0x8c010134` | `FUN_8c010134` | `notes/verifications/0x8c010134.json` — 2 oracle steps; r0=0 |
| `0x8c010138` | `FUN_8c010138` | `notes/verifications/0x8c010138.json` — 2 oracle steps; r0=0 |
| `0x8c01013c` | `FUN_8c01013c` | `notes/verifications/0x8c01013c.json` — 2 oracle steps; r0=0 |
| `0x8c010140` | `FUN_8c010140` | `notes/verifications/0x8c010140.json` — 2 oracle steps; r0=0 |
| `0x8c010144` | `FUN_8c010144` | `notes/verifications/0x8c010144.json` — 2 oracle steps; none |
| `0x8c010148` | `FUN_8c010148` | `notes/verifications/0x8c010148.json` — 4 oracle steps; store none; r0 = *ptr+1 |
| `0x8c010154` | `FUN_8c010154` | `notes/verifications/0x8c010154.json` — 4 oracle steps; none; r0 = table[idx], r4 <<= 2 |
| `0x8c010178` | `FUN_8c010178` | `notes/verifications/0x8c010178.json` — 2 oracle steps; none |
| `0x8c01017c` | `FUN_8c01017c` | `notes/verifications/0x8c01017c.json` — 2 oracle steps; none |
| `0x8c010438` | `FUN_8c010438` | `notes/verifications/0x8c010438.json` — 7 oracle steps; store at [slot]; r5/r6/r2 leftovers; T per path |
| `0x8c01044c` | `FUN_8c01044c` | `notes/verifications/0x8c01044c.json` — 2 oracle steps; none |
| `0x8c010d6c` | `FUN_8c010d6c` | `notes/verifications/0x8c010d6c.json` — 6 oracle steps; r1 = table base; r0 = byte&7 |
| `0x8c010d7c` | `FUN_8c010d7c` | `notes/verifications/0x8c010d7c.json` — 6 oracle steps; r1 = table base; r0 = byte&0x40 |
| `0x8c0192fc` | `FUN_8c0192fc` | `notes/verifications/0x8c0192fc.json` — 8 oracle steps; store [global]=r4, [r4..r4+8]=0; r1=0; r0=r4 |
| `0x8c019670` | `FUN_8c019670` | `notes/verifications/0x8c019670.json` — 4 oracle steps; r4 += 0x70; r0 = zero-extended byte |
| `0x8c01ace8` | `FUN_8c01ace8` | `notes/verifications/0x8c01ace8.json` — 6 oracle steps; r1=table base; r5 <<= 2; r0=final field load |
| `0x8c01acf4` | `FUN_8c01acf4` | `notes/verifications/0x8c01acf4.json` — 6 oracle steps; r1=table base; r5 <<= 2; r0=final field load |
| `0x8c01e484` | `FUN_8c01e484` | `notes/verifications/0x8c01e484.json` — 2 oracle steps; r0=r4 |
| `0x8c01e704` | `FUN_8c01e704` | `notes/verifications/0x8c01e704.json` — 4 oracle steps; r1=[r5]; [r4]=r1; r0=r4 |
| `0x8c01e70c` | `FUN_8c01e70c` | `notes/verifications/0x8c01e70c.json` — 2 oracle steps; none |
| `0x8c01e828` | `FUN_8c01e828` | `notes/verifications/0x8c01e828.json` — 11 oracle steps; [r4+8] rewritten twice; T=last tst; r4=0 iff decremented value nonzero; r0=r4 (possibly zeroed) |
| `0x8c010160` | `FUN_8c010160` | `notes/verifications/0x8c010160.json` — 4 oracle steps; store r5 at table_base + (r4<<2) |
| `0x8c010e50` | `FUN_8c010e50` | `notes/verifications/0x8c010e50.json` — 28 oracle steps; none; r0 = last-byte diff; r4/r7 advanced, r5 = match count, r2/r3 leftovers, T per exit |
| `0x8c019678` | `FUN_8c019678` | `notes/verifications/0x8c019678.json` — 10 oracle steps; T=(field==3); r0=field or flag base 0xac; r1=1; byte store at 0xac+r4 |
| `0x8c01c234` | `FUN_8c01c234` | `notes/verifications/0x8c01c234.json` — 10 oracle steps; T=(field==1); r0=field or flag base 0x85; r1=1; byte store at 0x85+r4 |
| `0x8c01ec2c` | `FUN_8c01ec2c (field getter)` | `notes/verifications/0x8c01ec2c.json` — 2 oracle steps; r0 = [r4+0xc] |
| `0x8c01f1cc` | `FUN_8c01f1cc (0x104-byte record copy)` | `notes/verifications/0x8c01f1cc.json` — 488 oracle steps; r4/r6 += 0x104 per record; r0=r6; r1=r6_iter+0x100; r3=0; r7=dst+0x100; fr0=copied float; T=1; 128B+4B stores per record when r6!=0 |
| `0x8c020394` | `FUN_8c020394 (stub)` | `notes/verifications/0x8c020394.json` — 2 oracle steps; none |
| `0x8c020398` | `FUN_8c020398 (stub)` | `notes/verifications/0x8c020398.json` — 2 oracle steps; none |
| `0x8c0203f8` | `FUN_8c0203f8 (indexed global store)` | `notes/verifications/0x8c0203f8.json` — 3 oracle steps; r0 = [0x8c020400] (0x10c); [r0+r4] = r5 |
| `0x8c02097c` | `FUN_8c02097c (16-byte record copy)` | `notes/verifications/0x8c02097c.json` — 46 oracle steps; r4/r6 += 0x10 per iteration; r0=r6 final; T=1; 4 stores per record when r6!=0 |
| `0x8c024c3c` | `FUN_8c024c3c (fr4 store at +0x14)` | `notes/verifications/0x8c024c3c.json` — 3 oracle steps; r4 += 0x14 before the store; [r4] = fr4 bits |
| `0x8c0252cc` | `FUN_8c0252cc (BST walk, result via [r15])` | `notes/verifications/0x8c0252cc.json` — 29 oracle steps; r3=0 (bt.s delay slot, even when root null); r4 += 4 only on the r4+4 store path; r6=0; r0=[r15]; r1/r0 hold last loop loads or final best[3]/key; T = key>best[3] or 0 when best null |
| `0x8c026a50` | `FUN_8c026a50 (16-byte header init)` | `notes/verifications/0x8c026a50.json` — 19 oracle steps; r4=0; r6=base; r1=base+4; r3=base+8; r0=base; stores at base..base+0xf with +0xc written twice (0, then base+4) |
| `0x8c026c90` | `FUN_8c026c90 (tagged list unlink)` | `notes/verifications/0x8c026c90.json` — 37 oracle steps; r6=next|old_flag; r3=0xfffffffe (unconditional literal load at 0x8c026cb2); r1=[r4+8]&~1; r0=[r4+8]&1; T=cmp/eq at 0x8c026cc6; node->next = next->prev (0x8c026c9a is @(4,r6), not @(4,r4)) |
| `0x8c04704c` | `FUN_8c04704c` | `notes/verifications/0x8c04704c.json` — 2 oracle steps; none |
| `0x8c04ace0` | `FUN_8c04ace0` | `notes/verifications/0x8c04ace0.json` — 9 oracle steps; r6 shifted in place (shll2); intermediate pointer left in r1; rts delay slot adds shifted r6 to r0 |
| `0x8c04b358` | `FUN_8c04b358` | `notes/verifications/0x8c04b358.json` — 4 oracle steps; second deref in rts delay slot |
| `0x8c04b420` | `FUN_8c04b420` | `notes/verifications/0x8c04b420.json` — 3 oracle steps; r0=0x98 literal-pool base left in r0; byte store in rts delay slot |
| `0x8c04b9ac` | `FUN_8c04b9ac` | `notes/verifications/0x8c04b9ac.json` — 3 oracle steps; r0=0xcc literal-pool base left in r0; word store in rts delay slot |
| `0x8c04ba4c` | `FUN_8c04ba4c` | `notes/verifications/0x8c04ba4c.json` — 5 oracle steps; r5 shifted in place (shll2); table base left in r1; element load in rts delay slot |
| `0x8c04d264` | `FUN_8c04d264` | `notes/verifications/0x8c04d264.json` — 20 oracle steps; r6=count, r3=index, r1=last element, r0=byte offset all live at return; T=1 on match, T=0 on -1 exit |
| `0x8c04d284` | `FUN_8c04d284` | `notes/verifications/0x8c04d284.json` — 6 oracle steps; r5 shifted in place; table base left in r1; element load in rts delay slot |
| `0x8c050c54` | `FUN_8c050c54` | `notes/verifications/0x8c050c54.json` — 2 oracle steps; r0=[r4+0x20] in rts delay slot |
| `0x8c050c58` | `FUN_8c050c58` | `notes/verifications/0x8c050c58.json` — 2 oracle steps; word store [r4+0x20]=r5 in rts delay slot |
| `0x8c0176f0` | `pat_match (r4=pattern, r5=string, r6=int* out)` | `notes/verifications/0x8c0176f0.json` — 61 oracle steps; stores 0 at [r6] on entry, parsed decimal at [r6] on the !..!..! capture path; r5 clobbered to -48 (delay slot 0x8c017778) on the capture path and to sext8(*r4) in the tail loop (0x8c01782e); r1/r3/r4 left advanced past consumed input; r1++/r4++ delay slots fire on taken AND fall-through branches; T=1 on every return-1 and on the NUL-mismatch exit, T=0 on the two compare-mismatch exits |
| `0x8c03c75c` | `FUN_8c03c75c` | `notes/verifications/0x8c03c75c.json` — 12 oracle steps; stores fr7/fr4/fr5/fr6 at [r4..r4+12]; leaves r3=r4+4, r1=r4+8; r0=r4 from rts delay slot |
| `0x8c03c950` | `FUN_8c03c950` | `notes/verifications/0x8c03c950.json` — 4 oracle steps; stores 0 at [0xfc+r4]; leaves r0=0xfc (literal base), r1=0 |
| `0x8c03c95c` | `FUN_8c03c95c` | `notes/verifications/0x8c03c95c.json` — 4 oracle steps; stores 0 at [0xf8+r4]; leaves r0=0xf8, r1=0 |
| `0x8c03eb7c` | `FUN_8c03eb7c` | `notes/verifications/0x8c03eb7c.json` — 4 oracle steps; stores r4 at [literal 0x8c23dbf0]; r0=r4 |
| `0x8c03ed84` | `FUN_8c03ed84` | `notes/verifications/0x8c03ed84.json` — 3 oracle steps; r0=0x3c4+r4 |
| `0x8c03ee9c` | `FUN_8c03ee9c` | `notes/verifications/0x8c03ee9c.json` — 4 oracle steps; byte-stores 1 at [0x3bc+r4]; leaves r0=0x3bc, r1=1 |
| `0x8c043474` | `FUN_8c043474` | `notes/verifications/0x8c043474.json` — 4 oracle steps; stores [r4] at [[r15]]; leaves r1=[r15], r0=[r4] |
| `0x8c04347c` | `FUN_8c04347c` | `notes/verifications/0x8c04347c.json` — 5 oracle steps; stores [[r4]] at [[r15]]; leaves r1=[r15], r0=[[r4]] |
| `0x8c043960` | `FUN_8c043960` | `notes/verifications/0x8c043960.json` — 3 oracle steps; stores r4 at [literal 0x8c23dbf8]; leaves r0=global addr |
| `0x8c0443ec` | `FUN_8c0443ec` | `notes/verifications/0x8c0443ec.json` — 24 oracle steps; writes 11 u32s at [r4..r4+40] (1.0f,1,0,1,0,0,1,1,fr0,fr0,fr0); leaves r6=1, r5=r4+32, r3=r4+36, r1=r4+40, r0=r4, fr0=[0x8c04441c] |
| `0x8c02b3a0` | `FUN_8c02b3a0` | `notes/verifications/0x8c02b3a0.json` — 151 oracle steps; stores: [r4+0x3f8]=state(0/1/2) u32, byte [r4+0x405]=0 (paths 1/2 only), 17 scaled f32 at [r4+4]+off, [r4+0x4f8]=0.0, [r4+0x4f0]=[r4+0x4ec], [r4+0x4f4]=1.0 (p1/p3) or 0.99f (p2, delay slot of bra); r4+=0x4f4 on all paths; r11-r14/r15 restored by epilogue; T=1 on p1/p2, 0 on p3; leftover regs p1: r0=0x405 r1=0 r3=0x4f4 r5=r4+0x4ec r6=r4+0x4f0 r7=r4+0x4f8; p2: r0=0x8c02b694 r1=0x4f4 r3=r4+0x4ec r5=r4+0x4f0 r6=r4+0x4f8 r7=0; p3: r0=0x4f4 r1=r4+0x4f0 r3=r4+0x4f8 r5=2; fr leftovers p1 fr1=[r4+0x4ec] fr2=[P+0x380]*1.15 fr3=1.15, p2 fr1=[P+0x380]*1.07 fr2=1.07 fr3=1.1, p3 fr0=1.0 |
| `0x8c026ce0` | `list_unlink` | `notes/verifications/0x8c026ce0.json` — 37 oracle steps; r6=node->next via bf.s delay slot (always); r3 ends 0xfffffffe (literal load @0x8c026d02); r1=node->f8&~1; r0=node->f8&1; r6=next|(node->f8&1); final T from cmp/eq r0,r4 not from trailing or |
| `0x8c026de4` | `set_two_fields` | `notes/verifications/0x8c026de4.json` — 5 oracle steps; r0 ends as [r4+0x30] (was 0x4c mid-function); second store [r0+0x34]=r5 is the rts delay slot |
| `0x8c026e34` | `move_pair_clear_src` | `notes/verifications/0x8c026e34.json` — 8 oracle steps; r0=0; r1 ends as second load [r5+0x30]; both zeroing stores incl. rts delay slot |
| `0x8c02727c` | `copy_float_field` | `notes/verifications/0x8c02727c.json` — 6 oracle steps; r4 ends base+4; r0 ends base+0x34; fr0 holds loaded bits; store at [base+0x34] is rts delay slot |
| `0x8c0273a0` | `copy_triple_to_0x68` | `notes/verifications/0x8c0273a0.json` — 8 oracle steps; r4 ends base+0x68 (arg register itself modified); r1=[r5]; r0=[r5+8]; third store is rts delay slot |
| `0x8c0273b0` | `copy_two_48B_blocks` | `notes/verifications/0x8c0273b0.json` — 187 oracle steps; r4 ends p1+0xb4 (not saved/restored); r1=6 r3=48 r7=6; r0=last sign-extended mov.w; T=0 from final cmp/hi 6>6; r10-r14 pushed/popped keep entry values |
| `0x8c027418` | `copy_48B_plus_ptr_table` | `notes/verifications/0x8c027418.json` — 138 oracle steps; r4 ends p1+0x144; r0=20 r1=6 r3=48 r7=24; T=0; the u32 store [r4+r0]=r13 is the bt.s delay slot |
| `0x8c0274c8` | `empty_rts` | `notes/verifications/0x8c0274c8.json` — 2 oracle steps; none (rts ; nop) |
| `0x8c0274cc` | `table_byte_lookup` | `notes/verifications/0x8c0274cc.json` — 4 oracle steps; r0 first becomes table base 0x170 (literal @0x8c0274d4) then the byte at 0x170+r4; extu.b rts delay slot zero-extends the sign-extending mov.b |
| `0x8c014bd4` | `FUN_8c014bd4 (bignum rol-left, mask 0x80)` | `notes/verifications/0x8c014bd4.json` — 73 oracle steps; r5=r6=end-1; r1=last&0x80; r3=signext(byte)<<1; r7=last mask bit (return); r0=0/1; T=final tst r7; r14 restored via push/pop; entry bf.s delay slot forces r6=r5; loop delay slot decrements r6 in lockstep |
| `0x8c015148` | `FUN_8c015148 (bignum ror, carry bit)` | `notes/verifications/0x8c015148.json` — 85 oracle steps; r4=r6=base+n; r0=r7=final carry; r3=last>>1; r12/r13/r14 pushed+popped (r13=1 set in entry delay slot); T=final cmp/ge (1 if loop ran, cmp/pl if skipped) |
| `0x8c0152fc` | `FUN_8c0152fc (guarded slot store)` | `notes/verifications/0x8c0152fc.json` — 12 oracle steps; rts delay slot mov #1,r0 on store path; r2=slot ptr (loaded from cell 0x8c015320); r1=0x450; r3=0x200; r4=p-q; zero-byte path keeps r3=*p sign-extended and r4 untouched with T=1; short path T=0 |
| `0x8c015eea` | `FUN_8c015eea (decimal string increment)` | `notes/verifications/0x8c015eea.json` — 49 oracle steps; r0 never written; r1=48; r7=57 via bf.s delay slot; r5/r6 decrement in lockstep (delay slot) so carry writes hit current byte and byte below; r3 ends as r5-1 pointer after last carry (or base[n-1]+1); r6 ends at r4, r5 at r4+1; T=final cmp/gt on base[1] |
| `0x8c0174c0` | `FUN_8c0174c0 (set global, return old)` | `notes/verifications/0x8c0174c0.json` — 8 oracle steps; r1=slot pointer loaded from cell 0x8c0174d0; r0=old value; r14 saved/restored; T untouched |
| `0x8c017844` | `FUN_8c017844 (push node on list)` | `notes/verifications/0x8c017844.json` — 12 oracle steps; r0=r4 (return); r1=old head; r3=head-pointer cell; store order node[0]=old, node[1]=r5, node[2]=r4, then *head=node; T untouched |
| `0x8c018fec` | `FUN_8c018fec (store to global, return arg)` | `notes/verifications/0x8c018fec.json` — 4 oracle steps; rts delay slot mov r4,r0 so r0=stored value even though r0 first held the pointer; T untouched |
| `0x8c017b94` | `FUN_8c017b94` | `notes/verifications/0x8c017b94.json` — 17 oracle steps; r6=unpacked; r3/r1=shifted src bytes on long paths; r4+=1..4; r0=r4new; T per path; [r5]=r6 |
| `0x8c017d6c` | `FUN_8c017d6c` | `notes/verifications/0x8c017d6c.json` — 10 oracle steps; [r4+12]=[r4+16]=pool ptr 0x8c1f2e88; r0=1 |
| `0x8c017e30` | `FUN_8c017e30` | `notes/verifications/0x8c017e30.json` — 119 oracle steps; r3=0/16; r1=-4; r5/r6 recomputed; r7=count2*4; T=0; stores two word runs + [r4+20] |
| `0x8c017efc` | `FUN_8c017efc` | `notes/verifications/0x8c017efc.json` — 19 oracle steps; r1=last compared left word; T=last cmp/eq; r0=0/1 |
| `0x8c017f24` | `FUN_8c017f24` | `notes/verifications/0x8c017f24.json` — 16 oracle steps; r0=0/1; T=last tst |
| `0x8c018274` | `FUN_8c018274` | `notes/verifications/0x8c018274.json` — 6 oracle steps; r2=0x8c1f2f98; r3=*(r2); r1=0; [r3+8]=0 |
| `0x8c018fbc` | `FUN_8c018fbc` | `notes/verifications/0x8c018fbc.json` — 2 oracle steps; none |
| `0x8c02bf14` | `FUN_8c02bf14` | `notes/verifications/0x8c02bf14.json` — 10 oracle steps; store fr0 at [r4+12] via rts delay slot; r4+=12 before store; T per path (1 if fr4<0 or fr4>1, else 0) |
| `0x8c02c0e8` | `FUN_8c02c0e8` | `notes/verifications/0x8c02c0e8.json` — 18 oracle steps; r0=r5&0xFF then overwritten with const 0x270; r1=[r4+4]+0x270; r4+=12 before all float access; up to 2 stores at [r4] (result, then 1.0f if result>1.0); T=1 only on zero path (tst) and clamp path |
| `0x8c02ca10` | `FUN_8c02ca10` | `notes/verifications/0x8c02ca10.json` — 14 oracle steps; r0=[r4+4]+0x278, r1=0x3a0, r3=[r4+4]+0x27c, r4+=0x3a0 all left behind; fr1/fr2 stay loaded; fmul in rts delay slot; no T update |
| `0x8c02ca34` | `FUN_8c02ca34` | `notes/verifications/0x8c02ca34.json` — 26 oracle steps; fldi1 fr0 in bf.s delay slot runs on EVERY path (it is the 1.0 return); r0 ends as slot value/r4+0x450/mova const; r1 ends 0x39c; r3,r4 advanced; T=0 zero path, 1 early-outs and fr2>fr3, 0 otherwise |
| `0x8c02cb28` | `FUN_8c02cb28` | `notes/verifications/0x8c02cb28.json` — 16 oracle steps; r0=0x3e0, r1=[r4+4]+0x3e0, r3=0x450, r4+=0x450 left behind; fr1/fr2 loaded; return value in fr0 (0.0 if quotient<0, 1.0 if >1, else quotient); T=1 on the two early-outs, 0 in-range |
| `0x8c031c0c` | `FUN_8c031c0c` | `notes/verifications/0x8c031c0c.json` — 11 oracle steps; r5=[r4+4]+0x274 via bf.s delay slot on BOTH paths; r0=0x3a4 then 0x8c; r4+=0x3a4; fr2=1.0, fr3=input stay; final fmul fr1,fr0 in rts delay slot |
| `0x8c0326a0` | `FUN_8c0326a0` | `notes/verifications/0x8c0326a0.json` — 17 oracle steps; 3 float stores at r4+0x4f8/0x4f0/0x4f4, the last in rts delay slot AFTER r4+=0x4f4; r0=0x4f4, r1=r4+0x4f0, r3=r4+0x4f8, r4 advanced; fr0 keeps loaded value |
| `0x8c03703c` | `FUN_8c03703c` | `notes/verifications/0x8c03703c.json` — 5 oracle steps; r0 loaded twice (ptr then value); r1=[r15] slot pointer; delay-slot store through it |
| `0x8c03943c` | `FUN_8c03943c` | `notes/verifications/0x8c03943c.json` — 2 oracle steps; r0=[r4+0x24] in rts delay slot - whole port is the delay slot |
| `0x8c039440` | `FUN_8c039440` | `notes/verifications/0x8c039440.json` — 2 oracle steps; r0=[r4+0x20] in rts delay slot - whole port is the delay slot |
| `0x8c015324` | `FUN_8c015324 (bignum sub, borrow)` | `notes/verifications/0x8c015324.json` — 110 oracle steps; r1 ends exactly at a (entry bf.s delay slot +1); r6=a-1; r7=b-1; r2=r3=last result byte; return -1 only if borrow&&r4==0 else 0 if all result bytes 0 else 1; T from last tst/cmp/eq; result store is mov.b r3,@-r1 (byte at pre-decremented r1, delay slot) |
| `0x8c01539c` | `FUN_8c01539c (memcmp-ish)` | `notes/verifications/0x8c01539c.json` — 35 oracle steps; bf.s delay slot copies r5->r7 even on n==0 return; r4 ends one past last byte examined; r7 then decremented; r2 keeps sign-extended b byte of the deciding pair; r3 reloaded zero-extended at exit; T=deciding compare (0 mismatch, 1 counter/n==0) |
| `0x8c01644c` | `FUN_8c01644c (unrolled memcpy)` | `notes/verifications/0x8c01644c.json` — 32 oracle steps; r2/r3/r4 pushed+popped so only r0/r1 survive; r0=sign-extended last src byte; r1=dst+4*full_blocks (add #4,r1 is not a delay slot); stores are mov.b at r1, r1+1..3 per block; T=final cmp/hi (1 only on n==0) |
| `0x8c010e84` | `FUN_8c010e84 (memmove)` | `notes/verifications/0x8c010e84.json` — 44 oracle steps; loop scratch in r1/r2/r3/r6/r7/r11/r12/r13; r1=3 only on word/dword paths (byte paths branch before mov #3,r1); r2=0 fwd / 1 bwd / n==0 leaves T clear; r11/r12/r13 restored by epilogue; rts delay slot sets r0=r4 |
| `0x8c010f44` | `FUN_8c010f44 (byte copy fwd/bwd)` | `notes/verifications/0x8c010f44.json` — 50 oracle steps; r3=last byte (fwd: u32 load of last src dword; bwd: sign-extended src[0]); r5=counter n; r7=dst+n (fwd) or dst (bwd); r2=0 on bwd path; bt.s delay slots set r7/r0 even when n==0; rts delay slot sets r0=r4; T always set |
| `0x8c010f88` | `FUN_8c010f88 (memset)` | `notes/verifications/0x8c010f88.json` — 47 oracle steps; r3=0 and r7=n (loop counter) left behind; r0 reloaded to r4 by rts delay slot; T set on all paths |
| `0x8c01125c` | `FUN_8c01125c (global store)` | `notes/verifications/0x8c01125c.json` — 3 oracle steps; r3 = loaded pointer 0x8c1f2eb0 left in r3; store r4 -> [0x8c1f2eb0] in rts delay slot |
| `0x8c0112a4` | `FUN_8c0112a4 (strcpy)` | `notes/verifications/0x8c0112a4.json` — 44 oracle steps; r6/r7 end one past the copied NUL (bf.s delay slot advances r6 even on the final store); r2=0 after extu.b; bra delay slot copies src r5 into r7; rts delay slot sets r0=r4; T set |
| `0x8c0112c4` | `FUN_8c0112c4 (strchr)` | `notes/verifications/0x8c0112c4.json` — 15 oracle steps; r4 clobbered by exts.b r5,r4 in the bra delay slot (sign-extended search char); r2=last byte read; found: r0=r6, T set; miss: r0=0 via rts delay slot, T set |
| `0x8c0112f4` | `FUN_8c0112f4 (strlen)` | `notes/verifications/0x8c0112f4.json` — 27 oracle steps; r4 ends one past the terminating dword; r3 = that dword (0); r5=count moved to r0 by rts delay slot; T set |
| `0x8c011344` | `FUN_8c011344 (strncpy)` | `notes/verifications/0x8c011344.json` — 47 oracle steps; r1=0 is the pad byte and stays in r1; r5=counter (bra delay slot zeroes it); r7=dst+stores (bf.s delay slot advances r7 even on the last pad store); r3=last load (u32 in loop1, byte at the head); r0 destroyed to r4 by rts delay slot; T set |
| `0x8c011374` | `FUN_8c011374 (scan haystack for needle byte)` | `notes/verifications/0x8c011374.json` — 103 oracle steps; r1=last needle dword read; r3=last load; r6=inner index at exit; r7=outer index; found: r0=r14 via bra delay slot, T clear; NULL: r0=0, T set; r12/r14 restored via stack; inner loop compares hay byte against u32 needle dwords (group-0 indexed loads are u32) |
| `0x8c017f48` | `FUN_8c017f48` | `notes/verifications/0x8c017f48.json` — 22 oracle steps; r2=sign-extended last byte (0); r4/r5 advanced past NUL; T=1 at exit; byte stores dst.. |
| `0x8c018f8c` | `FUN_8c018f8c` | `notes/verifications/0x8c018f8c.json` — 8 oracle steps; u32 r4 at 0x8c23da80 (now in the oracle RAM window); bytes 0 at [r4] and [r4+0x100]; r1=0; r0=r4 |
| `0x8c010000` | `fixture_sum_loop` | `notes/verifications/0x8c010000.json` — 19 oracle steps; harness regression fixture |
| `0x8c01011c` | `store_zero_return_arg` | `notes/verifications/0x8c01011c.json` — 4 oracle steps; delay slot copies the argument into r0 |
| `0x8c0274d8` | `FUN_8c0274d8` | `notes/verifications/0x8c0274d8.json` — 4 oracle steps; r0 = **[r4+0x174] via literal pool 0x8c0274e0; no other reg touched |
| `0x8c0274e4` | `FUN_8c0274e4` | `notes/verifications/0x8c0274e4.json` — 4 oracle steps; r0 = **[r4+0x178] via literal pool 0x8c0274ec |
| `0x8c0274f0` | `FUN_8c0274f0` | `notes/verifications/0x8c0274f0.json` — 6 oracle steps; r1=0x144 leftover, r5 scaled in place (<<2), r4 mutated in place (+=0x144), r0=[r5*4+r4+0x144] |
| `0x8c028d90` | `FUN_8c028d90` | `notes/verifications/0x8c028d90.json` — 7 oracle steps; fr5 overwritten on 2 of 3 paths, fr0=fr5 in rts delay slot, T=last fcmp (per path) |
| `0x8c02baf8` | `FUN_8c02baf8` | `notes/verifications/0x8c02baf8.json` — 48 oracle steps; r4+=0x4ac, r5+=40, r6=r5+36, r7=r4+0x4a4, r3=r4+0x4a8, r1=0x4ac, r0=[r4+0x4a0], fr0=f32[r4+0x4ac]; 10 stores at r5+0..28,32,36,40 |
| `0x8c02be1c` | `FUN_8c02be1c` | `notes/verifications/0x8c02be1c.json` — 9 oracle steps; r6=1, per-path r0/r1/r3/r5/fr0/fr1 leftovers, T=last cmp; stores [r4+0x3bc]=r7, u8[r4+0x3d0]=1, [r4+0x3b8]+=-1 (bra delay slot) or -2 |
| `0x8c02be90` | `FUN_8c02be90` | `notes/verifications/0x8c02be90.json` — 8 oracle steps; r5=0x3b8, r3=counter, r0/r1 leftovers, T per path; stores [r4+0x3bc]=r3, u8[r4+0x3d0]=1, [r4+0x3b8]+=2 (rts delay slot, -1 case) or +=1 |
| `0x8c02bef0` | `FUN_8c02bef0` | `notes/verifications/0x8c02bef0.json` — 11 oracle steps; r4+=24 (store uses the mutated r4), r0=0x8c02bf10, fr0=clamp(fr4,-1,1), T=last fcmp; f32 store at [r4+24] in rts delay slot |
| `0x8c027500` | `FUN_8c027500` | `notes/verifications/0x8c027500.json` — 24 oracle steps; r7=r4+0x114+r5*8, r3=r4+0x144, r5<<=2, r1=[r4+0x164], r0=r5<<2, T=bit29 of r5 (shll); 4 u16 stores at r7+0..6 + u32 store at r3+r0 in rts delay slot |
| `0x8c02753c` | `FUN_8c02753c` | `notes/verifications/0x8c02753c.json` — 26 oracle steps; r7=[r15] (dest pointer loaded via r15), r5*=8, r4+=0x11a, r6=first u16, r0=last u16, r3=0x118, r1=0x11a, T=bit29 of r5, 4 u16 stores at r7+0..6 (4th is rts delay slot) |
| `0x8c050d90` | `FUN_8c050d90 (cursor step back)` | `notes/verifications/0x8c050d90.json` — 104 oracle steps; r0=p (mov r4,r0); r1=*(p+0x34)-1; r3=copied +0x14 field; r5=*(p+8)+0x14; r6=p+0x14; r7=cur on T=1 branch but cur*4 (shll2 at 0x8c050e60) on T=0; delay-slot stores of both bras and of rts all fire; stores to p+0x24,+0x28,+8,+0xc,+0x10,+0x14..0x1c,+0x34 in pc order |
| `0x8c050ea0` | `FUN_8c050ea0 (cursor step fwd)` | `notes/verifications/0x8c050ea0.json` — 102 oracle steps; first store is *(p+0x28)=old cur (clobbers +0x28 at 0x8c050ea6); wrap branch stores *(p+0x30) BEFORE *(p+0x24); cmp/gt is SIGNED (count > cur+1); r0=p; r1=*(p+0x34)+1; r3=copied +0x14; r5=*(p+8)+0x14; r6=p+0x14; r7=cur vs cur*4 per branch; exit T is the second cmp/eq, not the first cmp/gt |
| `0x8c050fa8` | `FUN_8c050fa8 (walker reset)` | `notes/verifications/0x8c050fa8.json` — 98 oracle steps; real push/pop of r14 (net identity, stack window); fldi0 clobbers fr0 to +0.0 and fmov.s stores 4 bytes 0 at p+0x38 (decompiler's fr1/FPSCR_SZ double-store branch does not exist in machine code); r0=*(src+0x1c); r1=*(src+0x14); r3=*(p+8)+0x14; r5=p+0x14; r6=entries[1] (p+0x38 only transient); r7=sel*4; T untouched by any instruction |
| `0x8c011304` | `FUN_8c011304 (strncmp)` | `notes/verifications/0x8c011304.json` — 33 oracle steps; r7=r5 via bf.s delay slot even on n==0 (returns 0, T set); n>0: r4 past last byte read, r7 decremented back, r2=mismatched src byte or address r4-1, r3=zero-extended src byte, r5=count; T clear only on mismatch exit; r0=(u8)s1[k-1]-(u8)s2[k-1] |
| `0x8c055fec` | `FUN_8c055fec` | `notes/verifications/0x8c055fec.json` — 4 oracle steps; r0=[r4+0x38]+8; store [r0]=fr4 |
| `0x8c055ff4` | `FUN_8c055ff4` | `notes/verifications/0x8c055ff4.json` — 8 oracle steps; r0=[r4+0x38]+4; fr0=second float; stores [r5],[r6] |
| `0x8c056004` | `FUN_8c056004` | `notes/verifications/0x8c056004.json` — 4 oracle steps; r0=[r4+0x38]+12; fr0=[r0] |
| `0x8c05600c` | `FUN_8c05600c` | `notes/verifications/0x8c05600c.json` — 4 oracle steps; r0=[r4+0x38]+12; store [r0]=fr4 |
| `0x8c056014` | `FUN_8c056014` | `notes/verifications/0x8c056014.json` — 6 oracle steps; r0=[r4+0x38]+4; stores [base]=fr4,[base+4]=fr5 |
| `0x8c056020` | `FUN_8c056020` | `notes/verifications/0x8c056020.json` — 3 oracle steps; r0=[ [r4+0x38]+16 ] |
| `0x8c056028` | `FUN_8c056028` | `notes/verifications/0x8c056028.json` — 3 oracle steps; r0=[r4+0x38]; store [r0+16]=r5 |
| `0x8c0560f8` | `FUN_8c0560f8` | `notes/verifications/0x8c0560f8.json` — 49 oracle steps; r0=r4+12; r1=r4+8; fr0=[r4] raw; fr1=clamp; T=last cmp; stores at r4+0,4,8,12 |
| `0x8c056244` | `FUN_8c056244` | `notes/verifications/0x8c056244.json` — 59 oracle steps; r0=out+4; r3=out+8; r1=r4o+4; r4=r4o+12; fr0..fr8 leftovers; T=0>fr2 (or 1 on early exit); stores [out],[out+4],[out+8] |
| `0x8c0562f4` | `FUN_8c0562f4` | `notes/verifications/0x8c0562f4.json` — 20 oracle steps; r0=0x8c056324; r1=r4o+8; r3=r4o+4; r4=r4o+12; fr1=c2; fr2=c*c2; fr3=b*c1; fr0=sum |
| `0x8c010cf8` | `FUN_8c010cf8` | `notes/verifications/0x8c010cf8.json` — 16 oracle steps; zero-input branch only (fr4==0: *r4=0, r6=fr4bits&0x7f800000, fr0=fr4, T=1); non-zero branches blocked on global 0x8c23d8b8 outside image |
| `0x8c018474` | `FUN_8c018484` | `notes/verifications/0x8c018474.json` — 12 oracle steps; "*r4 |= 0x80 (tas.b) and T ends set; entry is 8 nops before the tas/bf loop; no register changes; only a zero byte terminates the spin" |
| `0x8c02c2a4` | `FUN_8c02c2a4` | `notes/verifications/0x8c02c2a4.json` — 143 oracle steps; "r4 advanced 4*0xac (bt.s delay `add r3,r4` runs on the final pass); r5 stays p0+0x3e0 for all 4 iterations; r14/r15 preserved via stack; r6=last p+0x70, r0=last p+0xcc; fr4 left negated; T ends clear (4>4 false)" |
| `0x8c04b360` | `FUN_8c04b360` | `notes/verifications/0x8c04b360.json` — 7 oracle steps; "r0 transiently holds pool offsets 0x98/0x9c before the result; ==6 path leaves r0 == T (movt); byte path leaves T clear; extu.b is the rts delay slot" |
| `0x8c051cfc` | `FUN_8c051cfc` | `notes/verifications/0x8c051cfc.json` — 2 oracle steps; "none: rts;nop stub, zero register/T/memory effects" |
| `0x8c051f5c` | `FUN_8c051f5c` | `notes/verifications/0x8c051f5c.json` — 3 oracle steps; "r0 passes through pool constant 0x194 before the delay-slot load overwrites it with *(u32*)(r4+0x194)" |
| `0x8c013bea` | `FUN_8c013bea` | `notes/verifications/0x8c013bea*.json` — both branches (8 and 5 oracle steps); taken: r2=*(r4+4), r3=0, byte *r2=0; T=(*r4==1) |
| `0x8c013bfa` | `FUN_8c013bfa` | `notes/verifications/0x8c013bfa*.json` — both branches (9 and 5 oracle steps); eq: r0=0; else r0=mem.b[*(r4+4)+16]&0x40, r1=16 |
| `0x8c013226` | `FUN_8c013226` | `notes/verifications/0x8c013226.*.json` — both branches (9 and 13 oracle steps); clear: *(r4+4)-=1 in r2; set: r1=*(r4+4), *(r1+4)+=1, **r1-=1 in r3 |
| `0x8c011cf8` | `FUN_8c011cf8` | `notes/verifications/0x8c011cf8.json` — 44 oracle steps; fr6=fr4^2, 5x fmac over table at 0x8c1cd028, r4=0, r5=0x8c1cd040, fr0=fr4, fr6=fr5 |
| `0x8c011c50` | `FUN_8c011c50` | `notes/verifications/0x8c011c50*.json` — all branches (34, 15, 6 oracle steps); r6=count, r2=last byte, r4 advanced, r0=0/1, T=last cmp/pl or cmp/ge |
| `0x8c0163e8` | `FUN_8c0163e8` | `notes/verifications/0x8c0163e8.json` — 39 oracle steps; record copy while cmp/str clear, 4-byte tail unrolled (4th byte sets no flag, T from 3rd); r3/r4/r5/r6 end as popped stack zeros, r15 net +8, r0=orig dst |
| `0x8c015194` | `FUN_8c015194` | `notes/verifications/0x8c015194*.json` — both specs (74 and 12 oracle steps); r7=8-r6/carry, r12=mask, r3=-exts.w(r6), r2/r1 last byte ops, r0=(final carry != 0), T=(carry==0); r12/r13/r14 net unchanged |
| `0x8c05dc1c` | `FUN_8c05dc1c` | `notes/verifications/0x8c05dc1c.json` — ok; r1=*r15, r0=**r4, store *r1=r0 in rts delay slot |
| `0x8c05ef7c` | `FUN_8c05ef7c` | `notes/verifications/0x8c05ef7c.*.json` — both branches; set: *(r4+8)\|=r5; clr: r1=*(r4+8)&~r5; T=(r6 byte==0) |
| `0x8c05e038` | `FUN_8c05e038` | `notes/verifications/0x8c05e038.*.json` — both branches; byte loop, r0=diff or 0 at NUL, r4/r5 advanced, T=last cmp/eq |
| `0x8c05de64` | `FUN_8c05de64` | `notes/verifications/0x8c05de64.json` — ok; struct init, [0]=pool value (single deref), [4]=0, [8]=byte(r5), [12]=r4+4; r0=r6, r4=0 |
| `0x8c05e108` | `FUN_8c05e108` | `notes/verifications/0x8c05e108.json` — ok; same shape, pool 0x8c1f7528 |
| `0x8c060e4c` | `FUN_8c060e4c` | `notes/verifications/0x8c060e4c.json` — ok; r4+=4, fr4 stored at new r4 in rts delay slot |
| `0x8c0616bc` | `FUN_8c0616bc` | `notes/verifications/0x8c0616bc.json` — ok; r0=*(u32*)(0x268+r4) in rts delay slot |
| `0x8c061764` | `FUN_8c061764` | `notes/verifications/0x8c061764.json` — ok; bare rts;nop stub, zero effects |
| `0x8c06be1c` | `FUN_8c06be1c` | `notes/verifications/0x8c06be1c*.json` — both branches; phase1 stores *(r4+68) at abs 68+*(r4+72) or at *r5; phase2 r4+=72, store at abs 72+*(r4+68); T=last tst |
| `0x8c06d1e8` | `FUN_8c06d1e8` | `notes/verifications/0x8c06d1e8.json` — all-clear path ok; taken branches store fr4 at 0x8c2400f0..0x8c240120 (outside image, unpinnable); r4 &= 0x04000000 in place, T=(r4==0) |
| `0x8c06d244` | `FUN_8c06d244` | `notes/verifications/0x8c06d244.json` — all-clear path ok; globals 0x8c2400f8..0x8c240128 outside image; same residue shape |
| `0x8c051f68` | `FUN_8c051f68` | `notes/verifications/0x8c051f68.json` — 3 oracle steps; "store *(u32*)(r4+0x194)=r5 is the rts delay slot; r0 left holding 0x194, not the stored value" |
| `0x8c014c1c` | `FUN_8c014c1c` | `notes/verifications/0x8c014c1c.json` — 79 oracle steps; "r1 = shad(n,0xff) mask left; r7 = final carry byte; r3 = -n; r5 ends r4-1 (r4+len-1 if loop skipped); r0 on the n==0 path is the caller's incoming r0; r14/r15 preserved via stack" |
| `0x8c014da4` | `FUN_8c014da4` | `notes/verifications/0x8c014da4.json` — 38 oracle steps; "r0=r6=r4+4 left; r7=-8 (delay `add #-8,r7` runs on final pass); r3=0; r2 keeps unshifted word (last shld amt 0); T left set" |
| `0x8c014dc0` | `FUN_8c014dc0` | `notes/verifications/0x8c014dc0.json` — 24 oracle steps; "r0=r6=r4+2 left; r7=-8; r2=0; r3=zero-extended u16; T left set; mov.w @r5 sign-extends before extu.w re-truncates" |
| `0x8c02c8d4` | `FUN_8c02c8d4` | `notes/verifications/0x8c02c8d4.json` — 83 oracle steps; writes byte@base+0x464, f32@base+0x460/0x468; r4 advanced to base+0x460 (early) or base+0x468; r6=base+0x460 used by rts delay slot; T=last fcmp/cmp of path |
| `0x8c032ba0` | `FUN_8c032ba0` | `notes/verifications/0x8c032ba0.json` — 85 oracle steps; writes f32@base+0x390/0x394/0x380 (block1) and base+0xe4/0xe8/0x174/0x178 (block2); r1 deref'd through pool ptr 0x8c032cb0 in bt.s delay slot on BOTH paths; r1 advanced by mode offset 0x88/0x98/0xa8/0xb8 via bra delay slots; r4 ends base+0x178; r5=r1+4; T=last tst/cmp |
| `0x8c033618` | `FUN_8c033618` | `notes/verifications/0x8c033618.json` — 128 oracle steps; writes 16 f32 fields incl. base+0x3e4 twice (write, reread, *1000, rewrite); r4 advanced base+0x11c; r13/r14 saved via r15 and restored; r14=0x3e4 scratch; constants via mova/pool reads |
| `0x8c03376c` | `FUN_8c03376c` | `notes/verifications/0x8c03376c.json` — 128 oracle steps; same 16 f32 stores as 0x8c033618 but divides by base+0x278, multiplies base+0x364 by C3, ends ([base+0x31c]*[base+0x358]/([base+0x11c]*pi))*[base+0x3e4]/1000; r4 advanced base+0x11c; r13/r14 via stack |
| `0x8c05665c` | `FUN_8c05665c` | `notes/verifications/0x8c05665c.json` — 2 oracle steps; r0=r4 comes from the rts delay slot (mov r4,r0), not from a visible move in the C body; T untouched |
| `0x8c0567ac` | `FUN_8c0567ac` | `notes/verifications/0x8c0567ac.json` — 23 oracle steps; r3 keeps the end sentinel r1+count*0x38; r1 ends at first 0x38-step >= r3; r0=0; store [r4+4]=0 is the rts delay slot; T ends 0 on every path (all exits are non-hi cmp/hi) |
| `0x8c056b18` | `FUN_8c056b18` | `notes/verifications/0x8c056b18.json` — 172 oracle steps; r1=dst+48, r3=src+44, r7=dst+44 left after each copied record (untouched when r6==0 skips the copy); r4=r5, r6+=56 per record even when copy skipped; r0=final r6; fr0=bits of src+48; r13/r14 restored by pops, r15 net 0; T=1 (loop exits only via cmp/eq equal) |
| `0x8c056bf8` | `FUN_8c056bf8` | `notes/verifications/0x8c056bf8.json` — 4 oracle steps; r0 ends as the argument r4 (rts delay slot mov r4,r0), not the pool value 0x8c1e8f88 that was stored; store [r4] happens before rts |
| `0x8c056c04` | `FUN_8c056c04` | `notes/verifications/0x8c056c04.json` — 6 oracle steps; r1 keeps first dword [0x8c23dd60]; r0 ends as second dword [0x8c23dd64]; [r4+4] store is the rts delay slot |
| `0x8c056c14` | `FUN_8c056c14` | `notes/verifications/0x8c056c14.json` — 3 oracle steps; the load mov.l @r0,r0 is the rts delay slot, so r0=[0x8c23dd50], not the pool pointer |
| `0x8c056c20` | `FUN_8c056c20` | `notes/verifications/0x8c056c20.json` — 33 oracle steps; r4=param+12; r1=fpul<<16; r0=0x8c1f6fc0 pool ptr; fpul=last ftrc; fr0/fr1/fr2=x3*c/x2*c/x1*c, fr3=255.0f from [0x8c056c64]; store at [0x8c1f6fc0] is rts delay slot; T never touched (shll8/16, or don't set T here) |
| `0x8c0572fc` | `FUN_8c0572fc` | `notes/verifications/0x8c0572fc.json` — 33 oracle steps; r4 is loaded from @(16,r15) = the dword at the ORIGINAL r15 (caller stack), not an argument; r2=r15-4, r3=r15-16 survive; r15 restored by rts delay slot add #16,r15; fr0/fr1 keep bits of [0x8c23dd6c]/[0x8c23dd64]; 4 spill stores at r15-16..r15-4 are real writes |
| `0x8c0573f4` | `FUN_8c0573f4` | `notes/verifications/0x8c0573f4.json` — 10 oracle steps; r0=[r4+12], r1=[r4+8], r3=0x8c23dd80 left behind; [0x8c23dd80+12] store is the rts delay slot |
| `0x8c05799c` | `FUN_8c05799c` | `notes/verifications/0x8c05799c.json` — 3 oracle steps; r0=76 (the displacement) survives; store [r4+76]=r5 is the rts delay slot (mov.l r5,@(r0,r4)) |
| `0x8c028da8` | `FUN_8c028da8` | `notes/verifications/0x8c028da8.json` — 8 oracle steps; r1=chosen offset, r0=0x8c1f47c0+offset via rts delay-slot add, T=0 on matched-bit paths / 1 on default, r5 clobbered to r5&0x80 on paths reaching the last test |
| `0x8c02d5e4` | `FUN_8c02d5e4` | `notes/verifications/0x8c02d5e4.json` — 78 oracle steps; stores: s+0x3d4 once, s+0x3dc four times (only last survives), s+0x3d8 once; T from idx normalization (1 for idx -1 or 0, else 0); r0=0x8c02d6b4 (mova), r1=w+0x360, r4=s+0x3d8, r5=0xec, r6=w+0x33c+4*idx, r7=0x3d8; fr0/fr1=last load/product, fr2=stored value, fr3/fr4=table floats |
| `0x8c03e534` | `FUN_8c03e534` | `notes/verifications/0x8c03e534.json` — 23 oracle steps; stores: 17 u32 into slot + [s+0x80]=new current (reset to base if == end) or byte s+0x44=0 on blind path; r5 advanced by 68 (or reloaded to base on wrap path), r3/r6/r7=scan ptrs, r1=current/reset, fr0=0.95 const, fr1=last slot float, T=last cmp/eq or cmp/hi |
| `0x8c050718` | `FUN_8c050718` | `notes/verifications/0x8c050718.json` — 88 oracle steps; stores: [r4+0x24]=r5 (bf.s delay slot, both paths), [r4+0x28], [r4+8], [r4+0xc], [r4+0x10], 3 u32 to [r4+0x14..0x1c], [r4+0x34]=r5 (rts delay slot); T = (count-1==r5); r5=r4+0x14, r3=[r4+8]+0x14, r6=4*idx1, r7=r5 or 4*r5, r0=r5, r1=first copied u32 |
| `0x8c058128` | `FUN_8c058128` | `notes/verifications/0x8c058128.json` — 2 oracle steps; delay-slot load: r0=*(r4+0x38); T untouched |
| `0x8c059054` | `FUN_8c059054` | `notes/verifications/0x8c059054.json` — 6 oracle steps; r4+=0x44 (permanent); r1=*(r4+0x44) kept; r5<<=2 (T unchanged); r0=*(r1+(r5<<2)) via delay slot |
| `0x8c05992c` | `FUN_8c05992c` | `notes/verifications/0x8c05992c.json` — 3 oracle steps; r0=*(r4+4) then delay-slot mov.b @(5,r0),r0 sign-extends over it |
| `0x8c059934` | `FUN_8c059934` | `notes/verifications/0x8c059934.json` — 4 oracle steps; r1=*(r4+4) kept; r0=0; delay-slot store byte [r1+5]=0 |
| `0x8c05993c` | `FUN_8c05993c` | `notes/verifications/0x8c05993c.json` — 4 oracle steps; r1=*(r4+4) kept; r0=1; delay-slot store byte [r1+5]=1 |
| `0x8c0599f0` | `FUN_8c0599f0` | `notes/verifications/0x8c0599f0.json` — 3 oracle steps; r0=*(r4+4) then delay-slot mov.b @(4,r0),r0 sign-extends over it |
| `0x8c059a88` | `FUN_8c059a88` | `notes/verifications/0x8c059a88.json` — 30 oracle steps; r4/r5 permanently advanced per byte compared (r5 in bt.s delay slot, runs even on mismatch); r3=last byte from r4; r1=byte diff (0 on NUL exit); r0=T of final tst r1,r1 (1=r4 string ended, 0=mismatch) |
| `0x8c059adc` | `FUN_8c059adc` | `notes/verifications/0x8c059adc.json` — 3 oracle steps; r0=mem[0x8c059ae4]=0x8c23de00 then delay-slot r0=*(0x8c23de00) |
| `0x8c059ae8` | `FUN_8c059ae8` | `notes/verifications/0x8c059ae8.json` — 3 oracle steps; r0=mem[0x8c059af0]=0x8c23de00 kept; delay-slot store *(0x8c23de00)=r4 |
| `0x8c059b30` | `FUN_8c059b30` | `notes/verifications/0x8c059b30.json` — 6 oracle steps; r1=orig *r4 kept; mem[r4]=*(*r4); r0=*sp; delay-slot store mem[*sp]=r1 (outside stack window when *sp points to RAM) |
| `0x8c059b3c` | `FUN_8c059b3c` | `notes/verifications/0x8c059b3c.json` — 4 oracle steps; r1=[r15], r0=[r4] left live; store to [r1] in rts delay slot |
| `0x8c059b44` | `FUN_8c059b44` | `notes/verifications/0x8c059b44.json` — 5 oracle steps; r1=[r15], r0=[[r4]] left live; store to [r1] in rts delay slot |
| `0x8c059ca4` | `FUN_8c059ca4` | `notes/verifications/0x8c059ca4.json` — 2 oracle steps; none |
| `0x8c059ca8` | `FUN_8c059ca8` | `notes/verifications/0x8c059ca8.json` — 3 oracle steps; r0=[r4+0x38] then reloaded to [r0] in rts delay slot |
| `0x8c059f84` | `FUN_8c059f84` | `notes/verifications/0x8c059f84.json` — 3 oracle steps; r0=0x8c23de08 (literal pool) left live; [0x8c23de08]=r4 in rts delay slot |
| `0x8c05ab8c` | `FUN_8c05ab8c` | `notes/verifications/0x8c05ab8c.json` — 130 oracle steps; r0=4,r5=0,r6=4,r7=3,r1=base+0x2c,r3=base+0x38,fr0/fr1=last pre-swap dwords,T=0; 6 dword swaps at base+{4,8,12}x{16,32,48}; r11-r14 pushed/popped (net zero) |
| `0x8c05abd8` | `FUN_8c05abd8` | `notes/verifications/0x8c05abd8.json` — 3 oracle steps; r0=[r4+4] then reloaded to [r0] in rts delay slot |
| `0x8c05abe0` | `FUN_8c05abe0` | `notes/verifications/0x8c05abe0.json` — 3 oracle steps; r0=[r4+4] then reloaded to [r0+4] in rts delay slot |
| `0x8c05ac20` | `FUN_8c05ac20` | `notes/verifications/0x8c05ac20.json` — 7 oracle steps; r0=base+0x10 left live; base reloaded from r4+4 for 2nd store; [base+0xc]=fr4, [base+0x10]=fr5 (2nd in rts delay slot) |
| `0x8c05ac30` | `FUN_8c05ac30` | `notes/verifications/0x8c05ac30.json` — 3 oracle steps; r0=[r4+4] left live; [r0+0x14]=r5 in rts delay slot |
| `0x8c012260` | `FUN_8c012260 (classify fr4 bits)` | `notes/verifications/0x8c012260.json` — 15 oracle steps; verified: r15 spilled -8/+8, r4/r2 keep stack addresses (r4 drops 4 more on the return-2 path), r5=0x7ff00000, r3=last tested word, T=last cmp/eq or tst (1 on return 0/2 paths, 0 on return 1); two fmov stores land in the stack window; return 0 when fr4 finite, 2 when fr4 is exactly +/-inf and fr5 bits are 0, else 1 |
| `0x8c0146bc` | `FUN_8c0146bc (ftrc-based float normalise)` | `notes/verifications/0x8c0146bc.json` — 26 oracle steps; verified positive large-exponent path (fr4=16777216.0): r15 spilled/restored, r2=r15+4, r3=last word, r4=sign bit of fr4, r5=masked exponent, r6=0x80000000, T=tst r3,r6 (1 positive); fr0=fr4; fmov stores land in stack window |
| `0x8c011d1c` | `FUN_8c011d1c (Horner fr4^2 over table 0x8c1cd040)` | `notes/verifications/0x8c011d1c.json` — 49 oracle steps; BLOCKED: oracle lacks fmov.s @Rm+,FRn (0xf459 hit at 0x8c011d24). Port written: loop target is 0x8c011d26 so fr4 is loaded once pre-loop then carries the accumulator via the bt.s delay slot; r5 ends 0x8c1cd05c, fr4=fr0=acc, fr5=fr4_input^2, T=0 |
| `0x8c011d3c` | `FUN_8c011d3c (Horner fr4^2 over table 0x8c1cd060)` | `notes/verifications/0x8c011d3c.json` — 51 oracle steps; BLOCKED: oracle lacks fmov.s @Rm+,FRn (0xf559 hit at 0x8c011d44). Port written: 6 iterations, r5 ends 0x8c1cd07c, fr5=fr6=acc, fr0=fr4*acc in rts delay slot, T=0 |
| `0x8c05ac38` | `FUN_8c05ac38` | `notes/verifications/0x8c05ac38.json` — 3 oracle steps; r0 = double deref [[r4+4]+0x24] via rts delay slot |
| `0x8c05ad44` | `FUN_8c05ad44` | `notes/verifications/0x8c05ad44.json` — 3 oracle steps; r0 = *0x8c23de10 via literal pool at 0x8c05ad4c + rts delay slot |
| `0x8c05b2ac` | `FUN_8c05b2ac` | `notes/verifications/0x8c05b2ac.json` — 5 oracle steps; T = ([r4+0x24]==0); null path r0 = [0x8c05b2bc] = 0x40000840 |
| `0x8c05cb04` | `FUN_8c05cb04` | `notes/verifications/0x8c05cb04.json` — 12 oracle steps; r4 = link addr of matched node, r0 = 0 and T = 1 on every exit, r1 = old [r5+8] only on match |
| `0x8c05cb24` | `FUN_8c05cb24` | `notes/verifications/0x8c05cb24.json` — 6 oracle steps; prepend: r0 = old head; walk: r0 = last node, r1 = 0; T = 1 on all paths |
| `0x8c05d274` | `FUN_8c05d274` | `notes/verifications/0x8c05d274.json` — 7 oracle steps; extu.b r6 decides (upper bytes ignored); set: r0 = old|mask; clear: r0 = ~mask, r1 = old&~mask; T = 0/1 |
| `0x8c05d28c` | `FUN_8c05d28c` | `notes/verifications/0x8c05d28c.json` — 2 oracle steps; none |
| `0x8c05d3bc` | `FUN_8c05d3bc` | `notes/verifications/0x8c05d3bc.json` — 3 oracle steps; r0 = *0x8c1f72a0 via literal pool at 0x8c05d3c4 + rts delay slot |
| `0x8c05d60c` | `FUN_8c05d60c` | `notes/verifications/0x8c05d60c.json` — 4 oracle steps; r1 = [r15] dest ptr, r0 = [r4] value; store in rts delay slot |
| `0x8c05dbfc` | `FUN_8c05dbfc` | `notes/verifications/0x8c05dbfc.json` — 4 oracle steps; r1 = [r5] src, r0 = original r4; store in rts delay slot |
| `0x8c027580` | `FUN_8c027580` | `notes/verifications/0x8c027580.json` — 312 oracle steps; stores: 6x(4xu16 p2->p1 at +0xb4/0xe4/0x114+off, u32 p2->p1 at +0x144+dw) in bt.s-delay-slot order; r0=20 (pre-inc r9), r1=last u32, r3/r6/r7=advanced +0x114/+0xe4 ptrs, r4/r5=+0x144 ptrs, T=0; r8-r15 restored by epilogue |
| `0x8c05f4f8` | `FUN_8c05f4f8` | `notes/verifications/0x8c05f4f8.json` — 3 oracle steps; r4 += 24 (arg register mutated); r0 = r4 set in rts delay slot |
| `0x8c05f500` | `FUN_8c05f500` | `notes/verifications/0x8c05f500.json` — 4 oracle steps; r4 += 16; r0 = zero-extended byte loaded mid-body (not in delay slot) |
| `0x8c05f508` | `FUN_8c05f508` | `notes/verifications/0x8c05f508.json` — 4 oracle steps; r0 = 16, r1 = 0 left behind; store is the rts delay slot mov.b r1,@(r0,r4) |
| `0x8c05f510` | `FUN_8c05f510` | `notes/verifications/0x8c05f510.json` — 4 oracle steps; r0 = 16, r1 = 1 left behind; store is the rts delay slot mov.b r1,@(r0,r4) |
| `0x8c05f51c` | `FUN_8c05f51c` | `notes/verifications/0x8c05f51c.json` — 3 oracle steps; r4 += 8 (arg register mutated); r0 = r4 set in rts delay slot |
| `0x8c05fa50` | `FUN_8c05fa50` | `notes/verifications/0x8c05fa50.json` — 3 oracle steps; r0 = mem32[mem32[0x8c05fa58]]; real load is the rts delay slot mov.l @r0,r0 |
| `0x8c05fcd0` | `FUN_8c05fcd0` | `notes/verifications/0x8c05fcd0.json` — 19 oracle steps; r4 = 0, r6 = param, r3 = param+8, r1 = param+4, r0 = param at exit; [r6+12] written twice (0 then param+4) |
| `0x8c060d84` | `FUN_8c060d84` | `notes/verifications/0x8c060d84.json` — 21 oracle steps; dst from [r15], src from [r4]; r6=dst+12, r5=src+12, r4=dst+4, r3=src+4, r1=dst+8, r0=src+8, fr0 = raw bits of mem32[src+12] |
| `0x8c060db0` | `FUN_8c060db0` | `notes/verifications/0x8c060db0.json` — 10 oracle steps; r3 = dst = [r4], r1 = mem32[r5+8], r0 = mem32[r5+12] at exit |
| `0x8c060e40` | `FUN_8c060e40` | `notes/verifications/0x8c060e40.json` — 6 oracle steps; r4 ends at r4+4 (add between the two loads); fr0 = raw bits of mem32[r4+4] |
| `0x8c013e94` | `FUN_8c013e94` | `notes/verifications/0x8c013e94.json` — 141 oracle steps; fr4 clobbered to NaN const 0x7f800001 by the bf.s delay slot at 0x8c013ed8 on EVERY path; negative input returns fr0=NaN; 1.0 special-case returns fr0=0.0; fpul=r6=unbiased exponent (e.g. 1 for 2.0) on normal path; r5=8, r7=1 constants left; r5 ends 0x8c1cd1e8 after @r5+ loop; r0 ends as FULL original fr4 bits (and r0,r6 is r6&=r0, not r0&=r6); r1=0x8c1cd1c4, r2=0x8c1ccfc8, r3=0x8c1cd1c0 leftovers; T=last cmp/pl r4 (=0) on normal path |
| `0x8c014810` | `FUN_8c014810` | `notes/verifications/0x8c014810.json` — 41 oracle steps; fr4 overwritten with 0.0 on the normal path (0x8c01484c); fr5=-Inf and fr6=-0.0 loaded unconditionally via branch delay slots; fr7=+Inf always; r0=0x7fffff only on Inf/NaN path; r14 saved/restored through stack, r6=r15-8; r1=0x8c1ccfd0, r2/r3/r5/r7 literal leftovers; T ends per path (1 for +Inf, +0, and positive-sign clamps; 0 for -Inf, -0, negative-sign clamps, NaN, build) |
| `0x8c057ec4` | `FUN_8c057ec4` | `notes/verifications/0x8c057ec4.json` — 27 oracle steps; r1=r5+4 and r0=r4+20 leftovers (r0 computed BEFORE r4+=8 on the last else path); r5+=8 on every path incl. early rts; r4+=8 only on last block's else; fr0/fr1 keep the last block's loaded pair; T=last fcmp/gt; stores go to hi slot (r4+12/16/20) when fr1>hi else lo slot (r4+0/4/8) when lo>fr1 |
| `0x8c058f5c` | `FUN_8c058f5c` | `notes/verifications/0x8c058f5c.json` — 29 oracle steps; r0/r1 clobbered every block - early exits leave r0=0 (rts delay mov #0,r0) with THAT block's r1=r4+off/r0=r5+off, final path leaves r1=r4+12; fr0/fr1 keep last compared pair; T=last fcmp/eq; return is movt r0 in rts delay slot |
| `0x8c051140` | `FUN_8c051140` | `notes/verifications/0x8c051140.json` — 60 oracle steps; fr0=src[15] (no FPSCR.SZ branch in machine code, plain fmov); r0=dst, r1=counter+1, r3=&counter, r5=src+60, r6=src+56, r7=dst+56; store order 0,4,32,8,12,16,44,20,24,28,36,40,48,52,56,60,counter; T untouched |
| `0x8c0516cc` | `FUN_8c0516cc` | `notes/verifications/0x8c0516cc.json` — 110 oracle steps; fpul=r5 (lds); fr2=(float)(i32)r5 from entry bra delay slot; fr0=fr2/3.6f per iter; r1=base1, r5=0xfffffeff, r3=i, r6=4i, r7=table2[idx], r0=count on exit; T=0; stores at 80*e+base (AND mask) and 80*e+base+16 (float) |
| `0x8c0525a4` | `FUN_8c0525a4` | `notes/verifications/0x8c0525a4.json` — 4 oracle steps; the +0x24 load runs in the rts delay slot chained off the first load; only r0 changes; offset 0x190 from PC-relative literal at 0x8c0525ac |
| `0x8c05264c` | `FUN_8c05264c` | `notes/verifications/0x8c05264c.json` — 2 oracle steps; none - bare rts;nop, zero register/flag/memory effect |
| `0x8c052c88` | `FUN_8c052c88` | `notes/verifications/0x8c052c88.json` — 13 oracle steps; third byte store is in the rts delay slot; r0 ends 0x1f2 (last literal), r1=last byte via extu.b; offsets 0x1f0-0x1f2 from PC-relative literals; T untouched |
| `0x8c053be0` | `FUN_8c053be0` | `notes/verifications/0x8c053be0.json` — 2 oracle steps; r0=r4 via rts delay slot (C shows plain return) |
| `0x8c055fe4` | `FUN_8c055fe4` | `notes/verifications/0x8c055fe4.json` — 4 oracle steps; rts delay slot fmov @r0,fr0 loads from the INCREMENTED r0 (ptr+8); fr0+r0 change, T untouched; no FPSCR.SZ double path in machine code |
| `0x8c017c0c` | `FUN_8c017c0c` | `notes/verifications/0x8c017c0c.json` — 38 oracle steps; r4 itself ends advanced (delay-slot mov r4,r0 mirrors it into r0); r6=decoded value; r3=b1<<16 and r1=b2<<8 (shll16/shll8 in place); T=bit2 of first byte (1 only on 4-byte path); follow-up bytes read at r4+1..3 (group-8 base=bits 7-4, disp=bits 3-0) |
| `0x8c05f29c` | `FUN_8c05f29c` | `notes/verifications/0x8c05f29c.json` — 162 oracle steps; r1=*0x8c26cff0 (eq path) or [r5+52] (copy path); r3=r4+56; r5+=12 on copy path; r6=0x8c1d0f24; r7=r4+52; fr0=+0.0 (fldi0); fr1=table[3]; T=(r4==r5); header masks at +4/+5 are dead stores (overwritten by mov.l at 0x8c05f37c / copy block); bt.s delay slot stores 0 at +0 on both paths; r8..r14 restored by epilogue pops |
| `0x8c05f3f4` | `FUN_8c05f3f4` | `notes/verifications/0x8c05f3f4.json` — 120 oracle steps; r1=r4+4; r3=r4+56; r5=0x8c1d0f24; r6=0; r7=0x8c26cff0; fr0=+0.0 (fldi0); fr1=table[3]; T untouched (no flag ops); header masks at +4/+5 dead (overwritten at 0x8c05f4cc); r8..r14 restored by pops |
| `0x8c05e8cc` | `FUN_8c05e8cc` | `notes/verifications/0x8c05e8cc.json` — 15 oracle steps; list path: r1=0, r0=last node, T=1, [r4]=node in rts delay slot; hash path: r5=0xfffffffe (from [0x8c05e908]), r3/r6=bucket chain, r1=next&~1, r0=[bucket]==r3, T=0 (shlr+shll pair), [r4] stored every iteration |
| `0x8c05ef44` | `FUN_8c05ef44` | `notes/verifications/0x8c05ef44.json` — 17 oracle steps; fr0=1.0; [r4+28]=fr4 always (bf.s delay slot); r3=0xffffff8f; r1=0xfffffff0; r0=sign-extended final byte; T=fcmp/eq result; byte [r4+4] stored twice per path |
| `0x8c05f21c` | `FUN_8c05f21c` | `notes/verifications/0x8c05f21c.json` — 28 oracle steps; r4+=12; r1=[r5+40]; r0=[r5+48]; 13 dword stores to r4+12.. in order |
| `0x8c05f294` | `FUN_8c05f294` | `notes/verifications/0x8c05f294.json` — 3 oracle steps; r4+=4; r0=new r4 (rts delay slot) |
| `0x8c05a828` | `FUN_8c05a828` | `notes/verifications/0x8c05a828.json` — 238 oracle steps; builds 4x4 float matrix at r4 from lit 2.0f@0x8c05a94c and fr4/fr5/fr6/fr7, transposes in place; final r0=4, r1=4, r2=sp+12, r3=r4+64, r5=16, r6=r4+44, r7=r4+56; r10-r14 restored by pops; T=0; fr0=last swapped val, fr1=0, fr2=1/(fr5-fr4), fr3=1, fr4=-fr4*fr2, fr5=fr5-fr4 |
| `0x8c06be50` | `unknown` | `notes/verifications/0x8c06be50.json` — 13 oracle steps |
| `0x8c06116c` | `FUN_8c06116c` | `notes/verifications/0x8c06116c.json` — 3 oracle steps; r0 = 0x8c23dea0 (pointer load survives); store r4 at 0x8c23dea0 in rts delay slot |
| `0x8c061768` | `FUN_8c061768` | `notes/verifications/0x8c061768.json` — 5 oracle steps; r0 = 0x8c1f7a18 base; r4 = base + (r4<<2); store fr4 at r4 in rts delay slot |
| `0x8c061be8` | `FUN_8c061be8` | `notes/verifications/0x8c061be8.json` — 2 oracle steps; none (rts; nop) |
| `0x8c0654b4` | `FUN_8c0654b4` | `notes/verifications/0x8c0654b4.json` — 3 oracle steps; r4 += 0x40 (wrapping, mutates the argument); r0 = incremented r4 in rts delay slot |
| `0x8c067f08` | `FUN_8c067f08` | `notes/verifications/0x8c067f08.json` — 4 oracle steps; r0 = 0x9c, r1 = 1 survive; byte store mem[0x9c+r4] = 1 in rts delay slot |
| `0x8c069260` | `FUN_8c069260` | `notes/verifications/0x8c069260.json` — 4 oracle steps; T = last cmp/gt; r6 = r4 on the fall path; r0 = r5/r6/r4 per path |
| `0x8c069c74` | `FUN_8c069c74` | `notes/verifications/0x8c069c74.json` — 25 oracle steps; r6 = out+12, r0 = out+8, r1 = out+4, r3 = in+4, r5 = in+8; fr0..fr3 overwritten with scaled values; 4 stores incl. [out+12] in rts delay slot |
| `0x8c06c854` | `store_arg_to_global` | `notes/verifications/0x8c06c854.json` — store is the rts delay slot; r0 ends as the pool address 0x8c23f438 |
| `0x8c06caa8` | `advance_arg_12` | `notes/verifications/0x8c06caa8.json` — r4 itself is advanced; r0 is the advanced value |
| `0x8c06cab4` | `advance_arg_4_store_float` | `notes/verifications/0x8c06cab4.json` — r4 advanced before the delay-slot fmov store |
| `0x8c06cabc` | `advance_arg_8_store_float` | `notes/verifications/0x8c06cabc.json` — verified |
| `0x8c06cac4` | `advance_arg_4_load_float` | `notes/verifications/0x8c06cac4.json` — verified |
| `0x8c06cacc` | `advance_arg_8_load_float` | `notes/verifications/0x8c06cacc.json` — verified |
| `0x8c070320` | `offset_0xb0` | `notes/verifications/0x8c070320.json` — verified |
| `0x8c070bd0` | `empty_stub` | `notes/verifications/0x8c070bd0.json` — verified |
| `0x8c070e74` | `load_global_8c240470` | `notes/verifications/0x8c070e74.json` — verified |
| `0x8c071e04` | `load_global_8c2648f0` | `notes/verifications/0x8c071e04.json` — verified |
| `0x8c071e10` | `store_global_8c2648f0` | `notes/verifications/0x8c071e10.json` — verified |
| `0x8c06c160` | `load_global_8c23f470` | `notes/verifications/0x8c06c160.json` — verified |
| `0x8c06f6cc` | `FUN_8c06f6cc` | `notes/verifications/0x8c06f6cc.json` — 131 oracle steps; r0=table+idx*12; r1=r7+4; r3=r6+8; r4=r6+4; r7=r7+8; T=bit31 of shll operand (idx bit15); fr0-fr12 clobbered, fr13-15 restored via stack; reads table ptr from [r15_entry] via @(28,r15) after 7 pushes |
| `0x8c06fdc0` | `FUN_8c06fdc0` | `notes/verifications/0x8c06fdc0.json` — 258 oracle steps; r4 += 0xbc and r5 += 0xd0 at the end (last 3 stores land at old_r4+0xbc..); leftovers r0=src[0xd8], r1=src[0xd0], r2=r15_entry-44, r3=src[0xc8], r6=src[0xc4], r7=old_r5+0xc4, r8=old_r4+0xb0, r9=old_r4+60, r10=old_r4+0xd8; only fr0 clobbered (=f32[old_r5+0xc0]); T unchanged |
| `0x8c070b40` | `store_arg_to_global` | `notes/verifications/0x8c070b40.json` — store is the rts delay slot |
| `0x8c01471c` | `float_classify_scale` | `notes/verifications/0x8c01471c.json` — verified |

Side effects found by the verifier and required by the port: the original
copies the argument into `r4` (not present here) and leaves the `+0x38` field in
`r3`. Neither is visible in the C, which is why the diff is the source of truth.


Nothing recovered yet: no binary. Add rows as evidence lands.

| address | name | kind | evidence |
| --- | --- | --- | --- |

## Blocked

A leaf whose behaviour depends on state the harness does not model cannot be
pinned by a spec. These are recorded, not retried: the oracle maps nothing at
the listed addresses, so any run dies with `data access ... outside mapped
memory` before a diff can mean anything.

| address | reason |
| --- | --- |
| `0x8c17275e` | hardware register access: `a05f688c` |
| `0x8c1aca8c` | hardware register access: `ff000000` |
| `0x8c216b6c` | hardware register access: `ff00` |
| `0x8c21b99a` | hardware register access: `ff00` |
| `0x8c21c414` | hardware register access: `a05f6c00` |
| `0x8c21c532` | hardware register access: `8c00fff0,8c00fff4` |
| `0x8c21cb46` | hardware register access: `ff000000,ff0000fc` |
| `0x8c220168` | hardware register access: `ff000010` |
| `0x8c2203dc` | hardware register access: `a05f688c` |
| `0x8c220614` | hardware register access: `a05f78bc` |
| `0x8c220704` | hardware register access: `a05f688c` |
| `0x8c222640` | hardware register access: `ff00` |
| `0x8c2266c8` | hardware register access: `ff00` |
| `0x8c22f820` | hardware register access: `a05f8000` |
| `0x8c22f82a` | hardware register access: `a05f8000` |
| `0x8c237f40` | hardware register access: `a05f6900,a05f6908,a05f6930,a05f6938` |
| `0x8c237fae` | hardware register access: `a05f6900,a05f6908,a05f6930,a05f6938` |
| `0x8c23bc20` | hardware register access: `8c0080f0,8c0080fc` |
| `0x8c23bc72` | hardware register access: `8c0080fd` |
| `0x8c018244` | `bra` to itself: the routine never returns, so a halting oracle cannot compare it |
| `0x8c010a34` | needs an unimplemented instruction: `div1` |
| `0x8c015f54` | needs an unimplemented instruction: `div1` |
| `0x8c016008` | needs an unimplemented instruction: `div1` |
| `0x8c0160b0` | needs an unimplemented instruction: `div1` |
| `0x8c016100` | needs an unimplemented instruction: `div1` |
| `0x8c0161c8` | needs an unimplemented instruction: `div1` |
| `0x8c017f90` | needs an unimplemented instruction: `div1` |
| `0x8c018030` | needs an unimplemented instruction: `div1` |
| `0x8c0180c8` | needs an unimplemented instruction: `div1` |
| `0x8c01817c` | needs an unimplemented instruction: `div1` |
| `0x8c01848c` | needs an unimplemented instruction: `.word` |
| `0x8c061d40` | needs an unimplemented instruction: `fipr,fschg,ftrv` |
| `0x8c061e00` | needs an unimplemented instruction: `fipr,ftrv` |
| `0x8c061ee0` | needs an unimplemented instruction: `fipr,fschg,ftrv` |
| `0x8c061fc0` | needs an unimplemented instruction: `fschg,ftrv` |
| `0x8c062060` | needs an unimplemented instruction: `ftrv` |
| `0x8c0620e0` | needs an unimplemented instruction: `ftrv` |
| `0x8c062160` | needs an unimplemented instruction: `fipr,fschg,ftrv` |
| `0x8c062220` | needs an unimplemented instruction: `fipr,fschg,ftrv` |
| `0x8c0b27a0` | needs an unimplemented instruction: `fsrra` |
| `0x8c0b27c0` | needs an unimplemented instruction: `fsca` |
| `0x8c0b27d0` | needs an unimplemented instruction: `fsca` |
| `0x8c0b27e0` | needs an unimplemented instruction: `fsca` |
| `0x8c0b27ee` | needs an unimplemented instruction: `fsca` |
| `0x8c0b2800` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2880` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2900` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2940` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2980` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b29c0` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2a00` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2a60` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2aa0` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2b40` | needs an unimplemented instruction: `frchg,ftrv` |
| `0x8c0b2ba0` | needs an unimplemented instruction: `fipr` |
| `0x8c0b2bc0` | needs an unimplemented instruction: `fipr` |
| `0x8c0b2be0` | needs an unimplemented instruction: `fipr,fsrra` |
| `0x8c0b2c20` | needs an unimplemented instruction: `fsrra` |
| `0x8c0b2c40` | needs an unimplemented instruction: `fipr` |
| `0x8c0b2c60` | needs an unimplemented instruction: `fipr` |
| `0x8c0b2c80` | needs an unimplemented instruction: `fipr,fsrra` |
| `0x8c0ce740` | needs an unimplemented instruction: `fschg,ftrv` |
| `0x8c0ea8e0` | needs an unimplemented instruction: `frchg` |
| `0x8c165b80` | needs an unimplemented instruction: `ftrv` |
| `0x8c1cadc8` | needs an unimplemented instruction: `frchg` |
| `0x8c22f4c0` | needs an unimplemented instruction: `fschg` |

## Auto-proposed names (debug strings)

Generated by `tools/name-from-strings.py` from literal-pool xrefs.
`evidence` is the string the function references; review before trusting.

| address | current | proposed | evidence |
| --- | --- | --- | --- |

## Auto-proposed names (Ghidra string labels)

From `tools/name-from-ghidra-strings.py`: each name comes from a string
Ghidra resolved as referenced by that function. Evidence column is the
string itself. Names are proposals until a spec verifies the behaviour.

| address | proposed | evidence |
| --- | --- | --- |
| `0x8c0178c0` | `exception` | exception  bad cast |
| `0x8c017ac4` | `exception_2` | exception  bad cast |
| `0x8c018c54` | `gokart` | GOKART |
| `0x8c020c0c` | `reflection_mapping` | ReflectionMapping |
| `0x8c0213d4` | `reflection_mapping_2` | ReflectionMapping |
| `0x8c0219c0` | `reflection_mapping_3` | ReflectionMapping |
| `0x8c021b88` | `reflection_mapping_4` | ReflectionMapping |
| `0x8c021dbc` | `reflection_mapping_5` | ReflectionMapping |
| `0x8c022288` | `reflection_mapping_6` | ReflectionMapping |
| `0x8c0226d0` | `reflection_mapping_7` | ReflectionMapping |
| `0x8c0230a4` | `reflection_mapping_8` | ReflectionMapping |
| `0x8c0233c8` | `reflection_mapping_9` | ReflectionMapping |
| `0x8c0237f8` | `reflection_mapping_10` | ReflectionMapping |
| `0x8c023894` | `reflection_mapping_11` | ReflectionMapping |
| `0x8c023b54` | `reflection_mapping_12` | ReflectionMapping |
| `0x8c023d10` | `reflection_mapping_13` | ReflectionMapping |
| `0x8c023f1c` | `reflection_mapping_14` | ReflectionMapping |
| `0x8c02419c` | `camera_debug_screen` | CameraDebugScreen |
| `0x8c02511c` | `camera_debug_screen_2` | CameraDebugScreen |
| `0x8c026a88` | `camera_debug_screen_3` | CameraDebugScreen |
| `0x8c027980` | `unnamed` | UNNAMED |
| `0x8c027f58` | `physics_diagnostics_screen` | PhysicsDiagnosticsScreen |
| `0x8c027f94` | `physics_diagnostics_screen_2` | PhysicsDiagnosticsScreen |
| `0x8c028da8` | `physics_diagnostics_screen_3` | PhysicsDiagnosticsScreen |
| `0x8c028e94` | `physics_diagnostics_screen_4` | PhysicsDiagnosticsScreen |
| `0x8c038ad8` | `race_start_camera_behaviour` | RaceStartCameraBehaviour |
| `0x8c039250` | `race_start_camera_behaviour_2` | RaceStartCameraBehaviour |
| `0x8c03a65c` | `reflection_mapping_15` | ReflectionMapping |
| `0x8c03a908` | `reflection_mapping_16` | ReflectionMapping |
| `0x8c03aacc` | `reflection_mapping_17` | ReflectionMapping |
| `0x8c03ae74` | `reflection_mapping_18` | ReflectionMapping |
| `0x8c03bab0` | `reflection_mapping_19` | ReflectionMapping |
| `0x8c03cb5c` | `reflection_mapping_20` | ReflectionMapping |
| `0x8c03e954` | `console_text` | ConsoleText |
| `0x8c05e90c` | `material` | Material |
| `0x8c05ec94` | `material_2` | Material |
| `0x8c05ed44` | `material_3` | Material |
| `0x8c05ee94` | `material_4` | Material |
| `0x8c05eedc` | `material_5` | Material |
| `0x8c05ef94` | `surface_properties` | SurfaceProperties |
| `0x8c0602c4` | `vertex_set` | VertexSet |
| `0x8c060548` | `vertex_set_2` | VertexSet |
| `0x8c0606a4` | `vertex_set_3` | VertexSet |
| `0x8c060860` | `vertex_set_4` | VertexSet |
| `0x8c060974` | `vertex_set_5` | VertexSet |
| `0x8c060aac` | `vertex_set_6` | VertexSet |
| `0x8c072e68` | `gen_tri_strip` | GenTriStrip  Revision  36    Dat |
| `0x8c073b90` | `gen_tri_strip_2` | GenTriStrip  Revision  36    Dat |
| `0x8c081c88` | `active` | active |
| `0x8c083d1c` | `font_set` | FontSet |
| `0x8c0845a0` | `font_set_2` | FontSet |
| `0x8c084984` | `font_set_3` | FontSet |
| `0x8c084a58` | `font_set_4` | FontSet |
| `0x8c084c40` | `font_set_5` | FontSet |
| `0x8c08fb58` | `bounding_volume` | BoundingVolume  Revision  22 |
| `0x8c092734` | `reflection_mapping_21` | ReflectionMapping |
| `0x8c092934` | `reflection_mapping_22` | ReflectionMapping |
| `0x8c092a88` | `reflection_mapping_23` | ReflectionMapping |
| `0x8c092b40` | `reflection_mapping_24` | ReflectionMapping |
| `0x8c093c90` | `reflection_mapping_25` | ReflectionMapping |
| `0x8c095208` | `reflection_mapping_26` | ReflectionMapping |
| `0x8c09cfec` | `reflection_mapping_27` | ReflectionMapping |
| `0x8c0a0a38` | `reflection_mapping_28` | ReflectionMapping |
| `0x8c0a0b10` | `reflection_mapping_29` | ReflectionMapping |
| `0x8c0a3bbc` | `scene_node` | SceneNode |
| `0x8c0a41e8` | `device_debug_windows` | DeviceDebugWindows |
| `0x8c0a452c` | `device_debug_windows_2` | DeviceDebugWindows |
| `0x8c0a47b4` | `device_debug_windows_3` | DeviceDebugWindows |
| `0x8c0a4878` | `device_debug_windows_4` | DeviceDebugWindows |
| `0x8c0a4a64` | `device_debug_windows_5` | DeviceDebugWindows |
| `0x8c0a4dbc` | `device_debug_windows_6` | DeviceDebugWindows |
| `0x8c0a5190` | `device_debug_windows_7` | DeviceDebugWindows |
| `0x8c0a5450` | `device_debug_windows_8` | DeviceDebugWindows |
| `0x8c0a5a40` | `device_debug_windows_9` | DeviceDebugWindows |
| `0x8c0a9200` | `fixed_camera` | FixedCamera |
| `0x8c0ab53c` | `fixed_camera_2` | FixedCamera |
| `0x8c0abbf4` | `fixed_camera_3` | FixedCamera |
| `0x8c0ac0ac` | `fixed_camera_4` | FixedCamera |
| `0x8c0ad918` | `fixed_camera_5` | FixedCamera |
| `0x8c0b2254` | `render_state_group` | RenderStateGroup |
| `0x8c0b509c` | `revision` | Revision  12 |
| `0x8c0b8438` | `revision_2` | Revision  30 |
| `0x8c0b9474` | `revision_3` | Revision  11 |
| `0x8c0bdf98` | `checkpoint` | Checkpoint |
| `0x8c0bf36c` | `checkpoint_2` | Checkpoint |
| `0x8c0bfba0` | `checkpoint_3` | Checkpoint |
| `0x8c0c0464` | `checkpoint_4` | Checkpoint |
| `0x8c0c0800` | `checkpoint_5` | Checkpoint |
| `0x8c0c9c3c` | `engine_sound_group` | EngineSoundGroup |
| `0x8c0c9e40` | `engine_sound_group_2` | EngineSoundGroup |
| `0x8c0ca00c` | `engine_sound_group_3` | EngineSoundGroup |
| `0x8c0cc1f8` | `version` | Version |
| `0x8c0cf48c` | `revision_4` | Revision  25 |
| `0x8c0d8f28` | `reflection_mapping_30` | ReflectionMapping |
| `0x8c0d92c8` | `reflection_mapping_31` | ReflectionMapping |
| `0x8c0dfc50` | `pl_collision_scene_edge_data` | plCollisionSceneEdgeData |
| `0x8c0e07c0` | `slew_camera` | SlewCamera |
| `0x8c0e2b20` | `console_text_2` | ConsoleText |
| `0x8c0e2ce4` | `console_text_3` | ConsoleText |
| `0x8c0e6628` | `console_text_4` | ConsoleText |
| `0x8c10585c` | `lm_track_info` | lmTrackInfo |
| `0x8c1061d4` | `lm_track_info_2` | lmTrackInfo |
| `0x8c1068c8` | `lm_track_info_3` | lmTrackInfo |
| `0x8c107198` | `fuel_load` | FuelLoad |
| `0x8c10740c` | `fuel_load_2` | FuelLoad |
| `0x8c111efc` | `a_brake_factors0` | aBrakeFactors0 |
| `0x8c112850` | `a_brake_factors0_2` | aBrakeFactors0 |
| `0x8c1133c8` | `a_brake_factors0_3` | aBrakeFactors0 |
| `0x8c1183dc` | `a_brake_factors0_4` | aBrakeFactors0 |
| `0x8c11b8c0` | `a_brake_factors0_5` | aBrakeFactors0 |
| `0x8c11c6d8` | `a_brake_factors0_6` | aBrakeFactors0 |
| `0x8c11d4bc` | `a_brake_factors0_7` | aBrakeFactors0 |
| `0x8c11eefc` | `a_brake_factors0_8` | aBrakeFactors0 |
| `0x8c121df0` | `track_name` | TrackName |
| `0x8c1228c8` | `track_name_2` | TrackName |
| `0x8c128af8` | `indicator` | Indicator |
| `0x8c12aac4` | `indicator_2` | Indicator |
| `0x8c12aea8` | `lm_championship_info` | lmChampionshipInfo |
| `0x8c146580` | `requested` | Requested   3fK for replay contr |
| `0x8c146744` | `requested_2` | Requested   3fK for replay contr |
| `0x8c14960c` | `poly_line` | PolyLine |
| `0x8c14aa94` | `lm_playback_physics_sync_data` | lmPlaybackPhysicsSyncData |
| `0x8c14b308` | `lm_playback_physics_sync_data_2` | lmPlaybackPhysicsSyncData |
| `0x8c14b418` | `lm_playback_physics_sync_data_3` | lmPlaybackPhysicsSyncData |
| `0x8c14b520` | `lm_playback_physics_sync_data_4` | lmPlaybackPhysicsSyncData |
| `0x8c14b6bc` | `lm_playback_physics_sync_data_5` | lmPlaybackPhysicsSyncData |
| `0x8c14cc60` | `lm_playback_physics_sync_data_6` | lmPlaybackPhysicsSyncData |
| `0x8c14ce60` | `lm_playback_physics_sync_data_7` | lmPlaybackPhysicsSyncData |
| `0x8c14d03c` | `lm_playback_physics_sync_data_8` | lmPlaybackPhysicsSyncData |
| `0x8c14d22c` | `lm_playback_physics_sync_data_9` | lmPlaybackPhysicsSyncData |
| `0x8c14d33c` | `lm_playback_physics_sync_data_10` | lmPlaybackPhysicsSyncData |
| `0x8c14e77c` | `lmscroll_text` | LMScrollText |
| `0x8c14ee0c` | `lmscroll_text_2` | LMScrollText |
| `0x8c14f174` | `lmscroll_text_3` | LMScrollText |
| `0x8c14f754` | `lmscroll_text_4` | LMScrollText |
| `0x8c14f96c` | `lmscroll_text_5` | LMScrollText |
| `0x8c14faf8` | `lmscroll_text_6` | LMScrollText |
| `0x8c14fb84` | `lmscroll_text_7` | LMScrollText |
| `0x8c15ae50` | `transmission` | Transmission |
| `0x8c15afc4` | `transmission_2` | Transmission |
| `0x8c15b0e0` | `transmission_3` | Transmission |
| `0x8c15bb7c` | `transmission_4` | Transmission |
| `0x8c15c574` | `transmission_5` | Transmission |
| `0x8c15cff8` | `transmission_6` | Transmission |
| `0x8c15d200` | `transmission_7` | Transmission |
| `0x8c15e184` | `key_gearup` | keyGEARUP |
| `0x8c15e3d8` | `key_gearup_2` | keyGEARUP |
| `0x8c15ecc8` | `key_gearup_3` | keyGEARUP |
| `0x8c15f2d8` | `key_gearup_4` | keyGEARUP |
| `0x8c15f9ec` | `key_gearup_5` | keyGEARUP |
| `0x8c15fb60` | `key_gearup_6` | keyGEARUP |
| `0x8c15fda0` | `key_gearup_7` | keyGEARUP |
| `0x8c163170` | `mainmenu` | mainmenu |
| `0x8c16543c` | `texture` | Texture |
| `0x8c170668` | `ac_digi_set_filter_envelope` | acDigiSetFilterEnvelope   can t s |
| `0x8c1707c2` | `ac_digi_set_amplitude_envelope` | acDigiSetAmplitudeEnvelope   can |
| `0x8c17083e` | `ac_digi_set_lfo_registers` | acDigiSetLfoRegisters   can t se |
| `0x8c170bb8` | `ac_digi_play_with_parameters` | acDigiPlayWithParameters   Warni |
| `0x8c170efa` | `ac_set_multi_play_mask` | acSetMultiPlayMask   upperMask o |
| `0x8c171060` | `ac_digi_multi_stop` | acDigiMultiStop   masks are 0 |
| `0x8c1710f8` | `ac_digi_open` | acDigiOpen   port is out of rang |
| `0x8c171262` | `ac_digi_set_sample_rate` | acDigiSetSampleRate   sampleRate |
| `0x8c171358` | `ac_digi_set_qsound_angle` | acDigiSetQSoundAngle   Warning  a |
| `0x8c1713c4` | `ac_dsp_load_programs` | acDspLoadPrograms   address is N |
| `0x8c171474` | `ac_dsp_load_output_mixer` | acDspLoadOutputMixer   address i |
| `0x8c1714fc` | `ac_dsp_set_mixer_channel` | acDspSetMixerChannel   can t sen |
| `0x8c171580` | `ac_digi_close` | acDigiClose   can t send driver c |
| `0x8c171720` | `ac_digi_set_volume` | acDigiSetVolume   Warning  chann |
| `0x8c1717b4` | `ac_digi_set_pan` | acDigiSetPan   Warning  aicaPan i |
| `0x8c171820` | `ac_digi_request_event` | acDigiRequestEvent   failed to s |
| `0x8c171892` | `ac_digi_stop` | acDigiStop   can t send driver c |
| `0x8c171ae8` | `ac_midi_close` | acMidiClose   can t send driver c |
| `0x8c171c54` | `ac_midi_stop` | acMidiStop   can t send driver c |
| `0x8c171ca6` | `ac_midi_request_event` | acMidiRequestEvent   can t send d |
| `0x8c171d10` | `ac_midi_pause` | acMidiPause   can t send driver c |
| `0x8c171e4c` | `ac_midi_reset` | acMidiReset   can t send driver c |
| `0x8c172046` | `ac_cd_set_pan` | acCdSetPan   can t send driver c |
| `0x8c1720a0` | `ac_cd_init` | acCdInit   can t send driver com |
| `0x8c1720fc` | `ac_cmd_queue_init` | acCmdQueueInit   buffer is NULL |
| `0x8c172780` | `ac_g2_fill` | acG2Fill   target is NULL |
| `0x8c172868` | `ac_g2_read` | acG2Read   target source is NULL |
| `0x8c172916` | `ac_g2_read_long` | acG2ReadLong   address is NULL |
| `0x8c172986` | `ac_g2_write_long` | acG2WriteLong   address is NULL |
| `0x8c172a18` | `ac_g2_write` | acG2Write   target source is NUL |
| `0x8c172be0` | `ac_int_init` | acIntInit   failed to add the AR |
| `0x8c172ea4` | `ac_system_get_int_array_start_offset` | acSystemGetIntArrayStartOffset |
| `0x8c172ee2` | `ac_system_get_int_array_start_offset_2` | acSystemGetIntArrayStartOffset |
| `0x8c172f5a` | `ac_system_get_int_array_start_offset_3` | acSystemGetIntArrayStartOffset |
| `0x8c172f96` | `ac_system_get_int_array_start_offset_4` | acSystemGetIntArrayStartOffset |
| `0x8c172fcc` | `ac_system_check_driver_revision` | acSystemCheckDriverRevision   dr |
| `0x8c173084` | `ac_system_get_driver_revision` | acSystemGetDriverRevision   driv |
| `0x8c17311c` | `ac_system_init` | acSystemInit   driverType is inv |
| `0x8c1732b8` | `ac_system_get_first_free_sound_memory` | acSystemGetFirstFreeSoundMemory |
| `0x8c17331c` | `ac_system_install_driver` | acSystemInstallDriver   driverIm |
| `0x8c1734c8` | `ac_set_transfer_mode` | acSetTransferMode   add handler f |
| `0x8c173592` | `ac_transfer` | acTransfer   request is NULL |
| `0x8c173a8c` | `am_bank_asset_type_is` | amBankAssetTypeIs   theBank is N |
| `0x8c173af0` | `am_bank_transfer` | amBankTransfer   target or sourc |
| `0x8c173b88` | `am_bank_get_size` | amBankGetSize   bankSize or file |
| `0x8c173bd2` | `am_bank_load` | amBankLoad   sh4buffer or filena |
| `0x8c173f72` | `am_bank_fetch_wave_loop_flag` | amBankFetchWaveLoopFlag   theBan |
| `0x8c17401e` | `am_bank_fetch_wave_random_pitch` | amBankFetchWaveRandomPitch   the |
| `0x8c17409c` | `am_bank_fetch_wave_sample_rate` | amBankFetchWaveSampleRate   theB |
| `0x8c17413e` | `am_bank_fetch_wave_bit_depth` | amBankFetchWaveBitDepth   theBan |
| `0x8c174276` | `am_bank_fetch_asset` | amBankFetchAsset   theBank  para |
| `0x8c174344` | `am_bank_get_asset_size` | amBankGetAssetSize   theBank or a |
| `0x8c1743a6` | `am_bank_get_number_of_assets` | amBankGetNumberOfAssets   number |
| `0x8c174488` | `am_dsp_fetch_program_bank` | amDspFetchProgramBank   theBank i |
| `0x8c1744e4` | `am_dsp_fetch_output_bank` | amDspFetchOutputBank   theBank i |
| `0x8c17460a` | `am_heap_get_info` | amHeapGetInfo   freeSoundMemory i |
| `0x8c17466e` | `am_heap_get_free` | amHeapGetFree   audio heap not i |
| `0x8c1746a4` | `am_heap_alloc` | amHeapAlloc   audio heap not ini |
| `0x8c17493c` | `am_heap_free` | amHeapFree   buffer is NULL |
| `0x8c174af6` | `am_heap_alloc_from_top` | amHeapAllocFromTop   Heap is no |
| `0x8c174cd4` | `am_heap_alloc_from_bottom` | amHeapAllocFromBottom   Heap is |
| `0x8c174e9e` | `am_heap_init` | amHeapInit   memoryPool is NULL |
| `0x8c175450` | `am_init` | amInit   amHeapGetInfo   failed |
| `0x8c17563c` | `am_sound_set_effects_buss` | amSoundSetEffectsBuss   wetLevel |
| `0x8c175746` | `am_sound_release_voice_channel` | amSoundReleaseVoiceChannel   the |
| `0x8c1757e8` | `am_sound_fetch_sample` | amSoundFetchSample   theSound is |
| `0x8c175918` | `am_sound_is_looping` | amSoundIsLooping   loopFlag is N |
| `0x8c175978` | `am_sound_allocate_voice_channel` | amSoundAllocateVoiceChannel   th |
| `0x8c1759ae` | `am_sound_get_sample_rate` | amSoundGetSampleRate   realWorld |
| `0x8c1759e2` | `am_sound_get_volume` | amSoundGetVolume   aicaVolume is |
| `0x8c175a16` | `am_sound_get_pan` | amSoundGetPan   pan is NULL |
| `0x8c175a4a` | `am_sound_get_voice_channel` | amSoundGetVoiceChannel   voiceCh |
| `0x8c175a7e` | `am_sound_get_callback` | amSoundGetCallback    theCallbac |
| `0x8c175ab4` | `am_sound_set_current_playback_rate` | amSoundSetCurrentPlaybackRate   t |
| `0x8c175b3a` | `am_sound_set_volume` | amSoundSetVolume   theSound is N |
| `0x8c175ba4` | `am_sound_set_pan` | amSoundSetPan   theSound is NULL |
| `0x8c175c28` | `am_sound_set_callback` | amSoundSetCallback   theSound is |
| `0x8c175c72` | `am_sound_is_playing` | amSoundIsPlaying   theSound is N |
| `0x8c175c98` | `am_sound_stop` | amSoundStop   theSound is NULL |
| `0x8c175e66` | `am_sound_close_voice_channel` | amSoundCloseVoiceChannel   theS |
| `0x8c175ea0` | `am_sound_play` | amSoundPlay   theSound is NULL |
| `0x8c175f92` | `am_sound_play_raw` | amSoundPlayRaw   voice channel n |
| `0x8c176228` | `am_stream_allocate_voice_channels` | amStreamAllocateVoiceChannels   t |
| `0x8c1764e0` | `am_stream_close` | amStreamClose   theStream is NUL |
| `0x8c176512` | `am_stream_is_done` | amStreamIsDone   theStream is NU |
| `0x8c176580` | `am_stream_is_idle` | amStreamIsIdle   theStream is NU |
| `0x8c1765b0` | `am_stream_is_playing` | amStreamIsPlaying   theStream is |
| `0x8c1765e0` | `am_stream_set_header_buffer` | amStreamSetHeaderBuffer   theStr |
| `0x8c176618` | `am_stream_start` | amStreamStart   theStream is NUL |
| `0x8c1766a2` | `am_stream_set_buffers` | amStreamSetBuffers   theStream o |
| `0x8c17678e` | `am_stream_set_isr` | amStreamSetIsr   theStream theIs |
| `0x8c1767b2` | `am_stream_init` | amStreamInit   theStream or file |
| `0x8c176d7e` | `am_stream_set_stereo_voice_channels` | amStreamSetStereoVoiceChannels |
| `0x8c176df2` | `am_stream_set_mono_voice_channel` | amStreamSetMonoVoiceChannel   t |
| `0x8c1778ea` | `am_util_align_number` | amUtilAlignNumber   theAlignment |
| `0x8c177926` | `am_util_get_length_in_frames` | amUtilGetLengthInFrames   channe |
| `0x8c17797e` | `am_util_get_nibbles_per_frame` | amUtilGetNibblesPerFrame   nibbl |
| `0x8c1779fe` | `am_util_get_sample_rate` | amUtilGetSampleRate   sampleRate |
| `0x8c177c3c` | `am_util_get_aica_sample_type` | amUtilGetAicaSampleType   aicaSa |
| `0x8c177e28` | `am_voice_free` | amVoiceFree   voiceChannel is ou |
| `0x8c177fa0` | `am_voice_install_user_callback` | amVoiceInstallUserCallback   voi |
| `0x8c177fe0` | `am_voice_allocate` | amVoiceAllocate   voiceChannel i |
| `0x8c182048` | `flame2` | flame2 |
| `0x8c182650` | `reflection_mapping_32` | ReflectionMapping |
| `0x8c1829c4` | `reflection_mapping_33` | ReflectionMapping |
| `0x8c182d04` | `reflection_mapping_34` | ReflectionMapping |
| `0x8c183808` | `reflection_mapping_35` | ReflectionMapping |
| `0x8c1839f8` | `driver` | driver |
| `0x8c183a7c` | `driver_2` | driver |
| `0x8c183b90` | `driver_3` | driver |
| `0x8c1851e0` | `transitioned_lighting_modification` | TransitionedLightingModification |
| `0x8c1858fc` | `transitioned_lighting_modification_2` | TransitionedLightingModification |
| `0x8c1864e0` | `transitioned_lighting_modification_3` | TransitionedLightingModification |
| `0x8c187464` | `transitioned_lighting_modification_4` | TransitionedLightingModification |
| `0x8c187670` | `transitioned_lighting_modification_5` | TransitionedLightingModification |
| `0x8c1885f8` | `transitioned_lighting_modification_6` | TransitionedLightingModification |
| `0x8c189944` | `track_name_3` | TrackName |
| `0x8c189b34` | `track_name_4` | TrackName |
| `0x8c189d54` | `track_name_5` | TrackName |
| `0x8c189f78` | `track_name_6` | TrackName |
| `0x8c18dd74` | `vmumessage_text` | VMUMessageText |
| `0x8c18de14` | `vmumessage_text_2` | VMUMessageText |
| `0x8c18df14` | `vmumessage_text_3` | VMUMessageText |
| `0x8c18e0a8` | `vmumessage_text_4` | VMUMessageText |
| `0x8c18e1f0` | `vmumessage_text_5` | VMUMessageText |
| `0x8c18feac` | `const_player` | constPLAYER |
| `0x8c1904dc` | `const_player_2` | constPLAYER |
| `0x8c19ab08` | `version_2` | Version |
| `0x8c19aca4` | `version_3` | Version |
| `0x8c19aed0` | `version_4` | Version |
| `0x8c19b044` | `reflection_mapping_36` | ReflectionMapping |
| `0x8c19b2b4` | `reflection_mapping_37` | ReflectionMapping |
| `0x8c19b7f4` | `version_5` | Version |
| `0x8c19f194` | `infogrames` | INFOGRAMES |
| `0x8c19f3d4` | `infogrames_2` | INFOGRAMES |
| `0x8c1a4a84` | `tdg_fatal_error` | tdgFatalError      WARNING  alre |
| `0x8c1ae098` | `reflection_mapping_38` | ReflectionMapping |
| `0x8c1aebb8` | `reflection_mapping_39` | ReflectionMapping |
| `0x8c1aec94` | `reflection_mapping_40` | ReflectionMapping |
| `0x8c1aedd8` | `reflection_mapping_41` | ReflectionMapping |
| `0x8c1af194` | `reflection_mapping_42` | ReflectionMapping |
| `0x8c1af2d4` | `reflection_mapping_43` | ReflectionMapping |
| `0x8c1af3e4` | `reflection_mapping_44` | ReflectionMapping |
| `0x8c1af5ac` | `reflection_mapping_45` | ReflectionMapping |
| `0x8c1b4b00` | `heures` | Heures |
| `0x8c1b52b4` | `heures_2` | Heures |
| … | | 10 more in analysis/labels.tsv |

## Verifier findings so far

Four ports, five spec runs, and every failure was a *hidden side effect* that the
decompiled C did not show:

| address | missed by the C |
| --- | --- |
| `0x8c175c72` | argument copied into `r4`; field left in `r3` |
| `0x8c18b9c8` | `base[1]` left in `r1`; T bit differs per branch |
| `0x8c1552e4` | inner pointer left in `r0` |

That is the argument for the oracle: register residue is invisible in pseudocode
and load-bearing for the caller.
