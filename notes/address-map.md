# Address map

## Verified ports

| address | name | evidence |
| --- | --- | --- |

Side effects found by the verifier and required by the port: the original
copies the argument into `r4` (not present here) and leaves the `+0x38` field in
`r3`. Neither is visible in the C, which is why the diff is the source of truth.


Nothing recovered yet: no binary. Add rows as evidence lands.

| address | name | kind | evidence |
| --- | --- | --- | --- |

| `0x8c010000` | `fixture_sum_loop` | `notes/verifications/0x8c010000.json` — 19 oracle steps; harness regression fixture |
| `0x8c010080` | `FUN_8c010080` | `notes/verifications/0x8c010080.json` — 2 oracle steps; none |
| `0x8c010084` | `release_and_clear` | `notes/verifications/0x8c010084.json` — first call-crossing port |
| `0x8c0100e4` | `release_and_maybe_owner` | `notes/verifications/0x8c0100e4.json` — 18 oracle steps |
| `0x8c01011c` | `store_zero_return_arg` | `notes/verifications/0x8c01011c.json` — 4 oracle steps; delay slot copies the argument into r0 |
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
| `0x8c010160` | `FUN_8c010160` | `notes/verifications/0x8c010160.json` — 4 oracle steps; store r5 at table_base + (r4<<2) |
| `0x8c010178` | `FUN_8c010178` | `notes/verifications/0x8c010178.json` — 2 oracle steps; none |
| `0x8c01017c` | `FUN_8c01017c` | `notes/verifications/0x8c01017c.json` — 2 oracle steps; none |
| `0x8c010394` | `call_two_helpers` | `notes/verifications/0x8c010394.json` — 33 oracle steps |
| `0x8c010438` | `FUN_8c010438` | `notes/verifications/0x8c010438.json` — 7 oracle steps; store at [slot]; r5/r6/r2 leftovers; T per path |
| `0x8c01044c` | `FUN_8c01044c` | `notes/verifications/0x8c01044c.json` — 2 oracle steps; none |
| `0x8c010b38` | `classify_float_result` | `notes/verifications/0x8c010b38.json` — 48 oracle steps |
| `0x8c14720c` | `wrap_difference_scale` | `notes/verifications/0x8c14720c*.json` - high 109, low 125, negative 113 |
| `0x8c010b88` | `saturate_and_classify` | `notes/verifications/0x8c010b88.json` — 105 oracle steps |
| `0x8c010cf8` | `FUN_8c010cf8` | `notes/verifications/0x8c010cf8.json` — 16 oracle steps; zero-input branch only (fr4==0: *r4=0, r6=fr4bits&0x7f800000, fr0=fr4, T=1); non-zero branches blocked on global 0x8c23d8b8 outside image |
| `0x8c010d6c` | `FUN_8c010d6c` | `notes/verifications/0x8c010d6c.json` — 6 oracle steps; r1 = table base; r0 = byte&7 |
| `0x8c010d7c` | `FUN_8c010d7c` | `notes/verifications/0x8c010d7c.json` — 6 oracle steps; r1 = table base; r0 = byte&0x40 |
| `0x8c010e50` | `FUN_8c010e50` | `notes/verifications/0x8c010e50.json` — 28 oracle steps; none; r0 = last-byte diff; r4/r7 advanced, r5 = match count, r2/r3 leftovers, T per exit |
| `0x8c010e84` | `FUN_8c010e84 (memmove)` | `notes/verifications/0x8c010e84.json` — 44 oracle steps; loop scratch in r1/r2/r3/r6/r7/r11/r12/r13; r1=3 only on word/dword paths (byte paths branch before mov #3,r1); r2=0 fwd / 1 bwd / n==0 leaves T clear; r11/r12/r13 restored by epilogue; rts delay slot sets r0=r4 |
| `0x8c010f44` | `FUN_8c010f44 (byte copy fwd/bwd)` | `notes/verifications/0x8c010f44.json` — 50 oracle steps; r3=last byte (fwd: u32 load of last src dword; bwd: sign-extended src[0]); r5=counter n; r7=dst+n (fwd) or dst (bwd); r2=0 on bwd path; bt.s delay slots set r7/r0 even when n==0; rts delay slot sets r0=r4; T always set |
| `0x8c010f88` | `FUN_8c010f88 (memset)` | `notes/verifications/0x8c010f88.json` — 47 oracle steps; r3=0 and r7=n (loop counter) left behind; r0 reloaded to r4 by rts delay slot; T set on all paths |
| `0x8c01125c` | `FUN_8c01125c (global store)` | `notes/verifications/0x8c01125c.json` — 3 oracle steps; r3 = loaded pointer 0x8c1f2eb0 left in r3; store r4 -> [0x8c1f2eb0] in rts delay slot |
| `0x8c0112a4` | `FUN_8c0112a4 (strcpy)` | `notes/verifications/0x8c0112a4.json` — 44 oracle steps; r6/r7 end one past the copied NUL (bf.s delay slot advances r6 even on the final store); r2=0 after extu.b; bra delay slot copies src r5 into r7; rts delay slot sets r0=r4; T set |
| `0x8c0112c4` | `FUN_8c0112c4 (strchr)` | `notes/verifications/0x8c0112c4.json` — 15 oracle steps; r4 clobbered by exts.b r5,r4 in the bra delay slot (sign-extended search char); r2=last byte read; found: r0=r6, T set; miss: r0=0 via rts delay slot, T set |
| `0x8c0112f4` | `FUN_8c0112f4 (strlen)` | `notes/verifications/0x8c0112f4.json` — 27 oracle steps; r4 ends one past the terminating dword; r3 = that dword (0); r5=count moved to r0 by rts delay slot; T set |
| `0x8c011304` | `FUN_8c011304 (strncmp)` | `notes/verifications/0x8c011304.json` — 33 oracle steps; r7=r5 via bf.s delay slot even on n==0 (returns 0, T set); n>0: r4 past last byte read, r7 decremented back, r2=mismatched src byte or address r4-1, r3=zero-extended src byte, r5=count; T clear only on mismatch exit; r0=(u8)s1[k-1]-(u8)s2[k-1] |
| `0x8c011344` | `FUN_8c011344 (strncpy)` | `notes/verifications/0x8c011344.json` — 47 oracle steps; r1=0 is the pad byte and stays in r1; r5=counter (bra delay slot zeroes it); r7=dst+stores (bf.s delay slot advances r7 even on the last pad store); r3=last load (u32 in loop1, byte at the head); r0 destroyed to r4 by rts delay slot; T set |
| `0x8c011374` | `FUN_8c011374 (scan haystack for needle byte)` | `notes/verifications/0x8c011374.json` — 103 oracle steps; r1=last needle dword read; r3=last load; r6=inner index at exit; r7=outer index; found: r0=r14 via bra delay slot, T clear; NULL: r0=0, T set; r12/r14 restored via stack; inner loop compares hay byte against u32 needle dwords (group-0 indexed loads are u32) |
| `0x8c0113b4` | `reverse_byte_search` | `notes/verifications/0x8c0113b4.json` — 65 oracle steps |
| `0x8c011c50` | `FUN_8c011c50` | `notes/verifications/0x8c011c50*.json` — all branches (34, 15, 6 oracle steps); r6=count, r2=last byte, r4 advanced, r0=0/1, T=last cmp/pl or cmp/ge |
| `0x8c011cf8` | `FUN_8c011cf8` | `notes/verifications/0x8c011cf8.json` — 44 oracle steps; fr6=fr4^2, 5x fmac over table at 0x8c1cd028, r4=0, r5=0x8c1cd040, fr0=fr4 then rts delay slot fmul fr6,fr0, fr6=fr5 (spec fixed: was `fr4` key + zeroed table, which masked the delay slot) |
| `0x8c011d1c` | `FUN_8c011d1c (Horner fr4^2 over table 0x8c1cd040)` | `notes/verifications/0x8c011d1c.json` — 49 oracle steps; BLOCKED: oracle lacks fmov.s @Rm+,FRn (0xf459 hit at 0x8c011d24). Port written: loop target is 0x8c011d26 so fr4 is loaded once pre-loop then carries the accumulator via the bt.s delay slot; r5 ends 0x8c1cd05c, fr4=fr0=acc, fr5=fr4_input^2, T=0 |
| `0x8c011d3c` | `FUN_8c011d3c (Horner fr4^2 over table 0x8c1cd060)` | `notes/verifications/0x8c011d3c.json` — 51 oracle steps; BLOCKED: oracle lacks fmov.s @Rm+,FRn (0xf559 hit at 0x8c011d44). Port written: 6 iterations, r5 ends 0x8c1cd07c, fr5=fr6=acc, fr0=fr4*acc in rts delay slot, T=0 |
| `0x8c012260` | `FUN_8c012260 (classify fr4 bits)` | `notes/verifications/0x8c012260.json` — 15 oracle steps; verified: r15 spilled -8/+8, r4/r2 keep stack addresses (r4 drops 4 more on the return-2 path), r5=0x7ff00000, r3=last tested word, T=last cmp/eq or tst (1 on return 0/2 paths, 0 on return 1); two fmov stores land in the stack window; return 0 when fr4 finite, 2 when fr4 is exactly +/-inf and fr5 bits are 0, else 1 |
| `0x8c013226` | `FUN_8c013226` | `notes/verifications/0x8c013226.*.json` — both branches (9 and 13 oracle steps); clear: *(r4+4)-=1 in r2; set: r1=*(r4+4), *(r1+4)+=1, **r1-=1 in r3 |
| `0x8c013bea` | `FUN_8c013bea` | `notes/verifications/0x8c013bea*.json` — both branches (8 and 5 oracle steps); taken: r2=*(r4+4), r3=0, byte *r2=0; T=(*r4==1) |
| `0x8c013bfa` | `FUN_8c013bfa` | `notes/verifications/0x8c013bfa*.json` — both branches (9 and 5 oracle steps); eq: r0=0; else r0=mem.b[*(r4+4)+16]&0x40, r1=16 |
| `0x8c013e94` | `FUN_8c013e94` | `notes/verifications/0x8c013e94.json` — 141 oracle steps; fr4 clobbered to NaN const 0x7f800001 by the bf.s delay slot at 0x8c013ed8 on EVERY path; negative input returns fr0=NaN; 1.0 special-case returns fr0=0.0; fpul=r6=unbiased exponent (e.g. 1 for 2.0) on normal path; r5=8, r7=1 constants left; r5 ends 0x8c1cd1e8 after @r5+ loop; r0 ends as FULL original fr4 bits (and r0,r6 is r6&=r0, not r0&=r6); r1=0x8c1cd1c4, r2=0x8c1ccfc8, r3=0x8c1cd1c0 leftovers; T=last cmp/pl r4 (=0) on normal path |
| `0x8c0146bc` | `FUN_8c0146bc (ftrc-based float normalise)` | `notes/verifications/0x8c0146bc.json` — 26 oracle steps; verified positive large-exponent path (fr4=16777216.0): r15 spilled/restored, r2=r15+4, r3=last word, r4=sign bit of fr4, r5=masked exponent, r6=0x80000000, T=tst r3,r6 (1 positive); fr0=fr4; fmov stores land in stack window |
| `0x8c01471c` | `float_classify_scale` | `notes/verifications/0x8c01471c.json` — verified |
| `0x8c014810` | `FUN_8c014810` | `notes/verifications/0x8c014810.json` — 41 oracle steps; fr4 overwritten with 0.0 on the normal path (0x8c01484c); fr5=-Inf and fr6=-0.0 loaded unconditionally via branch delay slots; fr7=+Inf always; r0=0x7fffff only on Inf/NaN path; r14 saved/restored through stack, r6=r15-8; r1=0x8c1ccfd0, r2/r3/r5/r7 literal leftovers; T ends per path (1 for +Inf, +0, and positive-sign clamps; 0 for -Inf, -0, negative-sign clamps, NaN, build) |
| `0x8c0148d0` | `call_and_scale` | `notes/verifications/0x8c0148d0.json` — 43 oracle steps |
| `0x8c014bd4` | `FUN_8c014bd4 (bignum rol-left, mask 0x80)` | `notes/verifications/0x8c014bd4.json` — 73 oracle steps; r5=r6=end-1; r1=last&0x80; r3=signext(byte)<<1; r7=last mask bit (return); r0=0/1; T=final tst r7; r14 restored via push/pop; entry bf.s delay slot forces r6=r5; loop delay slot decrements r6 in lockstep |
| `0x8c014c1c` | `FUN_8c014c1c` | `notes/verifications/0x8c014c1c.json` — 79 oracle steps; "r1 = shad(n,0xff) mask left; r7 = final carry byte; r3 = -n; r5 ends r4-1 (r4+len-1 if loop skipped); r0 on the n==0 path is the caller's incoming r0; r14/r15 preserved via stack" |
| `0x8c014da4` | `FUN_8c014da4` | `notes/verifications/0x8c014da4.json` — 38 oracle steps; "r0=r6=r4+4 left; r7=-8 (delay `add #-8,r7` runs on final pass); r3=0; r2 keeps unshifted word (last shld amt 0); T left set" |
| `0x8c014dc0` | `FUN_8c014dc0` | `notes/verifications/0x8c014dc0.json` — 24 oracle steps; "r0=r6=r4+2 left; r7=-8; r2=0; r3=zero-extended u16; T left set; mov.w @r5 sign-extends before extu.w re-truncates" |
| `0x8c014de0` | `wrap_byte_accessor` | `notes/verifications/0x8c014de0.json` — 128 oracle steps |
| `0x8c0851bc` | `guard_free` | `notes/verifications/0x8c0851bc*.json` - 6 specs, 9-90 steps |
| `0x8c085238` | `guard_free_clear_global` | `notes/verifications/0x8c085238*.json` - 5 specs, 12-90 steps |
| `0x8c085700` | `free_payload_clear_global_free_container` | `notes/verifications/0x8c085700*.json` - 4 specs, 9-90 steps |
| `0x8c090cf0` | `memmove_window` | `notes/verifications/0x8c090cf0.json` - 275 oracle steps |
| `0x8c090c68` | `clear_fields_then_free` | `notes/verifications/0x8c090c68*.json` - 4 specs, 12-90 steps |
| `0x8c092fe0` | `guard_free` | `notes/verifications/0x8c092fe0*.json` - 4 specs, 9-90 steps |
| `0x8c095afc` | `guard_free` | `notes/verifications/0x8c095afc*.json` - 4 specs |
| `0x8c095b20` | `guard_free` | `notes/verifications/0x8c095b20*.json` - 4 specs, 9-90 steps |
| `0x8c0dfc18` | `zero_block_and_publish` | `notes/verifications/0x8c0dfc18.json` - 2026 oracle steps (0x190-byte memset) |
| `0x8c0e0694` | `reload_published_object` | `notes/verifications/0x8c0e0694.json` - 9 oracle steps |
| `0x8c105ff0` | `array_index_thunk` | `notes/verifications/0x8c105ff0.json` - 14 oracle steps |
| `0x8c106008` | `array_index_thunk` | `notes/verifications/0x8c106008.json` - 14 oracle steps |
| `0x8c106878` | `strcmp_loop` | `notes/verifications/0x8c106878*.json` - 4 specs (5-57 steps) |
| `0x8c10b7a4` | `strcpy_offset_thunk` | `notes/verifications/0x8c10b7a4.json` - 37 oracle steps |
| `0x8c10616c` | `release_six_handles` | `notes/verifications/0x8c10616c*.json` - 2 specs (308 and 230 steps) |
| `0x8c14b698` | `advance_cursor_by_0x570` | `notes/verifications/0x8c14b698*.json` - 5 specs, 12-60 steps |
| `0x8c0ae798` | `free_payload_then_object` | `notes/verifications/0x8c0ae798*.json` - 3 specs (103/25/12 steps) |
| `0x8c0b5e4c` | `stamp_vtable_then_free` | `notes/verifications/0x8c0b5e4c*.json` - 4 specs (92/14/14/9 steps) |
| `0x8c164320` | `guard_free` | `notes/verifications/0x8c164320*.json` - 5 specs (9-90 steps) |
| `0x8c0b5180` | `write_element_fields` | `notes/verifications/0x8c0b5180*.json` - 5 specs (67/31 steps) |
| `0x8c18c060` | `guard_free` | `notes/verifications/0x8c18c060*.json` - 5 specs (9-90 steps) |
| `0x8c18d06c` | `guard_free` | `notes/verifications/0x8c18d06c*.json` - 5 specs (9-90 steps) |
| `0x8c18e40c` | `tail_thunk_0x8c1c3a2c` | `notes/verifications/0x8c18e40c.json` - 9 oracle steps |
| `0x8c0b8400` | `guard_free` | `notes/verifications/0x8c0b8400*.json` - 4 specs |
| `0x8c1bb45c` | `guard_free` | `notes/verifications/0x8c1bb45c*.json` - 5 specs (9-90 steps) |
| `0x8c1be874` | `guard_free_clear_byte` | `notes/verifications/0x8c1be874*.json` - 4 specs (9-93 steps) |
| `0x8c1bbe3c` | `clear_flag_then_boot_hook` | `notes/verifications/0x8c1bbe3c.json` - 19 oracle steps |
| `0x8c0bde70` | `element_lookup_108` | `notes/verifications/0x8c0bde70*.json` - 3 specs (6 steps) |
| `0x8c0c6104` | `tail_thunk_0x8c05d28c` | `notes/verifications/0x8c0c6104.json` - 5 oracle steps |
| `0x8c0c814c` | `warm_lfsr_then_hash` | `notes/verifications/0x8c0c814c.json` - 23,646 oracle steps |
| `0x8c2219fc` | `zero_0x400_at_global` | `notes/verifications/0x8c2219fc.json` - 5,136 oracle steps |
| `0x8c221e8e` | `copy_into_global_table` | `notes/verifications/0x8c221e8e.json` - 60 oracle steps |
| `0x8c220eda` | `tail_thunk` | `notes/verifications/0x8c220eda.json` - 52 oracle steps |
| `0x8c2214e4` | `index_load_bool` | `notes/verifications/0x8c2214e4.json` - 17 oracle steps |
| `0x8c2214fe` | `index_load_value` | `notes/verifications/0x8c2214fe.json` - 19 oracle steps |
| `0x8c222924` | `clear_table_then_init` | `notes/verifications/0x8c222924.json` - 13,141 oracle steps |
| `0x8c0cbe30` | `guard_free` | `notes/verifications/0x8c0cbe30*.json` - 5 specs (9-90 steps) |
| `0x8c0c9f98` | `free_ten_slots_then_array` | `notes/verifications/0x8c0c9f98*.json` - 3 specs (16-801 steps) |
| `0x8c0d1590` | `stamp_vtable_call_then_free` | `notes/verifications/0x8c0d1590*.json` - 4 specs (12-112 steps) |
| `0x8c0d0020` | `clamp_abs_to_range` | `notes/verifications/0x8c0d0020*.json` - 4 specs (23-24 steps) |
| `0x8c0f1b78` | `set_byte_flag_pair` | `notes/verifications/0x8c0f1b78*.json` - 4 specs (8-13 steps) |
| `0x8c0f1ae0` | `tail_thunk_0x8c1c3a2c` | `notes/verifications/0x8c0f1ae0.json` - 9 oracle steps |
| `0x8c0e2c48` | `find_table_slot` | `notes/verifications/0x8c0e2c48*.json` - 3 specs (123-124 steps) |
| `0x8c0fda3c` | `remap_args_forward` | `notes/verifications/0x8c0fda3c*.json` - 2 specs (7 and 35 steps) |
| `0x8c0f8f10` | `release_then_free` | `notes/verifications/0x8c0f8f10*.json` - 4 specs (12-127 steps) |
| `0x8c0f9ab2` | `strcmp_body` | `notes/verifications/0x8c0f9ab2*.json` - 4 specs (15-67 steps) |
| `0x8c106882` | `strcmp_body_2` | `notes/verifications/0x8c106882*.json` - 4 specs (15-67 steps) |
| `0x8c0ed854` | `zero_and_return` | `notes/verifications/0x8c0ed854*.json` - 61 oracle steps |
| `0x8c0fdc60` | `clear_global_then_free` | `notes/verifications/0x8c0fdc60*.json` - 3 specs (9-78 steps) |
| `0x8c0ed86c` | `release_four_then_zero` | `notes/verifications/0x8c0ed86c*.json` - 298 oracle steps |
| `0x8c237780` | `strided_table_search` | `notes/verifications/0x8c237780*.json` - 3 specs (15-48 steps) |
| `0x8c239820` | `slot_store_or_defer` | `notes/verifications/0x8c239820*.json` - 2 specs (12 and 21 steps) |
| `0x8c23991c` | `clear_slot_bit` | `notes/verifications/0x8c23991c*.json` - 2 specs (hit and miss, 83 steps each) |
| `0x8c23bb48` | `clear_record_flag` | `notes/verifications/0x8c23bb48*.json` - 2 specs (47 and 55 steps) |
| `0x8c23b8c4` | `claim_first_free_slot` | `notes/verifications/0x8c23b8c4*.json` - 4 specs (10-55 steps); `shll` T bit was the hidden side effect |
| `0x8c23a560` | `mark_found_flags` | `notes/verifications/0x8c23a560*.json` - 3 specs (52-62 steps) |
| `0x8c23a740` | `write_record_flags` | `notes/verifications/0x8c23a740*.json` - 4 specs (44-64 steps) |
| `0x8c23b040` | `release_slot` | `notes/verifications/0x8c23b040*.json` - 3 specs (148-155 steps) |
| `0x8c0f110c` | `free_array_field` | `notes/verifications/0x8c0f110c*.json` - 3 specs (7-72 steps) |
| `0x8c220ed4` | `tail_thunk` | `notes/verifications/0x8c220ed4.json` - 52 oracle steps |
| `0x8c1bb438` | `guard_free` | `notes/verifications/0x8c1bb438*.json` - 5 specs (9-90 steps) |
| `0x8c1bb414` | `guard_free` | `notes/verifications/0x8c1bb414*.json` - 5 specs (9-90 steps) |
| `0x8c0bdddc` | `guard_free` | `notes/verifications/0x8c0bdddc*.json` - 5 specs |
| `0x8c0b9eb8` | `guard_free` | `notes/verifications/0x8c0b9eb8*.json` - 5 specs |
| `0x8c0b9744` | `guard_free` | `notes/verifications/0x8c0b9744*.json` - 5 specs |
| `0x8c18e1e4` | `tail_thunk_0x8c1c3a2c` | `notes/verifications/0x8c18e1e4.json` - 9 oracle steps |
| `0x8c18d048` | `guard_free` | `notes/verifications/0x8c18d048*.json` - 5 specs (9-90 steps) |
| `0x8c18c570` | `guard_free` | `notes/verifications/0x8c18c570*.json` - 5 specs (9-90 steps) |
| `0x8c14cdcc` | `walk_48_byte_array_and_free` | `notes/verifications/0x8c14cdcc*.json` - 4 specs, 15-141 steps |
| `0x8c1466b8` | `publish_mode_and_clear_slots` | `notes/verifications/0x8c1466b8*.json` - 3 specs, 388-395 steps |
| `0x8c092a50` | `release_global_object` | `notes/verifications/0x8c092a50*.json` - 2 specs (158 and 9 steps) |
| `0x8c09035c` | `object_teardown` | `notes/verifications/0x8c09035c*.json` - 4 specs, 12-168 steps |
| `0x8c015148` | `FUN_8c015148 (bignum ror, carry bit)` | `notes/verifications/0x8c015148.json` — 85 oracle steps; r4=r6=base+n; r0=r7=final carry; r3=last>>1; r12/r13/r14 pushed+popped (r13=1 set in entry delay slot); T=final cmp/ge (1 if loop ran, cmp/pl if skipped) |
| `0x8c015194` | `FUN_8c015194` | `notes/verifications/0x8c015194*.json` — both specs (74 and 12 oracle steps); r7=8-r6/carry, r12=mask, r3=-exts.w(r6), r2/r1 last byte ops, r0=(final carry != 0), T=(carry==0); r12/r13/r14 net unchanged |
| `0x8c0152fc` | `FUN_8c0152fc (guarded slot store)` | `notes/verifications/0x8c0152fc.json` — 12 oracle steps; rts delay slot mov #1,r0 on store path; r2=slot ptr (loaded from cell 0x8c015320); r1=0x450; r3=0x200; r4=p-q; zero-byte path keeps r3=*p sign-extended and r4 untouched with T=1; short path T=0 |
| `0x8c015324` | `FUN_8c015324 (bignum sub, borrow)` | `notes/verifications/0x8c015324.json` — 110 oracle steps; r1 ends exactly at a (entry bf.s delay slot +1); r6=a-1; r7=b-1; r2=r3=last result byte; return -1 only if borrow&&r4==0 else 0 if all result bytes 0 else 1; T from last tst/cmp/eq; result store is mov.b r3,@-r1 (byte at pre-decremented r1, delay slot) |
| `0x8c01539c` | `FUN_8c01539c (memcmp-ish)` | `notes/verifications/0x8c01539c.json` — 35 oracle steps; bf.s delay slot copies r5->r7 even on n==0 return; r4 ends one past last byte examined; r7 then decremented; r2 keeps sign-extended b byte of the deciding pair; r3 reloaded zero-extended at exit; T=deciding compare (0 mismatch, 1 counter/n==0) |
| `0x8c015eea` | `FUN_8c015eea (decimal string increment)` | `notes/verifications/0x8c015eea.json` — 49 oracle steps; r0 never written; r1=48; r7=57 via bf.s delay slot; r5/r6 decrement in lockstep (delay slot) so carry writes hit current byte and byte below; r3 ends as r5-1 pointer after last carry (or base[n-1]+1); r6 ends at r4, r5 at r4+1; T=final cmp/gt on base[1] |
| `0x8c0163e8` | `FUN_8c0163e8` | `notes/verifications/0x8c0163e8.json` — 39 oracle steps; record copy while cmp/str clear, 4-byte tail unrolled (4th byte sets no flag, T from 3rd); r3/r4/r5/r6 end as popped stack zeros, r15 net +8, r0=orig dst |
| `0x8c01644c` | `FUN_8c01644c (unrolled memcpy)` | `notes/verifications/0x8c01644c.json` — 32 oracle steps; r2/r3/r4 pushed+popped so only r0/r1 survive; r0=sign-extended last src byte; r1=dst+4*full_blocks (add #4,r1 is not a delay slot); stores are mov.b at r1, r1+1..3 per block; T=final cmp/hi (1 only on n==0) |
| `0x8c0174c0` | `FUN_8c0174c0 (set global, return old)` | `notes/verifications/0x8c0174c0.json` — 8 oracle steps; r1=slot pointer loaded from cell 0x8c0174d0; r0=old value; r14 saved/restored; T untouched |
| `0x8c0176f0` | `pat_match (r4=pattern, r5=string, r6=int* out)` | `notes/verifications/0x8c0176f0.json` — 61 oracle steps; stores 0 at [r6] on entry, parsed decimal at [r6] on the !..!..! capture path; r5 clobbered to -48 (delay slot 0x8c017778) on the capture path and to sext8(*r4) in the tail loop (0x8c01782e); r1/r3/r4 left advanced past consumed input; r1++/r4++ delay slots fire on taken AND fall-through branches; T=1 on every return-1 and on the NUL-mismatch exit, T=0 on the two compare-mismatch exits |
| `0x8c017844` | `FUN_8c017844 (push node on list)` | `notes/verifications/0x8c017844.json` — 12 oracle steps; r0=r4 (return); r1=old head; r3=head-pointer cell; store order node[0]=old, node[1]=r5, node[2]=r4, then *head=node; T untouched |
| `0x8c017b94` | `FUN_8c017b94` | `notes/verifications/0x8c017b94.json` — 17 oracle steps; r6=unpacked; r3/r1=shifted src bytes on long paths; r4+=1..4; r0=r4new; T per path; [r5]=r6 |
| `0x8c017c0c` | `FUN_8c017c0c` | `notes/verifications/0x8c017c0c.json` — 38 oracle steps; r4 itself ends advanced (delay-slot mov r4,r0 mirrors it into r0); r6=decoded value; r3=b1<<16 and r1=b2<<8 (shll16/shll8 in place); T=bit2 of first byte (1 only on 4-byte path); follow-up bytes read at r4+1..3 (group-8 base=bits 7-4, disp=bits 3-0) |
| `0x8c017d6c` | `FUN_8c017d6c` | `notes/verifications/0x8c017d6c.json` — 10 oracle steps; [r4+12]=[r4+16]=pool ptr 0x8c1f2e88; r0=1 |
| `0x8c017e30` | `FUN_8c017e30` | `notes/verifications/0x8c017e30.json` — 119 oracle steps; r3=0/16; r1=-4; r5/r6 recomputed; r7=count2*4; T=0; stores two word runs + [r4+20] |
| `0x8c017efc` | `FUN_8c017efc` | `notes/verifications/0x8c017efc.json` — 19 oracle steps; r1=last compared left word; T=last cmp/eq; r0=0/1 |
| `0x8c017f24` | `FUN_8c017f24` | `notes/verifications/0x8c017f24.json` — 16 oracle steps; r0=0/1; T=last tst |
| `0x8c017f48` | `FUN_8c017f48` | `notes/verifications/0x8c017f48.json` — 22 oracle steps; r2=sign-extended last byte (0); r4/r5 advanced past NUL; T=1 at exit; byte stores dst.. |
| `0x8c018274` | `FUN_8c018274` | `notes/verifications/0x8c018274.json` — 6 oracle steps; r2=0x8c1f2f98; r3=*(r2); r1=0; [r3+8]=0 |
| `0x8c018474` | `FUN_8c018484` | `notes/verifications/0x8c018474.json` — 12 oracle steps; "*r4 |= 0x80 (tas.b) and T ends set; entry is 8 nops before the tas/bf loop; no register changes; only a zero byte terminates the spin" |
| `0x8c018f08` | `link_and_zero_blocks` | `notes/verifications/0x8c018f08.json` — 21802 oracle steps |
| `0x8c018f8c` | `FUN_8c018f8c` | `notes/verifications/0x8c018f8c.json` — 8 oracle steps; u32 r4 at 0x8c23da80 (now in the oracle RAM window); bytes 0 at [r4] and [r4+0x100]; r1=0; r0=r4 |
| `0x8c018fbc` | `FUN_8c018fbc` | `notes/verifications/0x8c018fbc.json` — 2 oracle steps; none |
| `0x8c018fec` | `FUN_8c018fec (store to global, return arg)` | `notes/verifications/0x8c018fec.json` — 4 oracle steps; rts delay slot mov r4,r0 so r0=stored value even though r0 first held the pointer; T untouched |
| `0x8c0192fc` | `FUN_8c0192fc` | `notes/verifications/0x8c0192fc.json` — 8 oracle steps; store [global]=r4, [r4..r4+8]=0; r1=0; r0=r4 |
| `0x8c019670` | `FUN_8c019670` | `notes/verifications/0x8c019670.json` — 4 oracle steps; r4 += 0x70; r0 = zero-extended byte |
| `0x8c019678` | `FUN_8c019678` | `notes/verifications/0x8c019678.json` — 10 oracle steps; T=(field==3); r0=field or flag base 0xac; r1=1; byte store at 0xac+r4 |
| `0x8c01ace8` | `FUN_8c01ace8` | `notes/verifications/0x8c01ace8.json` — 6 oracle steps; r1=table base; r5 <<= 2; r0=final field load |
| `0x8c01acf4` | `FUN_8c01acf4` | `notes/verifications/0x8c01acf4.json` — 6 oracle steps; r1=table base; r5 <<= 2; r0=final field load |
| `0x8c01c234` | `FUN_8c01c234` | `notes/verifications/0x8c01c234.json` — 10 oracle steps; T=(field==1); r0=field or flag base 0x85; r1=1; byte store at 0x85+r4 |
| `0x8c01e484` | `FUN_8c01e484` | `notes/verifications/0x8c01e484.json` — 2 oracle steps; r0=r4 |
| `0x8c01e704` | `FUN_8c01e704` | `notes/verifications/0x8c01e704.json` — 4 oracle steps; r1=[r5]; [r4]=r1; r0=r4 |
| `0x8c01e70c` | `FUN_8c01e70c` | `notes/verifications/0x8c01e70c.json` — 2 oracle steps; none |
| `0x8c01e828` | `FUN_8c01e828` | `notes/verifications/0x8c01e828.json` — 11 oracle steps; [r4+8] rewritten twice; T=last tst; r4=0 iff decremented value nonzero; r0=r4 (possibly zeroed) |
| `0x8c01ec2c` | `FUN_8c01ec2c (field getter)` | `notes/verifications/0x8c01ec2c.json` — 2 oracle steps; r0 = [r4+0xc] |
| `0x8c01f1cc` | `FUN_8c01f1cc (0x104-byte record copy)` | `notes/verifications/0x8c01f1cc.json` — 488 oracle steps; r4/r6 += 0x104 per record; r0=r6; r1=r6_iter+0x100; r3=0; r7=dst+0x100; fr0=copied float; T=1; 128B+4B stores per record when r6!=0 |
| `0x8c020394` | `FUN_8c020394 (stub)` | `notes/verifications/0x8c020394.json` — 2 oracle steps; none |
| `0x8c020398` | `FUN_8c020398 (stub)` | `notes/verifications/0x8c020398.json` — 2 oracle steps; none |
| `0x8c0203d4` | `call_if_flag_clear` | `notes/verifications/0x8c0203d4.json` — 35 oracle steps |
| `0x8c0203f8` | `FUN_8c0203f8 (indexed global store)` | `notes/verifications/0x8c0203f8.json` — 3 oracle steps; r0 = [0x8c020400] (0x10c); [r0+r4] = r5 |
| `0x8c02097c` | `FUN_8c02097c (16-byte record copy)` | `notes/verifications/0x8c02097c.json` — 46 oracle steps; r4/r6 += 0x10 per iteration; r0=r6 final; T=1; 4 stores per record when r6!=0 |
| `0x8c024c3c` | `FUN_8c024c3c (fr4 store at +0x14)` | `notes/verifications/0x8c024c3c.json` — 3 oracle steps; r4 += 0x14 before the store; [r4] = fr4 bits |
| `0x8c0252cc` | `FUN_8c0252cc (BST walk, result via [r15])` | `notes/verifications/0x8c0252cc.json` — 29 oracle steps; r3=0 (bt.s delay slot, even when root null); r4 += 4 only on the r4+4 store path; r6=0; r0=[r15]; r1/r0 hold last loop loads or final best[3]/key; T = key>best[3] or 0 when best null |
| `0x8c026a50` | `FUN_8c026a50 (16-byte header init)` | `notes/verifications/0x8c026a50.json` — 19 oracle steps; r4=0; r6=base; r1=base+4; r3=base+8; r0=base; stores at base..base+0xf with +0xc written twice (0, then base+4) |
| `0x8c026c90` | `FUN_8c026c90 (tagged list unlink)` | `notes/verifications/0x8c026c90.json` — 37 oracle steps; r6=next|old_flag; r3=0xfffffffe (unconditional literal load at 0x8c026cb2); r1=[r4+8]&~1; r0=[r4+8]&1; T=cmp/eq at 0x8c026cc6; node->next = next->prev (0x8c026c9a is @(4,r6), not @(4,r4)) |
| `0x8c026ce0` | `list_unlink` | `notes/verifications/0x8c026ce0.json` — 37 oracle steps; r6=node->next via bf.s delay slot (always); r3 ends 0xfffffffe (literal load @0x8c026d02); r1=node->f8&~1; r0=node->f8&1; r6=next|(node->f8&1); final T from cmp/eq r0,r4 not from trailing or |
| `0x8c026de4` | `set_two_fields` | `notes/verifications/0x8c026de4.json` — 5 oracle steps; r0 ends as [r4+0x30] (was 0x4c mid-function); second store [r0+0x34]=r5 is the rts delay slot |
| `0x8c026e34` | `move_pair_clear_src` | `notes/verifications/0x8c026e34.json` — 8 oracle steps; r0=0; r1 ends as second load [r5+0x30]; both zeroing stores incl. rts delay slot |
| `0x8c026e50` | `call_and_clear_flag` | `notes/verifications/0x8c026e50.json` — 15 oracle steps |
| `0x8c02727c` | `copy_float_field` | `notes/verifications/0x8c02727c.json` — 6 oracle steps; r4 ends base+4; r0 ends base+0x34; fr0 holds loaded bits; store at [base+0x34] is rts delay slot |
| `0x8c0273a0` | `copy_triple_to_0x68` | `notes/verifications/0x8c0273a0.json` — 8 oracle steps; r4 ends base+0x68 (arg register itself modified); r1=[r5]; r0=[r5+8]; third store is rts delay slot |
| `0x8c0273b0` | `copy_two_48B_blocks` | `notes/verifications/0x8c0273b0.json` — 187 oracle steps; r4 ends p1+0xb4 (not saved/restored); r1=6 r3=48 r7=6; r0=last sign-extended mov.w; T=0 from final cmp/hi 6>6; r10-r14 pushed/popped keep entry values |
| `0x8c027418` | `copy_48B_plus_ptr_table` | `notes/verifications/0x8c027418.json` — 138 oracle steps; r4 ends p1+0x144; r0=20 r1=6 r3=48 r7=24; T=0; the u32 store [r4+r0]=r13 is the bt.s delay slot |
| `0x8c0274c8` | `empty_rts` | `notes/verifications/0x8c0274c8.json` — 2 oracle steps; none (rts ; nop) |
| `0x8c0274cc` | `table_byte_lookup` | `notes/verifications/0x8c0274cc.json` — 4 oracle steps; r0 first becomes table base 0x170 (literal @0x8c0274d4) then the byte at 0x170+r4; extu.b rts delay slot zero-extends the sign-extending mov.b |
| `0x8c0274d8` | `FUN_8c0274d8` | `notes/verifications/0x8c0274d8.json` — 4 oracle steps; r0 = **[r4+0x174] via literal pool 0x8c0274e0; no other reg touched |
| `0x8c0274e4` | `FUN_8c0274e4` | `notes/verifications/0x8c0274e4.json` — 4 oracle steps; r0 = **[r4+0x178] via literal pool 0x8c0274ec |
| `0x8c0274f0` | `FUN_8c0274f0` | `notes/verifications/0x8c0274f0.json` — 6 oracle steps; r1=0x144 leftover, r5 scaled in place (<<2), r4 mutated in place (+=0x144), r0=[r5*4+r4+0x144] |
| `0x8c027500` | `FUN_8c027500` | `notes/verifications/0x8c027500.json` — 24 oracle steps; r7=r4+0x114+r5*8, r3=r4+0x144, r5<<=2, r1=[r4+0x164], r0=r5<<2, T=bit29 of r5 (shll); 4 u16 stores at r7+0..6 + u32 store at r3+r0 in rts delay slot |
| `0x8c02753c` | `FUN_8c02753c` | `notes/verifications/0x8c02753c.json` — 26 oracle steps; r7=[r15] (dest pointer loaded via r15), r5*=8, r4+=0x11a, r6=first u16, r0=last u16, r3=0x118, r1=0x11a, T=bit29 of r5, 4 u16 stores at r7+0..6 (4th is rts delay slot) |
| `0x8c027580` | `FUN_8c027580` | `notes/verifications/0x8c027580.json` — 312 oracle steps; stores: 6x(4xu16 p2->p1 at +0xb4/0xe4/0x114+off, u32 p2->p1 at +0x144+dw) in bt.s-delay-slot order; r0=20 (pre-inc r9), r1=last u32, r3/r6/r7=advanced +0x114/+0xe4 ptrs, r4/r5=+0x144 ptrs, T=0; r8-r15 restored by epilogue |
| `0x8c028d90` | `FUN_8c028d90` | `notes/verifications/0x8c028d90.json` — 7 oracle steps; fr5 overwritten on 2 of 3 paths, fr0=fr5 in rts delay slot, T=last fcmp (per path) |
| `0x8c028da8` | `FUN_8c028da8` | `notes/verifications/0x8c028da8.json` — 8 oracle steps; r1=chosen offset, r0=0x8c1f47c0+offset via rts delay-slot add, T=0 on matched-bit paths / 1 on default, r5 clobbered to r5&0x80 on paths reaching the last test |
| `0x8c02b3a0` | `FUN_8c02b3a0` | `notes/verifications/0x8c02b3a0.json` — 151 oracle steps; stores: [r4+0x3f8]=state(0/1/2) u32, byte [r4+0x405]=0 (paths 1/2 only), 17 scaled f32 at [r4+4]+off, [r4+0x4f8]=0.0, [r4+0x4f0]=[r4+0x4ec], [r4+0x4f4]=1.0 (p1/p3) or 0.99f (p2, delay slot of bra); r4+=0x4f4 on all paths; r11-r14/r15 restored by epilogue; T=1 on p1/p2, 0 on p3; leftover regs p1: r0=0x405 r1=0 r3=0x4f4 r5=r4+0x4ec r6=r4+0x4f0 r7=r4+0x4f8; p2: r0=0x8c02b694 r1=0x4f4 r3=r4+0x4ec r5=r4+0x4f0 r6=r4+0x4f8 r7=0; p3: r0=0x4f4 r1=r4+0x4f0 r3=r4+0x4f8 r5=2; fr leftovers p1 fr1=[r4+0x4ec] fr2=[P+0x380]*1.15 fr3=1.15, p2 fr1=[P+0x380]*1.07 fr2=1.07 fr3=1.1, p3 fr0=1.0 |
| `0x8c02baf8` | `FUN_8c02baf8` | `notes/verifications/0x8c02baf8.json` — 48 oracle steps; r4+=0x4ac, r5+=40, r6=r5+36, r7=r4+0x4a4, r3=r4+0x4a8, r1=0x4ac, r0=[r4+0x4a0], fr0=f32[r4+0x4ac]; 10 stores at r5+0..28,32,36,40 |
| `0x8c02be1c` | `FUN_8c02be1c` | `notes/verifications/0x8c02be1c.json` — 9 oracle steps; r6=1, per-path r0/r1/r3/r5/fr0/fr1 leftovers, T=last cmp; stores [r4+0x3bc]=r7, u8[r4+0x3d0]=1, [r4+0x3b8]+=-1 (bra delay slot) or -2 |
| `0x8c02be90` | `FUN_8c02be90` | `notes/verifications/0x8c02be90.json` — 8 oracle steps; r5=0x3b8, r3=counter, r0/r1 leftovers, T per path; stores [r4+0x3bc]=r3, u8[r4+0x3d0]=1, [r4+0x3b8]+=2 (rts delay slot, -1 case) or +=1 |
| `0x8c02bef0` | `FUN_8c02bef0` | `notes/verifications/0x8c02bef0.json` — 11 oracle steps; r4+=24 (store uses the mutated r4), r0=0x8c02bf10, fr0=clamp(fr4,-1,1), T=last fcmp; f32 store at [r4+24] in rts delay slot |
| `0x8c02bf14` | `FUN_8c02bf14` | `notes/verifications/0x8c02bf14.json` — 10 oracle steps; store fr0 at [r4+12] via rts delay slot; r4+=12 before store; T per path (1 if fr4<0 or fr4>1, else 0) |
| `0x8c02c0e8` | `FUN_8c02c0e8` | `notes/verifications/0x8c02c0e8.json` — 18 oracle steps; r0=r5&0xFF then overwritten with const 0x270; r1=[r4+4]+0x270; r4+=12 before all float access; up to 2 stores at [r4] (result, then 1.0f if result>1.0); T=1 only on zero path (tst) and clamp path |
| `0x8c02c2a4` | `FUN_8c02c2a4` | `notes/verifications/0x8c02c2a4.json` — 143 oracle steps; "r4 advanced 4*0xac (bt.s delay `add r3,r4` runs on the final pass); r5 stays p0+0x3e0 for all 4 iterations; r14/r15 preserved via stack; r6=last p+0x70, r0=last p+0xcc; fr4 left negated; T ends clear (4>4 false)" |
| `0x8c02c41c` | `accumulate_and_scatter` | `notes/verifications/0x8c02c41c.json` — 50 oracle steps |
| `0x8c02c8d4` | `FUN_8c02c8d4` | `notes/verifications/0x8c02c8d4.json` — 83 oracle steps; writes byte@base+0x464, f32@base+0x460/0x468; r4 advanced to base+0x460 (early) or base+0x468; r6=base+0x460 used by rts delay slot; T=last fcmp/cmp of path |
| `0x8c02ca10` | `FUN_8c02ca10` | `notes/verifications/0x8c02ca10.json` — 14 oracle steps; r0=[r4+4]+0x278, r1=0x3a0, r3=[r4+4]+0x27c, r4+=0x3a0 all left behind; fr1/fr2 stay loaded; fmul in rts delay slot; no T update |
| `0x8c02ca34` | `FUN_8c02ca34` | `notes/verifications/0x8c02ca34.json` — 26 oracle steps; fldi1 fr0 in bf.s delay slot runs on EVERY path (it is the 1.0 return); r0 ends as slot value/r4+0x450/mova const; r1 ends 0x39c; r3,r4 advanced; T=0 zero path, 1 early-outs and fr2>fr3, 0 otherwise |
| `0x8c02cb28` | `FUN_8c02cb28` | `notes/verifications/0x8c02cb28.json` — 16 oracle steps; r0=0x3e0, r1=[r4+4]+0x3e0, r3=0x450, r4+=0x450 left behind; fr1/fr2 loaded; return value in fr0 (0.0 if quotient<0, 1.0 if >1, else quotient); T=1 on the two early-outs, 0 in-range |
| `0x8c02d5e4` | `FUN_8c02d5e4` | `notes/verifications/0x8c02d5e4.json` — 78 oracle steps; stores: s+0x3d4 once, s+0x3dc four times (only last survives), s+0x3d8 once; T from idx normalization (1 for idx -1 or 0, else 0); r0=0x8c02d6b4 (mova), r1=w+0x360, r4=s+0x3d8, r5=0xec, r6=w+0x33c+4*idx, r7=0x3d8; fr0/fr1=last load/product, fr2=stored value, fr3/fr4=table floats |
| `0x8c031c0c` | `FUN_8c031c0c` | `notes/verifications/0x8c031c0c.json` — 11 oracle steps; r5=[r4+4]+0x274 via bf.s delay slot on BOTH paths; r0=0x3a4 then 0x8c; r4+=0x3a4; fr2=1.0, fr3=input stay; final fmul fr1,fr0 in rts delay slot |
| `0x8c0326a0` | `FUN_8c0326a0` | `notes/verifications/0x8c0326a0.json` — 17 oracle steps; 3 float stores at r4+0x4f8/0x4f0/0x4f4, the last in rts delay slot AFTER r4+=0x4f4; r0=0x4f4, r1=r4+0x4f0, r3=r4+0x4f8, r4 advanced; fr0 keeps loaded value |
| `0x8c032ba0` | `FUN_8c032ba0` | `notes/verifications/0x8c032ba0.json` — 85 oracle steps; writes f32@base+0x390/0x394/0x380 (block1) and base+0xe4/0xe8/0x174/0x178 (block2); r1 deref'd through pool ptr 0x8c032cb0 in bt.s delay slot on BOTH paths; r1 advanced by mode offset 0x88/0x98/0xa8/0xb8 via bra delay slots; r4 ends base+0x178; r5=r1+4; T=last tst/cmp |
| `0x8c033618` | `FUN_8c033618` | `notes/verifications/0x8c033618.json` — 128 oracle steps; writes 16 f32 fields incl. base+0x3e4 twice (write, reread, *1000, rewrite); r4 advanced base+0x11c; r13/r14 saved via r15 and restored; r14=0x3e4 scratch; constants via mova/pool reads |
| `0x8c03376c` | `FUN_8c03376c` | `notes/verifications/0x8c03376c.json` — 128 oracle steps; same 16 f32 stores as 0x8c033618 but divides by base+0x278, multiplies base+0x364 by C3, ends ([base+0x31c]*[base+0x358]/([base+0x11c]*pi))*[base+0x3e4]/1000; r4 advanced base+0x11c; r13/r14 via stack |
| `0x8c03703c` | `FUN_8c03703c` | `notes/verifications/0x8c03703c.json` — 5 oracle steps; r0 loaded twice (ptr then value); r1=[r15] slot pointer; delay-slot store through it |
| `0x8c03943c` | `FUN_8c03943c` | `notes/verifications/0x8c03943c.json` — 2 oracle steps; r0=[r4+0x24] in rts delay slot - whole port is the delay slot |
| `0x8c039440` | `FUN_8c039440` | `notes/verifications/0x8c039440.json` — 2 oracle steps; r0=[r4+0x20] in rts delay slot - whole port is the delay slot |
| `0x8c03c75c` | `FUN_8c03c75c` | `notes/verifications/0x8c03c75c.json` — 12 oracle steps; stores fr7/fr4/fr5/fr6 at [r4..r4+12]; leaves r3=r4+4, r1=r4+8; r0=r4 from rts delay slot |
| `0x8c03c950` | `FUN_8c03c950` | `notes/verifications/0x8c03c950.json` — 4 oracle steps; stores 0 at [0xfc+r4]; leaves r0=0xfc (literal base), r1=0 |
| `0x8c03c95c` | `FUN_8c03c95c` | `notes/verifications/0x8c03c95c.json` — 4 oracle steps; stores 0 at [0xf8+r4]; leaves r0=0xf8, r1=0 |
| `0x8c03e534` | `FUN_8c03e534` | `notes/verifications/0x8c03e534.json` — 23 oracle steps; stores: 17 u32 into slot + [s+0x80]=new current (reset to base if == end) or byte s+0x44=0 on blind path; r5 advanced by 68 (or reloaded to base on wrap path), r3/r6/r7=scan ptrs, r1=current/reset, fr0=0.95 const, fr1=last slot float, T=last cmp/eq or cmp/hi |
| `0x8c03eb7c` | `FUN_8c03eb7c` | `notes/verifications/0x8c03eb7c.json` — 4 oracle steps; stores r4 at [literal 0x8c23dbf0]; r0=r4 |
| `0x8c03ed84` | `FUN_8c03ed84` | `notes/verifications/0x8c03ed84.json` — 3 oracle steps; r0=0x3c4+r4 |
| `0x8c03ee9c` | `FUN_8c03ee9c` | `notes/verifications/0x8c03ee9c.json` — 4 oracle steps; byte-stores 1 at [0x3bc+r4]; leaves r0=0x3bc, r1=1 |
| `0x8c043474` | `FUN_8c043474` | `notes/verifications/0x8c043474.json` — 4 oracle steps; stores [r4] at [[r15]]; leaves r1=[r15], r0=[r4] |
| `0x8c04347c` | `FUN_8c04347c` | `notes/verifications/0x8c04347c.json` — 5 oracle steps; stores [[r4]] at [[r15]]; leaves r1=[r15], r0=[[r4]] |
| `0x8c043960` | `FUN_8c043960` | `notes/verifications/0x8c043960.json` — 3 oracle steps; stores r4 at [literal 0x8c23dbf8]; leaves r0=global addr |
| `0x8c0443ec` | `FUN_8c0443ec` | `notes/verifications/0x8c0443ec.json` — 24 oracle steps; writes 11 u32s at [r4..r4+40] (1.0f,1,0,1,0,0,1,1,fr0,fr0,fr0); leaves r6=1, r5=r4+32, r3=r4+36, r1=r4+40, r0=r4, fr0=[0x8c04441c] |
| `0x8c04704c` | `FUN_8c04704c` | `notes/verifications/0x8c04704c.json` — 2 oracle steps; none |
| `0x8c04ace0` | `FUN_8c04ace0` | `notes/verifications/0x8c04ace0.json` — 9 oracle steps; r6 shifted in place (shll2); intermediate pointer left in r1; rts delay slot adds shifted r6 to r0 |
| `0x8c04b358` | `FUN_8c04b358` | `notes/verifications/0x8c04b358.json` — 4 oracle steps; second deref in rts delay slot |
| `0x8c04b360` | `FUN_8c04b360` | `notes/verifications/0x8c04b360.json` — 7 oracle steps; "r0 transiently holds pool offsets 0x98/0x9c before the result; ==6 path leaves r0 == T (movt); byte path leaves T clear; extu.b is the rts delay slot" |
| `0x8c04b420` | `FUN_8c04b420` | `notes/verifications/0x8c04b420.json` — 3 oracle steps; r0=0x98 literal-pool base left in r0; byte store in rts delay slot |
| `0x8c04b9ac` | `FUN_8c04b9ac` | `notes/verifications/0x8c04b9ac.json` — 3 oracle steps; r0=0xcc literal-pool base left in r0; word store in rts delay slot |
| `0x8c04ba4c` | `FUN_8c04ba4c` | `notes/verifications/0x8c04ba4c.json` — 5 oracle steps; r5 shifted in place (shll2); table base left in r1; element load in rts delay slot |
| `0x8c04d264` | `FUN_8c04d264` | `notes/verifications/0x8c04d264.json` — 20 oracle steps; r6=count, r3=index, r1=last element, r0=byte offset all live at return; T=1 on match, T=0 on -1 exit |
| `0x8c04d284` | `FUN_8c04d284` | `notes/verifications/0x8c04d284.json` — 6 oracle steps; r5 shifted in place; table base left in r1; element load in rts delay slot |
| `0x8c050718` | `FUN_8c050718` | `notes/verifications/0x8c050718.json` — 88 oracle steps; stores: [r4+0x24]=r5 (bf.s delay slot, both paths), [r4+0x28], [r4+8], [r4+0xc], [r4+0x10], 3 u32 to [r4+0x14..0x1c], [r4+0x34]=r5 (rts delay slot); T = (count-1==r5); r5=r4+0x14, r3=[r4+8]+0x14, r6=4*idx1, r7=r5 or 4*r5, r0=r5, r1=first copied u32 |
| `0x8c050c54` | `FUN_8c050c54` | `notes/verifications/0x8c050c54.json` — 2 oracle steps; r0=[r4+0x20] in rts delay slot |
| `0x8c050c58` | `FUN_8c050c58` | `notes/verifications/0x8c050c58.json` — 2 oracle steps; word store [r4+0x20]=r5 in rts delay slot |
| `0x8c050d90` | `FUN_8c050d90 (cursor step back)` | `notes/verifications/0x8c050d90.json` — 104 oracle steps; r0=p (mov r4,r0); r1=*(p+0x34)-1; r3=copied +0x14 field; r5=*(p+8)+0x14; r6=p+0x14; r7=cur on T=1 branch but cur*4 (shll2 at 0x8c050e60) on T=0; delay-slot stores of both bras and of rts all fire; stores to p+0x24,+0x28,+8,+0xc,+0x10,+0x14..0x1c,+0x34 in pc order |
| `0x8c050ea0` | `FUN_8c050ea0 (cursor step fwd)` | `notes/verifications/0x8c050ea0.json` — 102 oracle steps; first store is *(p+0x28)=old cur (clobbers +0x28 at 0x8c050ea6); wrap branch stores *(p+0x30) BEFORE *(p+0x24); cmp/gt is SIGNED (count > cur+1); r0=p; r1=*(p+0x34)+1; r3=copied +0x14; r5=*(p+8)+0x14; r6=p+0x14; r7=cur vs cur*4 per branch; exit T is the second cmp/eq, not the first cmp/gt |
| `0x8c050fa8` | `FUN_8c050fa8 (walker reset)` | `notes/verifications/0x8c050fa8.json` — 98 oracle steps; real push/pop of r14 (net identity, stack window); fldi0 clobbers fr0 to +0.0 and fmov.s stores 4 bytes 0 at p+0x38 (decompiler's fr1/FPSCR_SZ double-store branch does not exist in machine code); r0=*(src+0x1c); r1=*(src+0x14); r3=*(p+8)+0x14; r5=p+0x14; r6=entries[1] (p+0x38 only transient); r7=sel*4; T untouched by any instruction |
| `0x8c051140` | `FUN_8c051140` | `notes/verifications/0x8c051140.json` — 60 oracle steps; fr0=src[15] (no FPSCR.SZ branch in machine code, plain fmov); r0=dst, r1=counter+1, r3=&counter, r5=src+60, r6=src+56, r7=dst+56; store order 0,4,32,8,12,16,44,20,24,28,36,40,48,52,56,60,counter; T untouched |
| `0x8c0516cc` | `FUN_8c0516cc` | `notes/verifications/0x8c0516cc.json` — 110 oracle steps; fpul=r5 (lds); fr2=(float)(i32)r5 from entry bra delay slot; fr0=fr2/3.6f per iter; r1=base1, r5=0xfffffeff, r3=i, r6=4i, r7=table2[idx], r0=count on exit; T=0; stores at 80*e+base (AND mask) and 80*e+base+16 (float) |
| `0x8c051cfc` | `FUN_8c051cfc` | `notes/verifications/0x8c051cfc.json` — 2 oracle steps; "none: rts;nop stub, zero register/T/memory effects" |
| `0x8c051f5c` | `FUN_8c051f5c` | `notes/verifications/0x8c051f5c.json` — 3 oracle steps; "r0 passes through pool constant 0x194 before the delay-slot load overwrites it with *(u32*)(r4+0x194)" |
| `0x8c051f68` | `FUN_8c051f68` | `notes/verifications/0x8c051f68.json` — 3 oracle steps; "store *(u32*)(r4+0x194)=r5 is the rts delay slot; r0 left holding 0x194, not the stored value" |
| `0x8c0525a4` | `FUN_8c0525a4` | `notes/verifications/0x8c0525a4.json` — 4 oracle steps; the +0x24 load runs in the rts delay slot chained off the first load; only r0 changes; offset 0x190 from PC-relative literal at 0x8c0525ac |
| `0x8c05264c` | `FUN_8c05264c` | `notes/verifications/0x8c05264c.json` — 2 oracle steps; none - bare rts;nop, zero register/flag/memory effect |
| `0x8c052c88` | `FUN_8c052c88` | `notes/verifications/0x8c052c88.json` — 13 oracle steps; third byte store is in the rts delay slot; r0 ends 0x1f2 (last literal), r1=last byte via extu.b; offsets 0x1f0-0x1f2 from PC-relative literals; T untouched |
| `0x8c053be0` | `FUN_8c053be0` | `notes/verifications/0x8c053be0.json` — 2 oracle steps; r0=r4 via rts delay slot (C shows plain return) |
| `0x8c055fe4` | `FUN_8c055fe4` | `notes/verifications/0x8c055fe4.json` — 4 oracle steps; rts delay slot fmov @r0,fr0 loads from the INCREMENTED r0 (ptr+8); fr0+r0 change, T untouched; no FPSCR.SZ double path in machine code |
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
| `0x8c057ec4` | `FUN_8c057ec4` | `notes/verifications/0x8c057ec4.json` — 27 oracle steps; r1=r5+4 and r0=r4+20 leftovers (r0 computed BEFORE r4+=8 on the last else path); r5+=8 on every path incl. early rts; r4+=8 only on last block's else; fr0/fr1 keep the last block's loaded pair; T=last fcmp/gt; stores go to hi slot (r4+12/16/20) when fr1>hi else lo slot (r4+0/4/8) when lo>fr1 |
| `0x8c058128` | `FUN_8c058128` | `notes/verifications/0x8c058128.json` — 2 oracle steps; delay-slot load: r0=*(r4+0x38); T untouched |
| `0x8c058f5c` | `FUN_8c058f5c` | `notes/verifications/0x8c058f5c.json` — 29 oracle steps; r0/r1 clobbered every block - early exits leave r0=0 (rts delay mov #0,r0) with THAT block's r1=r4+off/r0=r5+off, final path leaves r1=r4+12; fr0/fr1 keep last compared pair; T=last fcmp/eq; return is movt r0 in rts delay slot |
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
| `0x8c05a828` | `FUN_8c05a828` | `notes/verifications/0x8c05a828.json` — 238 oracle steps; builds 4x4 float matrix at r4 from lit 2.0f@0x8c05a94c and fr4/fr5/fr6/fr7, transposes in place; final r0=4, r1=4, r2=sp+12, r3=r4+64, r5=16, r6=r4+44, r7=r4+56; r10-r14 restored by pops; T=0; fr0=last swapped val, fr1=0, fr2=1/(fr5-fr4), fr3=1, fr4=-fr4*fr2, fr5=fr5-fr4 |
| `0x8c05ab8c` | `FUN_8c05ab8c` | `notes/verifications/0x8c05ab8c.json` — 130 oracle steps; r0=4,r5=0,r6=4,r7=3,r1=base+0x2c,r3=base+0x38,fr0/fr1=last pre-swap dwords,T=0; 6 dword swaps at base+{4,8,12}x{16,32,48}; r11-r14 pushed/popped (net zero) |
| `0x8c05abd8` | `FUN_8c05abd8` | `notes/verifications/0x8c05abd8.json` — 3 oracle steps; r0=[r4+4] then reloaded to [r0] in rts delay slot |
| `0x8c05abe0` | `FUN_8c05abe0` | `notes/verifications/0x8c05abe0.json` — 3 oracle steps; r0=[r4+4] then reloaded to [r0+4] in rts delay slot |
| `0x8c05ac20` | `FUN_8c05ac20` | `notes/verifications/0x8c05ac20.json` — 7 oracle steps; r0=base+0x10 left live; base reloaded from r4+4 for 2nd store; [base+0xc]=fr4, [base+0x10]=fr5 (2nd in rts delay slot) |
| `0x8c05ac30` | `FUN_8c05ac30` | `notes/verifications/0x8c05ac30.json` — 3 oracle steps; r0=[r4+4] left live; [r0+0x14]=r5 in rts delay slot |
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
| `0x8c05dc1c` | `FUN_8c05dc1c` | `notes/verifications/0x8c05dc1c.json` — ok; r1=*r15, r0=**r4, store *r1=r0 in rts delay slot |
| `0x8c05de64` | `FUN_8c05de64` | `notes/verifications/0x8c05de64.json` — ok; struct init, [0]=pool value (single deref), [4]=0, [8]=byte(r5), [12]=r4+4; r0=r6, r4=0 |
| `0x8c05e038` | `FUN_8c05e038` | `notes/verifications/0x8c05e038.*.json` — both branches; byte loop, r0=diff or 0 at NUL, r4/r5 advanced, T=last cmp/eq |
| `0x8c05e108` | `FUN_8c05e108` | `notes/verifications/0x8c05e108.json` — ok; same shape, pool 0x8c1f7528 |
| `0x8c05e8cc` | `FUN_8c05e8cc` | `notes/verifications/0x8c05e8cc.json` — 15 oracle steps; list path: r1=0, r0=last node, T=1, [r4]=node in rts delay slot; hash path: r5=0xfffffffe (from [0x8c05e908]), r3/r6=bucket chain, r1=next&~1, r0=[bucket]==r3, T=0 (shlr+shll pair), [r4] stored every iteration |
| `0x8c05ef44` | `FUN_8c05ef44` | `notes/verifications/0x8c05ef44.json` — 17 oracle steps; fr0=1.0; [r4+28]=fr4 always (bf.s delay slot); r3=0xffffff8f; r1=0xfffffff0; r0=sign-extended final byte; T=fcmp/eq result; byte [r4+4] stored twice per path |
| `0x8c05ef7c` | `FUN_8c05ef7c` | `notes/verifications/0x8c05ef7c.*.json` — both branches; set: *(r4+8)\|=r5; clr: r1=*(r4+8)&~r5; T=(r6 byte==0) |
| `0x8c05f21c` | `FUN_8c05f21c` | `notes/verifications/0x8c05f21c.json` — 28 oracle steps; r4+=12; r1=[r5+40]; r0=[r5+48]; 13 dword stores to r4+12.. in order |
| `0x8c05f294` | `FUN_8c05f294` | `notes/verifications/0x8c05f294.json` — 3 oracle steps; r4+=4; r0=new r4 (rts delay slot) |
| `0x8c05f29c` | `FUN_8c05f29c` | `notes/verifications/0x8c05f29c.json` — 162 oracle steps; r1=*0x8c26cff0 (eq path) or [r5+52] (copy path); r3=r4+56; r5+=12 on copy path; r6=0x8c1d0f24; r7=r4+52; fr0=+0.0 (fldi0); fr1=table[3]; T=(r4==r5); header masks at +4/+5 are dead stores (overwritten by mov.l at 0x8c05f37c / copy block); bt.s delay slot stores 0 at +0 on both paths; r8..r14 restored by epilogue pops |
| `0x8c05f3f4` | `FUN_8c05f3f4` | `notes/verifications/0x8c05f3f4.json` — 120 oracle steps; r1=r4+4; r3=r4+56; r5=0x8c1d0f24; r6=0; r7=0x8c26cff0; fr0=+0.0 (fldi0); fr1=table[3]; T untouched (no flag ops); header masks at +4/+5 dead (overwritten at 0x8c05f4cc); r8..r14 restored by pops |
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
| `0x8c060e4c` | `FUN_8c060e4c` | `notes/verifications/0x8c060e4c.json` — ok; r4+=4, fr4 stored at new r4 in rts delay slot |
| `0x8c06116c` | `FUN_8c06116c` | `notes/verifications/0x8c06116c.json` — 3 oracle steps; r0 = 0x8c23dea0 (pointer load survives); store r4 at 0x8c23dea0 in rts delay slot |
| `0x8c0616bc` | `FUN_8c0616bc` | `notes/verifications/0x8c0616bc.json` — ok; r0=*(u32*)(0x268+r4) in rts delay slot |
| `0x8c061764` | `FUN_8c061764` | `notes/verifications/0x8c061764.json` — ok; bare rts;nop stub, zero effects |
| `0x8c061768` | `FUN_8c061768` | `notes/verifications/0x8c061768.json` — 5 oracle steps; r0 = 0x8c1f7a18 base; r4 = base + (r4<<2); store fr4 at r4 in rts delay slot |
| `0x8c061be8` | `FUN_8c061be8` | `notes/verifications/0x8c061be8.json` — 2 oracle steps; none (rts; nop) |
| `0x8c0654b4` | `FUN_8c0654b4` | `notes/verifications/0x8c0654b4.json` — 3 oracle steps; r4 += 0x40 (wrapping, mutates the argument); r0 = incremented r4 in rts delay slot |
| `0x8c066098` | `FUN_8c066098` | `notes/verifications/0x8c066098.json` — 295 oracle steps; fpul:=[r4+4]; r3=r4+8+48*inner_total; r1=0; r7=[r4]+1; r5 stored verbatim at r3+36 and r3+60 (argument reg, not a pointer); fr12:=fr5 only at inner-loop exit (plain bt skips delay slot while taken); fr12..15/r15 restored on exit; T=0 |
| `0x8c06616c` | `FUN_8c06616c` | `notes/verifications/0x8c06616c.json` — 277 oracle steps; fpul:=[r4+4]; r5 clobbered from argument to r4+8+48*inner_total; r3 stays 0, r7 restarts at 0 each outer iter; r0/r1 end as r5/r5+20 of last store; only two float stores per inner iter (r5+20, r5+44); fr12:=fr5 only at inner exit; T=0 |
| `0x8c067f08` | `FUN_8c067f08` | `notes/verifications/0x8c067f08.json` — 4 oracle steps; r0 = 0x9c, r1 = 1 survive; byte store mem[0x9c+r4] = 1 in rts delay slot |
| `0x8c069260` | `FUN_8c069260` | `notes/verifications/0x8c069260.json` — 4 oracle steps; T = last cmp/gt; r6 = r4 on the fall path; r0 = r5/r6/r4 per path |
| `0x8c069c74` | `FUN_8c069c74` | `notes/verifications/0x8c069c74.json` — 25 oracle steps; r6 = out+12, r0 = out+8, r1 = out+4, r3 = in+4, r5 = in+8; fr0..fr3 overwritten with scaled values; 4 stores incl. [out+12] in rts delay slot |
| `0x8c06b68c` | `FUN_8c06b68c` | `notes/verifications/0x8c06b68c.json` — 291 oracle steps; r0=r4; r1,r3=loaded words; r2=entry_r15-32; r5=entry_r15-44; r6=r4+64; r7=r4+80; r8-r14,r15 restored by pops; fr0-fr8 clobbered (fr9-fr15 untouched); T/sysregs unchanged; 144 non-stack stores to the r4 output struct |
| `0x8c06be1c` | `FUN_8c06be1c` | `notes/verifications/0x8c06be1c*.json` — both branches; phase1 stores *(r4+68) at abs 68+*(r4+72) or at *r5; phase2 r4+=72, store at abs 72+*(r4+68); T=last tst |
| `0x8c06be50` | `unknown` | `notes/verifications/0x8c06be50.json` — 13 oracle steps |
| `0x8c06c160` | `load_global_8c23f470` | `notes/verifications/0x8c06c160.json` — verified |
| `0x8c06c16c` | `FUN_8c06c16c` | `notes/verifications/0x8c06c16c.json` — 18 oracle steps; r8:=-1, r3:=r5+0x68, r1:=0, r10:=1, fr0:=0.0 (fldi0); append stores fr0 to [base20+count*4] then (r5,node,0) at [base1c+count*12] and count++; r0 ends 1 if pair existed else base1c; search loop leaves r9/r11/r12/r13/r14; shll r9 @0x8c06c218 sets T to shifted-out bit; exit T=1 |
| `0x8c06c244` | `FUN_8c06c244` | `notes/verifications/0x8c06c244.json` — 58 oracle steps; r6:=-4, r3:=-12 (bra delay 0x8c06c254); on match r1 is dec then re-inc (net 0, slot retried with count-1), r7:=count-1, fr0:=moved float, r0:=base+12*scanned, r13:=4*i, r12/r14:=float pointers; exit T=0 |
| `0x8c06c2c0` | `FUN_8c06c2c0` | `notes/verifications/0x8c06c2c0.json` — 74 oracle steps; r14:=-4, r7:=-12 (bra delay 0x8c06c2d2); r3 dec on removal then re-inc (slot re-examined with count-1); r1 += 12 only on skip; fr0:=moved float only on removal; r0/r10/r11/r12/r13 hold node/pair pointers; exit T=0 |
| `0x8c06c854` | `store_arg_to_global` | `notes/verifications/0x8c06c854.json` — store is the rts delay slot; r0 ends as the pool address 0x8c23f438 |
| `0x8c06caa8` | `advance_arg_12` | `notes/verifications/0x8c06caa8.json` — r4 itself is advanced; r0 is the advanced value |
| `0x8c06cab4` | `advance_arg_4_store_float` | `notes/verifications/0x8c06cab4.json` — r4 advanced before the delay-slot fmov store |
| `0x8c06cabc` | `advance_arg_8_store_float` | `notes/verifications/0x8c06cabc.json` — verified |
| `0x8c06cac4` | `advance_arg_4_load_float` | `notes/verifications/0x8c06cac4.json` — verified |
| `0x8c06cacc` | `advance_arg_8_load_float` | `notes/verifications/0x8c06cacc.json` — verified |
| `0x8c06d1e8` | `FUN_8c06d1e8` | `notes/verifications/0x8c06d1e8.json` — all-clear path ok; taken branches store fr4 at 0x8c2400f0..0x8c240120 (outside image, unpinnable); r4 &= 0x04000000 in place, T=(r4==0) |
| `0x8c06d244` | `FUN_8c06d244` | `notes/verifications/0x8c06d244.json` — all-clear path ok; globals 0x8c2400f8..0x8c240128 outside image; same residue shape |
| `0x8c06d2a0` | `FUN_8c06d2a0` | `notes/verifications/0x8c06d2a0.json` — 8 oracle steps; r0=selected pointer; r1 keeps LAST mask loaded (0x02000000 on the 4th/default paths, not the matching mask); r4 clobbered to r4&0x04000000 on last two paths (`and r0,r4`); T=1 only on default path; fr0=f32 at selected pointer via rts delay slot |
| `0x8c06d308` | `FUN_8c06d308` | `notes/verifications/0x8c06d308.json` — 8 oracle steps; same pattern as 0x8c06d2a0: r0=pointer, r1=0x02000000 leftover, r4&=0x04000000 clobber, T=1 only default, fr0 from rts delay-slot fmov |
| `0x8c06f6cc` | `FUN_8c06f6cc` | `notes/verifications/0x8c06f6cc.json` — 131 oracle steps; r0=table+idx*12; r1=r7+4; r3=r6+8; r4=r6+4; r7=r7+8; T=bit31 of shll operand (idx bit15); fr0-fr12 clobbered, fr13-15 restored via stack; reads table ptr from [r15_entry] via @(28,r15) after 7 pushes |
| `0x8c06fdc0` | `FUN_8c06fdc0` | `notes/verifications/0x8c06fdc0.json` — 258 oracle steps; r4 += 0xbc and r5 += 0xd0 at the end (last 3 stores land at old_r4+0xbc..); leftovers r0=src[0xd8], r1=src[0xd0], r2=r15_entry-44, r3=src[0xc8], r6=src[0xc4], r7=old_r5+0xc4, r8=old_r4+0xb0, r9=old_r4+60, r10=old_r4+0xd8; only fr0 clobbered (=f32[old_r5+0xc0]); T unchanged |
| `0x8c06ffec` | `FUN_8c06ffec` | `notes/verifications/0x8c06ffec.json` — 253 oracle steps; r4/r5 clobbered to src+0xbc/dst+0xd0; r2 left = body r15 (r15_in-68); r0/r1/r3/r6 = last loaded src words; r8..r14 restored by epilogue; fr0 = *(src+0x5c); sr/T untouched; fmov always 32-bit even with FPSCR.SZ=1 (spec .sz1) |
| `0x8c070320` | `offset_0xb0` | `notes/verifications/0x8c070320.json` — verified |
| `0x8c07093c` | `byte_copy` | `notes/verifications/0x8c07093c*.json` — empty and 4-byte paths (9 and 60 steps); r6 becomes the end pointer even when empty |
| `0x8c070aa4` | `FUN_8c070aa4` | `notes/verifications/0x8c070aa4.json` — ok; in-place tolower until NUL, r5 walks, r6=last byte, r0=r4, T=(last==0) |
| `0x8c070ac8` | `FUN_8c070ac8` | `notes/verifications/0x8c070ac8*.json` — 3 branches (mismatch, n<0, equal-to-bound); strncasecmp with r14=-32 saved/restored, dt r6 bound, r0=r0-r1 |
| `0x8c070b0c` | `FUN_8c070b0c` | `notes/verifications/0x8c070b0c*.json` — both branches; strcasecmp, r0=lowered diff in rts delay, r7=-32, r3=90, r6=65, T=last tst/cmp |
| `0x8c070b40` | `store_arg_to_global` | `notes/verifications/0x8c070b40.json` — store is the rts delay slot |
| `0x8c070bd0` | `empty_stub` | `notes/verifications/0x8c070bd0.json` — verified |
| `0x8c070e74` | `load_global_8c240470` | `notes/verifications/0x8c070e74.json` — verified |
| `0x8c071e04` | `load_global_8c2648f0` | `notes/verifications/0x8c071e04.json` — verified |
| `0x8c071e10` | `store_global_8c2648f0` | `notes/verifications/0x8c071e10.json` — verified |
| `0x8c072c84` | `FUN_8c072c84` | `notes/verifications/0x8c072c84.json` — 55 oracle steps; bt.s delay slot mov #0,r14 runs on both entry paths so r0=0 unless a match with r7!=0 sets r14=1; r7 clobbered to 0xfffffffe on path E; final T = MSB(r13-1) from last shll r3, not the last cmp/tst; r8..r14/r15 restored by epilogue; *r6 = r13 written before pops |
| `0x8c07363c` | `FUN_8c07363c` | `notes/verifications/0x8c07363c.json` — 34 oracle steps; r0=param_6*0x30 if loop ran else 44 (plain bf skips its delay slot, so `mov r7,r0` at 0x8c073678 only runs when entered); r2=r15+44, r5=param_3lo, r6=r7=param_6 only if loop ran; count store at entry+0x28 is the only non-stack write; T=0 on all paths; r8..r14 restored by pops |
| `0x8c073738` | `FUN_8c073738` | `notes/verifications/0x8c073738.json` — 107 oracle steps; r1:=param_5 kept; r2:=entry_r15+4 set every loop iteration (r15 already -36); r3:=base+4 in loop but param_6 when loop skipped; r5:=p3low, r6:=flag byte, r7 ends at param_6; T clear on all exits; r8-r14 pushed/popped so finals are seeds |
| `0x8c07496c` | `record_init` | `notes/verifications/0x8c07496c.json` — 37 oracle steps; "r1=0x8c1f82e8 (literal load), r3=r4+32, r5=0, r0=r4 (rts delay mov r4,r0), fr0=1.0 (fldi1); stores at +4(incoming r5),+8,+12,+16,+17,+20,+28(=r6),+40,+44,+48,+0(fmov fr0=1.0f),+32..37(copied from 0x8c1f82e8),+52,+56,+60; T untouched" |
| `0x8c0749f0` | `table_scale` | `notes/verifications/0x8c0749f0*.json` — sentinel and scale paths; r5 shifted in place, r4 advanced only on the scale path |
| `0x8c074aa0` | `flag_test_0x4000` | `notes/verifications/0x8c074aa0.json` — 24 oracle steps; "r5<<=2, r4=base+16, r6=0xf8 if [base+8]==3 else 0xe8, r3=final flags word, r1=result via subc r1,r1;add #1,r1, r0=r1 (delay mov r1,r0), T=(flags&0x4000)==0" |
| `0x8c074aec` | `record_set_56` | `notes/verifications/0x8c074aec.json` — 22 oracle steps; "r6<<=2, r4=base+16, r3=rec, r1=0, r0=60; stores [rec+56]=r5 then byte [rec+60]=0; T=0" |
| `0x8c074b2c` | `FUN_8c074b2c` | `notes/verifications/0x8c074b2c*.json` — both branches; table dispatch v=*(r4+0xe8|0xf8+r5*4), v!=5 → r0=*(*(r4+16+v*4)+12), else r0=0 |
| `0x8c074b68` | `get_val_0x30_flag_0x80` | `notes/verifications/0x8c074b68.json` — 25 oracle steps; "r5<<=2, r4=base+16, r0=0x80 (literal), r1=flags&0x80, r3=[rec+8]+48 on set / [rec+8] on clear / 0xe8|0xf8 on idx5, fr0=[r3] via fmov or 0.0 via fldi0, T=(flags&0x80)==0" |
| `0x8c074bbc` | `get_val_0x2c_flag_0x40` | `notes/verifications/0x8c074bbc.json` — 24 oracle steps; "r5<<=2, r4=base+16, r0=flags&0x40 (and #64 immediate), r1=[rec+8]+44 or [rec+8], r3=0xe8/0xf8, fr0=[r1] or 0.0, T=(flags&0x40)==0" |
| `0x8c074c08` | `get_val_0x20_flag_0x08` | `notes/verifications/0x8c074c08.json` — 24 oracle steps; "r5<<=2, r4=base+16, r0=flags&8 (and #8 immediate), r1=[rec+8]+32 or [rec+8], r3=0xe8/0xf8, fr0=[r1] or 0.0, T=(flags&8)==0" |
| `0x8c074c54` | `get_val_0x1c_flag_0x04` | `notes/verifications/0x8c074c54.json` — 24 oracle steps; "r5<<=2, r4=base+16, r0=flags&4 (and #4 immediate), r1=[rec+8]+28 or [rec+8], r3=0xe8/0xf8, fr0=[r1] or 0.0, T=(flags&4)==0" |
| `0x8c074ca0` | `FUN_8c074ca0` | `notes/verifications/0x8c074ca0.json` — 33 oracle steps; r4=param1+0x28; r3=param1+0xe8/0xf8 on early exit; r5,r6,r7 left shll2'd; r1=result; r14 restored by rts delay slot; T from final tst (or cmp/eq #5 on idx==5 path) |
| `0x8c074cf4` | `FUN_8c074cf4` | `notes/verifications/0x8c074cf4.json` — 39 oracle steps; same as 0x8c074ca0 plus gate byte: r0=extu.b(*(0x108+p)) and T=1 when gate==0 (bt.s exit); mask at +0x14 |
| `0x8c074d58` | `FUN_8c074d58` | `notes/verifications/0x8c074d58.json` — 39 oracle steps; same as 0x8c074cf4; mask at +0x18 |
| `0x8c0753fc` | `FUN_8c0753fc` | `notes/verifications/0x8c0753fc.json` — 117 oracle steps; *param=0 then ++ per match; slots written at param+0xf8; r4=param+0xf8 if fill loop runs else param; r3=final count; r5=r7=4; r6=4*iterations (bt.s delay slot runs on last pass); r13/r14 restored from stack; r1=5; r0=last fill idx*4; T from cmp/hi |
| `0x8c07581c` | `clear_records` | `notes/verifications/0x8c07581c.json` — r4 advanced by 68 before the loop, r5 walks one byte per record |
| `0x8c0768e8` | `FUN_8c0768e8` | `notes/verifications/0x8c0768e8.*.json` — all 3 branches (hi/lo/mid); clamp r4 into [r6..r5], r0=result, r6=r4 on low path, T=last cmp/gt |
| `0x8c07886c` | `FUN_8c07886c` | `notes/verifications/0x8c07886c.json` — ok; r4+=12, r0=r4 in rts delay slot |
| `0x8c078928` | `FUN_8c078928` | `notes/verifications/0x8c078928.json` — ok; *r4=r5, r0=r4 in rts delay slot |
| `0x8c078930` | `FUN_8c078930` | `notes/verifications/0x8c078930.json` — ok; *r4=r5, r0=r4 in rts delay slot |
| `0x8c0789e8` | `FUN_8c0789e8` | `notes/verifications/0x8c0789e8.json` — ok; same shape as 0x8c078928 |
| `0x8c0789f0` | `FUN_8c0789f0` | `notes/verifications/0x8c0789f0.json` — ok; r0=*(r4+56) in rts delay slot |
| `0x8c078c18` | `FUN_8c078c18` | `notes/verifications/0x8c078c18.json` — ok; r0=*r15, r4+=4, *r0=r4 in rts delay slot |
| `0x8c079060` | `FUN_8c079060` | `notes/verifications/0x8c079060.json` — ok; bare rts;nop stub, zero effects |
| `0x8c079070` | `FUN_8c079070` | `notes/verifications/0x8c079070.json` — ok; r0=68, *(r4+68)=r5 in rts delay slot |
| `0x8c079078` | `advance_68_load` | `notes/verifications/0x8c079078.json` — verified |
| `0x8c0797a4` | `FUN_8c0797a4` | `notes/verifications/0x8c0797a4.json` — ok; r0=*(r4+12)+((r5+*(r4+8)) mod *r4)*4, T=(r5>=r0) per verifier |
| `0x8c079a50` | `FUN_8c079a50` | `notes/verifications/0x8c079a50.json` — ok; struct init, pool 0x8c1f8598 single deref, same shape as 0x8c05de64 |
| `0x8c079c78` | `FUN_8c079c78` | `notes/verifications/0x8c079c78.json` — ok; dec *(r4+4), conditional *(r4+8)=*(*r4+12)+(*r4)*4, then *(r4+8)-=4 in rts delay |
| `0x8c079c9c` | `FUN_8c079c9c` | `notes/verifications/0x8c079c9c.json` — ok; 3-word copy r5→r4, r0=r4, r1=last word |
| `0x8c079cac` | `store_three_args` | `notes/verifications/0x8c079cac.json` — verified |
| `0x8c07a27c` | `init_record` | `notes/verifications/0x8c07a27c.json` — verified |
| `0x8c07a2a8` | `dec_inc_wrap` | `notes/verifications/0x8c07a2a8.json` — verified |
| `0x8c07ae50` | `copy_three_words` | `notes/verifications/0x8c07ae50.json` — verified |
| `0x8c07ae60` | `store_three_args_2` | `notes/verifications/0x8c07ae60.json` — verified |
| `0x8c07b0c4` | `swap_four_words` | `notes/verifications/0x8c07b0c4.json` — verified |
| `0x8c07b0f4` | `FUN_8c07b0f4` | `notes/verifications/0x8c07b0f4.json` — 215 oracle steps; r5=C-B set in bf.s delay slot on every else path; P[1]-= (C-B) all else paths, P[2]=t+(C-B) (wrap-sub cap) only on A1/A4 paths; *A=D, A[1]=B, A[2]=E tail store on all paths; per-path leftovers r0=E, r1=cap/h/f+0x60, r2=f+0x40/f+0x50/f+0x68, r7=0/*F0<<2/((h+t) or buf+((h+t)-cap))<<2; T=1 on eq/A2/A3, (P[2]>=u cap) on A1/A4; mov #N immediates are decimal (spill P goes to dead local f+0x54, arg slot f+0x84 never clobbered); cmp/hi/hs unsigned and shll2 leaves T alone in the oracle |
| `0x8c07bcd8` | `max_counter_clamped` | `notes/verifications/0x8c07bcd8*.json` — both selection paths |
| `0x8c07bffc` | `flag_to_float` | `notes/verifications/0x8c07bffc.json` — verified |
| `0x8c07c024` | `float_is_zero_to_byte` | `notes/verifications/0x8c07c024.json` — verified |
| `0x8c07c2f4` | `load_pool_constant` | `notes/verifications/0x8c07c2f4.json` — verified |
| `0x8c07cb88` | `advance_28_load_float` | `notes/verifications/0x8c07cb88.json` — verified |
| `0x8c07cb90` | `advance_28_store_float` | `notes/verifications/0x8c07cb90.json` — verified |
| `0x8c07cb98` | `FUN_8c07cb98` | `notes/verifications/0x8c07cb98.json` — 35 oracle steps; r4 clobbered to base+0x1c on all compute paths; r3:=base+0x10; r1:=base+0x14 (plain) or base+0x1c (store paths); r0:=0 via rts delay slot on compute paths; fr0 ends 0.0/1.0 from fldi, fr1 = clamped source; T set iff a clamp store happened (or matched cmp/eq #n on early returns); early returns leave r1/r3/r4/fr at seeds |
| `0x8c07cdc8` | `load_pool_constant_2` | `notes/verifications/0x8c07cdc8.json` — verified |
| `0x8c07d36c` | `advance_44` | `notes/verifications/0x8c07d36c.json` — verified |
| `0x8c07d374` | `advance_52` | `notes/verifications/0x8c07d374.json` — verified |
| `0x8c07d9ac` | `FUN_8c07d9ac` | `notes/verifications/0x8c07d9ac.json` — 124 oracle steps; r1 live on every path (size from *struct, then scan pointer, then offset on match); r3:=end ptr, r4 clobbered to buffer base, r6:=last candidate byte compared (seed on len-1 match), r0:=result, T true on every -1 exit and false on match; r11-r14 pushed/popped |
| `0x8c07de64` | `advance_60` | `notes/verifications/0x8c07de64.json` — verified |
| `0x8c07e018` | `cursor_advance_wrap` | `notes/verifications/0x8c07e018*.json` — wrap and skip paths |
| `0x8c07e03c` | `copy_six_words` | `notes/verifications/0x8c07e03c.json` — verified |
| `0x8c07e8e4` | `store_arg_return_self` | `notes/verifications/0x8c07e8e4.json` — verified |
| `0x8c07eb10` | `store_advanced_arg_to_stack_pointer` | `notes/verifications/0x8c07eb10.json` — verified |
| `0x8c07fc24` | `store_two_args` | `notes/verifications/0x8c07fc24.json` — verified |
| `0x8c07fd6c` | `advance_12` | `notes/verifications/0x8c07fd6c.json` — verified |
| `0x8c08015c` | `FUN_8c08015c` | `notes/verifications/0x8c08015c.json` — ok; struct init, pool 0x8c1f8da8 single deref, same shape as 0x8c05de64 |
| `0x8c08087c` | `init_iterator` | `notes/verifications/0x8c08087c.json` — verified |
| `0x8c080b5c` | `FUN_8c080b5c` | `notes/verifications/0x8c080b5c.json` — ok; struct init, [0]=*0x8c1f8d90 (double deref), [4..24]=0, r0=r4, r4=pool value |
| `0x8c081fd0` | `set_mode_byte` | `notes/verifications/0x8c081fd0.json` — verified |
| `0x8c082054` | `init_struct` | `notes/verifications/0x8c082054.json` — verified |
| `0x8c082210` | `copy_four_words` | `notes/verifications/0x8c082210.json` — verified |
| `0x8c082224` | `advance_24_store_float` | `notes/verifications/0x8c082224.json` — verified |
| `0x8c08222c` | `advance_16_store_float` | `notes/verifications/0x8c08222c.json` — verified |
| `0x8c082234` | `store_at_32` | `notes/verifications/0x8c082234.json` — verified |
| `0x8c082238` | `store_at_28` | `notes/verifications/0x8c082238.json` — verified |
| `0x8c082260` | `FUN_8c082260` | `notes/verifications/0x8c082260.json` — ok; FPU struct init: [16]=1.0f, [20]=0, [24]=0x41400000, [28]=[32]=0, [0..12] from 0x8c1d0f28, r0=r4 |
| `0x8c0822a0` | `FUN_8c0822a0` | `notes/verifications/0x8c0822a0.json` — ok; 12-word copy r5→r4+56, r1=*(r5+40), r0=*(r5+44) |
| `0x8c0822d4` | `FUN_8c0822d4` | `notes/verifications/0x8c0822d4.json` — ok; 9-word copy r5→r4+20, final r1=*(r5+24), r0=*(r5+32) (last load into r0) |
| `0x8c082398` | `store_at_12` | `notes/verifications/0x8c082398.json` — verified |
| `0x8c0824c0` | `set_handler_default` | `notes/verifications/0x8c0824c0.json` — verified |
| `0x8c082660` | `FUN_8c082660` | `notes/verifications/0x8c082660.json` — 17 oracle steps; writes global float 0x8c264ed0 and string 0x8c264ed8 (leading+trailing space strip), *(r4+0x38), *r5 (mode 4, >1 words); leaves r6=r7=0x8c264ed0 (mode!=4), r3=r4+0x60 or r1-2, r0=r4+0x38 or 1, r4+=0x40 (modes 2/3 only), fpul=wordcount, fr0-fr4 clobbered, T=0 |
| `0x8c082de8` | `advance_12_2` | `notes/verifications/0x8c082de8.json` — verified |
| `0x8c082fc0` | `store_arg_return_self_2` | `notes/verifications/0x8c082fc0.json` — verified |
| `0x8c083b9c` | `FUN_8c083b9c` | `notes/verifications/0x8c083b9c.json` — 19 oracle steps; writes *param=*(0x8c1f92a8), param[1]=0, byte param[2]=*r5, param[3]=param+4; r4=0, r6=param, r3=param+8, r1=param+4, r0=param at exit (mov r6,r0 after the mov.b) |
| `0x8c084280` | `advance_12_3` | `notes/verifications/0x8c084280.json` — verified |
| `0x8c084308` | `FUN_8c084308` | `notes/verifications/0x8c084308.json` — 15 oracle steps; r3=base+r5*32 (or 0 when byte==0), r1=r0=zero-extended byte, r0=r3 via rts delay slot mov r3,r0; T=(byte==0) from tst |
| `0x8c0846d0` | `FUN_8c0846d0` | `notes/verifications/0x8c0846d0*.json` — both branches (found/miss); bounded memchr, cmp/hs T=(r6>=r4), r0=offset or -1 via rts delay |
| `0x8c0848d8` | `FUN_8c0848d8` | `notes/verifications/0x8c0848d8.json` — 19 oracle steps; writes *param=*(0x8c1f94c0), param[1]=0, byte param[2]=*r5, param[3]=param+4; r4=0, r6=param, r3=param+8, r1=param+4, r0=param at exit |
| `0x8c084ea4` | `FUN_8c084ea4 (copy 32B records r4->r6 until r4==r5)` | `notes/verifications/0x8c084ea4.json` — 98 oracle steps; r0=r6 on return (rts delay slot); r1=dst+24, r3=dst+20, r7=dst+16, fr0=float from src+24 of last copied record; r4=r5; T=1 (final cmp/eq); add #32,r6 runs in bf.s delay slot so r6 advances on the last iteration too |
| `0x8c084f0c` | `FUN_8c084f0c` | `notes/verifications/0x8c084f0c.json` — 131 oracle steps; writes 32-byte records into [r4,r5): byte@+0, dwords@+4..+24, word@+28; leaves r1=r6+4, r3=r6+20, r7=dst+20, r0=sext16(src+28), fr0=src dword@+24, T=1 |
| `0x8c084f8c` | `FUN_8c084f8c (copy r5 records r6->r4, stride 32 on r4)` | `notes/verifications/0x8c084f8c.json` — 134 oracle steps; r0=halfword from src+28 of last record (not a real return value); r1=src+4, r3=src+20, r7=dst+20 of last iteration; r5=0, r4=r4+32*count; T=1 (final tst); r8..r14 pushed/popped so unchanged; add #32,r4 in bf.s delay slot |
| `0x8c0850a4` | `FUN_8c0850a4 (time->float: [r4+4]=B[2]*fr4+B[1]-(h/1000+m+s*60+ms*3600), [r4]=1)` | `notes/verifications/0x8c0850a4.json` — 46 oracle steps; r0=1; r1=r4+4; r3=ms; r5=s; r6=m; fpul=m (last lds r6,fpul); fr1=B[2]*fr4+B[1]; fr2=3600.0 fr3=ms*3600 fr5=60.0 fr6=s*60 fr7=1000.0 fr8=sum; fr0=result; stores ordered f32@r4+4, f32@r4+4, u32@r4 (rts delay slot); T untouched; args are halfwords on the stack at r15, B via global pointer at 0x8c26c230 |
| `0x8c085110` | `FUN_8c085110` | `notes/verifications/0x8c085110.json` — 25 oracle steps; on state==2 path: r0=1, r1=param+4, r5=g+4, r3=g+8, fr0=old param[1], fr1=param[2], fr2=result, *param=1, f32 param[1]=g[2]*fr4+g[1]-param[2]+param[1]; T=(*param==2); skip path leaves r0=*param |
| `0x8c085148` | `FUN_8c085148` | `notes/verifications/0x8c085148.json` — 19 oracle steps; on state==1 path: r0=2, r1=param+8, r3=g+4, fr0=g[1], fr1=result, *param=2, f32 param[2]=g[2]*fr4+g[1]; T=(*param==1); skip path leaves r0=*param |
| `0x8c085174` | `FUN_8c085174` | `notes/verifications/0x8c085174.json` — 15 oracle steps; *param=0 happens in the rts DELAY SLOT (unconditional); r0=0, r1=param+8, r3=g+4, fr0=g[1], fr1=result; no cmp so T untouched |
| `0x8c085198` | `FUN_8c085198` | `notes/verifications/0x8c085198.json` — 15 oracle steps; *param=1 in the rts DELAY SLOT (unconditional); r0=1, r1=param+4, r3=g+4, fr0=g[1], fr1=result; T untouched |
| `0x8c0851e0` | `zero_word_and_two_floats` | `notes/verifications/0x8c0851e0.json` — verified, write order differs from its twin |
| `0x8c0851f8` | `FUN_8c0851f8` | `notes/verifications/0x8c0851f8.json` — 21 oracle steps; fmov fr4,@r0 follows an UNdelayed bf (0x8B) so param[2]:=fr4 only when param[2]==0.0; r1=fpul=old *param, r3=param+4, r0=old*param+1, fr0=re-read param[2], fr1=result; T=(orig param[2]==0.0) |
| `0x8c085224` | `zero_two_floats_then_word` | `notes/verifications/0x8c085224.json` — verified, write order differs from its twin |
| `0x8c085264` | `FUN_8c085264` | `notes/verifications/0x8c085264.json` — 13 oracle steps; *(0x8c26c230)=param; r1=param+4, fr0=0.0 (fldi0), r0=param via rts delay slot mov r4,r0 (return value); T untouched |
| `0x8c08539c` | `FUN_8c08539c` | `notes/verifications/0x8c08539c.*.json` — both branches; 4-halfword compare, r0 = 0 if all equal else 1, r1/r6/r7 match flags |
| `0x8c0853e8` | `FUN_8c0853e8` | `notes/verifications/0x8c0853e8.*.json` — both branches; inverted 4-halfword compare, r0 = 1 if all equal else 0 |
| `0x8c085434` | `FUN_8c085434 (compare 4 halfwords, high first; a[i]>b[i])` | `notes/verifications/0x8c085434.json` — 10 oracle steps; r0=T as 0/1 via movt in rts delay slot (not the loaded halfword); r1=sign-extended a[i] of the deciding field; T=final cmp/gt r0,r1 |
| `0x8c085480` | `FUN_8c085480 (compare 4 halfwords, high first; a[i]<b[i])` | `notes/verifications/0x8c085480.json` — 10 oracle steps; r0=T as 0/1 via movt; r1=sign-extended a[i]; T=cmp/gt r1,r0 (opposite operand order vs 0x8c085434); rts delay slot movt r0 on every return path |
| `0x8c085628` | `FUN_8c085628` | `notes/verifications/0x8c085628.json` — 44 oracle steps; r1=param_3 from @(4,r15) after r14 push; r14 restored by rts delay slot; r0=r3 at exit; T=last cmp/pz; only bt.s (0x8D) has a delay slot (sub r0,r3 always runs) — plain bt (0x89) is undelayed so add #60 runs only on the borrow fall-through |
| `0x8c085684` | `FUN_8c085684` | `notes/verifications/0x8c085684.json` — 70 oracle steps; time-add: r5 clobbered to 60, r7=1000, r0=final r4; mov.w loads sign-extend so sums are of sign-extended shorts; r12/r13/r14/r15 restored by pops; T=0 at exit |
| `0x8c08579c` | `FUN_8c08579c` | `notes/verifications/0x8c08579c.json` — 3 oracle steps; r0=r4 from rts delay slot (return value hidden by C) |
| `0x8c086b00` | `FUN_8c086b00` | `notes/verifications/0x8c086b00.json` — 9 oracle steps; r1=r4+4 left behind; r0=r4 not r4+8 (rts delay overwrites add #8 result) |
| `0x8c086b14` | `FUN_8c086b14` | `notes/verifications/0x8c086b14.json` — 43 oracle steps; r15 -24/+24 (net 0) so live r2=r15-16, r5=r15-24, r7=r15-12; r0=orig r4, r1=r4+12, r3=0, r4=0, r6=r4+16, fr0=0.0 |
| `0x8c08bf38` | `FUN_8c08bf38` | `notes/verifications/0x8c08bf38.json` — 3 oracle steps; r0 = **(0x8c23ddc0) via rts delay slot, not the pointer |
| `0x8c08bf80` | `FUN_8c08bf80` | `notes/verifications/0x8c08bf80.json` — 21 oracle steps; r5=r5+12 and r1=r4+4 left mid-copy; fr0=src[3]; r0=r4 via rts delay |
| `0x8c08bfac` | `FUN_8c08bfac` | `notes/verifications/0x8c08bfac.json` — 23 oracle steps; r3=r4+12, r1=r4+4, r0=0x8c1cfa18 (then) or r4+8 (else); fr0/fr1/fr2 clobbered; T=last fcmp/gt; path B stores p[2]=0 in rts delay slot with r4 already +8 |
| `0x8c08c05c` | `FUN_8c08c05c` | `notes/verifications/0x8c08c05c.json` — 6 oracle steps; fr0=0.0 (fldi0); r0=r4 via rts delay |
| `0x8c08c704` | `FUN_8c08c704` | `notes/verifications/0x8c08c704.json` — 77 oracle steps; stores: 14/12-u32 record copy + head ptr + flag byte |
| `0x8c08c978` | `FUN_8c08c978` | `notes/verifications/0x8c08c978.json` — 72 oracle steps; stores: 12-u32 record copy + head ptr + flag byte |
| `0x8c0900a8` | `FUN_8c0900a8` | `notes/verifications/0x8c0900a8.json` — ok; zero 9 words at r4, r1=0, r0=r4 |
| `0x8c090ac0` | `FUN_8c090ac0` | `notes/verifications/0x8c090ac0.json` — 45 oracle steps; r5=src+32, r6=src+20, r7=dst+20, r3=*(src+20), fr0=f32(src+32); the bf's are undelayed (0x8B) so on no-wrap r0=*(r4+4) unchanged and T=last cmp/eq |
| `0x8c090b30` | `FUN_8c090b30` | `notes/verifications/0x8c090b30.json` — 4 oracle steps; store to p[2] happens in rts delay slot |
| `0x8c090fcc` | `FUN_8c090fcc` | `notes/verifications/0x8c090fcc.json` — ok; fill loop r6=r4+0xc0, r1=46 per iter, tail stores r1/r5 alternating, r0=68 |
| `0x8c09100c` | `FUN_8c09100c` | `notes/verifications/0x8c09100c.json` — ok; two fill loops (r4+0x178 zeros while r7<=36; 36/0 alternating at r4+80/r4+72 while r1<=10), r13/r14 net unchanged |
| `0x8c09138c` | `FUN_8c09138c` | `notes/verifications/0x8c09138c*.json` — both branches; bounded byte scan at r4+72, nonzero byte → r0=0, 46 zeros → r0=1 (bt polarity: zero continues) |
| `0x8c0918e4` | `FUN_8c0918e4` | `notes/verifications/0x8c0918e4.*.json` — both branches; same scan as 0x8c09138c with bound 36 |
| `0x8c092928` | `FUN_8c092928` | `notes/verifications/0x8c092928.json` — ok; r4 += 8 + r5*4, fr0 = *(r4) in rts delay |
| `0x8c092b24` | `FUN_8c092b24` | `notes/verifications/0x8c092b24.json` — ok; r4 += 0x84, fr0 = *(r4) in rts delay |
| `0x8c092b30` | `FUN_8c092b30` | `notes/verifications/0x8c092b30.json` — ok; r4 += 0x84, fr0=0, rts delay *(r4)=fr0 |
| `0x8c093048` | `FUN_8c093048` | `notes/verifications/0x8c093048*.json` — both branches; r1 = 0 only if bytes at r4+0x95 and r4+0x96 both zero |
| `0x8c093070` | `FUN_8c093070` | `notes/verifications/0x8c093070.json` — 13 oracle steps; stores: flag byte at r4+0x96=1 on store paths |
| `0x8c09309c` | `FUN_8c09309c` | `notes/verifications/0x8c09309c.json` — 13 oracle steps; stores: flag byte at r4+0x5f=1 on store paths |
| `0x8c095724` | `FUN_8c095724` | `notes/verifications/0x8c095724.json` — ok; *(*r15) = *r4 in rts delay |
| `0x8c09572c` | `FUN_8c09572c` | `notes/verifications/0x8c09572c.json` — ok; like 0x8c124d50: *r4=**r4, *(*r15)=old in rts delay |
| `0x8c095738` | `load_double_indirect_store_to_stack` | `notes/verifications/0x8c095738.json` — verified |
| `0x8c095744` | `load_store_to_stack` | `notes/verifications/0x8c095744.json` — verified |
| `0x8c09574c` | `follow_pointer_and_store_back` | `notes/verifications/0x8c09574c.json` — verified |
| `0x8c09578c` | `load_double_indirect_store_to_stack_2` | `notes/verifications/0x8c09578c.json` — verified |
| `0x8c09d30c` | `FUN_8c09d30c` | `notes/verifications/0x8c09d30c.json` — ok; struct init, pool 0x8c1fb6b8 single deref, same shape as 0x8c05de64 |
| `0x8c09d598` | `init_node_1` | `notes/verifications/0x8c09d598.json` — verified |
| `0x8c09d774` | `init_node_2` | `notes/verifications/0x8c09d774.json` — verified |
| `0x8c09d950` | `init_node_3` | `notes/verifications/0x8c09d950.json` — verified |
| `0x8c0a3b70` | `empty_stub_2` | `notes/verifications/0x8c0a3b70.json` — verified |
| `0x8c0a41b0` | `set_flag_byte_one` | `notes/verifications/0x8c0a41b0.json` — verified |
| `0x8c0a41b8` | `empty_stub_3` | `notes/verifications/0x8c0a41b8.json` — verified |
| `0x8c0a4a4c` | `zero_struct` | `notes/verifications/0x8c0a4a4c.json` — verified |
| `0x8c0a4db4` | `store_byte_at_20` | `notes/verifications/0x8c0a4db4.json` — verified |
| `0x8c0a9030` | `FUN_8c0a9030` | `notes/verifications/0x8c0a9030.json` — 31 oracle steps; r0 keeps DAT_8c0a9070 ptr (0x26c); r1 zeroed; r3=r4+0x254; r5=4th src word; r6=r4+0x250; r7=3rd src word; r14 pushed/popped so seed survives; T untouched |
| `0x8c0ab080` | `indexed_load` | `notes/verifications/0x8c0ab080.json` — verified |
| `0x8c0ab08c` | `FUN_8c0ab08c` | `notes/verifications/0x8c0ab08c.json` — 254 oracle steps; r4->field4 stored even when all objects skipped; r0=1 after a store else last [obj+0x28] (or r14 on null); r1=27; r3=1; r6=last array entry; r7=27; T=0 after loop, 1 on early-out |
| `0x8c0ab3f0` | `FUN_8c0ab3f0` | `notes/verifications/0x8c0ab3f0.json` — 40 oracle steps; stores [r4+0x6c]=r5, byte [r4+0x84]=1, float [r4+0x88]=0, then per mode: 0->[r4+0x7c]=180/0 [r4+0x80]=90 [r4+0x8c]=4; 1->swap [r4+0x7c]<->[r4+0x80] with 0, [r4+0x8c]=2; 2/default->[r4+0x7c]=old[r4+0x80] [r4+0x80]=90 [r4+0x8c]=4; 3->byte [r4+0x84]=0; rts delay slot performs the [r4+0x8c] store; r4 ends +=0x8c, r0/r1/r3/r5/r6/r7 hold last constants; T=byte-test/matching cmp/eq/0(default) |
| `0x8c0ab680` | `FUN_8c0ab680` | `notes/verifications/0x8c0ab680.json` — ok; merge: r5→r4+108, r6→r4+120, r6→r4+0x90, r4+=0x90 |
| `0x8c0ab700` | `store_two_floats_at_f4_f8` | `notes/verifications/0x8c0ab700.json` — verified |
| `0x8c0ac3c4` | `FUN_8c0ac3c4` | `notes/verifications/0x8c0ac3c4.json` — 52 oracle steps; r4 itself +=16 on the flag path (final store lands on orig r4+0x10); r0=0x8c0ac430 (flag) or the flag byte (skip); r1=r4+0x10; r3=r4+0x20; r5=r4+0x18; fr0/fr1 keep intermediates; T=last fcmp/gt (flag) or 1 (skip) |
| `0x8c0adec0` | `clamp_float_to_range` | `notes/verifications/0x8c0adec0.json` — verified |
| `0x8c0adf14` | `load_through_field` | `notes/verifications/0x8c0adf14.json` — verified |
| `0x8c0adf1c` | `copy_three_words_at_28` | `notes/verifications/0x8c0adf1c.json` — verified |
| `0x8c0adf2c` | `copy_three_words_at_16` | `notes/verifications/0x8c0adf2c.json` — verified |
| `0x8c0ae11c` | `FUN_8c0ae11c` | `notes/verifications/0x8c0ae11c.json` — 115 oracle steps; r0 returns 0; r1=final outer index (mode 1) or 1 (mode 0); r3=table base (mode 1) or last index (mode 0); r6/r7 keep inner counters; r12-r14 pushed/popped; T=0 on every path |
| `0x8c0ae1a0` | `FUN_8c0ae1a0` | `notes/verifications/0x8c0ae1a0.json` — 117 oracle steps; r0=r1=match count; r1 is zeroed by the bf.s delay slot on every path; r3=final index (mode 1) or r4->count (mode 0); r7=table base; r11-r14 pushed/popped; T=0 |
| `0x8c0ae228` | `FUN_8c0ae228` | `notes/verifications/0x8c0ae228.json` — 114 oracle steps; r0 returns 0; r1=final index (mode 1) or last loaded u32 (mode 0); r3=table base; r6/r7 inner counters; r12-r14 pushed/popped; T=0 |
| `0x8c0ae64c` | `FUN_8c0ae64c` | `notes/verifications/0x8c0ae64c.json` — 95 oracle steps; stores [r4+0xc]/[r4+0x10] run on every path (mode not 0/1 stores 999999999/0); r0=last inner byte offset; r1=last entry read; r3/r5=min/max; r6=final index (mode 1) or count (mode 0); r7=last inner counter or count; compares signed; T=0 |
| `0x8c0ae74c` | `FUN_8c0ae74c` | `notes/verifications/0x8c0ae74c.json` — 62 oracle steps; out pointer is read from [r15] at entry (stack slot), not from r6; r4 is clobbered to the saved mode on every loop iteration; r0=mode; r1=iteration count; r3=table base; r6=count; r7=accumulated offset; T=0 |
| `0x8c0ae814` | `init_record_with_arg` | `notes/verifications/0x8c0ae814.json` — verified |
| `0x8c0b2670` | `advance_4` | `notes/verifications/0x8c0b2670.json` — verified |
| `0x8c0b274c` | `FUN_8c0b274c` | `notes/verifications/0x8c0b274c.json` — ok; merge: r5→r4+16 (3 words), r6→r4 (4 words), fr4→r4+28, fr5→r4+32, r4+=32 |
| `0x8c0b2780` | `abs_copy` | `notes/verifications/0x8c0b2780.json` — verified |
| `0x8c0b2c00` | `hypot` | `notes/verifications/0x8c0b2c00.json` — verified |
| `0x8c0b2ca0` | `FUN_8c0b2ca0` | `notes/verifications/0x8c0b2ca0.json` — 21 oracle steps; r4/r5 post-incremented to +12; r0=out+8 where out=[r15]; fr0 keeps fr4; fr5/fr6/fr7 keep pairwise differences; pref @r0 is a no-op; T untouched |
| `0x8c0b2e38` | `FUN_8c0b2e38` | `notes/verifications/0x8c0b2e38.json` — 83 oracle steps; stores land at r4+0,16,32,28,20,24,8,12,4 in that order; r4 ends +4; r0/r1/r3/r5/r6/r7/r14 all keep r4-derived offsets; fr13/fr14/fr15 and r14 pushed/popped so seeds survive; T untouched |
| `0x8c0b2f68` | `copy_three_words_at_36` | `notes/verifications/0x8c0b2f68.json` — verified |
| `0x8c0b2f78` | `FUN_8c0b2f78` | `notes/verifications/0x8c0b2f78.json` — 81 oracle steps; fr0-fr11 clobbered with products; r0/r1/r3/r4/r5/r6/r7/r14-left-as-r4+16-but-restored |
| `0x8c0b301c` | `FUN_8c0b301c` | `notes/verifications/0x8c0b301c.json` — 149 oracle steps; stores 9 floats to r4+0..0x20 (order +0,+12,+24,+4,+16,+28,+8,+20,+32); epilogue pops restore r8-r14, leaving r0=r4+20 r1=r4+8 r3=r4+28 r4=r4+32 r6=r5+12 r7=r5+24; fr0=1/cross fr5=cross stay live |
| `0x8c0b3148` | `FUN_8c0b3148` | `notes/verifications/0x8c0b3148.json` — 224 oracle steps; 9 f32 stores to r4[0..36); path A leaves r0=36, r1=stack[r15+36], r4+=32, r5=r4_orig+28, r6+=32, r2/r3/r7-r13 leftover pointers, T=0; path B (r4==r5||r4==r6) stages 9 results at r15+48..80 then copies to r4, leaves r5=r15+48, T=1; delay-slot fmov fr3,@r4 runs after bra; plain bf has no delay slot |
| `0x8c0b34bc` | `FUN_8c0b34bc` | `notes/verifications/0x8c0b34bc.json` — 222 oracle steps; stores 9 floats to *r4 (path r4!=r6 directly, path r4==r6 via 36-byte copy from stack); T=(r4==r6); path A leaves r4=dest+32, r6=src+32, r5=dest+28, r0=src+8, r1=src+28, r2=sp+20, r3=src+16, r7=src+4, r9=src+24, r10=src+24, r11=src+12, r12=dest+24, r13=src+20, fr0..fr4=last dot operands/result; path B leaves r5=entry_sp-64, r6=src+32, r2=sp+80, r0/r1=copy scratch |
| `0x8c0b3828` | `FUN_8c0b3828` | `notes/verifications/0x8c0b3828.json` — 71 oracle steps; fr0=[src+32], fr1=[src+12], fr2=[src+24]; r0=[src+32], r1=[src+8], r2=r15-4, r3=r15-36, r5=src+32, r6=src+28, r7=src+4 |
| `0x8c0b3950` | `FUN_8c0b3950` | `notes/verifications/0x8c0b3950.json` — 25 oracle steps; r7/r5/r3 = literal-pool globals; r6=r4+12, r4=r4+24, r0=[g3+8], r1=[g3] |
| `0x8c0b3990` | `FUN_8c0b3990` | `notes/verifications/0x8c0b3990.json` — 87 oracle steps; fr0=1.0, fr1=[src+44]; r0=r4, r1=r4+60, r3=r4+56, r5=src+44, r6=r4+52, r7=src+36 |
| `0x8c0b3b3c` | `FUN_8c0b3b3c` | `notes/verifications/0x8c0b3b3c.json` — 74 oracle steps; stores 4 floats to out+0,+4,+8,+12 where out=[r15+44] (stack arg); writes 3-int table [1,2,0]@0x8c1fcaf0 and 4 stack slots [r15+12..+28] that are read back (index 0 aliasing matters); r12/r11/r13/r14 restored by pops so r12 ends as seed; final r2=r15+24 r3=r15+12+4i r4=r15+12 r5=4k r6/r7 advanced; T=fcmp/eq(sqrt==0) on negative path, fcmp/gt(sum>0) on positive |
| `0x8c0b3d84` | `FUN_8c0b3d84` | `notes/verifications/0x8c0b3d84.json` — 92 oracle steps; dest r6 = [r15] at entry (offset 20 after 4 pushes+add #-4); fr0-fr11 clobbered; r0=dst+4, r1=r5+8, r3=r4+4, r4=r4+12, r5=r5+4, r6=dst+8 |
| `0x8c0b4000` | `FUN_8c0b4000` | `notes/verifications/0x8c0b4000.json` — 89 oracle steps; dest r7 = [r15] at entry (offset 32 after pushes+add #-16); fr0-fr11 clobbered; r2=r15-20, r0=r7+8, r1=r7+4, r3=r5+12, r4=r4+8, r5=r5+4, r6=r5+8, r7=r7+12 |
| `0x8c0b40b4` | `init_quaternion` | `notes/verifications/0x8c0b40b4.json` — verified |
| `0x8c0b4828` | `scale_float_at_32` | `notes/verifications/0x8c0b4828.json` — verified |
| `0x8c0b4854` | `store_float_at_32` | `notes/verifications/0x8c0b4854.json` — verified |
| `0x8c0b4860` | `copy_four_words_at_4` | `notes/verifications/0x8c0b4860.json` — verified |
| `0x8c0b487c` | `copy_three_words_at_20` | `notes/verifications/0x8c0b487c.json` — verified |
| `0x8c0b4894` | `advance_32_load_float` | `notes/verifications/0x8c0b4894.json` — verified |
| `0x8c0b489c` | `FUN_8c0b489c` | `notes/verifications/0x8c0b489c.json` — 22 oracle steps; dest r3 = [r15]; fr0=[r4+16]; r0=r3+8, r1=r3+4, r3=r3+12, r4=r4+16 |
| `0x8c0b48c8` | `copy_three_floats` | `notes/verifications/0x8c0b48c8.json` — r4 advanced by 28, third store in the delay slot |
| `0x8c0b4bb8` | `FUN_8c0b4bb8` | `notes/verifications/0x8c0b4bb8.json` — 43 oracle steps; fr0=[r6+8]; r0=r4 (rts delay slot mov r4,r0), r1=r4+28, r5=r5+12, r6=r6+8 |
| `0x8c0b4c10` | `FUN_8c0b4c10` | `notes/verifications/0x8c0b4c10.json` — 78 oracle steps; fr0=0.0, fr1=1.0; r0=r4, r1=r4+76, r3=r4+72, r5=r4+68, r6=r4+64, r7=r4+60 |
| `0x8c0b4d4c` | `lerp_two_pairs` | `notes/verifications/0x8c0b4d4c.json` — verified |
| `0x8c0b4d74` | `FUN_8c0b4d74` | `notes/verifications/0x8c0b4d74.json` — 32 oracle steps; dest r3 = [r15]; fr2=a2*b1 (clobbered by fmul), fr3=a1*b0, fr8=a0*b2; r0=r3+4, r1=r5+4, r3=r3+8, r4=r4+8, r5=r5+8 |
| `0x8c0b4db4` | `negate_three_floats` | `notes/verifications/0x8c0b4db4.json` — verified |
| `0x8c0b4dd8` | `FUN_8c0b4dd8` | `notes/verifications/0x8c0b4dd8.json` — 25 oracle steps; dest r6 = [r15]; fr0-fr3 = scaled values; r0=r6+8, r1=r6+4, r3=r4+4, r5=r4+8, r6=r6+12 |
| `0x8c0b4e0c` | `scale_three_floats` | `notes/verifications/0x8c0b4e0c.json` — verified |
| `0x8c0b4e30` | `scale_three_floats_2` | `notes/verifications/0x8c0b4e30.json` — verified |
| `0x8c0b4e54` | `dot2` | `notes/verifications/0x8c0b4e54.json` — verified |
| `0x8c0b4e68` | `scale_two_floats` | `notes/verifications/0x8c0b4e68.json` — verified |
| `0x8c0b4e80` | `subtract_two_pairs` | `notes/verifications/0x8c0b4e80.json` — verified |
| `0x8c0b4e98` | `add_two_pairs` | `notes/verifications/0x8c0b4e98.json` — verified |
| `0x8c0b4eb0` | `FUN_8c0b4eb0` | `notes/verifications/0x8c0b4eb0.json` — 15 oracle steps; dest r3 = [r15]; fr0=[r5+4], fr1/fr2 = differences; r0=r5+4, r1=r4+4, r3=r3+4 |
| `0x8c0b4ed0` | `FUN_8c0b4ed0` | `notes/verifications/0x8c0b4ed0.json` — ok; *r15 = r4[0]+r5[0], rts delay *(r3+4) = r4[1]+r5[1] |
| `0x8c0b4ef0` | `FUN_8c0b4ef0` | `notes/verifications/0x8c0b4ef0.json` — ok; 3 floats at r4 *= fr4 in place, r4+=8, r0=r4+4 residue |
| `0x8c0b4f0c` | `FUN_8c0b4f0c` | `notes/verifications/0x8c0b4f0c.json` — ok; 3 floats r4 -= r5 in place, r4+=8, r5+=8, r0=r5o+4, r1=r4o+4, fr0/fr1 last values |
| `0x8c0b4f34` | `FUN_8c0b4f34` | `notes/verifications/0x8c0b4f34.json` — ok; same with fadd |
| `0x8c0b4f5c` | `FUN_8c0b4f5c` | `notes/verifications/0x8c0b4f5c.json` — 27 oracle steps; stores: 3 f32 at stack-arg dst (last is rts delay slot) |
| `0x8c0b4f94` | `FUN_8c0b4f94` | `notes/verifications/0x8c0b4f94.json` — 27 oracle steps; stores: 3 f32 at stack-arg dst (last is rts delay slot) |
| `0x8c0b4fcc` | `FUN_8c0b4fcc` | `notes/verifications/0x8c0b4fcc.json` — ok; fr4→r4, r4+=4, rts delay fr5→r4 |
| `0x8c0b4fd4` | `FUN_8c0b4fd4` | `notes/verifications/0x8c0b4fd4.json` — ok; fr0=0 → r4+4, rts delay → r4 |
| `0x8c0b4fe0` | `FUN_8c0b4fe0` | `notes/verifications/0x8c0b4fe0.json` — ok; fr4→r4, fr5→r4+4, r4+=8, rts delay fr6→r4 |
| `0x8c0b4ff0` | `FUN_8c0b4ff0` | `notes/verifications/0x8c0b4ff0.json` — ok; 0.0f→r4+8, r4+4, rts delay r4 |
| `0x8c0b5004` | `FUN_8c0b5004` | `notes/verifications/0x8c0b5004.json` — ok; 4 floats at r4 *= fr4 in place (last in rts delay), r4+=12 |
| `0x8c0b5304` | `FUN_8c0b5304` | `notes/verifications/0x8c0b5304.json` — 76 oracle steps; stores: per 44-byte record f32 zeros x7, f32 [0x8c1cd000] at +4, u32 0 at +20/+36 |
| `0x8c0b5e78` | `FUN_8c0b5e78` | `notes/verifications/0x8c0b5e78.json` — ok; *r4=0x8c1eaaac, r0=r4 in rts delay |
| `0x8c0b5e84` | `subtract_from_global` | `notes/verifications/0x8c0b5e84.json` — verified |
| `0x8c0b5e94` | `FUN_8c0b5e94` | `notes/verifications/0x8c0b5e94.json` — 13 oracle steps; delay-slot store *G1=r4 fires on both paths of bt.s; r5 clamped to 1 when <=0; r0/r1/r3 end as loaded global ptrs/sum; T=(r5>0) original |
| `0x8c0b5ec0` | `reset_three_globals` | `notes/verifications/0x8c0b5ec0.json` — verified |
| `0x8c0b649c` | `store_arg_to_global` | `notes/verifications/0x8c0b649c.json` — verified |
| `0x8c0b64a8` | `FUN_8c0b64a8` | `notes/verifications/0x8c0b64a8.json` — 53 oracle steps; plain bt/bf have no delay slot; r4 ends = r5; r1 = sext last text byte; r0 = replacement byte on last match else sext *r6; T ends 1 |
| `0x8c0b64fc` | `FUN_8c0b64fc` | `notes/verifications/0x8c0b64fc.json` — 58 oracle steps; r4 argument destroyed (ends as count then last exts pattern byte); r6 last text byte; r1=base r3=base+count; r12/r13/r14 restored from stack; T=0 on offset-return paths, 1 on -1-via-end and FAIL-via-cmp/hs |
| `0x8c0b7414` | `delta_12_minus_8` | `notes/verifications/0x8c0b7414.json` — verified |
| `0x8c0b766c` | `init_three_vtables` | `notes/verifications/0x8c0b766c.json` — verified |
| `0x8c0b7be8` | `FUN_8c0b7be8` | `notes/verifications/0x8c0b7be8.json` — 15 oracle steps; four sequential stores to *r4 (write log keeps all, memory keeps last); byte store p+4=1; p+8 and p+12 = -1 (latter is rts delay slot); r1=-1; r0=r4 |
| `0x8c0b8340` | `FUN_8c0b8340` | `notes/verifications/0x8c0b8340.json` — ok; r0=r5, rts delay byte[r4+4]=r0 |
| `0x8c0b8340` | `store_byte_at_4` | `notes/verifications/0x8c0b8340.json` — verified |
| `0x8c0b8348` | `load_byte_at_4` | `notes/verifications/0x8c0b8348.json` — verified |
| `0x8c0b834c` | `init_three_vtables_2` | `notes/verifications/0x8c0b834c.json` — verified |
| `0x8c0b8380` | `store_byte_at_4_2` | `notes/verifications/0x8c0b8380.json` — verified |
| `0x8c0b8388` | `load_byte_at_4_2` | `notes/verifications/0x8c0b8388.json` — verified |
| `0x8c0b838c` | `init_three_vtables_3` | `notes/verifications/0x8c0b838c.json` — verified |
| `0x8c0b8fb4` | `FUN_8c0b8fb4` | `notes/verifications/0x8c0b8fb4.json` — 8 oracle steps; r4/r3/r1/r0 clobbered on init path (r0 ends reloaded as slot ptr); flag byte set to 1; T=(flag==0) |
| `0x8c0b90e8` | `load_byte_zext` | `notes/verifications/0x8c0b90e8.json` — verified |
| `0x8c0b90f0` | `store_byte_at_0` | `notes/verifications/0x8c0b90f0.json` — verified |
| `0x8c0b9768` | `zero_word_and_byte` | `notes/verifications/0x8c0b9768.json` — verified |
| `0x8c0b9784` | `FUN_8c0b9784` | `notes/verifications/0x8c0b9784.json` — 418 oracle steps; r0=4 (loop)/0 (early) r1=0 r3=5 r4=arg+4 r7=0; r5=last *entry or mov.w or inner2 base 0x8c27a038+24*r13; r6=inner1 base 0x8c27a038+24*[*entry] or 5; fr0=accumulator fr2=last reloaded weight+fr0 fr8=last row sum; fpul=last extu.w; T=0 loop/T=1 early (*r4==0) |
| `0x8c0b9edc` | `clear_byte_return_self` | `notes/verifications/0x8c0b9edc.json` — verified |
| `0x8c0b9f78` | `indexed_table_then_field` | `notes/verifications/0x8c0b9f78.json` — verified |
| `0x8c0ba554` | `FUN_8c0ba554` | `notes/verifications/0x8c0ba554.json` — 83 oracle steps; r0..r7 end with loop residue (r0=r4 base, r1=4, r3=r4+72, r6=0, r13=16, r12=96) but r12/r13/r14 restored from stack; T ends 0 (cmp/gt 4>4); delay-slot store @(16,r3) runs every iteration |
| `0x8c0bd484` | `FUN_8c0bd484` | `notes/verifications/0x8c0bd484.json` — 81 oracle steps; zero-stores end at offset 140 (36 words, not 0x94); r0:=r4 (rts delay mov r4,r0), r1:=literal [0x8c0bd528], r3:=r4+120, r5:=r4+72, r6:=72, r7:=68; r13/r14 net unchanged (pushed+popped); T untouched |
| `0x8c0bd52c` | `FUN_8c0bd52c` | `notes/verifications/0x8c0bd52c.json` — 162 oracle steps; param_3 is the stack arg at [r15_entry] (mov.l @(56,r15) after r15-=56); r0:=max(p1[11],p2[11]), r1:=r15_entry-32, r3:=p3+24, r4:=max(p1[10],p2[10]), r5:=p2+24, r6:=6, r7:=6, T:=0 (final cmp/gt r7,r6); 12 ascending stores to p3; r13/r14/r15 net unchanged |
| `0x8c0bd614` | `FUN_8c0bd614` | `notes/verifications/0x8c0bd614.json` — 166 oracle steps; r4/r5 end at arg+24 (not restored); r3=param_3+24; r1=frame+24; r6=6; r7=param_2[11]; r0=local[11]; r4 clobbered to local[10] by tail stores; r12/r13/r14 restored from stack (loop counter discarded); r15=entry sp; T=0 on exit (6>6 false); param_3 read from [entry sp], not r6 |
| `0x8c0bd700` | `FUN_8c0bd700` | `notes/verifications/0x8c0bd700.json` — 53 oracle steps; r0 ends = a pointer (mov r4,r0 mid-function); r1 = b[44]; r3 = final sum; T stays at cmp/gt for element 5 (b[5]>a[5]); store at offset 44 is the rts delay slot |
| `0x8c0bd76c` | `zero_twelve_words` | `notes/verifications/0x8c0bd76c.json` — verified |
| `0x8c0bd8c8` | `set_mode_or_clear` | `notes/verifications/0x8c0bd8c8.json` — verified |
| `0x8c0bde60` | `zero_three_words` | `notes/verifications/0x8c0bde60.json` — verified |
| `0x8c0bde6c` | `return_self` | `notes/verifications/0x8c0bde6c.json` — verified |
| `0x8c0bef98` | `store_arg_return_self_3` | `notes/verifications/0x8c0bef98.json` — verified |
| `0x8c0bf364` | `advance_12_4` | `notes/verifications/0x8c0bf364.json` — verified |
| `0x8c0bfd88` | `set_slot_if_empty` | `notes/verifications/0x8c0bfd88.json` — verified |
| `0x8c0c01ec` | `copy_fields_0_8_12` | `notes/verifications/0x8c0c01ec.json` — verified |
| `0x8c0c01fc` | `FUN_8c0c01fc` | `notes/verifications/0x8c0c01fc.json` — ok; *r4=-1, byte r4+4=0, words r4+8/+12=0, r0=r4 |
| `0x8c0c0328` | `FUN_8c0c0328` | `notes/verifications/0x8c0c0328.json` — 50 oracle steps; r4..r7 each end +4 (add #4 in body); r1=*r15 r0=*(r15+4) then r0=0/1 result; fr0..fr11 all hold intermediates, fr4=0.0 fr3=1.0; T = last fcmp executed; early rts when det==0 (can be -0.0 == +0.0) |
| `0x8c0c06e4` | `FUN_8c0c06e4` | `notes/verifications/0x8c0c06e4.json` — 22 oracle steps; r4 ends +4 only on r4-store paths (r3-store path leaves it); r0=*r15; r1=last key load; r6=0 at loop exit; T=1 on r3==0 paths else final cmp/gt; store target is rts delay slot |
| `0x8c0c079c` | `FUN_8c0c079c` | `notes/verifications/0x8c0c079c.json` — ok; struct init, pool 0x8c1fd8c0 single deref, same shape as 0x8c05de64 |
| `0x8c0c0d58` | `load_global_byte` | `notes/verifications/0x8c0c0d58.json` — verified |
| `0x8c0c0d64` | `load_float_via_global_32` | `notes/verifications/0x8c0c0d64.json` — verified |
| `0x8c0c0d74` | `load_float_via_global_36` | `notes/verifications/0x8c0c0d74.json` — verified |
| `0x8c0c0d84` | `load_float_via_global_28` | `notes/verifications/0x8c0c0d84.json` — verified |
| `0x8c0c0d94` | `float_is_zero_via_global` | `notes/verifications/0x8c0c0d94.json` — verified |
| `0x8c0c0da8` | `store_field_4_via_36` | `notes/verifications/0x8c0c0da8.json` — verified |
| `0x8c0c0ecc` | `indexed_load_via_36` | `notes/verifications/0x8c0c0ecc.json` — verified |
| `0x8c0c1288` | `empty_stub_4` | `notes/verifications/0x8c0c1288.json` — verified |
| `0x8c0c128c` | `empty_stub_5` | `notes/verifications/0x8c0c128c.json` — verified |
| `0x8c0c1720` | `store_field_24_via_36` | `notes/verifications/0x8c0c1720.json` — verified |
| `0x8c0c1728` | `load_field_24_via_36` | `notes/verifications/0x8c0c1728.json` — verified |
| `0x8c0c1730` | `load_byte_28_via_36` | `notes/verifications/0x8c0c1730.json` — verified |
| `0x8c0c173c` | `store_field_4_via_36_2` | `notes/verifications/0x8c0c173c.json` — verified |
| `0x8c0c1744` | `load_field_4_via_36` | `notes/verifications/0x8c0c1744.json` — verified |
| `0x8c0c2d9c` | `load_store_to_stack_2` | `notes/verifications/0x8c0c2d9c.json` — verified |
| `0x8c0c2da4` | `follow_pointer_and_store_back_2` | `notes/verifications/0x8c0c2da4.json` — verified |
| `0x8c0c2db0` | `load_double_indirect_store_to_stack_3` | `notes/verifications/0x8c0c2db0.json` — verified |
| `0x8c0c6904` | `FUN_8c0c6904` | `notes/verifications/0x8c0c6904.json` — 12 oracle steps; r0 = 0 (head match) / predecessor node / 0 (empty) / last node (r5==0 walk); r1 = seed / r5 / seed / 0; every exit path leaves T=1 |
| `0x8c0c817c` | `FUN_8c0c817c` | `notes/verifications/0x8c0c817c.json` — 78 oracle steps; stores:none; r0=r6 (rts delay), r1=shifted table index on tail path / last table entry on word-only path, r3=last word loaded, r4=ptr+4*nwords+rem, r5=0xFFFFFFFF on tail path (bf.s delay slot runs once more) else 0, r7=nwords, r14 restored, T=1 always |
| `0x8c0c81ec` | `FUN_8c0c81ec` | `notes/verifications/0x8c0c81ec.json` — 23596 oracle steps; stores: flag bytes at *0x8c27a6b0/*0x8c27a6a8 + 256 words at 0x8c27a2a8; r0=0x8c27a2a8 and r1=8 stay live (not 1/ptr), r3=0, r4=last LFSR feedback word (0 or 0x04c11db7), r5=0, r6=256, r7=8, T=0 (final cmp/gt), r8..r14 pushed/popped |
| `0x8c0c8ec4` | `store_arg_to_global_2` | `notes/verifications/0x8c0c8ec4.json` — verified |
| `0x8c0c8f18` | `load_field_56` | `notes/verifications/0x8c0c8f18.json` — verified |
| `0x8c0c9104` | `store_arg_return_self_4` | `notes/verifications/0x8c0c9104.json` — verified |
| `0x8c0c921c` | `load_double_indirect` | `notes/verifications/0x8c0c921c.json` — verified |
| `0x8c0c9460` | `FUN_8c0c9460` | `notes/verifications/0x8c0c9460.json` — ok; struct init, pool 0x8c1fe898 single deref |
| `0x8c0c9fe8` | `FUN_8c0c9fe8` | `notes/verifications/0x8c0c9fe8.json` — 48 oracle steps; stores: 40 zero bytes at r4 + r4 to *0x8c27a6e0; r0=r4 (rts delay, overwrites the 40 loop residue), r1=10, r3=r4+32, r5=0, r6=10, T=0 |
| `0x8c0cbba0` | `FUN_8c0cbba0` | `notes/verifications/0x8c0cbba0.json` — ok; fr4→r4+64, fr5→r4+68, fr5*=fr5→r4+72, r4+=72 |
| `0x8c0cbe54` | `FUN_8c0cbe54` | `notes/verifications/0x8c0cbe54.json` — 85 oracle steps; no FPSCR.SZ branches exist (all single fmov); store order floats 48..=72 first then ints 0,4,8,12,16,20,36,40,44,24,28,32; r0:=r4, r1:=src[8], r3:=src[6], r5:=src+24, r6:=dst+24, r7:=src+36, fr0:=src float at +72 (bit-exact, NaN payload); T untouched |
| `0x8c0cbf00` | `FUN_8c0cbf00` | `notes/verifications/0x8c0cbf00.json` — 65 oracle steps; stores: 7 fmovs at r4+48..72 (0,0,1.0f,0,0,0,0) then 12 dwords; fr0=1.0 and fr1=0.0 left live (fldi1/fldi0), r0=r4, r1=*(p6+8), r3=*p6, r5=0x8c1d3f40, r6=r4+24, r7=0x8c1d3f50, r12..r14 restored; store order matters (+0,+4,+8,+12..+20,+36,+40,+44,+24,+28,+32) |
| `0x8c0cc470` | `return_self_2` | `notes/verifications/0x8c0cc470.json` — verified |
| `0x8c0cc51c` | `FUN_8c0cc51c` | `notes/verifications/0x8c0cc51c.json` — 149 oracle steps; r0=O+0x60 r1=O+0x48 r3=O+0x50 r4=O+0x4c r5=0x8c27a730 r6=E+4 r7=E+8; fr2=1.0 fr3=0.0; T=0 body/T=1 skip; pointer cell 0x8c27a730 stored 4x (O+0x18/0x30/0x48/0x60) |
| `0x8c0cc8a8` | `load_pool_constant_3` | `notes/verifications/0x8c0cc8a8.json` — verified |
| `0x8c0ccef0` | `load_pool_constant_4` | `notes/verifications/0x8c0ccef0.json` — verified |
| `0x8c0cd03c` | `load_pool_constant_5` | `notes/verifications/0x8c0cd03c.json` — verified |
| `0x8c0cf068` | `test_bits_to_bool` | `notes/verifications/0x8c0cf068.json` — verified |
| `0x8c0cf6b4` | `load_store_to_stack_3` | `notes/verifications/0x8c0cf6b4.json` — verified |
| `0x8c0cf6bc` | `follow_pointer_and_store_back_3` | `notes/verifications/0x8c0cf6bc.json` — verified |
| `0x8c0cf6c8` | `load_double_indirect_store_to_stack_4` | `notes/verifications/0x8c0cf6c8.json` — verified |
| `0x8c0cf8dc` | `load_store_to_stack_4` | `notes/verifications/0x8c0cf8dc.json` — verified |
| `0x8c0d0058` | `FUN_8c0d0058` | `notes/verifications/0x8c0d0058.*.json` — both branches; byte at (*r4)+23 = (fr4==0 ? 0 : 1) in rts delay |
| `0x8c0d0074` | `FUN_8c0d0074` | `notes/verifications/0x8c0d0074.json` — 23 oracle steps; stores: f32 1.0 at *r4+12, product at *r4+44, flag byte r3 at *r4+22 (rts delay); fcmp/gt sets T=fr4>frN (operand order reversed vs listing); then-path: r0=22, r1=*r4, r3=1, fr0=0.9f, fr1=0.9*seed, T=1; else path mutates fr4 and fr2 |
| `0x8c0d00ec` | `FUN_8c0d00ec` | `notes/verifications/0x8c0d00ec.json` — 23 oracle steps; stores: f32 1.0 at *r4+8, product at *r4+40, flag byte at *r4+21 (rts delay); T=fr4>frN; then-path: r0=21, r1=*r4, r3=1, fr0=0.4f, fr1=0.4*seed, T=1 |
| `0x8c0d0aac` | `advance_40_2` | `notes/verifications/0x8c0d0aac.json` — verified |
| `0x8c0d0ab4` | `advance_28_2` | `notes/verifications/0x8c0d0ab4.json` — verified |
| `0x8c0d0edc` | `FUN_8c0d0edc` | `notes/verifications/0x8c0d0edc.json` — 24 oracle steps; no stores; r0=new r4 (rts delay), r1=12*(~r5&7), r3=r4+76+12*r5, r4=r4+68+12*min side, r6=12*r5, fr0/fr1 hold the two compared floats, T=fr0>fr1 (fcmp/gt fr1,fr0); bra delay slot add #68,r4 runs on the taken path too |
| `0x8c0d1288` | `indexed_pool_load` | `notes/verifications/0x8c0d1288.json` — verified |
| `0x8c0d1294` | `indexed_pool_load_2` | `notes/verifications/0x8c0d1294.json` — verified |
| `0x8c0d1360` | `offset_table_entry` | `notes/verifications/0x8c0d1360.json` — verified |
| `0x8c0d46f4` | `FUN_8c0d46f4` | `notes/verifications/0x8c0d46f4.json` — ok; 4-word copy r5→r4+64 |
| `0x8c0d474c` | `advance_61_load_byte` | `notes/verifications/0x8c0d474c.json` — verified |
| `0x8c0d4754` | `FUN_8c0d4754` | `notes/verifications/0x8c0d4754.json` — ok; byte[r4+61]=0, *(r4+80)=1.0f, r0=3→r4+56 in rts delay |
| `0x8c0d4768` | `FUN_8c0d4768` | `notes/verifications/0x8c0d4768.json` — ok; same, r0=2 |
| `0x8c0d4b60` | `zero_field_52` | `notes/verifications/0x8c0d4b60.json` — verified |
| `0x8c0d50bc` | `load_field_8` | `notes/verifications/0x8c0d50bc.json` — verified |
| `0x8c0d50c0` | `load_field_4` | `notes/verifications/0x8c0d50c0.json` — verified |
| `0x8c0d50c4` | `load_field_8_2` | `notes/verifications/0x8c0d50c4.json` — verified |
| `0x8c0d50c8` | `load_field_4_2` | `notes/verifications/0x8c0d50c8.json` — verified |
| `0x8c0d50e4` | `load_float_via_field_24` | `notes/verifications/0x8c0d50e4.json` — verified |
| `0x8c0d5148` | `advance_24_load_float` | `notes/verifications/0x8c0d5148.json` — verified |
| `0x8c0d5204` | `zero_three_words_2` | `notes/verifications/0x8c0d5204.json` — verified |
| `0x8c0d5210` | `FUN_8c0d5210` | `notes/verifications/0x8c0d5210.json` — 30 oracle steps; stores: 9 dwords r4+0..+32 then fr4 at +36, fr5 at +40; r0=r4+36, r1=*(r6+8), r3=*r6, r4=r4_entry+40, r7=r4_entry+24, r14 restored |
| `0x8c0d524c` | `FUN_8c0d524c` | `notes/verifications/0x8c0d524c.json` — 32 oracle steps; stores: 9 dwords r4+0..+32 then fr4 at +36, fr5 at +40; r0=r4+36, r1=*(r7+8), r3=*r7, r4=r4_entry+40, r13/r14 restored |
| `0x8c0d59a4` | `FUN_8c0d59a4` | `notes/verifications/0x8c0d59a4.json` — 76 oracle steps; fcmp/gt frA,frB sets T=(frB>frA); mov #0,r0 is the bf.s delay slot so r0 is zeroed on BOTH block-0 paths; bit 1<<2k when t=d-sum>0, else bit 2<<2k when d+sum<0; r1:=p+20,r3:=p+8,r4:=p+44,r5:=p+32, fr0:=0, fr1:=block2 t, fr2:=[p+44] or +sum, fr3:=block2 sum; T=1 unless block2 else-path with fr2>=0 |
| `0x8c0d5bf0` | `advance_24_2` | `notes/verifications/0x8c0d5bf0.json` — verified |
| `0x8c0d5bf8` | `store_two_args_at_0_4` | `notes/verifications/0x8c0d5bf8.json` — verified |
| `0x8c0d8aa8` | `FUN_8c0d8aa8` | `notes/verifications/0x8c0d8aa8.json` — ok; [r4+4/8/12]=0, [r4+16]=-1, [r4+20]=0 in rts delay, r1=0, r0=-1 |
| `0x8c0d8b74` | `store_float_to_global` | `notes/verifications/0x8c0d8b74.json` — verified |
| `0x8c0d8cfc` | `FUN_8c0d8cfc` | `notes/verifications/0x8c0d8cfc.json` — 71 oracle steps; fcmp/gt direction T=(2nd>1st): path A when fr4>[r4+120]; path C when fr4>[r4+124]; fr4 clobbered only on path C ((fr4-lo)*[r4+128]); bf.s delay fldi0 fr0 runs even when not taken so the fr5==1.0 return is fr0=0.0; fr5==0 path sets fpul:=count-1, r4:=r4+76, r0:=count-1; loop found path sets fpul:=i, r0:=4i, r4:=r4+88+4i, r1:=r4+92+4i; miss exit keeps r0:=r1+r5 (last probe addr), r3/r5 loop leftovers, T=0; fr1:=lo on check paths, last table float on miss path |
| `0x8c0d8ef0` | `FUN_8c0d8ef0` | `notes/verifications/0x8c0d8ef0.*.json` — all 3 branches; bounds-checked table lookup, byte r4+68==1 → **(r4+64), else *(*(r4+64)+r5*4), r0=0 if OOB |
| `0x8c0d91d0` | `reciprocal_at_124` | `notes/verifications/0x8c0d91d0.json` — verified |
| `0x8c0d91dc` | `reciprocal_at_120` | `notes/verifications/0x8c0d91dc.json` — verified |
| `0x8c0d98a0` | `FUN_8c0d98a0` | `notes/verifications/0x8c0d98a0.json` — 7 oracle steps; stores: literal value 0x8c1e8b2c then 0x8c1ec3f0 at r4 (second wins, single indirection - not *0x8c1e8b2c), r5 at r4+4; r0=r4 (rts delay), T untouched |
| `0x8c0da614` | `FUN_8c0da614` | `notes/verifications/0x8c0da614.json` — ok; fr4→r4, fr5→r4+4, r0=r4 in rts delay |
| `0x8c0da620` | `FUN_8c0da620` | `notes/verifications/0x8c0da620.json` — ok; 0.0f→r4, r4+4, r0=r4 in rts delay |
| `0x8c0da7b0` | `FUN_8c0da7b0` | `notes/verifications/0x8c0da7b0.json` — 135 oracle steps; r1=r4+108; r2=entry sp; r3=r4+104; r5=r4+100; r6=r4+96; r7=r4+92; r0=r4; fr0=@(entry sp) last load; fr4..fr11 untouched; r12-r14 restored; r15=entry sp; T untouched; args [[sp+8]]/[[sp+12]] are pointers read from stack slots |
| `0x8c0da8c0` | `FUN_8c0da8c0` | `notes/verifications/0x8c0da8c0.json` — 126 oracle steps; fr1=0.0 and fr3=1.0 (fldi clobber seeds); fr2=c0; fr0=g[3] last load; r0=r4; r1=r4+104; r3=g+8; r5=g+4; r7=r4+92; r2 untouched; r10-r14 restored; r15=entry sp; T untouched; r6=global ptr @0x8c0da9bc |
| `0x8c0dab10` | `FUN_8c0dab10` | `notes/verifications/0x8c0dab10.json` — 74 oracle steps; three paths: flag@0x8c27a8a8!=0 -> r0:=0x8c27a8c8,T=0; copy path stores dest+4..=64 <- inner+4..=64 (16 words) with r1:=inner[16],r3:=inner[15],r5:=inner[13],r6:=inner+36,r7:=dest+36,T=0; skip path only r0:=inner,T=1 (last tst); return [[0x5e0+r4]]+12 |
| `0x8c0dabbc` | `FUN_8c0dabbc` | `notes/verifications/0x8c0dabbc.json` — ok; r0 = *(r4+0x5d8) in rts delay |
| `0x8c0dabc8` | `FUN_8c0dabc8` | `notes/verifications/0x8c0dabc8.json` — ok; r0=*(r4+0x5d8) in rts delay |
| `0x8c0daf0c` | `FUN_8c0daf0c` | `notes/verifications/0x8c0daf0c.json` — ok; *(r4+0xd0) = *r15, rts delay byte[r4+0xcc] = 1 |
| `0x8c0daf28` | `FUN_8c0daf28` | `notes/verifications/0x8c0daf28.json` — ok; r4 += 0x5bc, rts delay *(r4)=fr4 |
| `0x8c0daf34` | `FUN_8c0daf34` | `notes/verifications/0x8c0daf34.json` — ok; r4 += 0x5bc, rts delay fr0=*(r4) |
| `0x8c0dfb04` | `FUN_8c0dfb04` | `notes/verifications/0x8c0dfb04.json` — ok; *(r4+0x190)=0 in rts delay |
| `0x8c0dfb10` | `FUN_8c0dfb10` | `notes/verifications/0x8c0dfb10.json` — 47 oracle steps; stores: byte 0 at r4+0x194, key/value/count updates on insert paths |
| `0x8c0e1f3c` | `FUN_8c0e1f3c` | `notes/verifications/0x8c0e1f3c.*.json` — all 3 branches; byte[r4+97] = (byte r5 && byte r4+98) in rts delay |
| `0x8c0e1f5c` | `store_byte_at_96` | `notes/verifications/0x8c0e1f5c.json` — verified |
| `0x8c0e1f64` | `store_word_at_92` | `notes/verifications/0x8c0e1f64.json` — verified |
| `0x8c0e2884` | `FUN_8c0e2884` | `notes/verifications/0x8c0e2884.*.json` — both branches; r0 = 7 if byte r5 != 0 else 2 |
| `0x8c0e2894` | `copy_three_floats_2` | `notes/verifications/0x8c0e2894.json` — verified |
| `0x8c0e28c0` | `FUN_8c0e28c0` | `notes/verifications/0x8c0e28c0.json` — ok; r0 = *(*[r4+8]+4) + 0x240 in rts delay |
| `0x8c0e2ed4` | `unknown` | `notes/verifications/0x8c0e2ed4.json` — 50 oracle steps |
| `0x8c0e2fc4` | `load_global_indirect` | `notes/verifications/0x8c0e2fc4.json` — verified |
| `0x8c0e3a00` | `FUN_8c0e3a00` | `notes/verifications/0x8c0e3a00.json` — 18 oracle steps; store r4..r7 to 0x8c29b720/28/30/38 + byte 1 at 0x8c29b740; r0=1, r1=0x8c29b740, r3=0x8c29b738 left set; r12/r13/r14 restored to INPUT values by the pops (delay-slot pop of r14 after rts) |
| `0x8c0e3a3c` | `unknown` | `notes/verifications/0x8c0e3a3c.json` — 45 oracle steps |
| `0x8c0e3fc4` | `unknown` | `notes/verifications/0x8c0e3fc4.json` — 211 oracle steps |
| `0x8c0e5988` | `FUN_8c0e5988` | `notes/verifications/0x8c0e5988.json` — 160 oracle steps; r0/r1 are clip flags set by delay slots (mov #1,r0 at 0x8c0e59b4/0x8c0e5a08, mov r1,r0 at 0x8c0e59c6/0x8c0e59e8/0x8c0e59fa, mov #1,r1 at 0x8c0e5a56); r0 ends as the queue pointer 0x8c27ad10 and r1 as r3+4 on the output path; r3/r4 are queue head/cursor; fr0-fr15 all churn (fr9/fr6 negated twice, fr13/fr14 restored from stack); T ends as last fcmp/gt or tst; every bf.s/bra delay slot (fmov fr14,fr1 / fmov fr13,fr2 / fmov fr0,fr1 / fadd fr3,fr0 / fneg fr9 / fmul fr2,fr10) runs per branch-taken rules; pref @r3/@r1 are no-ops; 12 queue stores incl. fr9 stored twice and one store at the un-incremented head |
| `0x8c0e6ea4` | `load_global_field_12` | `notes/verifications/0x8c0e6ea4.json` — verified |
| `0x8c0e7234` | `sr_imask` | `notes/verifications/0x8c0e7234.json` — verified |
| `0x8c0e7ab4` | `unknown` | `notes/verifications/0x8c0e7ab4.json` — 36 oracle steps |
| `0x8c0e8c60` | `empty_stub_6` | `notes/verifications/0x8c0e8c60.json` — verified |
| `0x8c0e8c64` | `FUN_8c0e8c64` | `notes/verifications/0x8c0e8c64.json` — 117 oracle steps; stores: u32 words to [r10+i*4], byte (r6-2)&0xff to [r7+rec]; regs: r0=rec idx before inc, r1=total words, r3=sum of counts, r4=rec count, r6=last count, r14 slot=[r15]=0; T=0 all paths |
| `0x8c0e8d70` | `unknown` | `notes/verifications/0x8c0e8d70.json` — 39 oracle steps |
| `0x8c0e8dfc` | `swap_float_global` | `notes/verifications/0x8c0e8dfc.json` — verified |
| `0x8c0e8e08` | `FUN_8c0e8e08` | `notes/verifications/0x8c0e8e08.json` — 75 oracle steps; r5<-0x01000000 and r6<-masked word on update path (r6 stays 0x8c1d6e00, r5 stays param_2 when bt.s taken); r7<-*r3&0x0ff300c0; r0 ends 112 on tail path but the tested mode/[r4+0x70] byte on early exits; r1 ends 0 on tail path but the mode on early exits; T=last cmp/eq #imm or tst; bt.s delay add r6,r1 runs on both paths; mode2/mode3 bra delay slots perform the final mov.l r0,@(12,r3) store; mode1 clobbers fr0-fr3, r12-r14 scratch and fpul (last ftrc) |
| `0x8c0e9ef0` | `FUN_8c0e9ef0` | `notes/verifications/0x8c0e9ef0.json` — 101 oracle steps; stores: 4x f32 at [out+0/4/8/12]; regs: r6=n, r7=24n, r3=base+16, r4=base+20 (pre-header runs once, back edge targets 0x8c0e9f0e), r0=out+8, r1=out+4, fr0..fr5 mins/max leftovers; T=0 all paths |
| `0x8c0e9f6c` | `FUN_8c0e9f6c` | `notes/verifications/0x8c0e9f6c.json` — 101 oracle steps; stores: 4x f32 at [out+0/4/8/12]; regs: same shape as 0x8c0e9ef0 (r3=base+16, r4=base+20, r7=24n, r0=out+8, r1=out+4), literals at 0x8c0e9fe0/0x8c0e9fe4; T=0 all paths |
| `0x8c0eacf4` | `empty_stub_7` | `notes/verifications/0x8c0eacf4.json` — verified |
| `0x8c0eaef8` | `advance_28_2` | `notes/verifications/0x8c0eaef8.json` — verified |
| `0x8c0f02fc` | `store_word_at_64` | `notes/verifications/0x8c0f02fc.json` — verified |
| `0x8c0f0814` | `FUN_8c0f0814` | `notes/verifications/0x8c0f0814.json` — ok; *(r4+0x90)=*(r4+0x94)=0 (latter in rts delay), r1=0 |
| `0x8c0f0828` | `FUN_8c0f0828` | `notes/verifications/0x8c0f0828.json` — ok; *(r4+0x90)+=r5, *(r4+0x94)+=r6 in rts delay |
| `0x8c0f1138` | `advance_to_limit` | `notes/verifications/0x8c0f1138.json` — verified |
| `0x8c0f134c` | `FUN_8c0f134c` | `notes/verifications/0x8c0f134c.json` — 36 oracle steps; copy [r4,r5) dwords to r6, return r6; r0=last dword copied (untouched on skipped stores); r4==r5 at end; r6 += 4 per iteration INCLUDING the final one (bf.s delay slot add #4,r6 runs whether or not taken); T=1 on every exit |
| `0x8c0f1508` | `indexed_load_offset_434` | `notes/verifications/0x8c0f1508.json` — verified |
| `0x8c0f1a70` | `load_pool_constant_6` | `notes/verifications/0x8c0f1a70.json` — verified |
| `0x8c0f1ad8` | `clear_byte_13` | `notes/verifications/0x8c0f1ad8.json` — verified |
| `0x8c0f1ca8` | `store_field_20` | `notes/verifications/0x8c0f1ca8.json` — verified |
| `0x8c0f32c4` | `load_store_to_stack_5` | `notes/verifications/0x8c0f32c4.json` — verified |
| `0x8c0f32cc` | `follow_pointer_and_store_back_4` | `notes/verifications/0x8c0f32cc.json` — verified |
| `0x8c0f32d8` | `load_double_indirect_store_to_stack_5` | `notes/verifications/0x8c0f32d8.json` — verified |
| `0x8c0f4094` | `follow_pointer_and_store_back_5` | `notes/verifications/0x8c0f4094.json` — verified |
| `0x8c0f6cc4` | `advance_68_load` | `notes/verifications/0x8c0f6cc4.json` — verified |
| `0x8c0f6ccc` | `advance_56_load_float` | `notes/verifications/0x8c0f6ccc.json` — verified |
| `0x8c0f72ac` | `advance_72_load` | `notes/verifications/0x8c0f72ac.json` — verified |
| `0x8c0f76b0` | `advance_56_load_float_2` | `notes/verifications/0x8c0f76b0.json` — verified |
| `0x8c0f7ae4` | `load_field_60` | `notes/verifications/0x8c0f7ae4.json` — verified |
| `0x8c0f7ae8` | `indexed_load_at_64` | `notes/verifications/0x8c0f7ae8.json` — verified |
| `0x8c0f7e6c` | `copy_word_return_self` | `notes/verifications/0x8c0f7e6c.json` — verified |
| `0x8c0f7e78` | `store_advanced_arg_to_stack_pointer_2` | `notes/verifications/0x8c0f7e78.json` — verified |
| `0x8c0f816c` | `advance_56` | `notes/verifications/0x8c0f816c.json` — verified |
| `0x8c0f8174` | `copy_three_words_at_56` | `notes/verifications/0x8c0f8174.json` — verified |
| `0x8c0f8850` | `load_field_12_via_36` | `notes/verifications/0x8c0f8850.json` — verified |
| `0x8c0f8858` | `advance_20` | `notes/verifications/0x8c0f8858.json` — verified |
| `0x8c0f8860` | `copy_four_words_at_20` | `notes/verifications/0x8c0f8860.json` — verified |
| `0x8c0f8874` | `advance_8` | `notes/verifications/0x8c0f8874.json` — verified |
| `0x8c0f8a10` | `advance_12_5` | `notes/verifications/0x8c0f8a10.json` — verified |
| `0x8c0f9aa8` | `FUN_8c0f9aa8` | `notes/verifications/0x8c0f9aa8.json` — 28 oracle steps; strcmp-style byte loop (no pseudocode existed); r4 and r5 both advance 1 per compared byte (bt.s delay slot add #1,r5 always runs); r3=zero-ext a-byte, r1=zero-ext b-byte; r0 = a-b on mismatch (rts delay sub r1,r0), 0 on NUL; T=0 mismatch / T=1 NUL exit |
| `0x8c0fca08` | `pick_smaller_pointer` | `notes/verifications/0x8c0fca08.json` — verified |
| `0x8c0fd168` | `FUN_8c0fd168` | `notes/verifications/0x8c0fd168.json` — 19 oracle steps; node init: r4=0 (argument destroyed by mov #0,r4), r6=param, r3=param+8, r1=param+4, r0=param (return); stores node[0]=*0x8c202fa8, node[1]=0, byte at node+8=*r5, and TWO stores at node+12 (0 then param+4) |
| `0x8c0fd3b8` | `load_field_12_via_0` | `notes/verifications/0x8c0fd3b8.json` — verified |
| `0x8c0fd3c0` | `FUN_8c0fd3c0` | `notes/verifications/0x8c0fd3c0.json` — 19 oracle steps; same as 0x8c0fd168; id cell via literal 0x8c0fd3e8 -> 0x8c202f90; r4=0, r6/r3/r1/r0 leftovers, double store at node+12 |
| `0x8c0fd5c8` | `FUN_8c0fd5c8` | `notes/verifications/0x8c0fd5c8.json` — 19 oracle steps; same as 0x8c0fd168; id cell via literal 0x8c0fd5f0 -> 0x8c202f78; r4=0, r6/r3/r1/r0 leftovers, double store at node+12 |
| `0x8c0fda54` | `FUN_8c0fda54` | `notes/verifications/0x8c0fda54.json` — 22 oracle steps; copy 4 floats src(r5)+0x30..0x3c to dst=@r15; r5 clobbered to src+0x3c (add #60,r5), r0=dst+8, r1=dst+4, r3=dst+12, fr0=src[0x3c]; stores in order dst+0,+4,+8,+12 with the last one in the rts DELAY SLOT |
| `0x8c0fda80` | `unknown` | `notes/verifications/0x8c0fda80.json` — 50 oracle steps |
| `0x8c0fdae8` | `copy_four_words_at_48` | `notes/verifications/0x8c0fdae8.json` — verified |
| `0x8c0fdafc` | `FUN_8c0fdafc` | `notes/verifications/0x8c0fdafc.json` — 32 oracle steps; stores: 4x f32 [r5+16..] or [r5+32..] -> [arg+0/4/8/12]; regs per path: A r0=r5o+24 r1=arg+8 r3=arg+4 r4=arg+12 r5=r5o+28; B r0=0x8c0fdbc8 r1/r4/r5/r7=RGBA bytes r6=arg+8 r14=arg+4(then restored) fpul=byte0 fr0..fr3=byte/255,255.0; C like A with +32/+40/+44; T=0 on A/B, T=1 on C |
| `0x8c0fdbcc` | `FUN_8c0fdbcc` | `notes/verifications/0x8c0fdbcc.json` — 17 oracle steps; stores: path A 16B [r6]->[r5+16..], path C 16B [r6]->[r5+32..], path B packed RGBA u32 at [r5+16]; regs: A/C r5+=16|32, r1=[r6+8], r0=[r6+12]; B r3=packed, r1=b2<<16, r0=int(f*255), r6+=12, fr0..fr3=f*255/255.0, fpul=int(f3*255); T=0 on A/B, T=1 on C |
| `0x8c0fdc58` | `empty_stub_8` | `notes/verifications/0x8c0fdc58.json` — verified |
| `0x8c0fdc5c` | `empty_stub_9` | `notes/verifications/0x8c0fdc5c.json` — verified |
| `0x8c0fdc8c` | `store_arg_to_global_3` | `notes/verifications/0x8c0fdc8c.json` — verified |
| `0x8c0fee78` | `store_advanced_arg_to_stack_pointer_3` | `notes/verifications/0x8c0fee78.json` — verified |
| `0x8c0ff28c` | `advance_12_6` | `notes/verifications/0x8c0ff28c.json` — verified |
| `0x8c0ff294` | `store_arg_return_self_5` | `notes/verifications/0x8c0ff294.json` — verified |
| `0x8c0ffac8` | `FUN_8c0ffac8` | `notes/verifications/0x8c0ffac8.json` — 32 oracle steps; find byte (char)r5 in base[r6..count), return offset or -1; r4=base+count, r7=base, r3=last byte scanned (sign-ext), r1=sign-ext needle computed once before loop; -1 returns come from rts/bt delay slots (mov #-1,r0), hit returns r0-base via delay sub r7,r0; T=1 on all exits |
| `0x8c0ffd38` | `FUN_8c0ffd38` | `notes/verifications/0x8c0ffd38.json` — 19 oracle steps; same as 0x8c0fd168; id cell via literal 0x8c0ffd60 -> 0x8c2032f0; r4=0, r6/r3/r1/r0 leftovers, double store at node+12 |
| `0x8c100198` | `FUN_8c100198` | `notes/verifications/0x8c100198.json` — 18 oracle steps; cursor bump: count++ at r4+4, offset(r4+8)+=4; r1=base[12]+4*base[0] on BOTH paths, r3=old offset+4, r0=r4 always (rts delay slot mov r4,r0 overwrites), T=cmp/eq result; wrap store offset=base[12] only when equal |
| `0x8c100330` | `store_float_indexed` | `notes/verifications/0x8c100330.json` — verified |
| `0x8c100340` | `load_float_indexed` | `notes/verifications/0x8c100340.json` — verified |
| `0x8c100350` | `empty_stub_10` | `notes/verifications/0x8c100350.json` — verified |
| `0x8c100354` | `FUN_8c100354` | `notes/verifications/0x8c100354.json` — 24 oracle steps; stores: 8 f32 zeros at 0x8c2ab318+0..28 (last is rts delay slot) |
| `0x8c1031dc` | `empty_stub_11` | `notes/verifications/0x8c1031dc.json` — verified |
| `0x8c103f94` | `store_byte_at_4_2` | `notes/verifications/0x8c103f94.json` — verified |
| `0x8c1045a0` | `empty_stub_12` | `notes/verifications/0x8c1045a0.json` — verified |
| `0x8c104768` | `advance_32_load_byte` | `notes/verifications/0x8c104768.json` — verified |
| `0x8c104a64` | `FUN_8c104a64` | `notes/verifications/0x8c104a64.json` — ok; 4 floats r5→r4, r5+=12, r0=r4, r1=r4+4, fr0=last |
| `0x8c105f1c` | `store_arg_return_self_6` | `notes/verifications/0x8c105f1c.json` — verified |
| `0x8c105f24` | `advance_12_7` | `notes/verifications/0x8c105f24.json` — verified |
| `0x8c105f70` | `FUN_8c105f70` | `notes/verifications/0x8c105f70.json` — ok; r0 = r5*20 + *(r4+4) in rts delay |
| `0x8c106044` | `load_float_via_field_72` | `notes/verifications/0x8c106044.json` — verified |
| `0x8c10604c` | `load_float_via_field_68` | `notes/verifications/0x8c10604c.json` — verified |
| `0x8c106054` | `load_float_via_field_64` | `notes/verifications/0x8c106054.json` — verified |
| `0x8c10605c` | `load_float_via_field_60` | `notes/verifications/0x8c10605c.json` — verified |
| `0x8c106064` | `load_float_via_field_56` | `notes/verifications/0x8c106064.json` — verified |
| `0x8c10606c` | `FUN_8c10606c` | `notes/verifications/0x8c10606c.json` — ok; r0=*r4+52, rts delay fr0=*(r0) |
| `0x8c10609c` | `FUN_8c10609c` | `notes/verifications/0x8c10609c.json` — ok; r0=*r4+0x22 (mov #34 is decimal) in rts delay |
| `0x8c1060cc` | `FUN_8c1060cc` | `notes/verifications/0x8c1060cc.json` — ok; r0=*(*r4+20) in rts delay |
| `0x8c1060d4` | `FUN_8c1060d4` | `notes/verifications/0x8c1060d4.json` — ok; r0=*r4+16, rts delay fr0=*(r0) |
| `0x8c1060dc` | `FUN_8c1060dc` | `notes/verifications/0x8c1060dc.json` — ok; r0=*r4, rts delay r0=*(r0+12) |
| `0x8c10610c` | `FUN_8c10610c` | `notes/verifications/0x8c10610c.json` — ok; r0=*r4, rts delay r0=*r0 |
| `0x8c106878` | `FUN_8c106878` | `notes/verifications/0x8c106878.json` — ok; byte-compare loop (same shape as 0x8c05e038), r0=diff in rts delay |
| `0x8c10689c` | `FUN_8c10689c` | `notes/verifications/0x8c10689c.json` — ok; struct init, pool 0x8c203ea8 single deref |
| `0x8c106afc` | `FUN_8c106afc` | `notes/verifications/0x8c106afc.json` — 3 oracle steps; stores r4 at global 0x8c2ab3b0 in the rts delay slot; r0 ends as the pointer |
| `0x8c106b08` | `FUN_8c106b08` | `notes/verifications/0x8c106b08.json` — 7 oracle steps; no stores; r0 = 0x8c2ab3c8 when *(0x8c2ab3b0)==0 else the loaded value; T ends as (value==0) |
| `0x8c106df8` | `FUN_8c106df8` | `notes/verifications/0x8c106df8.json` — ok; r4+=12, r0=r4 in rts delay |
| `0x8c1075c4` | `FUN_8c1075c4` | `notes/verifications/0x8c1075c4.json` — ok; r0=64, rts delay *(r4+64)=r5 |
| `0x8c10b6bc` | `FUN_8c10b6bc` | `notes/verifications/0x8c10b6bc.*.json` — 3 cases + default; switch(r5): r0 = r4+{76,0x94,0xcc,0x104} or 0 |
| `0x8c10b78c` | `FUN_8c10b78c` | `notes/verifications/0x8c10b78c.json` — ok; r0=0x23c, rts delay r0=r4+r0 |
| `0x8c10b798` | `FUN_8c10b798` | `notes/verifications/0x8c10b798.json` — ok; r0=0x13c, rts delay r0=r4+r0 |
| `0x8c10b950` | `FUN_8c10b950` | `notes/verifications/0x8c10b950.json` — ok; *(*r15)=*r4 in rts delay |
| `0x8c10b958` | `FUN_8c10b958` | `notes/verifications/0x8c10b958.json` — ok; *r4=**r4, *(*r15)=old in rts delay |
| `0x8c10b964` | `FUN_8c10b964` | `notes/verifications/0x8c10b964.json` — ok; *(*r15)=**r4 in rts delay |
| `0x8c10bcc0` | `empty_stub_13` | `notes/verifications/0x8c10bcc0.json` — verified |
| `0x8c10c3b0` | `load_store_to_stack_6` | `notes/verifications/0x8c10c3b0.json` — verified |
| `0x8c10c484` | `load_store_to_stack_7` | `notes/verifications/0x8c10c484.json` — verified |
| `0x8c10c558` | `load_store_to_stack_8` | `notes/verifications/0x8c10c558.json` — verified |
| `0x8c10c6c4` | `follow_pointer_and_store_back_6` | `notes/verifications/0x8c10c6c4.json` — verified |
| `0x8c10c6d0` | `load_double_indirect_store_to_stack_6` | `notes/verifications/0x8c10c6d0.json` — verified |
| `0x8c10c7b0` | `follow_pointer_and_store_back_7` | `notes/verifications/0x8c10c7b0.json` — verified |
| `0x8c10c7bc` | `load_double_indirect_store_to_stack_7` | `notes/verifications/0x8c10c7bc.json` — verified |
| `0x8c10c878` | `follow_pointer_and_store_back_8` | `notes/verifications/0x8c10c878.json` — verified |
| `0x8c10c884` | `load_double_indirect_store_to_stack_8` | `notes/verifications/0x8c10c884.json` — verified |
| `0x8c10d7c8` | `load_byte_at_4` | `notes/verifications/0x8c10d7c8.json` — verified |
| `0x8c10e278` | `store_arg_return_self_7` | `notes/verifications/0x8c10e278.json` — verified |
| `0x8c10e280` | `advance_12_8` | `notes/verifications/0x8c10e280.json` — verified |
| `0x8c10e288` | `store_arg_return_self_8` | `notes/verifications/0x8c10e288.json` — verified |
| `0x8c10e290` | `advance_12_9` | `notes/verifications/0x8c10e290.json` — verified |
| `0x8c10e344` | `load_byte_via_field_82` | `notes/verifications/0x8c10e344.json` — verified |
| `0x8c10e350` | `load_byte_via_field_81` | `notes/verifications/0x8c10e350.json` — verified |
| `0x8c10e35c` | `load_word_via_field_68` | `notes/verifications/0x8c10e35c.json` — verified |
| `0x8c10e364` | `load_field_plus_36` | `notes/verifications/0x8c10e364.json` — verified |
| `0x8c10e36c` | `load_field_plus_32` | `notes/verifications/0x8c10e36c.json` — verified |
| `0x8c10e374` | `load_double_indirect_2` | `notes/verifications/0x8c10e374.json` — verified |
| `0x8c1118d4` | `swap_three_by_key` | `notes/verifications/0x8c1118d4*.json` - pair 97, null 53, third 104 |
| `0x8c10e37c` | `load_word_via_field_60` | `notes/verifications/0x8c10e37c.json` — verified |
| `0x8c10e384` | `load_word_via_field_56` | `notes/verifications/0x8c10e384.json` — verified |
| `0x8c10e38c` | `load_byte_via_field_80` | `notes/verifications/0x8c10e38c.json` — verified |
| `0x8c10e394` | `load_word_via_field_52` | `notes/verifications/0x8c10e394.json` — verified |
| `0x8c10e39c` | `load_word_via_field_48` | `notes/verifications/0x8c10e39c.json` — verified |
| `0x8c10e3a4` | `load_field_plus_28_2` | `notes/verifications/0x8c10e3a4.json` — verified |
| `0x8c10e3ac` | `load_field_plus_24` | `notes/verifications/0x8c10e3ac.json` — verified |
| `0x8c10e3b4` | `load_field_plus_20` | `notes/verifications/0x8c10e3b4.json` — verified |
| `0x8c10e3bc` | `load_field_plus_16` | `notes/verifications/0x8c10e3bc.json` — verified |
| `0x8c10e3c4` | `FUN_8c10e3c4` | `notes/verifications/0x8c10e3c4.json` — 3 oracle steps; r0 = *r4 + 12 |
| `0x8c10e3cc` | `FUN_8c10e3cc` | `notes/verifications/0x8c10e3cc.json` — 3 oracle steps; r0 = *r4 + 8 |
| `0x8c10e3d4` | `FUN_8c10e3d4` | `notes/verifications/0x8c10e3d4.json` — 3 oracle steps; r0 = *r4 + 4 |
| `0x8c10e3dc` | `FUN_8c10e3dc` | `notes/verifications/0x8c10e3dc.json` — 4 oracle steps; r0 = *(u32*)(*r4 + 0x4c) |
| `0x8c10eae0` | `FUN_8c10eae0` | `notes/verifications/0x8c10eae0.json` — ok; byte-compare loop like 0x8c05e038 |
| `0x8c10eb04` | `FUN_8c10eb04` | `notes/verifications/0x8c10eb04.json` — ok; struct init, pool 0x8c2049b8 single deref |
| `0x8c10ec98` | `FUN_8c10ec98` | `notes/verifications/0x8c10ec98.json` — ok; struct init, pool 0x8c2049a0 single deref |
| `0x8c10ef38` | `FUN_8c10ef38` | `notes/verifications/0x8c10ef38.json` — 2 oracle steps; r0 = r4 |
| `0x8c10efc4` | `FUN_8c10efc4` | `notes/verifications/0x8c10efc4.json` — 4 oracle steps; byte store 0 at r4; r0 = r4 |
| `0x8c10fe08` | `FUN_8c10fe08` | `notes/verifications/0x8c10fe08.json` — ok; byte[r5+0x32]=byte[r6+0x15c], r5+=32, 4 halfwords r6+0x94..→stack→r5+0..6, r1=r15-8 |
| `0x8c11004c` | `FUN_8c11004c` | `notes/verifications/0x8c11004c.*.json` — both branches; r0 = ((1 shld r5) & *(r4+44))==0 via subc/add trick |
| `0x8c110308` | `FUN_8c110308` | `notes/verifications/0x8c110308.json` — 3 oracle steps; store *(r4+0x14)=r5; r0=1 via rts delay slot |
| `0x8c1104c4` | `FUN_8c1104c4` | `notes/verifications/0x8c1104c4.json` — ok; r4+=72, r0 = *(*(r4)+r5*4) in rts delay |
| `0x8c110b7c` | `FUN_8c110b7c` | `notes/verifications/0x8c110b7c.json` — ok; *r4=*(r4+4)=*(r4+8)=0 (last in rts delay), r1=0, r0=r4 |
| `0x8c11181c` | `FUN_8c11181c` | `notes/verifications/0x8c11181c.json` — 74 oracle steps; r3/r6/r7/r0/r1 clobbered; r14/r13 saved+restored; insertion store is the bt.s delay slot (runs both ways); bra delay slot pre-decrements r7; T=0 on all exits |
| `0x8c111c30` | `FUN_8c111c30` | `notes/verifications/0x8c111c30.json` — 38 oracle steps; BLOCKED: oracle lacks movt r1 (op 0x0129) at 0x8c111c6c/0x8c111c88; every path hits it |
| `0x8c123d9c` | `FUN_8c123d9c` | `notes/verifications/0x8c123d9c.*.json` — both branches; byte r5 clamped to 10 → r4+56, *(r4+60)=0.0f, byte r4+56→r4+57 if byte r4+57 is 0 or >10 |
| `0x8c124d48` | `FUN_8c124d48` | `notes/verifications/0x8c124d48.json` — 4 oracle steps; r1=*r15 and r0=*r4 left set; store *r1=r0 is the rts delay slot |
| `0x8c124d50` | `FUN_8c124d50` | `notes/verifications/0x8c124d50.json` — ok; *r4 = **r4, *(*r15) = *r4_old in rts delay |
| `0x8c124d5c` | `FUN_8c124d5c` | `notes/verifications/0x8c124d5c.json` — 5 oracle steps; r0=**r4, r1=*r15; store in rts delay slot |
| `0x8c1252cc` | `walk_circular_list` | `notes/verifications/0x8c1252cc*.json` - one step 99, empty 46 |
| `0x8c125364` | `walk_circular_list_reset` | `notes/verifications/0x8c125364*.json` - visit 110, cleanup 329 |
| `0x8c1261d4` | `FUN_8c1261d4` | `notes/verifications/0x8c1261d4.json` — 10 oracle steps; stores to global 0x8c2ab5e0, *r4, byte r4+4=1, r4+8 (rts delay); r0=r4, r1=*0x8c206c28 |
| `0x8c17ba64` | `publish_object` | `notes/verifications/0x8c17ba64.json` - 99 oracle steps; object+24 points at itself |
| `0x8c127c6c` | `FUN_8c127c6c` | `notes/verifications/0x8c127c6c.json` — 3 oracle steps; r4+=0x40; 32-bit fr4 store at new r4 in rts delay slot |
| `0x8c127c74` | `FUN_8c127c74` | `notes/verifications/0x8c127c74.json` — 3 oracle steps; r4+=0x4c; fr4 store in rts delay slot |
| `0x8c127c7c` | `FUN_8c127c7c` | `notes/verifications/0x8c127c7c.json` — 3 oracle steps; r4+=0x44; fr4 store in rts delay slot |
| `0x8c1282a4` | `FUN_8c1282a4` | `notes/verifications/0x8c1282a4.json` — 3 oracle steps; r4+=0x4c; fr0=mem32[r4] (bit-exact, NaN payload survives) in rts delay slot |
| `0x8c1282ac` | `FUN_8c1282ac` | `notes/verifications/0x8c1282ac.json` — 3 oracle steps; r4 += 72 sticks (rts delay slot loads from shifted ptr); r0 = mem32[r4+72]; T untouched |
| `0x8c1282b4` | `FUN_8c1282b4` | `notes/verifications/0x8c1282b4.json` — 3 oracle steps; r4 += 68 sticks; fr0 (not fr1) := raw bits at r4+68 in rts delay slot; T untouched |
| `0x8c1282bc` | `FUN_8c1282bc` | `notes/verifications/0x8c1282bc.json` — 3 oracle steps; r4 += 64 sticks; fr0 := raw bits at r4+64 in rts delay slot; T untouched |
| `0x8c1282c4` | `FUN_8c1282c4` | `notes/verifications/0x8c1282c4.json` — 3 oracle steps; r4 += 60 sticks; fr0 := raw bits at r4+60 in rts delay slot; T untouched |
| `0x8c12b3f8` | `FUN_8c12b3f8` | `notes/verifications/0x8c12b3f8.*.json` — found+miss; linear search r5 in array at (*r4)+32, r0=1/0 |
| `0x8c12b46c` | `FUN_8c12b46c` | `notes/verifications/0x8c12b46c.json` — 3 oracle steps; none beyond r0; +56 applied in rts delay slot; T untouched |
| `0x8c12b474` | `FUN_8c12b474` | `notes/verifications/0x8c12b474.json` — ok; same search at (*r4)+44 |
| `0x8c12b498` | `FUN_8c12b498` | `notes/verifications/0x8c12b498.json` — 4 oracle steps; r0 = mem32[r4]+28 (bump before rts); fr0 := raw bits at that address in delay slot; T untouched |
| `0x8c12b4a0` | `FUN_8c12b4a0` | `notes/verifications/0x8c12b4a0.json` — 3 oracle steps; none beyond r0; second deref @(12,r0) in rts delay slot; T untouched |
| `0x8c12b4a8` | `FUN_8c12b4a8` | `notes/verifications/0x8c12b4a8.json` — 3 oracle steps; none beyond r0; second deref @(8,r0) in rts delay slot; T untouched |
| `0x8c12b4b0` | `FUN_8c12b4b0` | `notes/verifications/0x8c12b4b0.json` — 3 oracle steps; none beyond r0; second deref @(4,r0) in rts delay slot; T untouched |
| `0x8c12b4b8` | `FUN_8c12b4b8` | `notes/verifications/0x8c12b4b8.json` — 36 oracle steps; r5=*r4+32, r4/r3/r1 clobbered (end=base+4*count, r1=4*count), r0 incremented in bf.s delay slot every iteration incl. last, T=1 on both exits; shll2 leaves T alone; header words re-read each iteration |
| `0x8c12b4e0` | `FUN_8c12b4e0` | `notes/verifications/0x8c12b4e0.json` — 7 oracle steps; shll2 clobbers r5 (T untouched); r1 left holding base ptr mem[mem[r4]+0x28]; return load mov.l @(r0,r1),r0 sits in rts delay slot |
| `0x8c12b504` | `FUN_8c12b504` | `notes/verifications/0x8c12b504.json` — 3 oracle steps; second dereference mov.l @r0,r0 runs in rts delay slot (runs regardless of branch) |
| `0x8c12d484` | `FUN_8c12d484` | `notes/verifications/0x8c12d484.json` — 13 oracle steps; two mov.b stores of r3=1 at mem[mem[mem[r4]+4]]+0x20 and 0x83+mem[0x8c26c378]; r0/r1/r3/r4 clobbered; add #106,r0 in rts delay slot sets return 0x8c207b62 |
| `0x8c12d4ac` | `FUN_8c12d4ac` | `notes/verifications/0x8c12d4ac.json` — 3 oracle steps; r0 loaded from ROM literal pool at 0x8c12d4b4 (=0x8c1da1a0); rts delay slot is nop |
| `0x8c12d718` | `FUN_8c12d718` | `notes/verifications/0x8c12d718.json` — 3 oracle steps; r0 from literal pool 0x8c12d720 (=0x8c1da198); delay slot nop |
| `0x8c12d940` | `FUN_8c12d940` | `notes/verifications/0x8c12d940.json` — 3 oracle steps; r0 from literal pool 0x8c12d948 (=0x8c1da190); delay slot nop |
| `0x8c12da80` | `FUN_8c12da80` | `notes/verifications/0x8c12da80.json` — 3 oracle steps; r0 from literal pool 0x8c12da88 (=0x8c1da188); delay slot nop |
| `0x8c12dd8c` | `FUN_8c12dd8c` | `notes/verifications/0x8c12dd8c.json` — 3 oracle steps; r0 from literal pool 0x8c12dd94 (=0x8c1da168); delay slot nop |
| `0x8c12dfa0` | `FUN_8c12dfa0` | `notes/verifications/0x8c12dfa0.json` — 3 oracle steps; r0 from literal pool 0x8c12dfa8 (=0x8c1da160); delay slot nop |
| `0x8c12e098` | `FUN_8c12e098` | `notes/verifications/0x8c12e098.json` — 3 oracle steps; r0 from literal pool 0x8c12e0a0 (=0x8c1da158); delay slot nop |
| `0x8c12e280` | `load_pool_constant_7` | `notes/verifications/0x8c12e280.json` — verified |
| `0x8c12e424` | `load_pool_constant_8` | `notes/verifications/0x8c12e424.json` — verified |
| `0x8c12e60c` | `load_pool_constant_9` | `notes/verifications/0x8c12e60c.json` — verified |
| `0x8c12e86c` | `load_pool_constant_10` | `notes/verifications/0x8c12e86c.json` — verified |
| `0x8c12ea64` | `load_pool_constant_11` | `notes/verifications/0x8c12ea64.json` — verified |
| `0x8c12eb68` | `load_pool_constant_12` | `notes/verifications/0x8c12eb68.json` — verified |
| `0x8c12ecc0` | `load_pool_constant_13` | `notes/verifications/0x8c12ecc0.json` — verified |
| `0x8c12ee78` | `load_pool_constant_14` | `notes/verifications/0x8c12ee78.json` — verified |
| `0x8c12f028` | `load_pool_constant_15` | `notes/verifications/0x8c12f028.json` — verified |
| `0x8c137918` | `FUN_8c137918` | `notes/verifications/0x8c137918*.json` — both branches; bounds-checked dispatch: r0 = *(*(r4+0xf0)+r5*4)+12, OOB → *0x8c208178 |
| `0x8c137944` | `FUN_8c137944` | `notes/verifications/0x8c137944.json` — ok; same with count r4+0xe0, table r4+0xe4 |
| `0x8c137970` | `FUN_8c137970` | `notes/verifications/0x8c137970.json` — ok; dispatch with count r4+0xd4, table r4+0xd8 |
| `0x8c13799c` | `FUN_8c13799c` | `notes/verifications/0x8c13799c.json` — ok; count r4+0xc8, table r4+0xcc |
| `0x8c1379c8` | `FUN_8c1379c8` | `notes/verifications/0x8c1379c8.json` — ok; count r4+0xbc, table r4+0xc0, OOB *0x8c208170 |
| `0x8c1379f4` | `FUN_8c1379f4` | `notes/verifications/0x8c1379f4.json` — ok; count r4+0xb0, table r4+0xb4, OOB *0x8c208170 |
| `0x8c137a20` | `FUN_8c137a20` | `notes/verifications/0x8c137a20.json` — ok; count r4+0xa4, table r4+0xa8 |
| `0x8c137a4c` | `FUN_8c137a4c` | `notes/verifications/0x8c137a4c.json` — ok; count r4+0x98, table r4+0x9c |
| `0x8c13cacc` | `FUN_8c13cacc` | `notes/verifications/0x8c13cacc.json` — 20 oracle steps; 36-byte node copy to *B or chain walk; stores node longs, head at r4+0x80, flag byte cleared; fr0/fr1 hold compare const and node float; 5 path specs all pass |
| `0x8c13cc50` | `FUN_8c13cc50` | `notes/verifications/0x8c13cc50.json` — 20 oracle steps; 24-byte variant of 0x8c13cacc; same store pattern with 24-byte stride and const f32[0x8c13cd58] |
| `0x8c13cd68` | `FUN_8c13cd68` | `notes/verifications/0x8c13cd68.json` — 23 oracle steps; 16-byte variant; pushes/pops r14, fldi0 (fr0=0.0) threshold compare at node+8; 5 path specs all pass |
| `0x8c13cfa4` | `FUN_8c13cfa4` | `notes/verifications/0x8c13cfa4.json` — ok; r4+=116, r5*=8, *r6=*(*(r4)+r5), rts delay *(r6+4)=*(*(r4)+r5+4) |
| `0x8c13cfbc` | `FUN_8c13cfbc` | `notes/verifications/0x8c13cfbc.json` — 35 oracle steps; on every i with [r4+8]+80i==r5 copies 8 bytes from [[r4+104]+8i] to r6/r6+4; r3=r4+104, r7=[r4+4] at exit |
| `0x8c13cff8` | `FUN_8c13cff8` | `notes/verifications/0x8c13cff8.json` — ok; like 0x8c13cfa4 with r4+=104 |
| `0x8c13d010` | `FUN_8c13d010` | `notes/verifications/0x8c13d010.json` — ok; r4+=116, r5*=8, r0 = (*(*r4+r5) == -1) ? 0 : 2 |
| `0x8c13d02c` | `FUN_8c13d02c` | `notes/verifications/0x8c13d02c.json` — 31 oracle steps; on first match r4 += 104, returns 0 if entry==-1 else 2, else 0; r1=[r4+8], r3=80i+[r4+8] at exit |
| `0x8c13d068` | `FUN_8c13d068` | `notes/verifications/0x8c13d068.json` — ok; like 0x8c13d010 with r4+=104 |
| `0x8c13d084` | `FUN_8c13d084` | `notes/verifications/0x8c13d084.json` — ok; r4+=92, r0 = (*r4 == 0) ? 0 : *(*r4+12) |
| `0x8c13dba4` | `FUN_8c13dba4` | `notes/verifications/0x8c13dba4.json` — ok; 2-word element copy r4→r6 until r4==r5 or r6==0, r0=r6 in rts delay |
| `0x8c13dbc0` | `FUN_8c13dbc0` | `notes/verifications/0x8c13dbc0.json` — 4 oracle steps; copies 8-byte elements r4..r5 to r6 unless r6==0; bf.s delay slot advances r6 every iteration incl. last; returns advanced r6 in r0 |
| `0x8c13edcc` | `store_arg_return_self_8` | `notes/verifications/0x8c13edcc.json` — verified |
| `0x8c13edd4` | `advance_12_10` | `notes/verifications/0x8c13edd4.json` — verified |
| `0x8c1431f8` | `store_word_at_132` | `notes/verifications/0x8c1431f8.json` — verified |
| `0x8c143204` | `FUN_8c143204` | `notes/verifications/0x8c143204.json` — ok; fr4→r4+20, r4+=24, fr5→r4 in rts delay |
| `0x8c143210` | `FUN_8c143210` | `notes/verifications/0x8c143210.json` — 40 oracle steps; stores 4 longs at r4+32 (f32[r5], f32[r5+4], f32[r5+8], 1.0 via stack frame) and 3 longs from r6 at r4+48; r4+=48, r5+=8, r7=r4+32, r2=r15-4, r3=r15-16 |
| `0x8c143e88` | `store_word_at_92` | `notes/verifications/0x8c143e88.json` — verified |
| `0x8c144ba8` | `FUN_8c144ba8` | `notes/verifications/0x8c144ba8.json` — 10 oracle steps; walks list at [r4+4]; stores r4+4 (r4 incremented) or last <= node r3 through [r15]; r6 ends 0 |
| `0x8c144c60` | `FUN_8c144c60` | `notes/verifications/0x8c144c60.json` — ok; struct init, pool 0x8c2089d0 single deref |
| `0x8c145580` | `load_store_to_stack_9` | `notes/verifications/0x8c145580.json` — verified |
| `0x8c145588` | `FUN_8c145588` | `notes/verifications/0x8c145588.json` — ok; like 0x8c124d50: *r4=**r4, *(*r15)=old in rts delay |
| `0x8c145594` | `load_double_indirect_store_to_stack_9` | `notes/verifications/0x8c145594.json` — verified |
| `0x8c14644c` | `FUN_8c14644c` | `notes/verifications/0x8c14644c.json` — 83 oracle steps; mem[r4]=0; mem[r4+20/24/48/52]=1, mem[r4+28/56]=-1; 7 fmov/fmul products stored at r4+4..44; r11-r14 saved+restored; r0=r4; r1=0xa4 |
| `0x8c14666c` | `load_word_at_2004` | `notes/verifications/0x8c14666c.json` — verified |
| `0x8c146678` | `store_word_at_164` | `notes/verifications/0x8c146678.json` — verified |
| `0x8c146728` | `indexed_load_shll2` | `notes/verifications/0x8c146728.json` — verified |
| `0x8c146730` | `FUN_8c146730` | `notes/verifications/0x8c146730.json` — ok; *(r4+r5*4)=r6, *(r4+0xa8)+=1 in rts delay |
| `0x8c146d78` | `FUN_8c146d78` | `notes/verifications/0x8c146d78.json` — 128 oracle steps; 6 stack bucket counters mem[r15+cat*4] incremented (stack window excluded from diff); r12=sum of bytes; r0=last counter+1; r8-r14 saved+restored; r1=r15-52, r13=r11, r3=101, r6=51, r8=151, r14=2 leak |
| `0x8c148310` | `FUN_8c148310` | `notes/verifications/0x8c148310.json` — 18 oracle steps; r0=mem[r4+24/36/48/60+r5*4] in rts delay slot with r4+=off, r5<<=2 on cases 0-3; default r0=mem[r4+16]; T=1 on cases, T=0 on default |
| `0x8c148368` | `FUN_8c148368` | `notes/verifications/0x8c148368.json` — 5 oracle steps; same shape, offsets 72/84/96/108, default r0=mem[r4+20] |
| `0x8c149dd0` | `FUN_8c149dd0` | `notes/verifications/0x8c149dd0.json` — ok; r0 = pointer-max(r4, r5) by unsigned *values, r5=r4 on low path |
| `0x8c14add8` | `FUN_8c14add8` | `notes/verifications/0x8c14add8.json` — 39 oracle steps; copies 8 dwords + 3 floats (44 bytes) r5 -> r4; r0=r4; r13/r14 saved+restored; r5+=40, r3=r5+36, r6=r4+36, r7=r5+32, r14=r4+32 leak |
| `0x8c14b1f4` | `FUN_8c14b1f4` | `notes/verifications/0x8c14b1f4.json` — 56 oracle steps; zero-inits 116-byte struct at r4, mem[r4]=0x8c1ef28c, mem[r4+16/36/52]=-1, mem[r4+92]=1, bytes 96/98=1; also zeroes 3 dwords at 0x8c1ef2a4..2ac (ROM); r0=r4; r4+=40, r5=-1 leak; r15 balanced via rts delay slot |
| `0x8c14b268` | `FUN_8c14b268` | `notes/verifications/0x8c14b268.json` — ok; r4 shad -r5, r1=(r4&0xff)-127→fpul, fr0=float(fpul)*pool, r0=mova 0x8c14b27c |
| `0x8c14b280` | `FUN_8c14b280` | `notes/verifications/0x8c14b280.json` — ok (24 steps); scale fr4 by pool, clamp [0,pool2], ftrc→fpul, shld into r4 masked by ~r6 |
| `0x8c14b2ec` | `FUN_8c14b2ec` | `notes/verifications/0x8c14b2ec.json` — 19 oracle steps; mem[r4+4]=0 stored in rts delay slot; r0=0; r1 advanced to >= mem[r4+8]+4*old mem[r4+4]; r3=r1+4*k; T=0 on exit |
| `0x8c14c680` | `FUN_8c14c680` | `notes/verifications/0x8c14c680.json` — 286 oracle steps; copies 172-byte struct r5 -> r4 (floats, bytes at 12/13/92/93/104/105, dwords at 16/20/68); r8-r14 saved+restored |
| `0x8c14c8e8` | `FUN_8c14c8e8` | `notes/verifications/0x8c14c8e8.json` — 367 oracle steps; copies 216-byte struct r5 -> r4, all floats incl. r5/r4+0x80..0xd8; r8-r14 saved+restored |
| `0x8c14cfac` | `FUN_8c14cfac` | `notes/verifications/0x8c14cfac.json` — 44 oracle steps; copies 48-byte record r5 -> r4 (8 dwords + 3 floats + dword at 44); r0=r4; r12-r14 saved+restored |
| `0x8c14d210` | `pass` | `notes/verifications/0x8c14d210.json` — 19 oracle steps; 19 |
| `0x8c14dd90` | `copy_words_until_limit` | `notes/verifications/0x8c14dd90.json` — verified |
| `0x8c14dda8` | `copy_words_until_limit_2` | `notes/verifications/0x8c14dda8.json` — verified |
| `0x8c14e740` | `copy_words_until_limit_3` | `notes/verifications/0x8c14e740.json` — verified |
| `0x8c14faf4` | `empty_stub_14` | `notes/verifications/0x8c14faf4.json` — verified |
| `0x8c14fdd8` | `pass` | `notes/verifications/0x8c14fdd8.json` — 10 oracle steps; 10 |
| `0x8c14fdf4` | `pass` | `notes/verifications/0x8c14fdf4.json` — 11 oracle steps; 11 |
| `0x8c14fe10` | `load_via_field_plus_24` | `notes/verifications/0x8c14fe10.json` — verified |
| `0x8c14fe18` | `pass` | `notes/verifications/0x8c14fe18.json` — 11 oracle steps; 11 |
| `0x8c14fe34` | `load_via_field_plus_24_2` | `notes/verifications/0x8c14fe34.json` — verified |
| `0x8c14fe68` | `FUN_8c14fe68` | `notes/verifications/0x8c14fe68.json` — ok; r0=*(r4+4)+12, rts delay r0=*(r0+4) |
| `0x8c14fe70` | `FUN_8c14fe70` | `notes/verifications/0x8c14fe70.json` — ok; r0=*(r4+4), rts delay *(r0+36)=r5 |
| `0x8c14ffb8` | `FUN_8c14ffb8` | `notes/verifications/0x8c14ffb8.json` — ok; rts; delay r0=r4 |
| `0x8c15039c` | `pass` | `notes/verifications/0x8c15039c.json` — 36 oracle steps; 36 |
| `0x8c1504cc` | `pass` | `notes/verifications/0x8c1504cc.json` — 225 oracle steps; 225 |
| `0x8c1506a4` | `pass` | `notes/verifications/0x8c1506a4.json` — 41 oracle steps; 41 |
| `0x8c1508b8` | `pass` | `notes/verifications/0x8c1508b8.json` — 8 oracle steps; 8 |
| `0x8c15132c` | `pass` | `notes/verifications/0x8c15132c.json` — 3 oracle steps; 3 |
| `0x8c151338` | `FUN_8c151338` | `notes/verifications/0x8c151338.json` — ok; r4+=32, r0=r4 in rts delay |
| `0x8c1514b0` | `FUN_8c1514b0` | `notes/verifications/0x8c1514b0.json` — ok; r4+=28, rts delay fr0=*(r4) |
| `0x8c1514b8` | `FUN_8c1514b8` | `notes/verifications/0x8c1514b8.json` — ok; fr0=*r5, r4+=28, rts delay *(r4)=fr0 |
| `0x8c1514c0` | `FUN_8c1514c0` | `notes/verifications/0x8c1514c0.json` — ok; r4+=16, r0=r4 in rts delay |
| `0x8c1514d8` | `FUN_8c1514d8` | `notes/verifications/0x8c1514d8.json` — ok; r4+=4, r0=r4 in rts delay |
| `0x8c151b1c` | `FUN_8c151b1c` | `notes/verifications/0x8c151b1c.json` — ok; r4+=108, r0=r4 in rts delay |
| `0x8c151b24` | `advance_96_return_self` | `notes/verifications/0x8c151b24.json` — verified |
| `0x8c151bd0` | `FUN_8c151bd0` | `notes/verifications/0x8c151bd0.json` — ok; fr4→r4+88, r4+=92, 1/fr4→r4 in rts delay |
| `0x8c151f60` | `store_to_global` | `notes/verifications/0x8c151f60.json` — verified |
| `0x8c152004` | `advance_76_return_self` | `notes/verifications/0x8c152004.json` — verified |
| `0x8c152084` | `store_fr4_at_72` | `notes/verifications/0x8c152084.json` — verified |
| `0x8c152100` | `advance_60_return_self` | `notes/verifications/0x8c152100.json` — verified |
| `0x8c152138` | `advance_44_return_self` | `notes/verifications/0x8c152138.json` — verified |
| `0x8c152bf8` | `pool_plus_arg` | `notes/verifications/0x8c152bf8.json` — verified |
| `0x8c152ee0` | `pass` | `notes/verifications/0x8c152ee0.json` — 114 oracle steps; 114 |
| `0x8c154ec4` | `round_up_to_12_boundary` | `notes/verifications/0x8c154ec4.json` — verified |
| `0x8c1550fc` | `FUN_8c1550fc` | `notes/verifications/0x8c1550fc.json` — ok; 3-word element copy r4→r6 until r4==r5 or r6==0, r0=r6 in rts delay |
| `0x8c155258` | `store_r5_at_40` | `notes/verifications/0x8c155258.json` — verified |
| `0x8c1552b4` | `FUN_8c1552b4` | `notes/verifications/0x8c1552b4.json` — ok; 11-word copy r5→*([r4+8]), last word in rts delay (into r0) |
| `0x8c1552e4` | `store_through_two_levels` | `notes/verifications/0x8c1552e4.json` — 4 oracle steps, stores match |
| `0x8c1556d0` | `count_nonzero_bytes` | `notes/verifications/0x8c1556d0.json` — verified |
| `0x8c155dfc` | `FUN_8c155dfc` | `notes/verifications/0x8c155dfc.*.json` — both branches; byte[r4+16]=(fr4==0?0:1), fr4→r4 in rts delay |
| `0x8c155e14` | `global_pointer_deref` | `notes/verifications/0x8c155e14.json` — verified |
| `0x8c155e4c` | `init_from_rom_block` | `notes/verifications/0x8c155e4c.json` — verified |
| `0x8c1566b0` | `FUN_8c1566b0` | `notes/verifications/0x8c1566b0.json` — ok; *r4=r5, r0=r4 in rts delay |
| `0x8c156f8c` | `init_node` | `notes/verifications/0x8c156f8c.json` — verified |
| `0x8c1570a0` | `store_r5_at_56` | `notes/verifications/0x8c1570a0.json` — verified |
| `0x8c159e64` | `store_byte_to_global` | `notes/verifications/0x8c159e64.json` — verified |
| `0x8c159e70` | `store_fr4_to_global` | `notes/verifications/0x8c159e70.json` — verified |
| `0x8c159e7c` | `copy_three_words_to_globals` | `notes/verifications/0x8c159e7c.json` — verified |
| `0x8c159fc0` | `pass` | `notes/verifications/0x8c159fc0.json` — 57 oracle steps; 57 |
| `0x8c15a068` | `pass` | `notes/verifications/0x8c15a068.json` — 39 oracle steps; 39 |
| `0x8c15a0fc` | `pass` | `notes/verifications/0x8c15a0fc.json` — 154 oracle steps; 154 |
| `0x8c15a1d8` | `empty_stub_15` | `notes/verifications/0x8c15a1d8.json` — verified |
| `0x8c15a1dc` | `pass` | `notes/verifications/0x8c15a1dc.json` — 152 oracle steps; 152 |
| `0x8c15a23c` | `find_block_by_size` | `notes/verifications/0x8c15a23c.json` — verified |
| `0x8c15a27c` | `double_indirect_global` | `notes/verifications/0x8c15a27c.json` — verified |
| `0x8c15ae48` | `FUN_8c15ae48` | `notes/verifications/0x8c15ae48.json` — ok; fpul=*(r4+12), rts delay fr0=float(fpul) |
| `0x8c15b410` | `FUN_8c15b410` | `notes/verifications/0x8c15b410.json` — ok; r0=0x8c1dcc58, rts;nop |
| `0x8c15d358` | `FUN_8c15d358` | `notes/verifications/0x8c15d358.json` — ok; r0=0x8c1dca68, rts;nop |
| `0x8c15d98c` | `FUN_8c15d98c` | `notes/verifications/0x8c15d98c.json` — ok; fpul=*(r4+12), rts delay fr0=float(fpul) |
| `0x8c15e584` | `FUN_8c15e584` | `notes/verifications/0x8c15e584.json` — ok; r0=0x8c1dcd58, rts;nop |
| `0x8c160194` | `FUN_8c160194` | `notes/verifications/0x8c160194.json` — ok; r0=0x8c1dcd10, rts;nop |
| `0x8c163440` | `FUN_8c163440` | `notes/verifications/0x8c163440.json` — ok; r4+=88, rts delay r0=*r4 |
| `0x8c163448` | `FUN_8c163448` | `notes/verifications/0x8c163448.json` — ok; r0=92, rts delay byte[r4+92]=r5 |
| `0x8c163450` | `byte_at_92` | `notes/verifications/0x8c163450.json` — verified |
| `0x8c163458` | `store_byte_indexed_global` | `notes/verifications/0x8c163458.json` — verified |
| `0x8c163464` | `load_byte_indexed_global` | `notes/verifications/0x8c163464.json` — verified |
| `0x8c164298` | `pass` | `notes/verifications/0x8c164298.json` — 11 oracle steps; 11 |
| `0x8c1642dc` | `pass` | `notes/verifications/0x8c1642dc.json` — 32 oracle steps; 32 |
| `0x8c164344` | `FUN_8c164344` | `notes/verifications/0x8c164344.json` — ok; struct init [2,1,1,1,2,0…,1,0,0,2,0], r0=r4 |
| `0x8c16436c` | `store_r5_at_20` | `notes/verifications/0x8c16436c.json` — verified |
| `0x8c164370` | `store_r5_at_28` | `notes/verifications/0x8c164370.json` — verified |
| `0x8c164374` | `store_r5_at_24` | `notes/verifications/0x8c164374.json` — verified |
| `0x8c164404` | `store_r5_at_48` | `notes/verifications/0x8c164404.json` — verified |
| `0x8c164408` | `store_indexed_at_32` | `notes/verifications/0x8c164408.json` — verified |
| `0x8c164414` | `FUN_8c164414` | `notes/verifications/0x8c164414.json` — ok; rts; delay *r4=r5 |
| `0x8c164418` | `FUN_8c164418` | `notes/verifications/0x8c164418.json` — ok; rts; delay *(r4+28)=r5 |
| `0x8c16441c` | `store_r5_at_20` | `notes/verifications/0x8c16441c.json` — verified |
| `0x8c164420` | `store_r5_at_16` | `notes/verifications/0x8c164420.json` — verified |
| `0x8c164424` | `store_r5_at_12` | `notes/verifications/0x8c164424.json` — verified |
| `0x8c164428` | `store_r5_at_8` | `notes/verifications/0x8c164428.json` — verified |
| `0x8c16442c` | `store_r5_at_4` | `notes/verifications/0x8c16442c.json` — verified |
| `0x8c164430` | `FUN_8c164430` | `notes/verifications/0x8c164430.json` — ok; r0 = *r5*8 + 0x8c1dd008 in rts delay |
| `0x8c164464` | `FUN_8c164464` | `notes/verifications/0x8c164464.json` — ok; struct init [0,4,3,1,0..,20..], r0=r4 |
| `0x8c16448c` | `empty_stub_16` | `notes/verifications/0x8c16448c.json` — verified |
| `0x8c165918` | `store_r5_at_0` | `notes/verifications/0x8c165918.json` — verified |
| `0x8c165ab0` | `add_through_pointer` | `notes/verifications/0x8c165ab0.json` — verified |
| `0x8c165abc` | `zero_field` | `notes/verifications/0x8c165abc.json` — verified |
| `0x8c165c84` | `store_r5_at_0_2` | `notes/verifications/0x8c165c84.json` — verified |
| `0x8c166514` | `pass` | `notes/verifications/0x8c166514.json` — 79 oracle steps; 79 |
| `0x8c166d8c` | `FUN_8c166d8c` | `notes/verifications/0x8c166d8c.json` — ok; like 0x8c1104c4 with r4+=92 |
| `0x8c166d98` | `FUN_8c166d98` | `notes/verifications/0x8c166d98.json` — ok; r4+=88, rts delay r0=*r4 |
| `0x8c16909c` | `pass` | `notes/verifications/0x8c16909c.json` — 385 oracle steps; 385 |
| `0x8c16c620` | `pass` | `notes/verifications/0x8c16c620.json` — 109 oracle steps; 109 |
| `0x8c16c97c` | `FUN_8c16c97c` | `notes/verifications/0x8c16c97c.json` — ok; *(r4+4)=r5, r0=1, rts delay byte[r4+8]=1 |
| `0x8c16c9b0` | `FUN_8c16c9b0` | `notes/verifications/0x8c16c9b0.json` — 87 oracle steps; stores: [r4]=0x8c1effa4, 0 at r4+4/byte r4+8, 3-word table 0x8c1d3f68 x5 at r4+12..84, pool f32 at r4+64/68/72, 1.0f at r4+88/+100, byte 0 at r4+92, 0.0f at r4+96 |
| `0x8c16ca6c` | `increment_until_nonpositive` | `notes/verifications/0x8c16ca6c.json` — verified |
| `0x8c16cbb4` | `store_through_stack_pointer` | `notes/verifications/0x8c16cbb4.json` — verified |
| `0x8c16cbbc` | `store_r5_return_self` | `notes/verifications/0x8c16cbbc.json` — verified |
| `0x8c16cbc4` | `advance_8_return_self` | `notes/verifications/0x8c16cbc4.json` — verified |
| `0x8c16d328` | `store_advanced_through_stack_pointer` | `notes/verifications/0x8c16d328.json` — verified |
| `0x8c16d330` | `copy_word_return_self` | `notes/verifications/0x8c16d330.json` — verified |
| `0x8c16d338` | `store_field8_through_stack_pointer` | `notes/verifications/0x8c16d338.json` — verified |
| `0x8c16da84` | `FUN_8c16da84` | `notes/verifications/0x8c16da84.json` — ok; struct init, pool 0x8c20ce38; byte→r6+8 then word 0→r6+8, rts delay r1→r6+8 |
| `0x8c16e438` | `float_through_two_fields` | `notes/verifications/0x8c16e438.json` — verified |
| `0x8c16ea60` | `FUN_8c16ea60` | `notes/verifications/0x8c16ea60.*.json` — both branches; if *(r4+8)&1: fr0 = *(float*)(*(r4+92)+96) in rts delay, else fr0=0 |
| `0x8c16ea8c` | `FUN_8c16ea8c` | `notes/verifications/0x8c16ea8c.json` — 30 oracle steps; stores: optional f32 fr4 at [r4+92]+40, f32 quotient at +44, byte 1/0 at +48 (rts delay slot); r4 left at r4+92 |
| `0x8c1031e0` | `apply_five_offsets` | `notes/verifications/0x8c1031e0*.json` - 45.0 path 81, 35.0 path 77 |
| `0x8c0c9d78` | `tail_apply_five` | `notes/verifications/0x8c0c9d78.json` - 84 oracle steps; r4 = [r4+4] |
| `0x8c16eadc` | `store_indexed_through_two_fields` | `notes/verifications/0x8c16eadc.json` — verified |
| `0x8c16eaec` | `FUN_8c16eaec` | `notes/verifications/0x8c16eaec.json` — 31 oracle steps; stores: fr4/fr5/fr6 at [r4+92]+60/64/68, copies at +72/76/80, OR 0x01800000 into [r4+8] |
| `0x8c16eb30` | `FUN_8c16eb30` | `notes/verifications/0x8c16eb30.json` — 31 oracle steps; stores: [r15]/[r15+4]/[r15+8] at [r4+92]+60/64/68, copies at +72/76/80, OR 0x01800000 into [r4+8] |
| `0x8c16ec20` | `set_float_if_flag_36` | `notes/verifications/0x8c16ec20.json` — verified |
| `0x8c16ec48` | `FUN_8c16ec48` | `notes/verifications/0x8c16ec48.json` — ok; r4+=92, byte[(*r4)+0x80]=r5, r0=*r4+0x84, rts delay *r0=fr4 |
| `0x8c16ed3c` | `FUN_8c16ed3c` | `notes/verifications/0x8c16ed3c.json` — 45 oracle steps; stores: clamped r5 at [r4+92]+108, clamped r6 at +112, OR 0x40000000 into [r4+8] (x2) |
| `0x8c16eda8` | `FUN_8c16eda8` | `notes/verifications/0x8c16eda8.json` — 34 oracle steps; stores: [obj+20] at obj+16 (bt.s delay slot, always), clamped r5 at obj+20, OR 0x00200000 into [r4+8] |
| `0x8c16ee30` | `set_float_if_flag_96` | `notes/verifications/0x8c16ee30.json` — verified |
| `0x8c16f658` | `lerp_ratio` | `notes/verifications/0x8c16f658.json` — verified |
| `0x8c16fe20` | `FUN_8c16fe20` | `notes/verifications/0x8c16fe20.json` — ok; fr4→r4+12, r4+=8, rts delay fr4→r4 |
| `0x8c16fe60` | `scale_and_truncate` | `notes/verifications/0x8c16fe60.json` — verified |
| `0x8c1705ca` | `FUN_8c1705ca` | `notes/verifications/0x8c1705ca.json` — 36 oracle steps; stores: one masked mov.w clear at 0x8c2b52f0/2/4/6 depending on r4 and [0x8c2b52dc]==2; r4 -=32/-48 on high paths |
| `0x8c172e10` | `invert_table_byte_if_enabled` | `notes/verifications/0x8c172e10.json` — verified |
| `0x8c172e2e` | `range_check_global` | `notes/verifications/0x8c172e2e.json` — verified |
| `0x8c1730f6` | `increment_global_n_times` | `notes/verifications/0x8c1730f6.json` — verified |
| `0x8c173282` | `FUN_8c173282` | `notes/verifications/0x8c173282.json` — ok; r3=0x8c20d450, rts delay r0=*r3 |
| `0x8c1738c6` | `table_entry_nonzero` | `notes/verifications/0x8c1738c6.json` — verified |
| `0x8c173a20` | `FUN_8c173a20` | `notes/verifications/0x8c173a20.json` — 22 oracle steps; no stores (ocbp/ocbi are cache hints); r1/r5 advanced by 32-step delay slots, r2=0xffffffe0, T=1 |
| `0x8c173a6c` | `FUN_8c173a6c` | `notes/verifications/0x8c173a6c.json` — 24 oracle steps; no stores (ocbwb cache hint); r4 mutated to (r4+r5-1)&~31, r2 advances past it, r1=0xffffffe0, T=1 |
| `0x8c1745b8` | `global_pointer` | `notes/verifications/0x8c1745b8.json` — verified |
| `0x8c1755ea` | `range_check_descriptor` | `notes/verifications/0x8c1755ea.json` — verified |
| `0x8c175c72` | `am_sound_is_playing` | `notes/verifications/0x8c175c72.json` — 9 oracle steps |
| `0x8c178288` | `FUN_8c178288` | `notes/verifications/0x8c178288.json` — ok; rts; delay r0=0 |
| `0x8c178e54` | `FUN_8c178e54` | `notes/verifications/0x8c178e54.json` — 29 oracle steps; stores: 3 u32 from [r5] at r4+0/4/8, f32 fr4-fr8 at r4+12..28, r6/r7 at r4+32/36 |
| `0x8c17a038` | `index_times_28` | `notes/verifications/0x8c17a038.json` — verified |
| `0x8c17ead0` | `store_r5_at_72` | `notes/verifications/0x8c17ead0.json` — verified |
| `0x8c17eb18` | `FUN_8c17eb18` | `notes/verifications/0x8c17eb18.json` — ok; r4+=61 (mov #61 decimal), rts delay r0=byte[r4] |
| `0x8c17eb20` | `reset_flag_1` | `notes/verifications/0x8c17eb20.json` — verified |
| `0x8c17eb38` | `reset_flag_0` | `notes/verifications/0x8c17eb38.json` — verified |
| `0x8c17fd14` | `store_through_stack_pointer_2` | `notes/verifications/0x8c17fd14.json` — verified |
| `0x8c17fd1c` | `deref_in_place` | `notes/verifications/0x8c17fd1c.json` — verified |
| `0x8c17fd28` | `store_double_deref_through_stack_pointer` | `notes/verifications/0x8c17fd28.json` — verified |
| `0x8c17fff4` | `float_at_80` | `notes/verifications/0x8c17fff4.json` — verified |
| `0x8c17fffc` | `float_at_64` | `notes/verifications/0x8c17fffc.json` — verified |
| `0x8c180004` | `float_at_48` | `notes/verifications/0x8c180004.json` — verified |
| `0x8c1816d0` | `pass` | `notes/verifications/0x8c1816d0.json` — 31 oracle steps; 31 |
| `0x8c182648` | `zero_byte_return_self` | `notes/verifications/0x8c182648.json` — verified |
| `0x8c182c2c` | `FUN_8c182c2c` | `notes/verifications/0x8c182c2c.json` — ok; r0=*r15, r4+=4, rts delay *r0=r4 |
| `0x8c182c34` | `FUN_8c182c34` | `notes/verifications/0x8c182c34.json` — ok; *r4=*r5, r0=r4, rts delay *(r4+4)=*r6 |
| `0x8c183198` | `compare_tables_at_index` | `notes/verifications/0x8c183198.json` — verified |
| `0x8c18320c` | `compare_table_prefix` | `notes/verifications/0x8c18320c.json` — verified |
| `0x8c183724` | `pass` | `notes/verifications/0x8c183724.json` — 30 oracle steps; 30 |
| `0x8c1837dc` | `FUN_8c1837dc` | `notes/verifications/0x8c1837dc.json` — ok; struct init, pool 0x8c20df70 single deref |
| `0x8c183b40` | `FUN_8c183b40` | `notes/verifications/0x8c183b40.json` — ok; 4-word copy r5→r4, 3-word r6→r4+20, last r5 word→r4+16 in rts delay |
| `0x8c184360` | `FUN_8c184360` | `notes/verifications/0x8c184360.json` — ok; fr0 chain from r4+0xdc/0xe0/0xe4 → r5/r6/r7, r4+=0xe8, rts delay fr0=[r4]→*r15 |
| `0x8c18439c` | `pass` | `notes/verifications/0x8c18439c.json` — 98 oracle steps; 98 |
| `0x8c184474` | `pass` | `notes/verifications/0x8c184474.json` — 61 oracle steps; 61 |
| `0x8c1844f0` | `FUN_8c1844f0` | `notes/verifications/0x8c1844f0.json` — ok; 6-word copy r4→r5 |
| `0x8c18598c` | `pass` | `notes/verifications/0x8c18598c.json` — 149 oracle steps; 149 |
| `0x8c185aec` | `pass` | `notes/verifications/0x8c185aec.json` — 89 oracle steps; 89 |
| `0x8c185ba4` | `pass` | `notes/verifications/0x8c185ba4.json` — 111 oracle steps; 111 |
| `0x8c18629c` | `copy_three_words_and_clear_byte` | `notes/verifications/0x8c18629c.json` — verified |
| `0x8c1863e8` | `pass` | `notes/verifications/0x8c1863e8.json` — 91 oracle steps; 91 |
| `0x8c187210` | `pass` | `notes/verifications/0x8c187210.json` — 66 oracle steps; 66 |
| `0x8c187344` | `pass` | `notes/verifications/0x8c187344.json` — 128 oracle steps; 128 |
| `0x8c187560` | `pass` | `notes/verifications/0x8c187560.json` — 124 oracle steps; 124 |
| `0x8c18787c` | `pass` | `notes/verifications/0x8c18787c.json` — 69 oracle steps; 69 |
| `0x8c18b8c8` | `pass` | `notes/verifications/0x8c18b8c8.json` — 28 oracle steps; 28 |
| `0x8c18b9c8` | `array_lookup` | `notes/verifications/0x8c18b9c8*.json` — both branches (12 and 7 oracle steps) |
| `0x8c18c058` | `FUN_8c18c058` | `notes/verifications/0x8c18c058.json` — ok; *r4=r5, r0=r4 in rts delay |
| `0x8c18c594` | `pass` | `notes/verifications/0x8c18c594.json` — 24 oracle steps; 24 |
| `0x8c18ca18` | `FUN_8c18ca18` | `notes/verifications/0x8c18ca18.json` — ok; same shape |
| `0x8c18ca20` | `advance_36_return_self` | `notes/verifications/0x8c18ca20.json` — verified |
| `0x8c18d28c` | `pass` | `notes/verifications/0x8c18d28c.json` — 60 oracle steps; 60 |
| `0x8c18d9d4` | `pass` | `notes/verifications/0x8c18d9d4.json` — 44 oracle steps; 44 |
| `0x8c18da2c` | `pass` | `notes/verifications/0x8c18da2c.json` — 58 oracle steps; 58 |
| `0x8c18db34` | `pass` | `notes/verifications/0x8c18db34.json` — 40 oracle steps; 40 |
| `0x8c18ddf0` | `pool_constant` | `notes/verifications/0x8c18ddf0.json` — verified |
| `0x8c18e010` | `pool_constant_2` | `notes/verifications/0x8c18e010.json` — verified |
| `0x8c18e364` | `FUN_8c18e364` | `notes/verifications/0x8c18e364.json` — ok; r0=0x8c1e3cd0, rts;nop |
| `0x8c18e548` | `FUN_8c18e548` | `notes/verifications/0x8c18e548.json` — ok; r0=0x8c1e3cc8, rts;nop |
| `0x8c18f780` | `FUN_8c18f780` | `notes/verifications/0x8c18f780.json` — ok; bare rts;nop |
| `0x8c18fbc8` | `zero_word_return_self` | `notes/verifications/0x8c18fbc8.json` — verified |
| `0x8c18fd54` | `pool_constant` | `notes/verifications/0x8c18fd54.json` — verified |
| `0x8c18fe2c` | `pool_constant_2` | `notes/verifications/0x8c18fe2c.json` — verified |
| `0x8c1900f4` | `pool_constant_3` | `notes/verifications/0x8c1900f4.json` — verified |
| `0x8c19033c` | `deref_pool_pointer` | `notes/verifications/0x8c19033c.json` — verified |
| `0x8c190b08` | `pass` | `notes/verifications/0x8c190b08.json` — 22 oracle steps; 22 |
| `0x8c190e1c` | `pool_constant_4` | `notes/verifications/0x8c190e1c.json` — verified |
| `0x8c192fe8` | `FUN_8c192fe8` | `notes/verifications/0x8c192fe8.json` — ok; rts; delay r0=r4 |
| `0x8c193188` | `FUN_8c193188` | `notes/verifications/0x8c193188.json` — ok; *r4=*(r4+4)=*(r4+8)=0 (last in rts delay), r1=0, r0=r4 |
| `0x8c193ac4` | `pass` | `notes/verifications/0x8c193ac4.json` — 66 oracle steps; 66 |
| `0x8c194e84` | `FUN_8c194e84` | `notes/verifications/0x8c194e84.json` — 7 oracle steps; store none; r0 = pool-selected pointer, fr0/fr1 only on byte1!=0 path; T per last tst/fcmp |
| `0x8c195610` | `FUN_8c195610` | `notes/verifications/0x8c195610.json` — 23 oracle steps; stores 4 floats at [r1..r1+12); r1+=12, r4=base+12, r0=r1+8, r3=r1+4, fr0=[base+12] |
| `0x8c195748` | `FUN_8c195748` | `notes/verifications/0x8c195748.json` — 23 oracle steps; stores 4 floats at [r1..r1+12); r1+=12, r4=base+12, r0=r1+8, r3=r1+4, fr0=[base+12] |
| `0x8c1959c8` | `flag_to_code` | `notes/verifications/0x8c1959c8.json` — verified |
| `0x8c197840` | `FUN_8c197840` | `notes/verifications/0x8c197840.json` — ok; rts; delay *(r4+56)=r5 |
| `0x8c197a5c` | `FUN_8c197a5c` | `notes/verifications/0x8c197a5c.json` — ok; list node copy + pointer fixup |
| `0x8c197a98` | `FUN_8c197a98` | `notes/verifications/0x8c197a98.json` — ok; r0=*r4, *(r4+4)=r0, rts delay *(r4+8)=r0 |
| `0x8c198ac8` | `copy_four_words_at_64` | `notes/verifications/0x8c198ac8.json` — verified |
| `0x8c198b20` | `byte_at_61` | `notes/verifications/0x8c198b20.json` — verified |
| `0x8c198b28` | `reset_and_set_state_3` | `notes/verifications/0x8c198b28.json` — verified |
| `0x8c198b3c` | `reset_and_set_state_2` | `notes/verifications/0x8c198b3c.json` — verified |
| `0x8c19a648` | `FUN_8c19a648` | `notes/verifications/0x8c19a648.json` — 393 oracle steps; stores 0xe4-byte struct copy r5 -> r4 (dwords 0/4, floats 8..32, words 36..106, dwords 108..128, floats/dwords/bytes per map); r0=r4, r1=byte[r5+0xe3], r3=r4+0xc8, r6=r5+0xd4, r7=r4+0xd4, fr0=f32[r5+0xd4], r9..r14/r15 restored, T untouched |
| `0x8c19ae00` | `find_entry_by_key` | `notes/verifications/0x8c19ae00.json` — verified |
| `0x8c19ae78` | `init_two_pointers` | `notes/verifications/0x8c19ae78.json` — verified |
| `0x8c19b01c` | `copy_12_byte_records` | `notes/verifications/0x8c19b01c.json` — verified |
| `0x8c19b530` | `FUN_8c19b530` | `notes/verifications/0x8c19b530.json` — ok; bare rts;nop |
| `0x8c19b7c8` | `FUN_8c19b7c8` | `notes/verifications/0x8c19b7c8.json` — 16 oracle steps; store fr2 at [r0] ([r15]); fr0=1.0 only on fr1>fr2 path; r0 = [r15] kept |
| `0x8c19b7f0` | `FUN_8c19b7f0` | `notes/verifications/0x8c19b7f0.json` — ok; bare rts;nop |
| `0x8c19ca78` | `FUN_8c19ca78` | `notes/verifications/0x8c19ca78.json` — ok; *(*r15)=*r4 in rts delay |
| `0x8c19c9b8` | `find_node_by_string` | `notes/verifications/0x8c19c9b8*.json` - empty 70, match 108, miss 106 |
| `0x8c19be80` | `search_then_clear` | `notes/verifications/0x8c19be80.json` - 89 oracle steps; fr4 forced to 0 |
| `0x8c19bef4` | `search_vtable_1480` | `notes/verifications/0x8c19bef4.json` - 85 steps; fr4 forced 0, vtable 0x8c1f1480, byte +24 cleared |
| `0x8c19c228` | `search_keep_fr4_r5` | `notes/verifications/0x8c19c228.json` - 97 steps; search sees r6/r7 and fr4=0, saved fr4 at +28, entry r5 at +48 |
| `0x8c19c5e4` | `search_store_fr4_twice` | `notes/verifications/0x8c19c5e4.json` - 94 steps; fr4 forced 0, saved fr4 at +24 and +28, byte +32 cleared |
| `0x8c19c664` | `search_store_fr4_fr5` | `notes/verifications/0x8c19c664.json` - 97 steps; search fr4=entry fr6, saved fr4 at +24, saved fr5 at +28 |
| `0x8c19c1e0` | `search_fr5_set_flag` | `notes/verifications/0x8c19c1e0.json` - 101 steps; search gets fr5, byte +24 is 1, fr4 at +32, r5 at +48 |
| `0x8c19c430` | `search_store_three_floats` | `notes/verifications/0x8c19c430.json` - 103 steps; fr4 forced 0, fr4/fr5/fr6 at +24/+28/+32, byte +48 cleared |
| `0x8c19c47c` | `search_fr7_flag_byte` | `notes/verifications/0x8c19c47c.json` - 107 steps; search gets fr7 via r6/r7, byte +48 is entry r5 |
| `0x8c19c4d0` | `search_fr7_three_floats` | `notes/verifications/0x8c19c4d0.json` - 103 steps; search gets fr7, same three-float stores as 0x8c19c430 |
| `0x8c19c61c` | `search_fr6_flag_byte` | `notes/verifications/0x8c19c61c.json` - 101 steps; search gets fr6, byte +32 is entry r5 |
| `0x8c19c764` | `search_fr5_store_r5` | `notes/verifications/0x8c19c764.json` - 98 steps; search gets fr5, entry r5 stored at +28, +32 cleared |
| `0x8c19c7a4` | `search_zero_store_r5` | `notes/verifications/0x8c19c7a4.json` - 98 steps; fr4 forced 0, entry r5 stored at +28 |
| `0x8c19c92c` | `search_fr5_clear_60` | `notes/verifications/0x8c19c92c.json` - 90 steps; search gets fr5, word at +60 cleared |
| `0x8c1a40b8` | `search_then_vtable_1788` | `notes/verifications/0x8c1a40b8.json` - 115 steps; calls 0x8c19c4d0 then overwrites the vtable with 0x8c1f1788 |
| `0x8c1841c4` | `release_record_array` | `notes/verifications/0x8c1841c4*.json` - null 18, empty 58, null slot 132, positive r5 frees the container 121 |
| `0x8c08c874` | `copy_record_into_ring` | `notes/verifications/0x8c08c874*.json` - refuse 25, forced 208, forward 239, skip 50, base scan 247 |
| `0x8c23a640` | `fill_entry_after_search` | `notes/verifications/0x8c23a640*.json` - miss 31 returns 0xeeee, hit 98, bit7 subtracts 24 (101) |
| `0x8c2196ce` | `scan_flag_bits` | `notes/verifications/0x8c2196ce*.json` - none 71, bit set but slot quiet 114, hit returns 1 (114) |
| `0x8c19ca80` | `FUN_8c19ca80` | `notes/verifications/0x8c19ca80.json` — ok; *(*r15)=**r4 in rts delay |
| `0x8c19ccf4` | `FUN_8c19ccf4` | `notes/verifications/0x8c19ccf4.json` — ok; *(*r15)=*r4 in rts delay |
| `0x8c19ccfc` | `FUN_8c19ccfc` | `notes/verifications/0x8c19ccfc.json` — ok; *(*r15)=**r4 in rts delay |
| `0x8c19cefc` | `deref_in_place_2` | `notes/verifications/0x8c19cefc.json` — verified |
| `0x8c19d33c` | `clear_float_and_byte` | `notes/verifications/0x8c19d33c.json` — verified |
| `0x8c19d39c` | `init_object` | `notes/verifications/0x8c19d39c.json` — verified |
| `0x8c19d550` | `deref_in_place_3` | `notes/verifications/0x8c19d550.json` — verified |
| `0x8c19f538` | `set_mode_and_clear` | `notes/verifications/0x8c19f538.json` — verified |
| `0x8c19f554` | `set_flag_byte` | `notes/verifications/0x8c19f554.json` — 11 oracle steps |
| `0x8c1a3a70` | `lookup_pair_by_index` | `notes/verifications/0x8c1a3a70.json` — verified |
| `0x8c1a3ad0` | `FUN_8c1a3ad0` | `notes/verifications/0x8c1a3ad0.json` — ok; table path when *(r4+16)>r5: *r6=*(*(r4+8)+r5*8), rts delay *r7=*(..+4); T=(r5>r0) per verifier |
| `0x8c1a3af4` | `FUN_8c1a3af4` | `notes/verifications/0x8c1a3af4.json` — ok; rts; delay r0=*(r4+16) |
| `0x8c1a3af8` | `FUN_8c1a3af8` | `notes/verifications/0x8c1a3af8.json` — ok; r0 = max(0, *(base+4+r5*8) - *(base+r5*8)) via cmp/pz, rts delay r0=r1 |
| `0x8c1a4a68` | `FUN_8c1a4a68` | `notes/verifications/0x8c1a4a68.*.json` — both branches; trailing-zero-bit count of r4, r0=r3 in rts delay |
| `0x8c1a4b1c` | `FUN_8c1a4b1c` | `notes/verifications/0x8c1a4b1c.json` — ok; bare rts;nop |
| `0x8c1a4b20` | `FUN_8c1a4b20` | `notes/verifications/0x8c1a4b20.json` — ok; r0=0x8c2111d8, rts delay r0=*r0 |
| `0x8c1a4b2c` | `store_to_global` | `notes/verifications/0x8c1a4b2c.json` — verified |
| `0x8c1a6de8` | `FUN_8c1a6de8` | `notes/verifications/0x8c1a6de8.json` — ok; struct init, pool 0x8c211470, zeros at +4..+24 |
| `0x8c1a9d00` | `FUN_8c1a9d00` | `notes/verifications/0x8c1a9d00.json` — ok; bare rts;nop |
| `0x8c1ac8d4` | `FUN_8c1ac8d4` | `notes/verifications/0x8c1ac8d4.json` — ok; zero floats r4+24..36, zeros/byte at +4/+12/+16/+40, r1=1→r4+20 in rts delay, r0=r4 |
| `0x8c1ac904` | `FUN_8c1ac904` | `notes/verifications/0x8c1ac904.json` — 1448 oracle steps; stores 2 floats per iteration at 0x8c2b7a30+4*r4'+... x64; r4 clobbered to r6+r13, r13/r14 restored, T=0 exit |
| `0x8c1aca80` | `store_byte_to_global` | `notes/verifications/0x8c1aca80.json` — verified |
| `0x8c1ade18` | `FUN_8c1ade18` | `notes/verifications/0x8c1ade18.json` — ok; *r4=*(r4+4)=*(r4+8)=0, r1=0, r0=r4 |
| `0x8c1ade6c` | `FUN_8c1ade6c` | `notes/verifications/0x8c1ade6c.json` — ok; same zero-init |
| `0x8c1aeb94` | `FUN_8c1aeb94` | `notes/verifications/0x8c1aeb94.json` — 54 oracle steps; no stores; r0 = 0 (r3&0xff==0) or last a-b, r3/r1 = last bytes, r4/r5 advanced; T=1 on zero exit else 0 |
| `0x8c1af68c` | `copy_16_bytes_in_32_byte_strides` | `notes/verifications/0x8c1af68c.json` — verified |
| `0x8c1af6b4` | `FUN_8c1af6b4` | `notes/verifications/0x8c1af6b4.json` — 104 oracle steps; stores 8 floats per 32-byte block r4->r6 while r6!=0; r4/r6 += 32 per iter, r0 = r6 final (rts delay), T=1 exit |
| `0x8c1b5cb0` | `FUN_8c1b5cb0` | `notes/verifications/0x8c1b5cb0.json` — 108 oracle steps; stores u32[sp+32..44) at r4+16..28, u32[sp+16..28) at r4+0..16, floats 0x8c1b5d90/94/98/9c at r4+32/36/40/44, u32[sp+0..16) at r4+48..64; r0=r4, r1=u32[sp+12], r2=sp+12, r3=u32[sp+8], r5=sp, r6=r4+48, r7=r4+40, r10..r14/r15 restored |
| `0x8c1b67f4` | `FUN_8c1b67f4` | `notes/verifications/0x8c1b67f4.json` — ok; rts; delay byte[r4]=0 |
| `0x8c1b69e8` | `FUN_8c1b69e8` | `notes/verifications/0x8c1b69e8.json` — ok; rts; delay *(r4+16)=r5 |
| `0x8c1b7824` | `FUN_8c1b7824` | `notes/verifications/0x8c1b7824.json` — ok; r5==0→r0=7, r5==1→r0=1, else 0 |
| `0x8c1b7840` | `FUN_8c1b7840` | `notes/verifications/0x8c1b7840.json` — ok; r5==0/1→r0=7, r5==2→r0=3, else 0 |
| `0x8c1b7998` | `pass` | `notes/verifications/0x8c1b7998.json` — 9 oracle steps; 9 |
| `0x8c1b79d0` | `pass` | `notes/verifications/0x8c1b79d0.json` — 10 oracle steps; 10 |
| `0x8c1b7d7c` | `FUN_8c1b7d7c` | `notes/verifications/0x8c1b7d7c.json` — ok; *r4=0x8c1f1b44 (overwrite), [4]=[8]=[24]=0, [12]=1, r0=r4 |
| `0x8c1b8248` | `FUN_8c1b8248` | `notes/verifications/0x8c1b8248.json` — ok; *r4=0x8c1f1b68 (overwrite), zeros +4..+28/+32, r0=r4 |
| `0x8c1bb298` | `FUN_8c1bb298` | `notes/verifications/0x8c1bb298.json` — ok; *r4=r5, r0=r4 in rts delay |
| `0x8c1bb2a0` | `FUN_8c1bb2a0` | `notes/verifications/0x8c1bb2a0.json` — ok; r4+=12, r0=r4 in rts delay |
| `0x8c1bb6a8` | `FUN_8c1bb6a8` | `notes/verifications/0x8c1bb6a8.json` — ok; struct init, pool 0x8c212298 |
| `0x8c1bb8d0` | `FUN_8c1bb8d0` | `notes/verifications/0x8c1bb8d0.json` — ok; struct init, pool 0x8c212280 |
| `0x8c1bbf74` | `pass` | `notes/verifications/0x8c1bbf74.json` — 19 oracle steps; 19 |
| `0x8c1bc370` | `pass` | `notes/verifications/0x8c1bc370.json` — 26 oracle steps; 26 |
| `0x8c1bcc04` | `sum_two_fields` | `notes/verifications/0x8c1bcc04.json` — verified |
| `0x8c1be870` | `FUN_8c1be870` | `notes/verifications/0x8c1be870.json` — ok; bare rts;nop |
| `0x8c1bfb20` | `FUN_8c1bfb20` | `notes/verifications/0x8c1bfb20.json` — ok; rts; delay r0=extu.b(byte[r4+0xe0]) |
| `0x8c1c0eac` | `FUN_8c1c0eac` | `notes/verifications/0x8c1c0eac.json` — ok; rts; delay r0=0 |
| `0x8c1c1394` | `pass` | `notes/verifications/0x8c1c1394.json` — 7 oracle steps; 7 |
| `0x8c1c13b2` | `FUN_8c1c13b2` | `notes/verifications/0x8c1c13b2.json` — ok; strchr-style scan, r0=r5/0, T=1 |
| `0x8c1c3a2c` | `FUN_8c1c3a2c` | `notes/verifications/0x8c1c3a2c.json` — ok; r0=*r4+8, r4+=12, fr0=*(r4), rts delay *r0=fr0 |
| `0x8c1cb4bc` | `FUN_8c1cb4bc` | `notes/verifications/0x8c1cb4bc.json` — ok; like 0x8c124d50: *r4=**r4, *(*r15)=old in rts delay |
| `0x8c1cb4c8` | `FUN_8c1cb4c8` | `notes/verifications/0x8c1cb4c8.json` — ok; *(*r15)=*r4 in rts delay |
| `0x8c1cb4d0` | `FUN_8c1cb4d0` | `notes/verifications/0x8c1cb4d0.json` — ok; *(*r15)=**r4 in rts delay |
| `0x8c1cbd88` | `FUN_8c1cbd88` | `notes/verifications/0x8c1cbd88.json` — ok; r5==1→r0=*(r4+44), else *(r4+48) |
| `0x8c1cbda8` | `FUN_8c1cbda8` | `notes/verifications/0x8c1cbda8.json` — ok; r5==1→r0=*(r4+32), else *(r4+36) |
| `0x8c1cc338` | `conditional_global_byte` | `notes/verifications/0x8c1cc338.json` — verified |
| `0x8c1cc35c` | `conditional_global_byte_2` | `notes/verifications/0x8c1cc35c.json` — verified |
| `0x8c1cc380` | `clear_and_copy_flag` | `notes/verifications/0x8c1cc380.json` — verified |
| `0x8c1cc3a0` | `set_two_flags` | `notes/verifications/0x8c1cc3a0.json` — verified |
| `0x8c1cc3b4` | `pass` | `notes/verifications/0x8c1cc3b4.json` — 8 oracle steps; 8 |
| `0x8c1cc608` | `store_to_global_return_self` | `notes/verifications/0x8c1cc608.json` — verified |
| `0x8c1ccc18` | `init_record_a` | `notes/verifications/0x8c1ccc18.json` — verified |
| `0x8c1ccc3c` | `init_record_b` | `notes/verifications/0x8c1ccc3c.json` — verified |
| `0x8c1ccc68` | `init_record_c` | `notes/verifications/0x8c1ccc68.json` — verified |
| `0x8c215ec8` | `pass` | `notes/verifications/0x8c215ec8.json` — 124 oracle steps; 124 |
| `0x8c215f6c` | `FUN_8c215f6c` | `notes/verifications/0x8c215f6c.json` — ok; CRC-style loop, r0=~u16(r6) |
| `0x8c215f6c` | `pass` | `notes/verifications/0x8c215f6c.json` — 77 oracle steps; 77 |
| `0x8c215fac` | `FUN_8c215fac` | `notes/verifications/0x8c215fac.*.json` — both branches; bounded memcmp, cmp/pl r6 then delay r6-=1, r0=0/1 in rts delay |
| `0x8c215fc4` | `word_copy_rounded` | `notes/verifications/0x8c215fc4.json` — verified |
| `0x8c215fe4` | `byte_copy_do_while` | `notes/verifications/0x8c215fe4.json` — verified |
| `0x8c215ff4` | `FUN_8c215ff4` | `notes/verifications/0x8c215ff4.json` — ok; fill r5 bytes 0xff from r4 (r4 advances), r5=0 |
| `0x8c216008` | `pass` | `notes/verifications/0x8c216008.json` — 285 oracle steps; 285 |
| `0x8c216040` | `FUN_8c216040` | `notes/verifications/0x8c216040.json` — 1802 oracle steps; stores 192 bytes at r5 (r1=r6<<6 iterations); r1/r2/r4/r5/r6/r7/r0 clobbered; r11/r12/r15 saved+restored; final T=0 |
| `0x8c216590` | `FUN_8c216590` | `notes/verifications/0x8c216590.json` — ok; validate + struct init via 0x8c23d418 |
| `0x8c2165cc` | `FUN_8c2165cc` | `notes/verifications/0x8c2165cc.json` — ok; validate + struct init via 0x8c23d418 (variant) |
| `0x8c2165cc` | `FUN_8c2165cc` | `notes/verifications/0x8c2165cc.json` — 26 oracle steps; stores [r6+28]=2,[r6+32]=2,[r6+40]=r4,[r6+48]=r5,[r6+36]=0; r4 zeroed; r0=-11/-9/-5/0 |
| `0x8c216a48` | `FUN_8c216a48` | `notes/verifications/0x8c216a48.json` — ok; r0=0x8c23c010, bare rts;nop |
| `0x8c216bd4` | `FUN_8c216bd4` | `notes/verifications/0x8c216bd4.json` — ok; r0=((0xf0&sext8(r4))>>a4)*10 + (15&sext8(r4)) in rts delay |
| `0x8c216d44` | `FUN_8c216d44` | `notes/verifications/0x8c216d44.json` — ok; 'a'..'z' → r4-=32, r0=r4 in rts delay |
| `0x8c216d58` | `copy_bytes_count` | `notes/verifications/0x8c216d58.json` — verified |
| `0x8c216e64` | `unpack_counted_record` | `notes/verifications/0x8c216e64*.json` - success 214, short 18, bit-clear 186 |
| `0x8c216dc0` | `streq` | `notes/verifications/0x8c216dc0.json` — 27 oracle steps |
| `0x8c216e18` | `FUN_8c216e18` | `notes/verifications/0x8c216e18.json` — ok; strstr-style count, r0=r7, T=1 |
| `0x8c217326` | `FUN_8c217326` | `notes/verifications/0x8c217326.json` — ok; struct init at r4, r0=r4 (or 0 early) |
| `0x8c217326` | `FUN_8c217326` | `notes/verifications/0x8c217326.json` — 4 oracle steps; init path stores 11 fields at r4+4..r4+82 and returns r4; rts paths return 0 |
| `0x8c217374` | `FUN_8c217374` | `notes/verifications/0x8c217374.json` — ok; *r4=0, *(r4+4)=r5, *(r4+8)=0, r0=r4 in rts delay |
| `0x8c2173d0` | `get_field_if_gated` | `notes/verifications/0x8c2173d0.json` — 11 oracle steps |
| `0x8c21741e` | `FUN_8c21741e` | `notes/verifications/0x8c21741e.json` — 21 oracle steps; bt.s delay slot loads r7=[r4+16] on all paths; success stores [r4+20]=r6, r0=0; else r0=-10/-16/-17 |
| `0x8c217948` | `word_field_80` | `notes/verifications/0x8c217948.json` — verified |
| `0x8c217aa4` | `word_field_82` | `notes/verifications/0x8c217aa4.json` — verified |
| `0x8c217e18` | `load_word_from_bytes` | `notes/verifications/0x8c217e18.json` — 19 oracle steps |
| `0x8c217e3e` | `FUN_8c217e3e` | `notes/verifications/0x8c217e3e.json` — ok; memcmp 5 bytes r4+1 vs 0x8c23c2f0, r0=1/0 |
| `0x8c217e60` | `FUN_8c217e60` | `notes/verifications/0x8c217e60.json` — ok; *(*(0x8c23d428)+48)=r4, r0=1 in rts delay |
| `0x8c218156` | `copy_field_if_version` | `notes/verifications/0x8c218156.json` — verified |
| `0x8c2192e4` | `FUN_8c2192e4` | `notes/verifications/0x8c2192e4.json` — 608 oracle steps; 24 iterations copying 24-byte records (stride 44 src, 24 dst) with field permutation; r4/r5/r6/r7/r2/r3/r0 clobbered |
| `0x8c2199b8` | `FUN_8c2199b8` | `notes/verifications/0x8c2199b8.json` — ok; r0=r4*24+0x8c2b88f8 in rts delay |
| `0x8c219c9e` | `FUN_8c219c9e` | `notes/verifications/0x8c219c9e.json` — ok; lowest-set-bit scan of *r4, r0=r7/0 |
| `0x8c21a036` | `global_word` | `notes/verifications/0x8c21a036.json` — 3 oracle steps |
| `0x8c21a04a` | `set_indexed_word_if_in_range` | `notes/verifications/0x8c21a04a.json` — 8 oracle steps |
| `0x8c21a5e0` | `FUN_8c21a5e0` | `notes/verifications/0x8c21a5e0.json` — 11264 oracle steps; stores 192 bytes near r4+0xba (net r4 = entry-6); r5 advanced 1536; r1/r2/r3/r6/r7/r0 clobbered; final T=1 |
| `0x8c21ae2c` | `set_enable_and_clear_flag` | `notes/verifications/0x8c21ae2c.json` — 11 oracle steps |
| `0x8c21aff2` | `slot_ptr_8` | `notes/verifications/0x8c21aff2*.json` - 3 specs; `*[0x8c2ba634]+4+8*r4`; T is bit 29 of r4 from `shll` |
| `0x8c219fb6` | `field_selector_dispatch` | `notes/verifications/0x8c219fb6*.json` - 4 specs; tail-jumps on r4 in {0..9 except the listed order} to the 0x8c21b0* field accessors, else -1 |
| `0x8c2199ca` | `call_slot_ptr_120` | `notes/verifications/0x8c2199ca.json` - 16 oracle steps; jsr 0x8c21afe0 |
| `0x8c2199e6` | `tail_slot_word32_eq` | `notes/verifications/0x8c2199e6*.json` - 2 specs; jmp 0x8c21b034 |
| `0x8c219c92` | `tail_slot_byte12` | `notes/verifications/0x8c219c92.json` - 19 oracle steps; jmp 0x8c21b04e |
| `0x8c219c98` | `tail_slot_byte13_set` | `notes/verifications/0x8c219c98.json` - 19 oracle steps; jmp 0x8c21b05e |
| `0x8c21c3ac` | `bra_to_tagged_write` | `notes/verifications/0x8c21c3ac.json` - 33 oracle steps; double bra to 0x8c21b8de |
| `0x8c21bb5e` | `flag_from_two_bytes` | `notes/verifications/0x8c21bb5e*.json` - 4 specs; stores 0 or 1 from bytes +7/+8, returns 0 |
| `0x8c21bf1e` | `stamp_byte_pattern` | `notes/verifications/0x8c21bf1e.json` - 340 oracle steps; zeroes 30 words then stamps +20..+51; the fill loop reloads r3 from r14 every trip |
| `0x8c21cc80` | `record_ptr_52` | `notes/verifications/0x8c21cc80*.json` - 2 specs; `0x8c2ba644+52*r4`, signed mul |
| `0x8c21ccf6` | `tail_call_slot_ptr_120` | `notes/verifications/0x8c21ccf6.json` - 21 oracle steps; jmp delay restores r15 |
| `0x8c21cf1c` | `slot_bit8` | `notes/verifications/0x8c21cf1c*.json` - 2 specs (30/31); 1 if `[slot]&0x100` |
| `0x8c21fb1a` | `publish_string` | `notes/verifications/0x8c21fb1a*.json` - 2 specs (10/49); each nonzero byte through 0x8c21f9d8 |
| `0x8c21ce48` | `tail_call_slot_bit8` | `notes/verifications/0x8c21ce48.json` - 33 oracle steps; jmp to 0x8c21cf1c |
| `0x8c21d0f2` | `copy_record_bytes` | `notes/verifications/0x8c21d0f2*.json` - 2 specs (241/283); hit copies two bytes, miss returns -2 |
| `0x8c21caf6` | `fill_or_clear_pair` | `notes/verifications/0x8c21caf6*.json` - 2 specs (47/64); field!=1 copies the stack word, field==1 runs 0x8c21cad4 |
| `0x8c21fb50` | `publish_hex_pointer` | `notes/verifications/0x8c21fb50.json` - 219 oracle steps; eight nibbles of r4, mode >= 2 so the device poll is skipped |
| `0x8c221eee` | `tail_call_reschedule` | `notes/verifications/0x8c221eee.json` - 138 oracle steps; jmp to 0x8c21d6bc |
| `0x8c222430` | `table_byte_and_slot_bit1` | `notes/verifications/0x8c222430*.json` - 3 specs (49/52); 1 only when the byte is nonzero and bit 1 is set |
| `0x8c223812` | `admit_record` | `notes/verifications/0x8c223812*.json` - null 14, zero-byte 22, slot-miss 98 |
| `0x8c225534` | `first_zero_stride32` | `notes/verifications/0x8c225534*.json` - 2 specs (15/28); index of the first zero byte, or -1 |
| `0x8c222470` | `table_byte_slot_byte12_bit1` | `notes/verifications/0x8c222470*.json` - 3 specs (49/73/77); 1 only when the byte is nonzero, slot+12 is 0, and bit 1 is set |
| `0x8c222964` | `scan_slots_0_to_7` | `notes/verifications/0x8c222964*.json` - 2 specs (476/480); bit i set when 0x8c222430(i) is nonzero |
| `0x8c2253c0` | `strings_equal_capped` | `notes/verifications/0x8c2253c0*.json` - 3 specs (39/58/89); equal capped lengths and matching bytes |
| `0x8c22299e` | `record_slot_and_byte8` | `notes/verifications/0x8c22299e*.json` - 3 specs (66/97/98); 1 when 0x8c222470 hits and byte +8 is nonzero |
| `0x8c2259a4` | `record_cursor_plus_32` | `notes/verifications/0x8c2259a4*.json` - 2 specs; `[record+40]+32*r5`, T from shll of r5*16 |
| `0x8c1aeb9e` | `strcmp_diff` | `notes/verifications/0x8c1aeb9e*.json` - 2 specs (25/41); 0 on a shared terminator, else the zero-extended difference |
| `0x8c216dc8` | `streq` | `notes/verifications/0x8c216dc8*.json` - 3 specs (9/16/23); 1 only when both strings end together |
| `0x8c215fea` | `copy_bytes` | `notes/verifications/0x8c215fea*.json` - 2 specs (5/23); count 0 still decrements r6 |
| `0x8c1aeb90` | `bra_strcmp_diff` | `notes/verifications/0x8c1aeb90.json` - 14 oracle steps; bra to 0x8c1aeb9e |
| `0x8c215fe0` | `bra_copy_bytes` | `notes/verifications/0x8c215fe0.json` - 19 oracle steps; bra to 0x8c215fea |
| `0x8c216dbc` | `bra_streq` | `notes/verifications/0x8c216dbc.json` - 11 oracle steps; bra to 0x8c216dc8 |
| `0x8c215fb8` | `bytes_equal_counted` | `notes/verifications/0x8c215fb8*.json` - 3 specs (6/9/20); 1 if the next r6 bytes match |
| `0x8c2259bc` | `store_1_at_record_plus_shift` | `notes/verifications/0x8c2259bc*.json` - 2 specs; byte 1 at record+72+(r5>>4) |
| `0x8c225a04` | `memset_record_72` | `notes/verifications/0x8c225a04.json` - 371 oracle steps; memset(r4, 0, 72) |
| `0x8c228c6e` | `flag_search_or_one` | `notes/verifications/0x8c228c6e*.json` - 3 specs (9/33/81); 0 only when the search returns -1 |
| `0x8c22c3a0` | `tail_call_fill_struct` | `notes/verifications/0x8c22c3a0.json` - 35 oracle steps; jmp to 0x8c236e60 |
| `0x8c215fa8` | `bra_bytes_equal_counted` | `notes/verifications/0x8c215fa8.json` - 15 oracle steps; bra to 0x8c215fb8 |
| `0x8c2313a0` | `clamp_and_set_bit2` | `notes/verifications/0x8c2313a0*.json` - 3 specs (22/23); clamp r4 to 0..255, bit 2 set at +4 and cleared at +0 |
| `0x8c231420` | `set_bit1_store_fr4` | `notes/verifications/0x8c231420.json` - 17 oracle steps; bit 1 at +4, cleared at +0, fr4 stored at +32 |
| `0x8c231820` | `set_bit6_store_r4` | `notes/verifications/0x8c231820.json` - 16 oracle steps; bit 6 at +4, cleared at +0, r4 stored at +44 |
| `0x8c2315e0` | `tail_slot_pair_5` | `notes/verifications/0x8c2315e0.json` - 21 oracle steps; 0x8c239820(5, r4, r5) |
| `0x8c231620` | `tail_slot_pair_0` | `notes/verifications/0x8c231620.json` - 21 oracle steps; 0x8c239820(0, r4, r5) |
| `0x8c231660` | `tail_slot_pair_8` | `notes/verifications/0x8c231660.json` - 20 oracle steps; 0x8c239820(8, r4, 0x8c2c6f5c) |
| `0x8c2317a0` | `tail_slot_pair_2` | `notes/verifications/0x8c2317a0.json` - 21 oracle steps; 0x8c239820(2, r4, r5) |
| `0x8c22c420` | `scan_kind_code` | `notes/verifications/0x8c22c420*.json` - 5 specs (15/19/20/27/70); 0x100/0x101/102/105 from the kind word, or 105 after 8 misses |
| `0x8c231b00` | `tail_slot_pair_1` | `notes/verifications/0x8c231b00.json` - 21 oracle steps; 0x8c239820(1, r4, r5) |
| `0x8c231eca` | `size_bit31_code` | `notes/verifications/0x8c231eca*.json` - 5 specs (9/14/15); 8/16/32/64 map to 1..8, else 5 |
| `0x8c216348` | `copy_60_check_tag` | `notes/verifications/0x8c216348*.json` - 2 specs (12/397); -6 if the halfword is nonzero, 0 if the tag is 0xffff |
| `0x8c231f90` | `scaled_product` | `notes/verifications/0x8c231f90*.json` - 2 specs (14/29); bit 31 clear shifts the product, bit set halves both factors first |
| `0x8c232014` | `scaled_product_plus_800` | `notes/verifications/0x8c232014*.json` - 3 specs (17/21/30); halves first, adds 0x800, +16 when both fields are 8 |
| `0x8c21a740` | `slot_kind_14` | `notes/verifications/0x8c21a740*.json` - 2 specs (286/329); 1 only when the slot word is 14 and the unpacked record matches |
| `0x8c22f514` | `zero_fill_aligned` | `notes/verifications/0x8c22f514*.json` - 4 specs (15/18/20/23); byte, half, word, or paired fmov after fschg |
| `0x8c220d32` | `any_slot128_nonzero` | `notes/verifications/0x8c220d32.json` - 26 oracle steps; -1 if 0x8c221da8(0) is nonzero |
| `0x8c23292a` | `set_first_clear_bit0` | `notes/verifications/0x8c23292a*.json` - 2 specs (12/39); stores 1 in the first clear halfword, or returns 0 |
| `0x8c2342d8` | `flag_words_from_bits` | `notes/verifications/0x8c2342d8*.json` - 4 specs (19/20); bits 1/4/5/6 select the two flag words |
| `0x8c232da0` | `pack_into_slot_20` | `notes/verifications/0x8c232da0*.json` - 2 specs (30/50); 0xeeee on a miss, else the packed pair at +20 |
| `0x8c232dde` | `pack_into_slot_24` | `notes/verifications/0x8c232dde*.json` - 2 specs (30/50); same pack, stored at +24 |
| `0x8c225ba2` | `unpack_eight_bytes` | `notes/verifications/0x8c225ba2.json` - 133 oracle steps; byte 0 times 100 plus byte 1 as a halfword, bytes 2..6 split, byte 7 indexes 0x8c23cca0 |
| `0x8c221660` | `fill_slot_or_minus1` | `notes/verifications/0x8c221660*.json` - 2 specs (28/348); -1 if the slot word is set, else fill via five callees |
| `0x8c2363a0` | `store_r4_nonzero` | `notes/verifications/0x8c2363a0*.json` - 2 specs (15/20); 0xeeee if the word is set, else stores 1 or 0 |
| `0x8c236400` | `slot_bit16` | `notes/verifications/0x8c236400*.json` - 3 specs (29/48/49); sets or clears bit 16 of slot+36 |
| `0x8c236cd0` | `unlink_node` | `notes/verifications/0x8c236cd0*.json` - 2 specs (19/20); unlinks r5 from the list at r4, head or predecessor |
| `0x8c231e32` | `min_to_code` | `notes/verifications/0x8c231e32*.json` - 4 specs (7/11/25/34); min(r4,r5) maps 1/2/4/16/32/64/0x80/0x100/0x200/0x400 to 0..2,4..10; 8 and anything else return 3 |
| `0x8c232096` | `scaled_product_28000000` | `notes/verifications/0x8c232096*.json` - 3 specs (17/21/35); bit 31 clear returns the product, the 0x28000000 constant halves it or adds 43 |
| `0x8c237932` | `clear_descriptor` | `notes/verifications/0x8c237932*.json` - 2 specs (26/63); zeros the 24-stride and 76-stride lists plus the fixed words |
| `0x8c216fca` | `find_checksum_record` | `notes/verifications/0x8c216fca*.json` - 3 specs (42/69/77); 1-based index when byte +9 matches the checksum and the string compares, else 0 |
| `0x8c17fca0` | `walk_until_self` | `notes/verifications/0x8c17fca0.json` - 42 oracle steps; exits when `[r5]` equals the captured `[[r5]]` |
| `0x8c218876` | `copy_found_record` | `notes/verifications/0x8c218876*.json` - 2 specs (58/110); -8 if the scan misses, else three fields from the matching record |
| `0x8c0ba4d4` | `store_or_clear_slot` | `notes/verifications/0x8c0ba4d4*.json` - 5 specs (18/20/25/75/80); stores a slot pointer at `0x8c27a038+24*r5+4*r6`, then clears it if the table word is nonzero |
| `0x8c231f4c` | `dispatch_scaled_product` | `notes/verifications/0x8c231f4c*.json` - 4 specs (36/36/41/87); halfword 3, bit 30, bit 15, or the plain path, result stored at +20 |
| `0x8c224be2` | `validate_record_u16` | `notes/verifications/0x8c224be2*.json` - 3 specs (22/179/302); 0xffffff01 on a failed slot test, 0xffffff03 unless 16 0x55 bytes and the table word is zero |
| `0x8c220f96` | `tail_call_validate_record` | `notes/verifications/0x8c220f96.json` - 25 oracle steps; jmp to 0x8c224be2 |
| `0x8c230282` | `copy_triples_build_packet` | `notes/verifications/0x8c230282.json` - 360 oracle steps; three pointer slots, then 0x8c2395c0 |
| `0x8c230320` | `pack_scaled_channels` | `notes/verifications/0x8c230320*.json` - small 438, wrap 446 |
| `0x8c230502` | `copy_five_floats_build_packet` | `notes/verifications/0x8c230502.json` - 359 oracle steps |
| `0x8c230dba` | `copy_five_floats_build_packet_2` | `notes/verifications/0x8c230dba.json` - 359 oracle steps; same body, different pool slot |
| `0x8c2305a6` | `copy_wide_triples_build_packet` | `notes/verifications/0x8c2305a6.json` - 396 oracle steps; also stores +16/+24/+28 |
| `0x8c230e5e` | `copy_wide_triples_build_packet_2` | `notes/verifications/0x8c230e5e.json` - 396 oracle steps; same body, different pool slot |
| `0x8c236736` | `claim_free_record` | `notes/verifications/0x8c236736*.json` - hit 75 steps, full table 59 steps |
| `0x8c23213c` | `code_plus_table` | `notes/verifications/0x8c23213c*.json` - 2 specs (44/61); adds a table word indexed by the min-code, accumulating first when bit 31 is set |
| `0x8c21b034` | `slot_word32_eq` | `notes/verifications/0x8c21b034*.json` - 2 specs (21/21); 1 iff `[slot+32]==r5` |
| `0x8c21b04e` | `slot_byte12` | `notes/verifications/0x8c21b04e.json` - 16 oracle steps; zero-extended byte 12 |
| `0x8c21b05e` | `slot_byte13_set` | `notes/verifications/0x8c21b05e.json` - 16 oracle steps; stores 1 at byte 13 in the rts delay slot |
| `0x8c21b06e` | `store_shifted_field_10c0` | `notes/verifications/0x8c21b06e.json` — 6 oracle steps |
| `0x8c21b07a` | `load_field_10c0_high_half` | `notes/verifications/0x8c21b07a.json` — 8 oracle steps |
| `0x8c21b08a` | `store_bit_field_10c4` | `notes/verifications/0x8c21b08a.json` — 7 oracle steps |
| `0x8c21b098` | `load_field_10c4` | `notes/verifications/0x8c21b098.json` — 5 oracle steps |
| `0x8c21b0a2` | `store_field_10cc` | `notes/verifications/0x8c21b0a2.json` — 5 oracle steps |
| `0x8c21b0ac` | `load_field_10cc` | `notes/verifications/0x8c21b0ac.json` — 5 oracle steps |
| `0x8c21b0b6` | `store_field_10d0` | `notes/verifications/0x8c21b0b6.json` — 5 oracle steps |
| `0x8c21b0c0` | `load_field_10d0` | `notes/verifications/0x8c21b0c0.json` — 5 oracle steps |
| `0x8c21b0e2` | `store_field_10c8` | `notes/verifications/0x8c21b0e2.json` — 5 oracle steps |
| `0x8c21b0ec` | `load_field_10c8` | `notes/verifications/0x8c21b0ec.json` — 5 oracle steps |
| `0x8c21b0f6` | `store_field_10d8` | `notes/verifications/0x8c21b0f6.json` — 5 oracle steps |
| `0x8c21b100` | `load_field_10d8` | `notes/verifications/0x8c21b100.json` — 5 oracle steps |
| `0x8c21b10a` | `store_two_fields_10e0` | `notes/verifications/0x8c21b10a.json` — 8 oracle steps |
| `0x8c21b11a` | `load_field_10e4` | `notes/verifications/0x8c21b11a.json` — 5 oracle steps |
| `0x8c21b12e` | `store_field_10dc` | `notes/verifications/0x8c21b12e.json` — 5 oracle steps |
| `0x8c21b138` | `load_field_10dc` | `notes/verifications/0x8c21b138.json` — 5 oracle steps |
| `0x8c21b142` | `store_field_fac` | `notes/verifications/0x8c21b142.json` — 5 oracle steps |
| `0x8c21b14c` | `load_field_fb0` | `notes/verifications/0x8c21b14c.json` — 5 oracle steps |
| `0x8c21b17c` | `store_field_fbc` | `notes/verifications/0x8c21b17c.json` — 5 oracle steps |
| `0x8c21b186` | `load_field_fbc` | `notes/verifications/0x8c21b186.json` — 5 oracle steps |
| `0x8c21b198` | `byteswap_words` | `notes/verifications/0x8c21b198.json` — 18 oracle steps |
| `0x8c21b418` | `pass` | `notes/verifications/0x8c21b418.json` — 44 oracle steps; 44 |
| `0x8c21b8de` | `pass` | `notes/verifications/0x8c21b8de.json` — 29 oracle steps; 29 |
| `0x8c21bba6` | `pass` | `notes/verifications/0x8c21bba6.json` — 33 oracle steps; 33 |
| `0x8c21c3e0` | `set_high_bit_on_field` | `notes/verifications/0x8c21c3e0.json` — 10 oracle steps |
| `0x8c21c3f4` | `fill_words` | `notes/verifications/0x8c21c3f4.json` — 21 oracle steps |
| `0x8c21c40a` | `mask_28_bits` | `notes/verifications/0x8c21c40a.json` — 3 oracle steps |
| `0x8c21c902` | `pass` | `notes/verifications/0x8c21c902.json` — 26 oracle steps; 26 |
| `0x8c21cad4` | `FUN_8c21cad4` | `notes/verifications/0x8c21cad4.json` — ok; r5=(r4>>10) masked/ored per bits, r0=r5 in rts delay |
| `0x8c21ccec` | `indexed_word_from_table` | `notes/verifications/0x8c21ccec.json` — 5 oracle steps |
| `0x8c21d8b0` | `set_global_or_minus_one` | `notes/verifications/0x8c21d8b0.json` — 7 oracle steps |
| `0x8c21d8c2` | `FUN_8c21d8c2` | `notes/verifications/0x8c21d8c2.json` — ok; *0x8c23d450=0, r0=0 in rts delay |
| `0x8c21d97a` | `FUN_8c21d97a` | `notes/verifications/0x8c21d97a.json` — ok; r0=r4-48 in rts delay |
| `0x8c21dab4` | `pass` | `notes/verifications/0x8c21dab4.json` — 71 oracle steps; 71 |
| `0x8c21e0c8` | `conditional_append` | `notes/verifications/0x8c21e0c8.json` — 6 oracle steps |
| `0x8c21e124` | `FUN_8c21e124` | `notes/verifications/0x8c21e124.json` — ok; r0=r5-r4 in rts delay |
| `0x8c21fa8a` | `mode_code` | `notes/verifications/0x8c21fa8a.json` — 16 oracle steps |
| `0x8c2205b0` | `FUN_8c2205b0` | `notes/verifications/0x8c2205b0.json` — ok; r4 = *r4*32 + **0x8c23d468, *(r4+28)=0, r0=0 in rts delay |
| `0x8c2218a0` | `FUN_8c2218a0` | `notes/verifications/0x8c2218a0.json` — ok; copy (r6>>2) words r5→r4, dt loop, delay r4+=4 |
| `0x8c221d54` | `fill_stride_116` | `notes/verifications/0x8c221d54.json` — 49 oracle steps |
| `0x8c221d70` | `fill_stride_120` | `notes/verifications/0x8c221d70.json` — 49 oracle steps |
| `0x8c221da8` | `index_times_128_load` | `notes/verifications/0x8c221da8.json` — 8 oracle steps |
| `0x8c221244` | `gate_then_admit` | `notes/verifications/0x8c221244*.json` - live index 25, admit 52 |
| `0x8c221db8` | `index_times_128_store` | `notes/verifications/0x8c221db8.json` — 8 oracle steps |
| `0x8c221dc8` | `fill_slot_record` | `notes/verifications/0x8c221dc8.json` — 18 oracle steps |
| `0x8c221f1c` | `FUN_8c221f1c` | `notes/verifications/0x8c221f1c.json` — ok; *0x8c23d480=0 in rts delay |
| `0x8c221f24` | `FUN_8c221f24` | `notes/verifications/0x8c221f24.json` — ok; bare rts;nop |
| `0x8c222410` | `FUN_8c222410` | `notes/verifications/0x8c222410.json` — ok; byte-compare loop 0x200, r0=r7 in rts delay |
| `0x8c222410` | `compare_512_bytes` | `notes/verifications/0x8c222410.json` — 5095 oracle steps |
| `0x8c222518` | `install_mode_handlers` | `notes/verifications/0x8c222518.json` — 18 oracle steps |
| `0x8c225392` | `FUN_8c225392` | `notes/verifications/0x8c225392.json` — ok; strlen capped 12, r0=r5, T=1 in rts delay |
| `0x8c2259d6` | `alloc_slot` | `notes/verifications/0x8c2259d6.json` — 15 oracle steps |
| `0x8c225a0c` | `FUN_8c225a0c` | `notes/verifications/0x8c225a0c.json` — ok; rts; delay r0=0 |
| `0x8c225ae4` | `index_times_10_plus_low_nibble` | `notes/verifications/0x8c225ae4.json` — 12 oracle steps |
| `0x8c226868` | `crc16` | `notes/verifications/0x8c226868.json` — 407 oracle steps |
| `0x8c2268b4` | `FUN_8c2268b4` | `notes/verifications/0x8c2268b4.json` — ok; *0x8c2c65c8=r4, r0=r4 in rts delay |
| `0x8c226de0` | `FUN_8c226de0` | `notes/verifications/0x8c226de0.json` — ok; *(0x8c2c6e2c)=30, *(0x8c2c6df4)=r4-1 (neg if <=0), r0=0 in rts delay |
| `0x8c227280` | `insert_bits_gated_a` | `notes/verifications/0x8c227280.json` — 6 oracle steps |
| `0x8c2272e0` | `FUN_8c2272e0` | `notes/verifications/0x8c2272e0.json` — ok; *(r4+4) = (r5&3)<<27 | (*(r4+4)&0xe7ffffff); r0=0 in rts delay |
| `0x8c227320` | `insert_bits_179f8` | `notes/verifications/0x8c227320.json` — 6 oracle steps |
| `0x8c227380` | `insert_bits_20000_shift29` | `notes/verifications/0x8c227380.json` — 18 oracle steps |
| `0x8c2273e0` | `insert_bits_gated_b` | `notes/verifications/0x8c2273e0.json` — 25 oracle steps |
| `0x8c227440` | `insert_bits_gated_c` | `notes/verifications/0x8c227440.json` — 6 oracle steps |
| `0x8c2274a0` | `insert_bits_gated_d` | `notes/verifications/0x8c2274a0.json` — 6 oracle steps |
| `0x8c227500` | `insert_bits_gated_e` | `notes/verifications/0x8c227500.json` — 25 oracle steps |
| `0x8c227560` | `FUN_8c227560` | `notes/verifications/0x8c227560.json` — ok; bit insert into *r4, r0=0 (or 10) in rts delay |
| `0x8c227560` | `set_bit_from_flag` | `notes/verifications/0x8c227560.json` — 16 oracle steps |
| `0x8c2275c0` | `insert_bits_gated_f` | `notes/verifications/0x8c2275c0.json` — 6 oracle steps |
| `0x8c227620` | `FUN_8c227620` | `notes/verifications/0x8c227620.json` — ok; bit insert <<3 into *r4 (mask 0xff3f), r0=0/10, T=1 |
| `0x8c227620` | `insert_bits_181ff` | `notes/verifications/0x8c227620.json` — 6 oracle steps |
| `0x8c2276c0` | `insert_bits_gated_g` | `notes/verifications/0x8c2276c0.json` — 26 oracle steps |
| `0x8c227720` | `insert_bits_gated_h` | `notes/verifications/0x8c227720.json` — 6 oracle steps |
| `0x8c227780` | `FUN_8c227780` | `notes/verifications/0x8c227780.json` — ok; validate + bit-pack, r0=10/0, T=1 |
| `0x8c227800` | `insert_bits_20000` | `notes/verifications/0x8c227800.json` — 19 oracle steps |
| `0x8c227fc0` | `FUN_8c227fc0` | `notes/verifications/0x8c227fc0.json` — ok; table scan @0x8c2c6fa0 stride 0x1c4, T=1 |
| `0x8c227fe2` | `scan_slots_for_free` | `notes/verifications/0x8c227fe2.json` — 71 oracle steps |
| `0x8c228040` | `scan_slots_for_free_2` | `notes/verifications/0x8c228040.json` — 71 oracle steps |
| `0x8c228120` | `clear_bit_and_test` | `notes/verifications/0x8c228120.json` — 23 oracle steps |
| `0x8c22817e` | `set_bit_and_bump` | `notes/verifications/0x8c22817e.json` — 23 oracle steps |
| `0x8c22828e` | `FUN_8c22828e` | `notes/verifications/0x8c22828e.json` — ok; flag bit search, r0=index/-1, T=1 |
| `0x8c22f2e0` | `FUN_8c22f2e0` | `notes/verifications/0x8c22f2e0.json` — ok; struct init @0x8c2c6600 linked into *(r4+36) |
| `0x8c22f40c` | `FUN_8c22f40c` | `notes/verifications/0x8c22f40c.json` — ok; validation, r0=0/0xeeee, T=1 |
| `0x8c22f40c` | `state_gate` | `notes/verifications/0x8c22f40c.json` — 9 oracle steps |
| `0x8c22fb00` | `FUN_8c22fb00` | `notes/verifications/0x8c22fb00.json` — ok; bit ops at 0x8c2c9d78, r4=0, r0=r4 in rts delay |
| `0x8c22fb00` | `set_slot_or_error` | `notes/verifications/0x8c22fb00.json` — 8 oracle steps |
| `0x8c2316a0` | `FUN_8c2316a0` | `notes/verifications/0x8c2316a0.json` — ok; *0x8c2c8a5c=r4; *0x8c2c8a50 |= 2; r0=0 in rts delay |
| `0x8c2316e0` | `FUN_8c2316e0` | `notes/verifications/0x8c2316e0.json` — ok; *0x8c2c8a58=*r15; *0x8c2c8a50|=1; r0=0 in rts delay |
| `0x8c2317e0` | `FUN_8c2317e0` | `notes/verifications/0x8c2317e0.json` — ok; *(0x8c2c6600+0x808+{0,8,4,12}) = r4,r6,r5,r7; r0=0 in rts delay |
| `0x8c23296c` | `FUN_8c23296c` | `notes/verifications/0x8c23296c.json` — ok; *(r5+4)=*r4; rts delay *r4=r5 |
| `0x8c232f60` | `FUN_8c232f60` | `notes/verifications/0x8c232f60.json` — ok; r5=((r4&-8)>>1)+(r4&3)+flags*0x400000, r0=r5 in rts delay |
| `0x8c233780` | `FUN_8c233780` | `notes/verifications/0x8c233780.json` — ok; *r4 &= 0xdfffffff in rts delay |
| `0x8c2337c0` | `FUN_8c2337c0` | `notes/verifications/0x8c2337c0.json` — ok; *r4 |= 0x40000000 in rts delay |
| `0x8c2364e0` | `FUN_8c2364e0` | `notes/verifications/0x8c2364e0.json` — ok; *r4 |= 0x20000000 in rts delay |
| `0x8c236520` | `FUN_8c236520` | `notes/verifications/0x8c236520.json` — ok; *r4 &= 0xbfffffff in rts delay |
| `0x8c236880` | `FUN_8c236880` | `notes/verifications/0x8c236880.json` — ok; slot search @0x8c2c8740, clear *(slot+40), r0=0 |
| `0x8c236880` | `release_slot` | `notes/verifications/0x8c236880.json` — 36 oracle steps |
| `0x8c236a44` | `FUN_8c236a44` | `notes/verifications/0x8c236a44.json` — ok; hash lookup @0x8c23d7b0, r0=r7 in rts delay |
| `0x8c236a44` | `hash_lookup` | `notes/verifications/0x8c236a44.json` — 17 oracle steps |
| `0x8c236d40` | `pack_five_fields` | `notes/verifications/0x8c236d40.json` — 24 oracle steps |
| `0x8c237240` | `size_class` | `notes/verifications/0x8c237240.json` — 19 oracle steps |
| `0x8c237600` | `FUN_8c237600` | `notes/verifications/0x8c237600.json` — ok; r4=0x8c2c8b08, rts delay r0=*(r4+4) |
| `0x8c237640` | `FUN_8c237640` | `notes/verifications/0x8c237640.json` — ok; r3=0x8c2c9db8, rts delay r0=*r3 |
| `0x8c238998` | `init_or_reset_global` | `notes/verifications/0x8c238998.json` — 6 oracle steps |
| `0x8c239020` | `FUN_8c239020` | `notes/verifications/0x8c239020.json` — ok; r3=0x8c2c9db4, rts delay r0=*r3 |
| `0x8c239158` | `FUN_8c239158` | `notes/verifications/0x8c239158.json` — ok; list search, r0=r7 (found), T=1 in rts delay |
| `0x8c2391be` | `FUN_8c2391be` | `notes/verifications/0x8c2391be.json` — ok; list node init at r4/r5; r0=0 in rts delay |
| `0x8c239860` | `find_slot_by_key` | `notes/verifications/0x8c239860.json` — 41 oracle steps |
| `0x8c23a300` | `FUN_8c23a300` | `notes/verifications/0x8c23a300.json` — ok; base-4 unpack of r4 to r5/r6/r7/stack |
| `0x8c23a300` | `extract_five_fields` | `notes/verifications/0x8c23a300.json` — 28 oracle steps |
| `0x8c23a4c0` | `FUN_8c23a4c0` | `notes/verifications/0x8c23a4c0.json` — ok; rts; delay *(r4+24)=r5 |
| `0x8c23a4e0` | `FUN_8c23a4e0` | `notes/verifications/0x8c23a4e0.json` — ok; switch(r4): store r5 at 0x8c2343a0+{16,20}, else r0=0xeeee |
| `0x8c23a4e0` | `store_by_index` | `notes/verifications/0x8c23a4e0.json` — 9 oracle steps |
| `0x8c23a540` | `FUN_8c23a540` | `notes/verifications/0x8c23a540.json` — ok; rts; delay *(r4+44)=r5 |
| `0x8c23a544` | `FUN_8c23a544` | `notes/verifications/0x8c23a544.json` — ok; rts; delay r0=*(r4+44) |
| `0x8c23a8a0` | `FUN_8c23a8a0` | `notes/verifications/0x8c23a8a0.json` — ok; bit-pack into 0x8c2c8698, r0=0 in rts delay |
| `0x8c23a8a0` | `insert_into_control_word` | `notes/verifications/0x8c23a8a0.json` — 26 oracle steps |
| `0x8c23a900` | `FUN_8c23a900` | `notes/verifications/0x8c23a900.json` — ok; struct init at r4, w+28=*(r15+4)|0x8000, w+30=0 in rts delay |
| `0x8c23a960` | `FUN_8c23a960` | `notes/verifications/0x8c23a960.json` — ok; struct init at r4 (r6,r5|0x11000000,r7,0,8,w:0x8000,w:0), r0=0 |
| `0x8c23ae00` | `unknown` | `notes/verifications/0x8c23ae00.json` — 5 oracle steps |
| `0x8c23aef8` | `FUN_8c23aef8` | `notes/verifications/0x8c23aef8.json` — ok; scatter writes via 0x8c2c86cc, r0=0 in rts delay |
| `0x8c23aef8` | `link_object` | `notes/verifications/0x8c23aef8.json` — 22 oracle steps |
| `0x8c23b0c0` | `FUN_8c23b0c0` | `notes/verifications/0x8c23b0c0.json` — ok; *(0x8c2c8a88+r4*8)=*(0x8c2c8a84); *(+4)=0; r0=0 in rts delay |
| `0x8c23b94c` | `FUN_8c23b94c` | `notes/verifications/0x8c23b94c.json` — ok; array search @0x8c23d798, clear word, T=1 |
| `0x8c23b990` | `FUN_8c23b990` | `notes/verifications/0x8c23b990.json` — ok; list insert, bare rts;nop |
| `0x8c23b9b4` | `FUN_8c23b9b4` | `notes/verifications/0x8c23b9b4.json` — ok; list insert, *(r6+8)=r5, T=1 in rts delay |
| `0x8c23b9d0` | `unlink_node` | `notes/verifications/0x8c23b9d0.json` — 18 oracle steps |
| `0x8c23bbc0` | `list_max` | `notes/verifications/0x8c23bbc0.json` — 14 oracle steps |
| `0x8c010d8c` | `classify_float_result_2` | `notes/verifications/0x8c010d8c.json` — 168 oracle steps |
| `0x8c013c14` | `guarded_float_transform` | `notes/verifications/0x8c013c14*.json` — 3 seeds (192/202/205 oracle steps) |
| `0x8c01b748` | `classify_and_dispatch` | `notes/verifications/0x8c01b748.json` — 36 oracle steps |
| `0x8c024d84` | `vector_blend_pipeline` | `notes/verifications/0x8c024d84.json` — 191 oracle steps |
| `0x8c0109e4` | `classify_float_result_3` | `notes/verifications/0x8c0109e4.json` — 209 oracle steps |
| `0x8c0113f4` | `bounded_substring_search` | `notes/verifications/0x8c0113f4.json` — 204 oracle steps |
| `0x8c011878` | `cursor_token_scan` | `notes/verifications/0x8c011878.json` — 103 oracle steps; call-crossing port (callee 0x8c011374); `bra` at 0x896 skips the `add #1,r4`, delay slot resets r4 = r13 = 0 |
| `0x8c013d1c` | `exp_kernel` | `notes/verifications/0x8c013d1c*.json` — 9 seeds (224/32/33/50/51/36/49/225/235 oracle steps); call-crossing port (callee 0x8c014810); bt.s delay slots always run (mov #8,r6, mov #1,r7, fmov write-backs); jsr delay slot fdiv fr6,fr4 feeds the callee |
| `0x8c01412c` | `sin_kernel` | `notes/verifications/0x8c01412c*.json` — 14 seeds (203/198/207/205/215/216/214/209/203/207/219/39/39/207 oracle steps); call-crossing port (callees 0x8c01471c, 0x8c011d3c, 0x8c011d1c); bt.s delay store `mov.l r0,@r15` always runs; bra delay fmov fr0,fr5 feeds finish; fneg fixups per octant + r14 sign flag |
| `0x8c014480` | `atan_kernel` | `notes/verifications/0x8c014480*.json` — 15 seeds (286/57/224/282/247/73/288/251/253/291/78/293/66/70/51 oracle steps); call-crossing port (callees 0x8c01471c, 0x8c014810, 0x8c011cf8); missed `mov r15,r4` before reduce jsr was the first failure; exposed missing rts delay slot in 0x8c011cf8 spec/port |
| `0x8c014936` | `double_normalize_store` | `notes/verifications/0x8c014936*.json` — 9 seeds (94/94/94/68/418/1268/418/94/94 oracle steps); call-crossing port (callees 0x8c012260, 0x8c014da4, 0x8c014bd4); norm loop inverted from first reading — `bt` on bit-clear rotates via 0x8c014bd4, bit-set falls through to the jsr pair once; every jsr delay slot is `lds r3,fpscr` with the 0xffe7ffff mask |
| `0x8c014ab4` | `double_round_truncate` | `notes/verifications/0x8c014ab4*.json` — 9 seeds (80/82/82/80/80/105/80/82/80 oracle steps); call-crossing port (callee 0x8c012260); exponent-nibble shift/mask path with shad -5 word rounding and the neg-path not/add bit fiddle; common tail sets double FPSCR mode and bit7 of [r15+7] on fcmp/gt |
| `0x8c0153d0` | `double_frexp_decompose` | `notes/verifications/0x8c0153d0*.json` — 8 seeds (395/395/395/395/395/395/396/395 oracle steps); call-crossing port (callees 0x8c014da4, 0x8c010e84, 0x8c014c1c); five FPSCR-masked calls; `mov.b @r14,r0` sign-extends — zero-extending it was the only failure |
| `0x8c010450` | `masked_list_sum_max` | `notes/verifications/0x8c010450*.json` — 3 seeds (67/47/52 oracle steps); raises IMASK to 15, walks circular list summing (count-1) and tracking max, both ×64; `bt.s` delay slot `add r0,r14` runs on both paths so the sum is unconditional |
| `0x8c0612f8` | `set_imask` | `notes/verifications/0x8c0612f8*.json` — 2 seeds (11/11 oracle steps); SR IMASK = r4 & 15, other bits preserved |
| `0x8c0e7218` | `set_imask_2` | `notes/verifications/0x8c0e7218.json` — 11 oracle steps; twin of 0x8c0612f8 |
| `0x8c1739f4` | `raise_imask_unless_high` | `notes/verifications/0x8c1739f4*.json` — both branches (22/15 oracle steps); raises IMASK to 15 then lowers to r4 unless old IMASK already >= r4; SR seeds use the top-level `sr` key, not `regs` |
| `0x8c21c448` | `save_imask_raise` | `notes/verifications/0x8c21c448.json` — 12 oracle steps; saves IMASK to *r4, raises to 15 |
| `0x8c21c460` | `restore_imask` | `notes/verifications/0x8c21c460.json` — 11 oracle steps; SR IMASK = *r4 & 15, companion of 0x8c21c448 |
| `0x8c21ef74` | `masked_table_zero` | `notes/verifications/0x8c21ef74.json` — 1824 oracle steps; IMASK-masked zero-fill of two RAM tables (0x8c2bc9a8+12 step 16, 0x8c2bd9a8 step 4) |
| `0x8c220678` | `masked_row_flag_set` | `notes/verifications/0x8c220678.json` — 32 oracle steps; IMASK-masked write of 1 to [[0x8c23d468]] + (*r4*32) + 28; r0 = 0 via rts delay slot |
| `0x8c221e18` | `slot_string_fill` | `notes/verifications/0x8c221e18*.json` — 2 seeds (401/209 oracle steps); zeros slot+44..57 and +60..73, copies ≤12 bytes from r5/r6 stopping before the first zero; dual cursors (peek @r4, consume @r0/@r14+) |
| `0x8c225c14` | `validate_desc_map_count` | `notes/verifications/0x8c225c14*.json` — 9 seeds (38/53/57/60/7/14/17/59/47 oracle steps); descriptor range/shape validation mapping count 0xff/0x1ff/0x3ff/0x7ff to [r4+4]; fail returns -1 via rts delay slot |
| `0x8c228f60` | `bitmask_update_unlink` | `notes/verifications/0x8c228f60*.json` — 7 seeds (57/7/27/26/25/75/39 oracle steps); gate on [r4+20]==[r4+24], three -1/limit-skippable 1<<idx OR/AND bitmask updates, node unlink via prev/next; bt.s delay slots (`mov #1,r5`, `shad`, next-pointer load) all load-bearing |
| `0x8c231720` | `quant_table_build` | `notes/verifications/0x8c231720.json` — 2322 oracle steps; builds 128-entry ftrc(t*255) table at 0x8c2c8850 from the f32 table at r4 (read descending), sets bit 2 of [0x8c2c8a50]; r4 is a caller-provided table (callers at 0x8c092716/0x8c0a8fde) |
| `0x8c232e40` | `masked_slot_init` | `notes/verifications/0x8c232e40.json` — 44 oracle steps; IMASK-masked slot init writing r5/r6/0/r4 at offsets 0/12/4/8 of slot r4 (stride 16) at 0x8c2c8b08+48 |
| `0x8c236480` | `cond_imask_set` | `notes/verifications/0x8c236480*.json` — both branches (17/7 oracle steps); when [0x8c2c8b08+4] is zero sets IMASK from [0x8c2c8b08+8] and returns 0, else returns 0xeeee; both returns via rts delay slot `mov r5,r0` |
| `0x8c236e60` | `desc_count_fill` | `notes/verifications/0x8c236e60*.json` — 2 seeds (62/32 oracle steps); fills [r4+4..28] with paired list lengths (next at +8, except +4 for the [68] list) and copied descriptor words |
| `0x8c2395c0` | `cond_packet_build` | `notes/verifications/0x8c2395c0*.json` — 4 seeds (236/232/169/236 oracle steps); word0 = ([r4+4] & 0xffc30000) \| (([r4] & 15) << 22), words copied from struct fields with bits 22/24/25 gating optional fields; [dest+4] = byte count |
| `0x8c01097c` | `atan_checked` | `notes/verifications/0x8c01097c*.json` — 5 seeds (303/74/77/83/310 oracle steps); call-crossing port (callee 0x8c014480); NaN result writes 0x44d / Inf writes 0x44c to 0x8c23d8b8, finite writes nothing |
| `0x8c010ae8` | `exp_checked` | `notes/verifications/0x8c010ae8*.json` — 5 seeds (241/62/75/242/245 oracle steps); call-crossing port (callee 0x8c013d1c); same wrapper shape as 0x8c01097c |
| `0x8c0111d0` | `sin_checked` | `notes/verifications/0x8c0111d0*.json` — 5 seeds (220/65/65/228/222 oracle steps); call-crossing port (callee 0x8c01412c); same wrapper shape as 0x8c01097c (first run failed: cloned port still called the exp callee) |
| `0x8c014664` | `atan2_kernel` | `notes/verifications/0x8c014664*.json` — 6 seeds (95/109/100/105/79/304 oracle steps); call-crossing port (callee 0x8c014480); jsr delay slot `fdiv fr3,fr4` feeds the kernel; ±π fixup from spilled input sign bits |
| `0x8c0148ec` | `dbl_log2_trunc` | `notes/verifications/0x8c0148ec*.json` — 6 seeds (240/185/236/240/240/233 oracle steps); call-crossing port (callees 0x8c014936, 0x8c014ab4); normalize → scale by 1.6505 → round → ftrc int return via rts delay slot `sts fpul,r0` |
| `0x8c017d40` | `skip_cond_unpack` | `notes/verifications/0x8c017d40*.json` — both branches (35/36 oracle steps); call-crossing port (callee 0x8c017b94); bt.s delay slot advances r4 once on both paths, bit6 set advances twice |
| `0x8c04a578` | `oneshot_list_init` | `notes/verifications/0x8c04a578*.json` — both branches (35/10 oracle steps); call-crossing port (callee 0x8c017844); zeroes node + pushes unless flag set; returns node ptr on both paths; spec trap: overlapping memory seeds apply in order, later wins |
| `0x8c04ba78` | `guarded_byte_dispatch` | `notes/verifications/0x8c04ba78*.json` — both branches (17/9 oracle steps); call-crossing port (callee 0x8c104768); loads [r4+0xa0], calls when nonzero else returns 0; spec trap: short seed silently took the null path (step count is the branch witness) |
| `0x8c059954` | `set_inner_byte4` | `notes/verifications/0x8c059954.json` — 14 oracle steps; byte [*(r4+4)+4]=1; r0=1; r1=*(r4+4) kept; r14/pr restored |
| `0x8c01092c` | `atan2_checked` | `notes/verifications/0x8c01092c.json` — 112 oracle steps; writes 0x44c/0x44d to 0x8c23d8b8 on Inf/NaN result |
| `0x8c0205e4` | `find_table_entry_by_masked_key` | `notes/verifications/0x8c0205e4.json` — 62 oracle steps; writes [entry+12] via callee 0x8c0eaef8 when empty; stack spills (filtered) |
| `0x8c051a0c` | `reassign_slot_and_relink_owner` | `notes/verifications/0x8c051a0c.json` — 46 oracle steps; mem: A[r4]=A[r5] B[r4]=B[r5] A[r5]=-1 B[r5]=0 *(chain)=r4 *(flag)=1 node zero+push on first touch; regs: r4=-1 r5/r6/r7/r13/r0/r1 clobbered r14/r15/pr restored T=last tst |
| `0x8c018e64` | `publish_block_and_clear_fields` | `notes/verifications/0x8c018e64.json` — 2009 oracle steps; mem: [0x8c23da90]=r4 (delay slot of 1st jsr); 6x memset 64B zero at r4+0x40,r4,r4+0xc0,r4+0x80,r4+0x140,r4+0x100; regs: r0=r4, r4=r4+0x100, r5=0, r6=64, r7=64, r3=0, r14/pr restored, T=1 |
| `0x8c050c5c` | `point_row_distance` | `notes/verifications/0x8c050c5c.json` — 51 oracle steps; r15/pr save-restore; stack-only writes; calls hypot 0x8c0b2c00 on mismatch paths |
| `0x8c02fbf4` | `scaled_table_interpolate` | `notes/verifications/0x8c02fbf4.json` — 149 oracle steps; fr0 result; fr1/fr2/fr5/fpul/r0/r1/r3/T clobbered; calls 0x8c010b88 |
| `0x8c05ad90` | `init_record_link_owner` | `notes/verifications/0x8c05ad90.json` — 21 oracle steps; mem: [r4]=0x8c1e9120, [r4+4]=r5, [r5+44]=r4 (+ callee 0x8c056bf8 store [r4]=0x8c1e8f88 overwritten); regs: r0=r4, r13/r14/pr restored |
| `0x8c03c8bc` | `store_param_and_fanout` | `notes/verifications/0x8c03c8bc.json` — 58 oracle steps; mem: *(r4+0xC8)=r5; gates @r4+{0xE0,0xE4,0xE8,0xF0,0xF4,0x108,0xFC}: nonzero -> stores/calls (jsr 0x8c143e88, 0x8c197840); regs r0/r1/r4/r5 clobbered, r13/r14/pr restored, T=gate7==0 |
| `0x8c051a60` | `fetch_indexed_float` | `notes/verifications/0x8c051a60.json` — 69 oracle steps; mem: 4B float to *r6 (main path only); regs: r0=ret, r1=entry-ptr, r3=idx*4, r6=r5*4 (callee), fr0=copied float; stack spills (excluded) |
| `0x8c06c8e0` | `retire_node` | `notes/verifications/0x8c06c8e0.json` — 137 oracle steps; mem: busy-list unlink (G+0xc=node->next when node->back==0, else node->back+0x44=node->next; then node->next+0x48=node->back when next!=0); free-list push (oldhead+0x48=node, node+0x48=0, node+0x44=oldhead, G+0x10=node); G+0x14-- (jsr delay slot, stored before callee); pair-table removal copies last entry 12B + last float 4B over match and count--; regs: r4=G r5=node r0/r1/r3/r6/r7/r11/r12/r13 clobbered, r14/r15/pr restored, T=0 |
| `0x8c060d48` | `vl_vertex_set_ctor` | `notes/verifications/0x8c060d48.json` — 28 oracle steps; stores 0x8c1e8f88 then 0x8c1e9198 then 0x8c1e91fc at [r4]; [r4+4]=0; [r4+8]=r7; [r4+12]=r6; returns r4 in r0 |
| `0x8c06cbec` | `object_ctor_vtable_8c1e94e8` | `notes/verifications/0x8c06cbec.json` — 33 oracle steps; stores vtable 0x8c1e94e8 at +0; 0x8c0b4ff0 zeroes +12,+16,+20; f32 0.0 at +24; f32 0.5 at +4,+8; r0=this, r1=this+8, r3=this+4, r4=this+12, fr0=0.5; r14/r15/pr restored |
| `0x8c05d378` | `init_object_with_vtbl` | `notes/verifications/0x8c05d378.json` — 31 oracle steps; mem: [r4]=0x8c1e9148, [r4+4]=0, [r4+8]=0, [r4+12]=0, [r4+16]=0xf2, [r4+20..52]=0; callee 0x8c056bf8 store [r4]=0x8c1e8f88 recorded first then overwritten; r0=r4, r1=0xf2, r3=0, r14/pr restored by stack |
| `0x8c025038` | `fill_half_angle_quad` | `notes/verifications/0x8c025038.json` — 494 oracle steps; writes four f32 to the caller-supplied out pointer (first stack arg, sp+0); reads the global quad at 0x8c1d3ec8; calls 0x8c0111d0 (sin) and 0x8c0109e4, which may record 0x44d/0x44c in the global at 0x8c23d8b8 for non-finite x/2; preserves r13, r14, fr12-fr15 and pr |
| `0x8c017d88` | `parse_tagged_field` | `notes/verifications/0x8c017d88.json` — 74 oracle steps; stores: [obj+0xb4]=tag&0x20, [obj+0xb0]=tag&0x10, [obj+0xa8]=tag&0x0f, [obj+0xac]=0 or sign_extend(byte1), [obj+0xa0]=unpacked word (via 0x8c017b94), [obj+0xa4]=0, [obj+0x18]=[obj+0x54] if tag&0x20 else [obj+0x14]; regs: r4=cursor, r5=obj+0xa0, r6=value, r7=tag&0x40, r3/r1 from unpacker, T=([obj+0xb4]==0) |
| `0x8c031e0c` | `stick_friction_decay` | `notes/verifications/0x8c031e0c.json` — 123 oracle steps; clears byte[obj+0x411]; subtracts 0.13 (plus 0.125 when slot flag byte at obj+0xc0+i*0xac is set) from float[obj+0x10] for each of 4 slots (obj+0xb0+i*0xac) whose float is >1.0; clamps float[obj+0x10] to [0,1]; sets byte[obj+0x411]=1; calls 0x8c151b1c([obj+0x24]); saves/restores r13/r14/pr, clobbers r0-r7 |
| `0x8c07c180` | `construct_object` | `notes/verifications/0x8c07c180.json` — 47 oracle steps; calls 0x8c082054 on r4+36 when exts.w(r5)!=0; [r4]=r4+36; [r4+4]=0x8c1e9774; [*[r4]]=0x8c1e9794; [*[r4]+24]=(r4+36)-*[r4]; zeroes words r4+8/+12/+16/+20/+24/+28 and byte r4+32; r0=r4 |
| `0x8c074264` | `swap_records_remap_refs` | `notes/verifications/0x8c074264.json` — 534 oracle steps; writes record array at r6: swaps 0x30-byte records r4/r5 through a stack buffer, then rewrites non-negative index slots +0x14..+0x27 of r7 records; 76-byte stack frame; T=1 only on the r4==r5 early return |
| `0x8c081ad0` | `find_byte_backward` | `notes/verifications/0x8c081ad0.json` — 61 oracle steps; stack spills only (no non-stack stores); sets sr T; leaves r0=index/-1, r1 and r3 sign-extended low bytes, r4=last probed address |
| `0x8c04f204` | `find_matching_entry_two_fields` | `notes/verifications/0x8c04f204.json` — 139 oracle steps; r0,r1,r3,r4,r5,r10-r14 clobbered; 24-byte r14/r13/r12/r11/r10/pr push-pop; pr saved/restored; 9 calls: 0x8c10e344, 0x8c10e364, 0x8c10e36c, 0x8c10e3b4, 0x8c10e3bc once each and 0x8c070b0c four times; T=0 from loop cmp/hi on exit-0 paths, T=1 from final tst on return-1 paths; no stores outside stack |
| `0x8c0820b4` | `init_rect_norm` | `notes/verifications/0x8c0820b4.json` — 83 oracle steps; writes 12 f32 at [r4]..[r4+44]; calls 0x8c056c04 (r4=base+24) which copies globals 0x8c23dd60/64 into words 6/7; clobbers r0,r1,r3,r4,r5,r6,r7,r14 and fr0,fr1,fr2; fr12..fr15,r14,pr saved/restored |
| `0x8c082178` | `init_projection_matrix` | `notes/verifications/0x8c082178.json` — 76 oracle steps; writes 48 bytes at [r4]: 0.0 at +0/+4 (the +4 store is the jsr delay slot); callee 0x8c056c04 copies the two dwords at 0x8c23dd60 to +0x18/+0x1c; then writes 1/f18 at +0x20, 1/f1c at +0x24, (1/f18)*0.0 at +0x08, (1/f1c)*0.0 at +0x14, 640*(1/f18) at +0x0c, 480*(1/f1c) at +0x10, and [+0x0c]-[+0x08] at +0x28, [+0x10]-[+0x14] at +0x2c; pushes/saves r14, r13, pr on the stack and restores them; leaves r0=param, r1=param+0x2c, r3=param+0x28, r4=param+0x14, r5=param+0x10, r6=param+0x0c, r7=param+0x08; fr0=[param+0x14], fr1=[param+0x10]-[param+0x14], fr2=0.0; no conditional branches, single code path |
| `0x8c050cd0` | `ground_plane_projection` | `notes/verifications/0x8c050cd0.json` — 104 oracle steps; stores 4B float at *r6 (len on guard arm; len*cos on sqrt arm; clamp arm stores len*cos then overwrites with len); returns fr0 (0 on guard/clamp, sqrt(len^2-v^2) on sqrt arm); fr1..fr4 clobbered; final T from last fcmp/gt; residues r0=0x8c050d8c r1=base+20 r2=frame+12 r3=base+24 r4=frame+4 r5=base+4 r6=base+12 r7=arg2+8; r13/r14/r15/pr restored |
| `0x8c01438c` | `fpu_asin` | `notes/verifications/0x8c01438c.json` — 390 oracle steps; r4 class/sign bitmap (1 NaN, 2 Inf, 4 zero, 8 negative); r5=0x8c1ccf98; r6=1; r7=4; fr3/fr2/fr0/fr1/fr15 scratch; stack pushes fr15+pr and 8-byte frame, restored; callees clobber r0-r7 and fr0-fr7/fr12-fr14; pr/r15/fr15 restored |
| `0x8c0209a0` | `compare_descriptor_records` | `notes/verifications/0x8c0209a0.json` — 397 oracle steps; pushes r14/r13/pr and restores them; calls 0x8c070ac8 three times; r0=0/1 with T=result; r1/r3/r4/r5/r6 hold path-dependent leftovers (success leaves r1=[b+8], r3=[a+8], r4/r5/r6 from callee); no non-stack memory writes |
| `0x8c03eea8` | `push_history_record` | `notes/verifications/0x8c03eea8.json` — 310 oracle steps; shifts slots 2,1,0 into 3,2,1 of a 100-byte-stride array at r4+0x40 (strcpy 0x8c017f48 plus 16-byte field at +0x50 and word at -4); inserts: strcpy(r4+0x40, r5), four stack words -> r4+0x90, r6 -> r4+0x3c, 0 -> r4+0x1cc; ends r0=0x1cc r1=0 r2=0 r3=stack[2] r4/r5 past copied NUL r6=entry sp r7=r4+0x90 r8-r14/pr restored T=1 MACH=MACL=0 |
| `0x8c048ef0` | `tick_mode_counter_and_switch` | `notes/verifications/0x8c048ef0.json` — 52 oracle steps; mem at field=r4: +0x244 counter cleared (gate fail / 1.5 fail / count<=1) or bumped by 1; switch paths copy 12B table to +0x230 and store +0x23c=4|0x200, +0x244=0, +0x248=0 (table 0x8c1f6680 when struct+0x40<0, 0x8c1f6690 otherwise); regs: r0=0|1, r1=0|1, r3=mode|0x244, r4=p|arg+108|0x200|0, r5/r6/r7 clobbered, r12/r13/r14/pr restored, fr0/fr1 written, T=last cmp |
| `0x8c024f58` | `quat_mul_half_angles` | `notes/verifications/0x8c024f58.json` — 1191 oracle steps; stores 16 bytes (four f32) at destination pointer from entry [r15]: quaternion product of quad(angle0/2) x quad(angle1/2); stack-frame traffic only otherwise; r13/r14/pr and fr12-fr15 restored; r0-r7 and fr0-fr11 clobbered; T left by second 0x8c025038 call |
| `0x8c0490f4` | `FUN_8c0490f4` | `notes/verifications/0x8c0490f4.json` — 62 oracle steps; byte flags +0x298/+0x299/+0x29a, f32 +0x290 latch/accumulate, stack frame fr15/r14/pr, calls 0x8c0326a0 (r4=[obj+0x190], fr4=+0x288/+0x28c/+0x290) and 0x8c04b360 (r4=[ptr+0x48], r5=0) |
| `0x8c013fc0` | `FUN_8c013fc0 pow kernel (double in fr5:fr4 -> fr0 = 2^(fr5*log2(|fr4|)) with pow sign/NaN rules)` | `notes/verifications/0x8c013fc0.json` — 439 oracle steps; fr0=result; clobbers r0-r7/fr0-fr7 via 0x8c013e94+0x8c013d1c+0x8c014810 calls; restores r12/r13/r14/fr14/fr15/pr; r15 nets 0; T = last cmp/tst; fpul set on truncation paths |
| `0x8c0614b8` | `init_block_and_alloc_buffers` | `notes/verifications/0x8c0614b8.json` — 216 oracle steps; mem: P+0x260/0x264/0x268=0; P+0=0x1f8 P+4=0x31 (|0x100 if [0x8c23ecb0], |0x40 if [0x8c23ecb8]); P+8=P+0x1f8 P+12=2 P+20=r5 P+24=[0x8c1f7a38] P+28=[0x8c1f7a40] P+0x1f8=P+0x200 P+0x1fc=P+0x220 P+0x260=ret1 P+32=align32(ret1) P+36=P+0x240 P+40=[0x8c1f7a30] P+0x268=B (2nd jsr delay slot, before callee) P+0x264=ret2 P+44=align32(ret2) (|0xa0000000 if [0x8c1f7a48]) P+48=B P+52=1 P+56/P+60/P+68/P+92=0 P+72..P+88=f32 [0x8c1f7a18..0x8c1f7a28] P+96/P+104=1 P+100/P+108=3; plus callee 0x8c15a068 heap writes; regs: r0=P r1=0x8c1f7a28 r3=0x8c1f7a24 r4=0x8c1f7a20 r5=0x8c1f7a1c r6=0x8c1f7a18 r7=0 r11-r14/r15/pr restored MACL=76*[0x8c1f7a40] T=0 |
| `0x8c02b8e8` | `flag_component_blend` | `notes/verifications/0x8c02b8e8.json` — 233 oracle steps; mach/macl = signed 0xac*r5 product (mul.l); r13 = macl; r12 = r4+108; r4 becomes *(u32*)0x8c2b7578 then +24/+44/+64/+104/+124; r1 = 0x100; fr13 accumulates g+84/+104/+124, fr14/fr15 hold the weighted sums; pr = 0x8c02b908/0x8c02b9b6/0x8c02b9ce/0x8c02b9de at the four jsr sites; fr13/fr14/fr15 pushed+restored; T = last flag test or fcmp/gt fr0,fr13 |
| `0x8c026b64` | `rb_tree_insert_rebalance` | `notes/verifications/0x8c026b64.json` — 70 oracle steps; calls 0x8c026ce0 (rotate left) and 0x8c026c90 (rotate right) at the jsr sites with r5=r15; recolours/relinks nodes through the +8 tagged parent|colour field; writes r0/r1/r3/r4/r5/r6 on paths that run, leaves r2 and r7-r9, restores r10-r14 and pr, T=0 on return |
| `0x8c04acf4` | `rebuild_pair_distances` | `notes/verifications/0x8c04acf4.json` — 726 oracle steps; for each i<j writes dx,dy,dx*dx+dy*dy (f32) to row_i[j][+4,+8,+0] then row_j[i]; bubble-sort-with-early-exit over each row flat u32 list: adjacent window (list[k],list[k+1]) swaps when row_i[a]>row_i[b] (r9+=4, r14=n-2); tail memsets global 0x8c23dc10 buffer (len *(0x8c23dc18)<<2) via 0x8c010f88 when non-null; leaves r0,r1,r3,r5,r6,r7 and T=1; pr/r8..r14 restored |
| `0x8c0343f8` | `FUN_8c0343f8` | `notes/verifications/0x8c0343f8.json` — 4272 oracle steps; initialises ~0x3e0-byte object: 1.0 header floats, three-word zeroes via 0x8c0b4ff0 (r4+0x24, r4+0x30), five memsets via 0x8c010f88 (0x50/32, 0x70/0x90, 0x110/0x90, 0x1a0/0xd0, 0x274/0x108), -0.5 triple at 0x3c0 and 0.5 triple at 0x3cc, 0.0 at 0x3d8/0x3dc, 1.0 at 0xd0/0xd4/0xcc/0x160/0x164/0x15c; gap at r4+0x3a4 |
| `0x8c0236a0` | `find_descriptor_two_pass` | `notes/verifications/0x8c0236a0.json` — 443 oracle steps; reads only outside the filtered stack; pass 1 returns first record with comparator 0x8c0209a0 match, byte +0x1d != 0, byte +0x1c == 0; pass 2 rescans and returns first match with +0x1d != 0 and (+0x1f == 0 or +0x1c == 0), else 0; r0 = record pointer or 0, r1/r4/r5/r6 as left by last callee, r11/r12/r13/r14/pr restored, T = last tst/cmp |
| `0x8c082ca0` | `ctor_vtable_object` | `notes/verifications/0x8c082ca0.json` — 147 oracle steps; vtable object init: +0=0x8c1e9b50, +4=f32 0, +8=u8 1, +12=param_2, +16=0x8c0822fc, +20..35=copy of 0x8c1d0f28[0..3], +36=f32 1, +40=f32 0, +44=f32 12, +48/+52=0, +56..103=12-word rect via 0x8c0820b4; r0=r1=base, r3=0x8c0822fc, callee leaves r4=base+68 r5=base+64 r6=base+92 r7=base+88, r14/pr restored, T clear |
| `0x8c0579a4` | `FUN_8c0579a4` | `notes/verifications/0x8c0579a4*.json` — 11 seeds (198/208/204/75/71/80/147/429/172/187/200 oracle steps); classifies node type bits 7..9 via 0x8c05f294; four runtime vtable calls (+0xc gate, +0x14 count, +0x38 descriptor, +0x40 close) dispatched through registry::lookup — CAVEAT: equivalence holds only while each slot holds an address that has a port; an unported slot target panics instead of running. Specs seed the slots with ported addresses |
| `0x8c042d48` | `FUN_8c042d48` | `notes/verifications/0x8c042d48.json` — 432 oracle steps; writes param+0x00..+0x17c (mixed byte/long/f32 fields); fr0=0.6 fr1=95.0 fr2=25.0; calls 0x8c01e484(param+0x174) |
| `0x8c091a80` | `zero_record_blocks` | `notes/verifications/0x8c091a80.json` — 1672 oracle steps; mem: 0xb8 bytes zeroed at r4 then 0x90 at r4+0xb8 (two calls into 0x8c010f88 memset); regs: r0=r4, r4=r4+0xb8, r5=0, r6=0x90, r7=0x90, r3=0, r14/r15/pr restored, T=1 |
| `0x8c014370` | `acos` | `notes/verifications/0x8c014370.json` — 400 oracle steps; stack pr save/restore; jsr 0x8c01438c (pr=0x8c014378); r3=0x8c1ccfc0; fr3=asin result; fr0=pi/2-asin(fr4); T left by callee |
| `0x8c0a7028` | `FUN_8c0a7028` | `notes/verifications/0x8c0a7028.json` — 37 oracle steps; clears bit1 then sets bit0 of *(u32*)(*(u32*)(param_1+0x18c)+16) via 0x8c05d274, forwarding param_2 (r6) and reloading the pointer between calls; stores happen in the callee; returns callee r0/r1/T, r4 = last reloaded pointer; saves/restores r13, r14, pr |
| `0x8c090d5c` | `copy_block_then_delay` | `notes/verifications/0x8c090d5c.json` — 246 oracle steps; mem: 0x90 bytes copied r4+0x178 -> r4+0x2c0 via memmove 0x8c010e84 (backward arm; dword/word/byte by r4&3); regs: r0=10, r1=10, r2=1, r3=first source unit (0x83828180 dword / 0xffff8180 word / 0xffffff80 byte), r4=r4+0x2c0, r6=0, r5=dst & r7=src on dword+byte but r5=src & r7=dst on word; r11-r13 keep entry values, r15/pr restored, T=0 |
| `0x8c0506e4` | `ground_plane_projection_store` | `notes/verifications/0x8c0506e4.json` — 134 oracle steps; mem: f32 at r4+0x3c = callee fr0 (sqrt(len^2-v^2), or 0 on the guard/degenerate arms); f32 at r4+0x38 = callee's *r6 output (len or len*cos) divided by f32 at *(r4+8)+0x2c, stored in pc order (+0x3c first); stack: r14/pr saves and the 4-byte local (stack window); regs: r0=base+0x2c, r1=base+0x14, r2=local address, r3=base+0x18, r4..r7 and fr2..fr4 = callee residue, fr0=divisor, fr1=quotient, r8..r13/r14/r15/pr restored, T untouched |
| `0x8c086aa4` | `FUN_8c086aa4` | `notes/verifications/0x8c086aa4.json` — 46 oracle steps; 40-byte record at r4: +0,+4,+8=0.0, +12=0.0 (via 0x8c08c05c), +16,+20,+24=-1e10, +28,+32,+36=+1e10; r0=r4 |
| `0x8c0a579c` | `FUN_8c0a579c` | `notes/verifications/0x8c0a579c.json` — 34 oracle steps; stack-only 16-byte float record {1.0,0.0,0.0,0.0}; fr0=0.0; r0=0x8c0fdc5c, r2=record+12, r4=0, r5=record; r14/pr/r15 restored |
| `0x8c0108dc` | `asin_checked` | `notes/verifications/0x8c0108dc.json` — 408 oracle steps; calls asin (0x8c01438c) on fr4; NaN result stores u32 0x44d at 0x8c23d8b8, Inf stores u32 0x44c, finite stores nothing; returns fr0 bit-exact |
| `0x8c010fa0` | `pow_checked` | `notes/verifications/0x8c010fa0.json` — 456 oracle steps; NaN result writes 0x44d to 0x8c23d8b8, Inf result writes 0x44c; finite result writes nothing; returns fr0 |
| `0x8c0b26a4` | `vlSceneEnvironment_ctor` | `notes/verifications/0x8c0b26a4.json` — 64 oracle steps; object at r4: [0]=vtable 0x8c1eaa00, [+4..+19]=4 words from 0x8c1d0f28, [+20..+31]=3 floats from 0x8c1d3f50 negated via 0x8c0b4db4, [+32] and [+36]=0.5f; returns r4 |
| `0x8c01088c` | `acos_checked` | `notes/verifications/0x8c01088c.json` — 418 oracle steps; calls 0x8c014370 (acos) on fr4; NaN fr0 stores 0x44d at 0x8c23d8b8 (Inf arm 0x44c unreachable through acos); returns fr0 |
| `0x8c0b23cc` | `resolve_state_code` | `notes/verifications/0x8c0b23cc.json` — 51 oracle steps; mem: no non-stack writes (8-byte frame copy + r14/r13/pr spills filtered); regs: r0=code (0/2/6), r1=sign-extended state byte [r4+15], r4+=8 (callee), fr1=[r5+16] and fr0=1.0 on the float arm, r13/r14/pr restored, T from the last compare |
| `0x8c0b2ee4` | `FUN_8c0b2ee4` | `notes/verifications/0x8c0b2ee4.json` — 72 oracle steps; identity 3x3 float matrix at r4+0..+32, calls 0x8c0b4ff0 to zero r4+36..+44, r0..r7 leftovers, r14/r15/pr restored |
| `0x8c0b38b8` | `mat3_apply_vec_delta` | `notes/verifications/0x8c0b38b8.json` — 100 oracle steps; writes out[0..3] f32 at *(sp) = M * (v - m[9..12]); r13/r14/r15/pr frame-restored; single code path, calls 0x8c0b4f5c |
| `0x8c0854cc` | `FUN_8c0854cc` | `notes/verifications/0x8c0854cc.json` — 94 oracle steps; calls 0x8c085684(r4=dest, r5=src, scratch ptr @r15-16) then copies the 4 normalised halfwords to *dest; r0=dest; r1=r15entry-16; r14/pr/r15 restored; T from the callee's last cmp/ge; r3..r7 keep callee clobbers |
| `0x8c084328` | `record_insert` | `notes/verifications/0x8c084328.json` — 71 oracle steps; copies 8 words from r5 to (*(u32*)(r4+24))+((r6&0xffff)<<5), sets that slot's byte 0 to 1 (table pointer re-read after the copy), and raises *(f32*)(r4+44) to |src.f16-src.f8| when larger; fr0=abs, fr1=old max, fr4=signed diff; T ends from shll r0 (always 0) |
| `0x8c085bd8` | `FUN_8c085bd8` | `notes/verifications/0x8c085bd8.json` — 173 oracle steps; writes [r4+0..12] (3 lanes) and [r4+24] *= -0.8; stack frame/vector scratch (filtered); r14/pr restored; calls 0x8c0b4f94 twice, 0x8c0b4ef0 once (fr4=0.8) |
| `0x8c08f5fc` | `array_normalize_pass` | `notes/verifications/0x8c08f5fc.json` — 202 oracle steps; 4x 3-float in-place adds via 0x8c0b4f34 per qualifying node, threshold -= [0x8c1d1cd0], [r4+64] zeroed/incremented, byte [r4+68]=1 when none processed, mach/macl=mul.l |
| `0x8c071e1c` | `FUN_8c071e1c` | `notes/verifications/0x8c071e1c.json` — 459 oracle steps; initialises caller struct at r4: two 92-byte sub-objects at r4+16 and r4+108 (via callee 0x8c152ee0), float words at r4+0xcc..0x108, r4+0x10c..0x118, r4+0x11c..0x128, r4+0x12c..0x138, r4+0x13c..0x148, header r4+0..12 = 3,2,0,0 and bytes r4+0xc8/r4+0xc9 = 0; stack scratch filtered; fr0..fr6 = 0.8,0.2,0.25,0.55,0.1,0.3,1.0; r8..r14/pr restored, leaked r0 = r4, r1 = r3 = 0x3e800000, r2 = r15_frame+44, r4 = r4+0x13c, r5 = 0x3f800000, r6 = r4+0x130, r7 = r4+0x12c; T preserved |
| `0x8c05f254` | `store_slot_and_merge_state_nibble` | `notes/verifications/0x8c05f254.json` — 75 oracle steps; stores r6 at [r4+(r5<<2)]; pushes r14/pr, r5<<=2; when the flag byte at 0x8c1f76e0 is nonzero calls 0x8c0b23cc(r6,0) and merges its 0/2/6 code into the low nibble of [r4+4] under the preserved high nibble; r0 = merged byte (or the flag byte on the skip path), r4 = code, r1/r3 clobbered on the call path, T left by the last compare; no FPU use |
| `0x8c0b4838` | `add_three_floats_at_20` | `notes/verifications/0x8c0b4838.json` — 30 oracle steps; adds floats [r4+20..r4+28] += [r5..r5+8]; clears byte at [r4]; calls 0x8c0b4f34; restores r14/pr |
| `0x8c0b4990` | `scale_vec3_then_matrix_transform` | `notes/verifications/0x8c0b4990.json` — 136 oracle steps; mem: out[0..2] (f32, caller out pointer from [entry r15]) = M(r4+4) * temp, where temp[0..2] = vec3(r5) * f32[r4+32] and temp is the 12-byte frame scratch; regs: r0=out+4 r1=temp+8 r3=r4+8 r4=r4+16 r5=temp+4 r6=out+8, r2/r7-r13 and fr12-fr15 untouched, r14/r15/pr restored, T unchanged |
| `0x8c0b4a08` | `affine_transform_scaled_vector` | `notes/verifications/0x8c0b4a08.json` — 173 oracle steps; stores 3 f32 at the caller's stack-arg pointer: dst = M*(s*v)+t, with s=f32[r4+32], M=[r4+4..r4+16] (m00,m01,m02,m10), v=[r5..r5+8], t=[r4+20..r4+28]; the two stack scratches (r15-32, r15-20) are stack-filtered; leftover regs r0=dst+4, r1=t+4, r3=scratch2+4, r4=scratch2, r5=t, r6=t+8, r7=scratch2+8, r14/r15/pr restored; fr0..fr3 from the vector add, fr4..fr11 from the matrix product, fr12..fr15 preserved |
| `0x8c0a8d48` | `FUN_8c0a8d48 (bt.s slots run even when untaken)` | `notes/verifications/0x8c0a8d48.json` — 5085 oracle steps; base+0x4c: 128-float table cleared (memset 0x8c010f88, 0x200 bytes), cubic t^3 ramp fr6->fr7 over [trunc(fr0),fr1), 1.0 tail, then 1-x inversion; latches fr0@0x38 fr1@0x3c fr6@0x40 fr7@0x44 and byte 1@0x24c; r0=0x24c r1=1 r3=base+0x3c r4=0x80 r5=r6=r7=0x200, T=0; r14/pr/r15 and fr12-fr15 preserved |
| `0x8c0b7498` | `init_buffer_span` | `notes/verifications/0x8c0b7498.json` — 36 oracle steps; calls 0x8c0b834c; *r4=0x8c1ead34 after four earlier dword stores (0x8c1eb124, 0x8c1eb0b4, 0x8c1eafd4, 0x8c1eae14); r4[4]=1 byte; r4[8]=r4[12]=r5; r4[16]=r5+r6; r0=r4; r1=0x8c1ead34; r3=r5+r6 |
| `0x8c0b633c` | `FUN_8c0b633c` | `notes/verifications/0x8c0b633c.json` — 23 oracle steps; [r4]=0x8c1eaaf0; [r4+4/8/12/20]=0; [r4+16] byte=0; returns r4 in r0, r1=0 |
| `0x8c0b92f4` | `buffer_append` | `notes/verifications/0x8c0b92f4.json` — 93 oracle steps; copy r5 bytes from r4 into [0x8c26d0d8]+[0x8c26d0d0] via 0x8c07093c, then [0x8c26d0d0] += r5 and r0=0; if [0x8c26d0c0] < used+r5 (signed cmp/ge) r0=-1 with no store; r2=sign-extended last byte when len>0, r5=r6=src+len, pr/r14 popped from stack |
| `0x8c0b9608` | `guarded_weight_sum` | `notes/verifications/0x8c0b9608.json` — 435 oracle steps; fr0 = callee weight sum or 0.0; callee writes weights at entry+52; stack pr push/pop (filtered); r4 becomes [r4+4] on the call path, untouched otherwise; r0 = callee r0 or 0; T=1 on both guard paths else callee T |
| `0x8c0b9598` | `FUN_8c0b9598` | `notes/verifications/0x8c0b9598.json` — 89 oracle steps; vtable 0x8c1eb144 @+68; [obj]=5 then arg2; ints +4..+24 zeroed; f32 +28,+32 zeroed; [obj+56]=4 (jsr delay slot); memset 6B @+60; f32 +36..+52 zeroed; fr0=+0.0; T=1; r0/r13=obj, r1=obj+40, r3=obj+44, r4=obj+48, r5=0, r6=r7=6; r13/r14 restored |
| `0x8c0b4764` | `build_transform_record` | `notes/verifications/0x8c0b4764.json` — 169 oracle steps; mem: out[0]=0 (byte only); out+4..out+16 = four floats from 0x8c0b3b3c over src[0..8]; out+20..out+28 = src+36,40,44; out+32=1.0f; out+1..3 and out+17..19 untouched; regs: r0=0 r1=out+32 r2=callee ptr r3=[src+44] r4=[src+36] r5=out+20 r6=r15 r7=out+4 r13/r14/r15/pr restored, fr0=1.0f |
| `0x8c093ba4` | `apply_mode_flag_transition` | `notes/verifications/0x8c093ba4.json` — 72 oracle steps; mem: byte[*(0x8c2ab460)+0x345]=0 on mode1/mode2 with flag4!=0 and on mode3 flag4==0 flag5!=0, =1 on mode3 flag4!=0; every store path calls 0x8c1cc380 (byte[0x8c215d80]=0, byte[0x8c2b8740]=byte[0x8c2b8738]); regs: r0=0 on modes 1-3 else param_2, r12/r13/r14/r15/pr restored, r1/r3/r4/r5/r6/r7 clobbered by callees, T=(flag4==0) on mode paths else 0 |
| `0x8c0b4a5c` | `FUN_8c0b4a5c` | `notes/verifications/0x8c0b4a5c.json` — 163 oracle steps; negates f32 [r4+4]; if [r4+32]!=1.0 overwrites [r4+32] with 1.0/[r4+32]; writes 3 f32 at [r4+20..+32] = -(M*v)/[r4+32] (M=[r4+4..+20] after the negation, v=old [r4+20..+32]); clears byte [r4]; other stores are stack callee-arg/scratch only |
| `0x8c0b48e8` | `FUN_8c0b48e8` | `notes/verifications/0x8c0b48e8.json` — 178 oracle steps; lazy one-shot init of the 0x54-byte record at r4: when flag byte +0 is 0, builds the 3x3 float matrix at +0x24 from the floats at +4 (0x8c0b2e38), copies the three words at +0x14 to +0x48 (0x8c0b2f68), then scales +0x24/+0x30/+0x3c by the scalar float at +0x20 unless that scalar is exactly 1.0 (0x8c0b4ef0 x3, fr4 reloaded each time), and sets +0 to 1; T from tst/fcmp, pr/r14/r15 restored |
| `0x8c0b4b04` | `compose_transform` | `notes/verifications/0x8c0b4b04.json` — 308 oracle steps; writes the 36-byte transform at r4 in place: byte 0 at +0, 4-float quat product at +4..+19, 3-float transformed offset at +20..+31, scale product at +32; r15/r13/r14/pr return to entry values |
| `0x8c0b9e7c` | `FUN_8c0b9e7c` | `notes/verifications/0x8c0b9e7c.json` — 109 oracle steps; zeroes 36-byte block at r4 (word +4,+8,+12,+16,+20,+24,+28,+32; byte +0); calls 0x8c0ba554 (clears 4x u32 at [0x8c27a028]/[0x8c27a098] and 4x24B structs at [0x8c27a038]); writes byte 1 to 0x8c27a0b8; returns r0=1, r1=0x8c27a0b8, r4=[0x8c27a038], T=0 |
| `0x8c0bdbd0` | `FUN_8c0bdbd0` | `notes/verifications/0x8c0bdbd0.json` — 71 oracle steps; object ctor: calls 0x8c14b1f4, stores 0x8c1eb328 at [r4] and 0 at [r4+104], returns r4 |
| `0x8c0248f4` | `init_record_if_unset` | `notes/verifications/0x8c0248f4.json` — 190 oracle steps; skip: only the entry pr push and epilogue pop (stack), r0=zero-extended flag byte, T=(flag==0); call: same stack traffic plus 0x8c0b48e8(record) sets record->flag=1 and initialises the 3x3 matrix at +0x24/+0x30/+0x3c, copying the three words at +0x14 to +0x48; callee-clobbered regs; pr/r15 restored |
| `0x8c094df8` | `validate_object_class` | `notes/verifications/0x8c094df8.json` — 225 oracle steps; reads *0x8c264da8 and [r4+0xf4]; calls 0x8c074b2c then 0x8c074ca0/0x8c074d58 gates; writes byte 1 to [r4+0x83] iff every gate passes; r0=0x83/r1=1 on store path, r14/pr restored |
| `0x8c0bdc80` | `FUN_8c0bdc80` | `notes/verifications/0x8c0bdc80.json` — 94 oracle steps; r0=1 iff callee ret != 0; r1/r2 = stack out-slot addrs; r4..r7 advanced +4 by callee; r14 scratch then restored; r15 restored; mach/macl = 108*idx; T = (callee ret lo byte == 0); stores only stack scratch |
| `0x8c0b49cc` | `FUN_8c0b49cc` | `notes/verifications/0x8c0b49cc.json` — 303 oracle steps; when record->flag (byte 0) is zero: 0x8c0b48e8 builds the 0x24 matrix from record+4, copies record+0x14 words to +0x48 and scales the matrix by the record+0x20 float unless 1.0, setting flag=1; then 0x8c0b38b8 writes M*(vec-(float*)(record+0x48)) as 3 floats to the caller's stack-arg pointer; r13/r14/pr restored, r15 balanced |
| `0x8c091b34` | `FUN_8c091b34` | `notes/verifications/0x8c091b34.json` — 7760 oracle steps; writes the 0x148-byte signed per-column-max record to the stack-passed destination pointer; r0/r1/r3-r7 and T change, r2/r8/r9-r14/pr/r15 keep entry values |
| `0x8c091c34` | `min_fields_across_records` | `notes/verifications/0x8c091c34.json` — 6783 oracle steps; 5 specs (6783 default / 2865 negcount / 5571 allbig / 5407 allsmall / 4218 one-row oracle steps): merges N records of 0x148 bytes into the stack-supplied destination pointer (entry-sp slot, left pointing at dest+0x148), signed minimum of each of the 46 first-block and 36 second-block dwords after seeding 0x1869f; 0x148-byte scratch zeroed via 0x8c091a80; regs r0=last copied dword r1=dest+0x148 r2=entry sp r3=0 r4=scratch+0x148 r5=N r6=N*0x148 r7=0x90, r8-r14/r15/pr restored, T=1 |
| `0x8c0bee14` | `advance_time_state_machine` | `notes/verifications/0x8c0bee14.json` — 105 oracle steps; mem: f32 [param+0x20] = (g[2]*fr4 + g[1]) - (h/1000 + m + s*60 + ms*3600) then [param+0x1c]=1 via 0x8c0850a4; state!=0 also calls 0x8c085148 (state==1: f32 [param+0x24]=g[2]*fr4+g[1], [param+0x1c]=2); 4 caller-stack halfwords copied to r15-16 (stack, filtered); regs: r0=1 r1=param+0x20 r3=sext(ms) r4=param+0x1c r5=sext(s) r6=sext(m) r14/r15/pr restored, T=0, fr0..fr8/fpul=m from 0x8c0850a4 |
| `0x8c0b8fd8` | `path_walk_lookup` | `notes/verifications/0x8c0b8fd8.json` — 446 oracle steps; stores two u32 through the caller's stack-arg pointer ([+0] = payload or 0, [+4] = kind or 0); 512-byte frame below r15 plus the 5 saved words (stack, filtered); r0 = payload/0, r1 = kind/0, r2 = 0x200, r3 = search base once the list compare runs else the split's last load, r4/r5/r6/r7 left by the last callee or byte compare, r8-r10 untouched, r11-r14 restored, r15 unchanged, T=1; pr = original |
| `0x8c0102cc` | `thunk_free` | `notes/verifications/0x8c0102cc.json` — 7 oracle steps; forwards r4 to free (0x8c159fc0) and returns its residue: the jmp leaves pr alone, so free rts-es to the caller; r0=0x8c159fc0 survives the null path (T=1), every other path keeps free's store set and r0-r6 residue |
| `0x8c018484` | `FUN_8c018484` | `notes/verifications/0x8c018484.json` — 4 oracle steps; one-byte store at *r4 (old|0x80), T ends set, no GPR/FPR/system-register changes; only a byte that read zero exits the spin |
| `0x8c01016c` | `thunk_FUN_8c15a1d8` | `notes/verifications/0x8c01016c.json` — 5 oracle steps; r0 = 0x8c15a1d8 from literal pool at 0x8c010174; tail-jump (jmp @r0, nop delay) to empty stub 0x8c15a1d8; no memory stores |
| `0x8c01032c` | `thunk_FUN_8c159fc0` | `notes/verifications/0x8c01032c.json` — 60 oracle steps; tail-call to free() at 0x8c159fc0; r0=0x8c159fc0 survives the null-block path |
| `0x8c01113c` | `FUN_8c01113c (LCG rand: state=state*0x41c64e6d+0x3039 at *(u32*)0x8c011160, returns (state>>16)&0x7fff)` | `notes/verifications/0x8c01113c.json` — 15 oracle steps; r0=15-bit result; r1=sext16(*(u16*)0x8c01115a)=0x3039; r2=sext16(*(u16*)0x8c01115c)=0x7fff; r3=new 32-bit state, stored to *(u32*)r4; r4=*(u32*)0x8c011160 (state pointer, 0x8c1f2eb0 in the image); MACH=high half of the signed state*multiplier product; MACL pushed at entry and restored by the rts delay slot; r15 back at its entry value; T untouched |
| `0x8c011168` | `context_save` | `notes/verifications/0x8c011168.json` — 51 oracle steps; writes the 152-byte context block at r4: +0x00 r15, +0x04 r14, +0x08 pr, +0x0c..+0x40 r13..r0, +0x44 mach, +0x48 macl, +0x4c gbr, +0x50..+0x8c fr0..fr15, +0x90 fpul, +0x94 fpscr; reloads the caller's r4 from +0x30 and returns r0=0 (rts delay slot); r1-r13/r14/r15, pr, sr/T, mach/macl/gbr/fpul/fpscr and all fr are unchanged |
| `0x8c010ddc` | `FUN_8c010ddc` | `notes/verifications/0x8c010ddc.json` — 55 oracle steps; stores incoming r5 (4 bytes) to 0x8c23d890; restores FR0-FR15, FPUL, FPSCR, GBR, MACH, MACL, r1-r15 and PR from the r4 save block; r0 = recorded r5 or 1 if it was zero; T = 1 iff recorded r5 == 0 |
| `0x8c014ee4` | `copy_pow5_entry` | `notes/verifications/0x8c014ee4.json` — 57 oracle steps; stores: [r15-4]=r4 index, [r15-8]=r5 dst into the 8-byte scratch frame; tail-calls memmove 0x8c010e84 with dst=r5, src=0x8c1cd290+(index<<3) (shll2/shll, 32-bit wrap), n=8 (forward dword copy when dst<src, backward dword when dst>=src, no copy when dst==src); regs: r0=dst, r1=3, r2=0 fwd / 1 bwd / 0x8c010e84 eq, r3=last dword, r4=dst, r5=dst (bwd) or src+8 (fwd), r6=0, r7=src (bwd) or dst+8 (fwd), r11=4, r12=2, r13 restored, r15 entry-equal, pr untouched, T=1 |
| `0x8c0184a0` | `FUN_8c0184a0` | `notes/verifications/0x8c0184a0.json` — 8 oracle steps; r0,r1,r2,r3 written; T = last cmp/eq; no memory stores |
| `0x8c016484` | `FUN_8c016484 (strcpy)` | `notes/verifications/0x8c016484.json` — 49 oracle steps; r0=original dst, r1=one past the copied zero byte; r3/r4/r5/r6 restored to entry values and r15 unchanged (entry pushes r3,r4; the aligned path tail-calls 0x8c0163e8, whose exit pops r6,r5,r4,r3 and unwinds the frame); stores are mov.b at r3+0..3 per block with add #4,r3 in the bra delay slot, or on the both-aligned path the callee's mov.l per zero-free word plus a mov.b tail up to the zero; T=1 on the byte path (exit bt always follows a matching cmp/eq #0), aligned T=0 only when the zero is the 4th byte of its word |
| `0x8c018260` | `FUN_8c018260` | `notes/verifications/0x8c018260.json` — 14 oracle steps; pushes pr at [r15-4] then restores it and r15; r3=0x8c1f2f98, r4=[0x8c1f2f98]+8, r2=0x8c018484; calls tas lock 0x8c018484: T=1 and one byte store [r4]|=0x80; no other state. |
| `0x8c018e14` | `FUN_8c018e14` | `notes/verifications/0x8c018e14.json` — 75 oracle steps; if r4 != 0 and (i16)r5 > 0, frees r4 via thunk 0x8c01032c (0x8c159fc0); pushes/restores r14 and pr; r0 = r4; T left by callee, 1 on null path, 0 on flag<=0 path |
| `0x8c018edc` | `FUN_8c018edc` | `notes/verifications/0x8c018edc.json` — 78 oracle steps; if r4 != 0: zeroes the flag word at 0x8c23da88 (bf.s delay slot, runs on both outcomes); if (i16)r5 > 0 additionally frees r4 via thunk 0x8c01032c -> 0x8c159fc0; pushes/restores r14,pr; r0 = r4 (null returns 0); r3 = 0x8c23da88 on non-null exits before the callee; T = 1 when r4 == 0, 0 when (i16)r5 <= 0, callee's T on the free path |
| `0x8c018fa4` | `thunk_FUN_8c15a27c` | `notes/verifications/0x8c018fa4.json` — 7 oracle steps; none |
| `0x8c014e4c` | `FUN_8c014e4c (load + normalize 5^i mantissa)` | `notes/verifications/0x8c014e4c.json` — 346 oracle steps; pushes r14,r13,r12,r11,pr then a 16-byte frame and restores them; the word at *r5 is first set to r4+63, then decremented by 8 per all-zero byte and by the leading-zero bit count of the first non-zero byte (one word store per step, final value r4+63-8*zero_bytes-zero_bits); 0x8c014ee4 copies the 8-byte big-endian record 5^r4 from the table at 0x8c1cd290 into frame+8; 0x8c010f88 zero-fills r6 (8 bytes); 0x8c014c1c left-shifts the (8-zero_bytes)-byte mantissa starting at frame+8+zero_bytes by zero_bits (carry r0 stays 0 because the shift is exactly the leading-zero count); 0x8c010e84 copies those bytes to r6 and leaves the routine through r0 = r6, r1-r7 = memmove residue, T = memmove exit T; r8/r9/r10 untouched; r11/r12 are popped back to their entry values, r13 = entry r5, r14 = frame+8+zero_bytes, pr = 0x8c014ec2 before the epilogue pops the entry pr |
| `0x8c018bb4` | `FUN_8c018bb4` | `notes/verifications/0x8c018bb4.json` — 90 oracle steps; free(r4) through the 0x8c01032c thunk when r4 != NULL and sext16(r5) > 0; r0 = r4 on every path; r14 and pr restored from the frame (seeds survive); r15 restored; T = callee T after a call, tst set on the NULL path, cmp/pl clear on the non-positive path |
| `0x8c018cf0` | `FUN_8c018cf0` | `notes/verifications/0x8c018cf0.json` — 61 oracle steps; free(r4) through 0x8c01032c when r4 != 0 and (s16)r5 > 0; pushes r14/pr on the stack; callee register residue, T and heap stores pass through; returns r0 = r4 |
| `0x8c018e38` | `FUN_8c018e38` | `notes/verifications/0x8c018e38.json` — 97 oracle steps; if r4!=0: store 0 at 0x8c23da90 (bf.s delay slot) and, when exts.w(r5)>0, free(r4) via 0x8c01032c; returns r4 |
| `0x8c018f60` | `FUN_8c018f60 (conditional clear + free)` | `notes/verifications/0x8c018f60.json` — 88 oracle steps; clears u32 at 0x8c23da80 whenever r4!=0, including the branch that skips the free; if sign-extended low16(r5)>0, calls free(r4) through thunk 0x8c01032c (free's r0-r6/write residue); returns r4; r14/r15/pr restored; T=1 only on the r4==0 path, T=0 on the skip path, free leaves T=0 |
| `0x8c018fb0` | `thunk_FUN_8c15a23c` | `notes/verifications/0x8c018fb0.json` — 22 oracle steps; no stores; no stack frame; pr untouched; loads 0x8c15a23c from [0x8c018fb8] and tail-jumps; only the callee residue changes r0/r1/r3/r4/r5/T, r2 and r6-r15 pass through |
| `0x8c01e460` | `FUN_8c01e460` | `notes/verifications/0x8c01e460.json` — 90 oracle steps; free(r4) through the 0x8c01032c thunk when r4 != NULL and sext16(r5) > 0; r0 = r4 on every path; r14 and pr restored from the frame; r15 restored; callee register residue, T and heap stores pass through on the call path; T = tst set on the NULL path and cmp/pl clear on the non-positive path |
| `0x8c01a6ec` | `FUN_8c01a6ec` | `notes/verifications/0x8c01a6ec.json` — 75 oracle steps; frees r4 via 0x8c01032c when r4 != 0 and (int16)r5 > 0, else returns r4 untouched (T=1 for null, T=0 for non-positive length) |
| `0x8c01e078` | `FUN_8c01e078` | `notes/verifications/0x8c01e078.json` — 75 oracle steps; r14 = r4; r0 = r14; pushes/pops r14 and pr on the stack; on (r4 != 0 && (int16)r5 > 0) calls the free thunk 0x8c01032c (free 0x8c159fc0, r4 unchanged) and inherits its r0-r6/T residue; early exits leave T from tst/cmp-pl |
| `0x8c01e6f8` | `FUN_8c01e6f8` | `notes/verifications/0x8c01e6f8.json` — 63 oracle steps; r0 = 0x8c01032c (literal at 0x8c01e700); r4 = r5 in the jmp delay slot; tail-call to thunk 0x8c01032c -> free 0x8c159fc0, so every other register, T and all heap stores are the callee residue; pr untouched |
| `0x8c018fc0` | `FUN_8c018fc0` | `notes/verifications/0x8c018fc0.json` — 88 oracle steps; stores 0 to dword 0x8c23da98 when r4 != 0; calls free (0x8c01032c) when the sign-extended low half of r5 is positive; returns r4; r3 keeps its caller value on the r4 == 0 exit |
| `0x8c026e70` | `FUN_8c026e70` | `notes/verifications/0x8c026e70.json` — 6 oracle steps; tail-calls 0x8c050c58([r4+0x2c],1); writes u32 1 at [r4+0x20] of loaded field; r0=0x8c050c58, r4=field, r5=1 |
| `0x8c01e808` | `FUN_8c01e808` | `notes/verifications/0x8c01e808.json` — 8 oracle steps; if r5 != 0: free([r5+12]) through the 0x8c01032c thunk (r4 = [old r5+12] in the jsr delay slot; r14 = r5+4 is tested after the call); r14/pr saved in an 8-byte frame at r15-4/r15-8 and both restored, r15 entry-equal; r0 untouched on the r5 == 0 path, else the callee residue (0x8c159fc0 when free early-exits); T = 1 on the null path, else (r5+4 == 0) |
| `0x8c01a0d4` | `FUN_8c01a0d4` | `notes/verifications/0x8c01a0d4.json` — 10 oracle steps; recursive integer ladder: pushes r14/pr in an 8-byte frame and restores both (r15 entry-equal); n<=0 -> r0=0 with r1/r3 untouched; n==1 -> r0=1, r1=-1; n==2 -> r0=3, r1=-2, r3=0; n==3 -> r0=6, r1=0, r3=1; n>=4 -> self-call via the 0x8c01a13c literal with r4=n-4, then r0=4n-6+f(n-4), r1=n-3+f(n-4), r3=2n-5+f(n-4), r4 left at the innermost frame's n-4k residue; r14=n; r2/r5-r13 untouched; T=0 on every exit; no stores outside the stack frame |
| `0x8c0192d0` | `FUN_8c0192d0` | `notes/verifications/0x8c0192d0.json` — 78 oracle steps; *0x8c23daa0 = 0 when r4 != 0 (bf.s delay slot, both paths); free(r4) via 0x8c01032c when exts.w(r5) > 0; returns r4 |
| `0x8c0112e0` | `FUN_8c0112e0 (strcpy thunk)` | `notes/verifications/0x8c0112e0.json` — 35 oracle steps; pushes pr, then r1=r5, r0=r4 and calls 0x8c016484 (r0=dst, r1=src); pops its own pr and rts. r0=dst, r1=one past the copied NUL, r3=literal 0x8c016484 (callee restores it), r4/r5/r6 and r15 keep entry values, T from the copy's last cmp/eq |
| `0x8c026a7c` | `thunk_FUN_8c0112f4` | `notes/verifications/0x8c026a7c.json` — 30 oracle steps; tail-jump to strlen 0x8c0112f4 (pr untouched); residue is strlen's: r0=r5=len, r4 past the NUL, r3=0, T=1 |
| `0x8c020748` | `FUN_8c020748` | `notes/verifications/0x8c020748.json` — 87 oracle steps; pushes pr; reads descriptor {count @+4, ptr @+8}; null ptr skips the call; otherwise r1 = ptr + (count<<4) (32-bit wrap), r3 advances by 16 while r1 > r3, then jsr 0x8c01032c with r4=ptr; final r0-r6 and T are free() residue (T=1 only on null path); stack push/pop cancels |
| `0x8c02b698` | `FUN_8c02b698` | `notes/verifications/0x8c02b698.json` — 82 oracle steps; e = r4 + 0xac*r5 + 88; mul.l sets mach:macl = (i32)0xac*(i32)r5 and r5 = macl; the bf.s delay slot loads r3 = *(u32*)0x8c2b7578 on every path; r1 = fl = *(u32*)(e+20); fr0 = 1.0; fl==0 -> r0=68, r3=global, r4=e+76, T=1, writes 0 at e+68 and 1.0f at e+76; fl&8 -> r3=global+52, fr0=*(f32*)(global+52), r0=8, T=0, r4=e, no writes; fl&4 -> r3=global+72, fr0=*(f32*)(global+72), r0=4, T=0, r4=e, no writes; otherwise strict max of global+92, +132, +112, +132, +112 into fr0 (bits 0x10,0x40,0x20,0x100,0x200), r5=0x200, r3=global or global+112 when 0x200, r0=68, r4=e+76, writes fl at e+68 and fr0 at e+76, T = final 0x200 tst (1) or its fcmp/gt; fr1 keeps the last field loaded; bits 0x80 etc. are ignored |
| `0x8c01efb8` | `FUN_8c01efb8` | `notes/verifications/0x8c01efb8.json` — 7 oracle steps; pushes pr to [r15-4] and restores it (r15 entry-equal); r5=[r4+8]; if r5!=0 computes r1=0x104*[r4+4]+r5 (u32 wrap), sets T=(r1>r5), always puts r5 in r3 via the bf.s delay slot, and when T iterates r3+=0x104 while r1>r3; then jsr 0x8c01032c (delay slot r4=r5) passes through free's full residue over r0-r6/T and its heap stores (insert seed: r0=4, r1=0x74616964, r3=0x8c2b1ef8, r4=0x8c200ff8, r5=1, r6=0x8c210000, T=0); r2 and r7-r14 pass through; on r5==0 no call, T=1 from tst, r0/r1/r3 keep their entry values |
| `0x8c02ba34` | `FUN_8c02ba34 (record flag-word field max, returned in fr0)` | `notes/verifications/0x8c02ba34.json` — 14 oracle steps; no stores; flags = u32 at r4+108+r5*172 (mul.l by the pool constant 172 leaves MACH/MACL = high/low of the signed product and r5 = MACL); r6 = *(u32*)0x8c2b7578, advanced by 16/36/56/96 on the matching flag bits; fr0 = the largest of [r6+16] (flags 0), [r6+36] (bit 8), [r6+56] (bit 4), [r6+76] (bit 16), [r6+116] (bit 64), [r6+96] (bit 32), [r6+116] (bit 0x100) and [r6+96] (bit 0x200); fcmp/gt is strict so NaN and equal never win and leave T=0; fr1 = the last compared field (seeded value if no comparison arm runs); r0 = index/8/4 on the early exits, otherwise the last field address (r6+76/116/96) or 0x200; r1 = 0x100 once the masks past bit 16 run; r3 = the original flag word until the final and #0x200 reduces it to 0 or 0x200; r4 += 108; r5 = MACL; T = last tst/fcmp/gt |
| `0x8c02b82c` | `FUN_8c02b82c` | `notes/verifications/0x8c02b82c.json` — 18 oracle steps; writes flag word to P+0x44 and fr0 cache to P+0x50 (P = obj + 0xac*index + 0x58); signed mul.l sets mach/macl and r5 |
| `0x8c02b77c` | `FUN_8c02b77c` | `notes/verifications/0x8c02b77c.json` — 67 oracle steps; fr0 = scale from flags; writes u32 flags at base+0x9c, f32 scale at base+0xa0; T per path |
| `0x8c018bd8` | `FUN_8c018bd8` | `notes/verifications/0x8c018bd8.json` — 194 oracle steps; releases one reference: r4 == 0 returns r4 with T=1 and no store; otherwise r1 = [r4] (object), refcount = [r1+8] lazily set to 1 when 0 (extra store, then the decremented value is stored on top, two stores), new count = refcount-1 stored at [r1+8]; bf.s on the new count clears the handle ([r4]=0) when it is nonzero, else [r4] keeps the object and the destroy block runs: free([obj+12]) through 0x8c01032c (jsr delay r4=[obj+12]), r12=obj+4, 0x8c018bb4(obj+4, 0) (r5=0 so the guard declines; its own r14/pr frame push and pop cancel), then free([r4]) = free(obj); exts.w(r5) > 0 calls free(r4) (the handle) last; epilogue pops pr/r12/r13/r14 from the 16-byte frame, so r15 is entry-equal, r12/r13/r14/pr keep their seeds and r0 = r4 on every path; T = 1 on the r4 == 0 exit, otherwise free()'s final cmp/hi residue when the handle is freed, else cmp/pl on sext16(r5) |
| `0x8c02c488` | `FUN_8c02c488 (f32 at r4 + 0xec + 0xac*r5)` | `notes/verifications/0x8c02c488.json` — 8 oracle steps; r0 = 0xec; mul.l overwrites mach/macl with the signed 0xac*r5 product (r5 = macl); r4 = r4 + 0xec + macl (wrapping); fr0 = f32[r4]; rts delay slot; no stores, sr/pr untouched |
| `0x8c051868` | `FUN_8c051868 (tail-call 0x8c02baf8 on [r4+0x190])` | `notes/verifications/0x8c051868.json` — 53 oracle steps; r0 = 0x190 then 0x8c02baf8; r4 = mem[r4+0x190] then callee residue (r4 = base+0x4ac); jmp leaves pr; nop delay slot; all stores/reg residue from 0x8c02baf8; sr/pr untouched by the thunk itself |
| `0x8c021064` | `FUN_8c021064 (recursive tree free)` | `notes/verifications/0x8c021064.json` — 703 oracle steps; frees node->[0x18] through the 0x8c01032c thunk (free at 0x8c159fc0) and zeroes it, for the node, its +0x0c list, each list node's +0x0c list, and (via a self jsr to 0x8c021064) every node in that third-level list's +0x0c list; pushes/restores r14,r13,r12,pr; r0=0 and T=1 on every exit; r1..r7 keep the last free callee's residue |
| `0x8c056660` | `FUN_8c056660` | `notes/verifications/0x8c056660.json` — 90 oracle steps; free(r4) via thunk 0x8c01032c (0x8c159fc0) when r4 != 0 and (i16)r5 > 0; pushes r14/pr on the stack and restores them; r0 = r4 on every path; r14/r15/pr restored; T = 1 on the null path, 0 when (i16)r5 <= 0, callee's T on the free path |
| `0x8c043a68` | `FUN_8c043a68 (free [r4+8] if nonzero)` | `notes/verifications/0x8c043a68.json` — 56 oracle steps; r15 -= 4, [r15] = pr; r4 = [entry r4+8]; T = (r4 == 0). Null exit: T stays 1, r0 and the remaining regs untouched, pr popped, r15 restored. Call exit: r0 = literal 0x8c01032c, pr = 0x8c043a76, 0x8c01032c (-> free 0x8c159fc0) runs with r4 as the block pointer; its residue (r0/r1/r3/r4/r5/r6/T and heap stores) survives the pr pop and rts delay nop |
| `0x8c0443c8` | `FUN_8c0443c8` | `notes/verifications/0x8c0443c8.json` — 75 oracle steps; Calls free (0x8c01032c -> 0x8c159fc0) with r4 when r4 != 0 and (int16)r5 > 0; returns r4; saves/restores r14, pr |
| `0x8c03eb50` | `FUN_8c03eb50` | `notes/verifications/0x8c03eb50.json` — 97 oracle steps; free(r4) via thunk 0x8c01032c (0x8c159fc0) when r4 != 0 and (i16)r5 > 0; *0x8c23dbf0 = 0 in the bf.s delay slot on both outcomes of the comparison, but not on the r4 == 0 bt exit; r0 = r4 on every path; r14/r15/pr restored; T = 1 on the null exit, (i16)r5 > 0 on the skip exit, callee T on the free exit |
| `0x8c026900` | `FUN_8c026900` | `notes/verifications/0x8c026900.json` — 1065 oracle steps; if [r5] != 0: for each of [r5] and [r5+4] that is non-null, recursively frees that childs [c] and [c+4] subtrees via self (r4 = context passthrough, r5 = grandchild) and then frees the child via thunk 0x8c01032c; free([r5]) itself last, always; r4 is never dereferenced; r11-r14 and pr saved/restored on the stack; r0-r6 and T are the final free calls residue |
| `0x8c043bac` | `FUN_8c043bac` | `notes/verifications/0x8c043bac.json` — 94 oracle steps; T = (r4 == 0) on the null exit; T = ((int16)r5 > 0) on the skip exit; free()'s residue on the call exit. r0 = r4 on return; r14/r15/pr restored; callee residue (0x8c01032c -> 0x8c159fc0) survives when r4 != 0 and (int16)r5 > 0. |
| `0x8c02c318` | `FUN_8c02c318` | `notes/verifications/0x8c02c318.json` — 40 oracle steps; gathers three strided floats plus a fourth into out[0..2]: base=[r4+4]; out[0]=f32[base+0x1ac+52*i], out[1]=f32[base+0x1b0+52*i], out[2]=f32[base+0x1b4+52*i]+f32[r4+88+172*i]; i=r5 signed, addresses wrap; r14/r13/r12 pushed then popped (return to entry values); r0=out+4, r1=0x1ac, r6=0x1b0, r3=base+0x1b0+52*i, r4=r4+88+172*i, r5=low32(172*i), r7=base+0x1b4+52*i, MACH:MACL=signed 172*i product; three f32 stores at out/out+4/out+8; sr,gbr,vbr,ssr,spc,sgr,dbr,fpul,fpscr and r2/r8-r11 unchanged |
| `0x8c02c4a0` | `FUN_8c02c4a0 (clamp fr4 to [0,1]; store f32 at r4 + 0xec + 0xac*r5)` | `notes/verifications/0x8c02c4a0.json` — 15 oracle steps; r0 = 0xec; mul.l overwrites mach (high) and macl (low) with the signed 0xac*r5 product (r5 = macl); r4 = r4 + 0xec + macl (wrapping) is the store address; T = last fcmp/gt; fr0 = 0.0 / 1.0 / fr4 by path; rts delay slot fmov fr0,@r4 stores 4 bytes; pr/fpscr untouched |
| `0x8c059a92` | `FUN_8c059a92` | `notes/verifications/0x8c059a92.json` — 40 oracle steps; r0,r1,r3,r4,r5,sr(T) |
| `0x8c066604` | `FUN_8c066604` | `notes/verifications/0x8c066604.json` — 7 oracle steps; r0=0x8c067f08; delay-slot r4=[r4+0x38]; tail-call 0x8c067f08 leaves r0=0x9c, r1=1, byte [r4+0x9c]=1 |
| `0x8c056bcc` | `FUN_8c056bcc` | `notes/verifications/0x8c056bcc.json` — 9 oracle steps; pushes r14 and pr; when r4 != 0 stores 0x8c1e8f88 at *r4 (the store sits in the bf.s delay slot, so it runs whether or not the branch is taken); if exts.w(r5) > 0 calls thunk 0x8c01032c -> free(0x8c159fc0) with pr=0x8c056be6; returns r0 = r4 with pr/r14/r15 restored; final T = entry tst on the null path, cmp/pl on the skip path, otherwise free's exit T |
| `0x8c059ab0` | `FUN_8c059ab0` | `notes/verifications/0x8c059ab0.json` — 92 oracle steps; store 0x8c1e90cc at *r4 (bf.s delay slot, both outcomes); call free(r4) when r4!=0 and (i16)r5>0 |
| `0x8c04af74` | `FUN_8c04af74 (free_square_table)` | `notes/verifications/0x8c04af74.json` — 713 oracle steps; zeroes the descriptor's +0/+4/+8 words, every element slot and every row slot, plus free()'s own heap bookkeeping; frees in the order nested-diagonal, element, row, then the row table; residue r0=0, r1=0, r3=0x8c2b1ef8, r4=free's block header (array-8; the coalesced previous block when that path is taken), r5=bin index, r6=heap state; r2/r7/r8 and the popped r9..r14 keep their entry values; T=1 on the dimension-0 early exit, otherwise free()'s last cmp/hi |
| `0x8c020a78` | `FUN_8c020a78` | `notes/verifications/0x8c020a78.json` — 199 oracle steps; releases handles at param1+8/+4/+0 via 0x8c018bd8, then frees param1 via 0x8c01032c when sext16(r5)>0; returns param1 |
| `0x8c05e042` | `FUN_8c05e042` | `notes/verifications/0x8c05e042.json` — 38 oracle steps; byte-compare loop entered at body: r0=d-s or 0 at NUL, r1=last src byte, r3=last dst byte, r4/r5 advanced, T from last cmp/eq/tst |
| `0x8c05d900` | `FUN_8c05d900` | `notes/verifications/0x8c05d900.json` — 132 oracle steps; release handle at r4+4 (count -1); if sext16(r5)>0 free(r4); returns r4 |
| `0x8c05f8cc` | `FUN_8c05f8cc (release list node, optional free)` | `notes/verifications/0x8c05f8cc.json` — 275 oracle steps; pushes r14/r13/pr and pops them, so r13/r14/pr end at entry values; r13=r4 (node), r14=r5 (threshold); if node!=0 calls release 0x8c018bd8 with r4=node, r5=-1 (delay slot), which dereferences *node, may decrement/free object+inner and clear *node, then if exts.w(threshold)>0 calls free 0x8c01032c with r4=node (delay slot); r0=node; T set by tst on the NULL exit, clear by cmp/pl when the low 16 bits are <=0, otherwise the free callee T; r1-r12/r15 and all stores follow the callees plus the three stack pushes |
| `0x8c056778` | `FUN_8c056778` | `notes/verifications/0x8c056778.json` — 70 oracle steps; frees *(r4+8) via thunk 0x8c01032c after a dead 0x38-stride walk; NULL path stores nothing and returns T=1 with r0-r4 intact; non-NULL path ends with the callee residue (T=0) |
| `0x8c060dc4` | `FUN_8c060dc4` | `notes/verifications/0x8c060dc4.json` — 141 oracle steps; pushes r14/r13/pr and pops them so r13/r14/pr end at entry values; r13=r4 (block), r14=r5 (count); if block!=0 calls free thunk 0x8c01032c with r4=[block] (delay slot), then if exts.w(low 16 bits of count)>0 calls it again with r4=block (delay slot); r0=block; T is tst's set flag on the NULL return, cmp/pl's clear flag when the low half is <=0, otherwise the callee free T; r1-r12 follow the calls, r15 restored |
| `0x8c061064` | `FUN_8c061064` | `notes/verifications/0x8c061064.json` — 75 oracle steps; pushes r14/pr and restores both; r0 = incoming r4; T = 1 when r4 == 0 (tst), 0 when the sign-extended low half of r5 is non-positive (cmp/pl), otherwise free's residue (false); calls thunk 0x8c01032c -> free (0x8c159fc0) with the original r4/r5 when r4 != 0 and (int16)r5 > 0, so the callee's r0-r6, T and heap stores are this function's |
| `0x8c05d788` | `FUN_8c05d788` | `notes/verifications/0x8c05d788.json` — 112 oracle steps; Calls release 0x8c018bd8 with r5=-1 on every non-NULL handle: it decrements the object's refcount at object+8 and writes *handle (0 while a reference remains, the object pointer when the count reaches zero, freeing the object and its inner in that case); then only if sext16(the caller's r5) > 0 it calls free thunk 0x8c01032c with r4=handle, whose chunk-header stores are observable too. r0 returns the handle (0 for NULL), r13/r14/pr/r15 are restored from the frame, r4/r1/r3/r5/r6 keep the last callee's residue; T is the entry tst's 1 on the NULL path, the wrapper's cmp/pl 0 when the free is skipped, or free's exit T when it runs |
| `0x8c01a710` | `FUN_8c01a710` | `notes/verifications/0x8c01a710.json` — 579 oracle steps; releases handles at param1+8/+4/+0 via 0x8c018bd8, hands param1+12 to 0x8c01efb8 (spans and frees [param1+20] unless NULL), then frees param1 via 0x8c01032c when sext16(r5)>0; returns param1; NULL path stores nothing and leaves T=1 |
| `0x8c05caac` | `FUN_8c05caac` | `notes/verifications/0x8c05caac.json` — 184 oracle steps; stamps r5 at [r4+0x30], r5+1 at every [child+0x30] (child=[node+0x0c], sibling=[node+0x08]), r5+2 at every [grandchild+0x30], then self-recurses (`jsr @[0x8c05cb00]`, literal 0x8c05caac) with r4=great-grandchild and the delay-slot depth r5+3; r11-r15 and pr restored, T=1, r0 is the deepest frame's residue: SELF literal 0x8c05caac if the last r0 write was a jsr setup (its callee exited at first tst), else depth+1/depth+2 of the last frame that wrote r0 via `add`, r4 is the last recursed node (else the argument), r5 is the last delay-slot depth+3 (else the argument depth); only stack writes below r15 |
| `0x8c05b1ec` | `FUN_8c05b1ec` | `notes/verifications/0x8c05b1ec.json` — 281 oracle steps; recursive read-only name lookup: compares [node+0x24] (or literal 0x8c1f7328 when null) with param_2 byte-by-byte, returns the node on a match, else scans the +0x0c child list and recurses into each child's children, returning 0 when exhausted; r12/r13/r14/pr preserved, T left by the final tst, no memory stores |
| `0x8c043a80` | `FUN_8c043a80` | `notes/verifications/0x8c043a80.json` — 22 oracle steps; walks the circular list at *param_1 until a node equals the reloaded head; per node release(node+8, -1) via 0x8c018bd8 then free(node) via 0x8c01032c; finally [head] = head and *(head+4) = head, re-arming the sentinel; r0 = the reloaded head, T = 1, r1-r10 hold the last callee residue, r11-r14/pr restored from the frame |
| `0x8c021d70` | `FUN_8c021d70` | `notes/verifications/0x8c021d70.json` — 186 oracle steps; pushes r14,r13,pr; NULL container: no store, r0=0, T=1; otherwise releases container+8, container+4, container via 0x8c018bd8 with r5=-1 (each decrements the object refcount at [*handle+8], writes *handle=0 while a reference remains, and destroys object->inner/object through the free thunk when it reaches zero), then frees container via 0x8c01032c only when exts.w(r5)>0 (the high half is ignored); r0=container, r13/r14/pr restored from the frame; T = tst's 1 on the NULL path, cmp/pl's 0 when the free is skipped, else free's exit T |
| `0x8c06ae60` | `FUN_8c06ae60` | `notes/verifications/0x8c06ae60.json` — 548 oracle steps; teardown of the struct at r4: pushes r14,r13,r12,r11,r10,r9,pr (r15 entry-equal at exit; r9-r14/pr restored); if r4 != 0, frees *(p+0x14)[0..*(i32*)(p+0x2c)) through 0x8c0102cc (signed count, stride 4, table base re-read each trip), then *(p+0x14),*(p+0x18),*(p+0x1c), then *(p+0x20)[row*16+{0,4,8,12}] for row < *(i32*)(p+0x24) (inner loop always 4, bt.s jumps back past the init), then *(p+0x20); if (i16)r5 > 0 also frees p through 0x8c01032c; r0 = p on return; free's r1-r6 and heap-store residue passes through; T = 1 when p == 0, 0 when (i16)r5 <= 0, callee T after the final free |
| `0x8c05d4bc` | `FUN_8c05d4bc` | `notes/verifications/0x8c05d4bc.json` — 70 oracle steps; frees every node after the head of the ring list held by *r4 through the 0x8c01032c thunk (the jsr delay slot reads the next pointer before the call); then stores *head=head and head[1]=head; r0=head; after any free call r1/r3/r4/r5/r6 hold that free's residue (with a self-looping head they pass through); r2/r7-r12 pass through; r13/r14/pr/r15 restored; T=1 |
| `0x8c059a84` | `thunk_FUN_8c059a92` | `notes/verifications/0x8c059a84.json` — 59 oracle steps; tail-jump bra 0x8c059a92 with nop delay slot; the thunk changes nothing itself, the residue is the callee's byte compare: r0=1 iff the bytes matched through NUL else 0, r1=left-right (both zero-extended) or 0 on the NUL exit, r3=last zero-extended left byte, r4/r5 point one past the bytes compared, sr(T) from the final tst; pr untouched |
| `0x8c05d7bc` | `FUN_8c05d7bc (release handle, optional free)` | `notes/verifications/0x8c05d7bc.json` — 131 oracle steps; pushes r14/r13/pr and pops them, so r13/r14/pr end at entry values; r13=r4 (handle), r14=r5 (count); if handle!=0 calls release 0x8c018bd8 with r4=handle, r5=-1 (delay slot), which dereferences *handle, may decrement/free object+inner and clears *handle while a reference remains, then if exts.w(low 16 bits of count)>0 calls free 0x8c01032c with r4=handle (delay slot), whose chunk-header stores are observable; r0=handle (0 on the NULL exit); T is tst's set flag on the NULL return, cmp/pl's clear flag when the low half is <=0, otherwise the free callee T; r1-r12/r15 and all stores follow the callees plus the three stack pushes |
| `0x8c05e034` | `thunk_FUN_8c05e042` | `notes/verifications/0x8c05e034.json` — 40 oracle steps; no state of its own: bra 0x8c05e042 with nop delay slot, pr untouched; r0/r1/r3/r4/r5/T and return value are the callee's residue |
| `0x8c05ad54` | `FUN_8c05ad54` | `notes/verifications/0x8c05ad54.json` — 97 oracle steps; r0=arg1 on return; r1=0x8c1e8f88 after callee (0x8c1e9120 store in delay slot is overwritten by the callee's 0x8c1e8f88 at [r13]); r4 unchanged on null/no-free paths, free residue on the free path; r5=0 before callee then free residue; r13/r14/pr/r15 restored; T=1 on the null path (bt), otherwise cmp/pl(exts.w arg2) then whatever the free exit leaves (0 on no-free / measured 0 in the free spec); stores: [r15-4]=r14, [r15-8]=r13, [r15-12]=pr, [r13]=0x8c1e9120 then [r13]=0x8c1e8f88, plus free's writes on the positive-halfword path |
| `0x8c06cbc0` | `FUN_8c06cbc0` | `notes/verifications/0x8c06cbc0.json` — 77 oracle steps; saves r14/pr on the stack; returns r0=r4 (and restores r14, r15, pr); when r4!=0 stamps [r4]=0x8c1e94e8 (unconditional: it is the bf.s delay-slot store, so it lands on both branch outcomes) and sets r1=0x8c1e94e8; T is 1 only on the r4==0 exit, otherwise comes from cmp/pl of exts.w(r5) (T=1 iff the low 16 bits of r5 sign-extend positive); on that positive path it calls free via 0x8c01032c->0x8c159fc0 with r4 unchanged, and T/r1/other residue are the callee's, while r0 is overwritten with r4 on return |
| `0x8c06c3b8` | `FUN_8c06c3b8 (cursor: next 44-byte record)` | `notes/verifications/0x8c06c3b8.json` — 14 oracle steps; r3 = index = [r4+48], r0 = count = [r4+40]; signed cmp/ge sets T=1 when index>=count and the routine returns r0=0 with the cursor untouched (mach/macl and every other register keep their seeds); otherwise T=0, [r4+48]=index+1, MACH:MACL = sign-extended 44*index, r3=macl, r1=index+1, r0=[r4+44]+macl |
| `0x8c05adf0` | `FUN_8c05adf0` | `notes/verifications/0x8c05adf0.json` — 83 oracle steps; r0 = saved r4 (entry r4); r13/r14/r15/pr restored from the entry frame; when r4 != 0 stores 0x8c1e9130 at *r4 (jsr delay slot) then 0x8c056bcc overwrites it with 0x8c1e8f88; 0x8c056bcc is called with r5=0 so its own free branch never runs, leaving r1=0x8c1e8f88 and its r0/stack residue; if exts.w(orig r5) > 0 calls free 0x8c01032c, whose r1/r2/r3/r4/r5/r6/T residue survives except r0, overwritten with the saved r4 at the exit; T = 1 only on the r4 == 0 path (entry tst), otherwise 0 (cmp/pl or free's final cmp/hi) |
| `0x8c061178` | `FUN_8c061178` | `notes/verifications/0x8c061178.json` — 99 oracle steps; clears *(u32*)0x8c23dea0 when it equals r4; calls 0x8c061064(r4+16, 0xffffffff); when sext16(r5) > 0 calls 0x8c01032c (free(r4)); r0 = r4 on exit, r13/r14/pr restored from the frame; T ends as tst on the r4==0 early exit, else cmp/pl or the free residue; only non-stack store is the registry clear |
| `0x8c05f530` | `FUN_8c05f530` | `notes/verifications/0x8c05f530.json` — 110 oracle steps; pushes r14,r13,pr; r13=r4, r14=r5; NULL returns r0=0 with T=1; otherwise stores 0x8c1e9174 at *object, releases the handles at +24 then +20 through 0x8c018bd8 (count -1: pointee refcount at +8 decremented, handle slot kept at zero refs and pointee plus its +12 inner freed by free 0x8c159fc0), stores 0x8c1e9198 at *object, then 0x8c056bcc(object,0) overwrites it with 0x8c1e8f88 in its bf.s delay slot; finally frees object via thunk 0x8c01032c only when sext16(r5)>0; r0=object on every path; r13/r14/pr restored from the frame, r15 entry-equal; final T=1 on the NULL path, 0 when the low half is <=0 (0x00010000 and 0xffff0000 do not free), otherwise free residue (r0-r6 likewise) |
| `0x8c079064` | `thunk_FUN_8c05fa50` | `notes/verifications/0x8c079064.json` — 6 oracle steps; no memory stores; `mov.l 0x8c07906c,r0` loads 0x8c05fa50, `jmp @r0` tail-calls it (nop delay, pr left alone so the callee's rts returns to the caller); r0 ends as the callee's global load mem32[0x8c23de98], other regs untouched |
| `0x8c079670` | `FUN_8c079670` | `notes/verifications/0x8c079670.json` — 90 oracle steps; free(r4) through the 0x8c01032c thunk when r4 != NULL and sext16(low 16 bits of r5) > 0; r0 = r4 on every path; r14 and pr restored from the 8-byte frame (seeds survive); r1-r13 keep the last callee residue; T = callee T when the free runs, tst set on the NULL path, cmp/pl clear when the 16-bit count <= 0 |
| `0x8c078c68` | `FUN_8c078c68` | `notes/verifications/0x8c078c68.json` — 354 oracle steps; releases object+8 then object+4 with count -1 (freeing each object whose refcount reaches 0); frees object through 0x8c01032c when the low 16 bits of r5 are positive; returns r4; NULL r4 skips all calls |
| `0x8c079694` | `FUN_8c079694` | `notes/verifications/0x8c079694.json` — 90 oracle steps; calls free thunk 0x8c01032c when r4 != 0 and (i16)r5 > 0; r14 and pr pushed/restored; r0 = r4 on every path |
| `0x8c07964c` | `FUN_8c07964c` | `notes/verifications/0x8c07964c.json` — 61 oracle steps; pushes r14/pr then restores them; when r4!=0 and (int16)r5>0 calls the free thunk 0x8c01032c with r4/r5 raw (callee residue passes through, T included); otherwise skips the call; returns r4 in r0; T = 1 on the r4==0 skip, 0 on the cmp/pl skip |
| `0x8c07687c` | `FUN_8c07687c` | `notes/verifications/0x8c07687c.json` — 48 oracle steps; stores [r4+0x20] and [r4+0x24] (shadow cursor/accumulator reset to [r4+0xc],0 when byte([r4+0x10])&0x7f == [r4+0x28], else [r4+0x20]+=84*that byte wrapping down by 84*[r4+0x28] when it reaches [r4+0xc]+84*[r4+0x28] and [r4+0x24]-=byte; both paths), [r4+0x10] (=old+12 wrapping by 12*[r4+0x18] at [r4+0x08]+12*[r4+0x18]) and [r4+0x14] (=old-1); r0=countdown-1, r1=[r4+0x08], r3=12*[r4+0x18], r5=new cursor, r6=new shadow cursor only on the non-equal path, sr(T) from final cmp/hs (new cursor+12 >= [r4+0x08]+12*[r4+0x18] before wrap), MACH/MACL from the last mul.l on the non-equal path (signed 84*[r4+0x28]) and untouched on the equal path; all other registers and pr unchanged |
| `0x8c074338` | `free_if_positive` | `notes/verifications/0x8c074338.json` — 75 oracle steps; Calls free (0x8c01032c -> 0x8c159fc0) with r4 when r4 != 0 and the sign-extended low 16 bits of r5 are > 0; returns r4 in r0 and restores r14; T is 1 for null r4, 0 for a non-positive count, else the callee's final T |
| `0x8c083ce8` | `FUN_8c083ce8` | `notes/verifications/0x8c083ce8.json` — 112 oracle steps; pushes r14/r13/pr; if r4 != 0 calls release 0x8c018bd8(handle, -1), which decrements the object refcount and may clear the handle slot and destroy/free the object; if sext16(r5) > 0 then calls free thunk 0x8c01032c(handle); returns the handle in r0; restores pr/r13/r14; T = entry tst on the NULL path, wrapper cmp/pl on the non-positive path, otherwise the last callee's flags |
| `0x8c0796b8` | `FUN_8c0796b8` | `notes/verifications/0x8c0796b8.json` — 116 oracle steps; three-word frame; free(mem[handle+12]) when non-null; free(handle) when exts.w(count) > 0; free-list/top-chunk stores; T=1 on null exit, T=0 when count <= 0, else free residue |
| `0x8c08223c` | `FUN_8c08223c` | `notes/verifications/0x8c08223c.json` — 75 oracle steps; when r4 != 0 and (i16)r5 > 0 calls free(r4) through 0x8c01032c (r4/r5 unchanged); returns r4 in r0, restores r14/pr/r15; T=1 on null early exit, T=0 on cmp/pl skip, else callee T |
| `0x8c082154` | `FUN_8c082154` | `notes/verifications/0x8c082154.json` — 75 oracle steps; free(r4) when r4 != 0 and (s16)r5 > 0; returns r4; T = 1 on null r4, cmp/pl result otherwise, free residue on the call path; pushes/restores r14 and pr; r0 = r4 |
| `0x8c082c74` | `FUN_8c082c74` | `notes/verifications/0x8c082c74.json` — 77 oracle steps; stores 0x8c1e9b50 at *r4 whenever r4 != 0 (store is the bf.s delay slot, so it also runs when the call is skipped); when exts.w(r5) > 0 calls 0x8c01032c (free(r4), r5 passed through untouched) and keeps its residue; then r0 = r4, pr/r14/r15 restored to entry; final T is tst's (null), cmp/pl's (skipped call) or free's (call) |
| `0x8c07c2a0` | `FUN_8c07c2a0` | `notes/verifications/0x8c07c2a0.json` — 275 oracle steps; calls release thunk 0x8c018bd8 with r5=-1 when r4!=0, then frees the handle through 0x8c01032c when sext16(entry r5)>0; pushes r14/r13/pr at entry and restores them (r15 back to entry value, pr = seed, r14 = entry r5, r13 = entry r4); final r0 = entry r4; T = 1 on the NULL-handle path (tst), 0 after a hold (cmp/pl), or the free callee's exit T on the free path; every other register is the release/free callee residue |
| `0x8c078874` | `FUN_8c078874` | `notes/verifications/0x8c078874.json` — 98 oracle steps; releases one reference on the handle in r4 via 0x8c018bd8 with count -1 (delay slot), then frees the handle through 0x8c01032c when the low 16 bits of r5 sign-extend positive; returns the handle in r0; r13/r14/pr restored, T=1 only on the NULL-handle exit and otherwise the callee-free residue |
| `0x8c06e5cc` | `FUN_8c06e5cc` | `notes/verifications/0x8c06e5cc.json` — 57 oracle steps; no stores; clobbers r0,r1,r3,r5,r6,r7,mach,macl,T |
| `0x8c07fe68` | `FUN_8c07fe68` | `notes/verifications/0x8c07fe68.json` — 127 oracle steps; push r14/r13/pr; non-NULL handle: release 0x8c018bd8(handle,-1) via the -1 delay slot; then exts.w/cmp/pl on the low 16 bits of the original r5: positive -> free thunk 0x8c01032c(handle); return r13 in r0 with r13/r14/pr reloaded from the frame; T=1 from the entry tst on the NULL path, T=0 from cmp/pl on the skip path, callee residue (r1-r7, T) on the release/free paths |
| `0x8c082fe4` | `FUN_8c082fe4` | `notes/verifications/0x8c082fe4.json` — 275 oracle steps; pushes r14/r13/pr; if r4 != 0 calls release 0x8c018bd8(handle, -1) (delay slot), which decrements the object refcount and may clear the handle slot and destroy/free the object; then exts.w/cmp/pl on the low 16 bits of the original r5: positive -> free thunk 0x8c01032c(handle); r0 = handle in every arm, pr/r13/r14 restored and r15 back to its entry value; T = entry tst (set) on the NULL path, cmp/pl (clear) when only the release ran, otherwise the free callee's residue |
| `0x8c07a938` | `FUN_8c07a938 (ring index advance)` | `notes/verifications/0x8c07a938.json` — 24 oracle steps; stores [r4+8]=cursor then [r4+4]=index, no other stores; r0=r4, r1=index+r5, r3=H (rewind path) or H[0] (append path), r6=H (append) / (-r5)<<2 (rewind non-wrap) / -r5 (rewind wrap); T = cmp/hi H[0] > H[2]+index+r5 (append) or cmp/hs H[2]+index >= -r5 (rewind); all other registers unchanged |
| `0x8c07c100` | `destroy_object` | `notes/verifications/0x8c07c100.json` — 157 oracle steps; writes [r4+4]=0x8c1e9774, [*[r4]]=0x8c1e9794, [*[r4]+24]=(r4+36)-*[r4]; frees *[r4+8],*[r4+12],*[r4+16],*[r4+20],*[r4+24],*[r4+28] through 0x8c0102cc; when exts.w(r5)!=0 writes [r4+36]=0x8c1e9ad4; when exts.w(r5)>0 frees r4 through 0x8c01032c; r0=r4 at exit, T=1 on r4==0, 0 on the no-final-free paths, else free's T |
| `0x8c0c010c` | `thunk_advance_8` | `notes/verifications/0x8c0c010c.json` — 6 oracle steps; tail-call to 0x8c0f8874: r4 = [r4+4] in the jmp delay slot, callee adds 8 and leaves it in r4/r0; pr untouched, no stores |
| `0x8c0c0100` | `FUN_8c0c0100 (tail-call thunk to advance_56, 0x8c0f816c)` | `notes/verifications/0x8c0c0100.json` — 6 oracle steps; r0 = literal [0x8c0c0108] (=0x8c0f816c) then overwritten by the callee; the jmp delay slot loads r4 = u32[r4+4]; tail-jump leaves pr untouched, so the callee rts returns to the caller; callee does r4 += 56 and its rts delay slot sets r0 = r4; no stack frame, no stores; r0/r4 residue = u32[r4+4]+56 |
| `0x8c0bfef0` | `FUN_8c0bfef0` | `notes/verifications/0x8c0bfef0.json` — 46 oracle steps; initialises the 10-word struct at r4: tail at r4+28..+40 via 0x8c0c079c (source byte from stack), then zeroes r4+0..+24; returns r4 |
| `0x8c0bfea8` | `FUN_8c0bfea8` | `notes/verifications/0x8c0bfea8.json` — 90 oracle steps; pushes r14 then pr (4 bytes each) and pops them, so both end at entry values; r0 = entry r4 on every exit; calls the 0x8c01032c thunk -> free(0x8c159fc0) with r4 unchanged only when r4 != 0 and sext16(r5) > 0, keeping free's r1-r6 residue, stores and T; otherwise no call and r1-r13 keep entry values; T = 1 on the null path, 0 on the other two (free's exit cmp/hi falls through false) |
| `0x8c0bfecc` | `FUN_8c0bfecc` | `notes/verifications/0x8c0bfecc.json` — 75 oracle steps; pushes r14 then pr (4 bytes each) and pops them, so both end at entry values; r0 = entry r4 on every exit; calls the 0x8c01032c thunk -> free(0x8c159fc0) with r4 unchanged only when r4 != 0 and sext16(r5) > 0, keeping free's r1-r6 residue, stores and T; otherwise no call and r1-r13 keep entry values; T = 1 on the null path, 0 on the other two (free's exit cmp/hi falls through false) |
| `0x8c0c1818` | `FUN_8c0c1818` | `notes/verifications/0x8c0c1818.json` — 182 oracle steps; pushes r14, r13, r12 then pr and pops all four, so r12/r13/r14 end at entry values (r14's body copy of r5 is discarded); r13=object(r4); if object!=0 stores 0 to *0x8c27a238; r12=[object+0x24]; if r12!=0: [if r12+0x2c!=0] calls 0x8c043a68(r4=r12+0x2c), 0x8c01e460(r4=r12+0x2c, r5=-1 in the jsr delay slot), then free thunk 0x8c01032c(r4=r12); afterwards if sext16(entry r5)>0 calls free thunk with r4=object; r0=object on every path (overwriting callee residue); T = 1 from tst on the object-NULL path, 0 from cmp/pl when the low 16 bits are <= 0, else the last callee's; r1-r11 and r15 plus all non-stack stores follow the callees; the twin tst at 0x1838/0x183c guards only the two inner calls, not the free(r12) |
| `0x8c0bffdc` | `FUN_8c0bffdc` | `notes/verifications/0x8c0bffdc.json` — 12 oracle steps; stores 0x8c1eb43c at [r4] and 0 at [r4+12]; if r4 != 0 and r4+16 != 0: frees [r4+24] when non-null through the 0x8c01032c thunk, calls 0x8c01e078(r4+16, 0), then frees r4 when (int16)r5 > 0; returns r4; r13/r14/pr saved in a 12-byte frame and restored, r15 entry-equal |
| `0x8c0c2f30` | `FUN_8c0c2f30` | `notes/verifications/0x8c0c2f30.json` — 181 oracle steps; frees every node after the head via 0x8c01032c (0x8c159fc0), then writes head->next=head and head->prev=head; returns head in r0 with T=1 |
| `0x8c0d515c` | `FUN_8c0d515c` | `notes/verifications/0x8c0d515c.json` — 6 oracle steps; r0=pool[0x8c0d5164]=0x8c06caa8; r4=[r4+8] |
| `0x8c0c0614` | `release_pointer_tree` | `notes/verifications/0x8c0c0614.json` — 666 oracle steps; recursively frees a tree of two-pointer nodes three levels per call via thunk 0x8c01032c, then the argument struct; leaves r0-r6 and T from the last free, r11-r15 restored to entry (r12/r13 too) |
| `0x8c0d528c` | `FUN_8c0d528c` | `notes/verifications/0x8c0d528c.json` — 43 oracle steps; scales the two 3-float vectors at r4+12 and r4+24 in place by the pool literal at 0x8c0d52b0 (-1.0) via 0x8c0b4ef0, called twice (fr4 reloaded each time; second arg r4+24). Residue r0=arg+28, r4=arg+32, fr0=[arg+32]*-1; r14 and pr restored by the epilogue; T/fpscr untouched. |
| `0x8c0d5150` | `FUN_8c0d5150` | `notes/verifications/0x8c0d5150.json` — 6 oracle steps; Reads the pool literal 0x8c06caa8 into r0 and tail-jumps to it; the jmp delay slot loads r4 = *(u32*)(r4+4), and the callee 0x8c06caa8 advances it by 12, so r4 and r0 both end at payload+12. No stores, no other register or flag change (sr/T and pr untouched; pr stays HALT_PC). |
| `0x8c0d2188` | `FUN_8c0d2188` | `notes/verifications/0x8c0d2188.json` — 181 oracle steps; Walks the three-level tree rooted at [r4+0xc]: at each level reads child +0xc and follows sibling +0x8 chains until null, calling itself (jsr through literal 0x8c0d21e0, pr=0x8c0d21b4, r4=innermost node) on every innermost node; each frame starts its level totals at 1 and accumulates r9+=callee r0, r11+=r9, r14+=r11. Returns the total in r0 (14 for the rich-tree spec); r9-r14 and pr restored from the 28-byte frame, r15 entry-equal; r4 is left at the last node passed to the deepest/last jsr (0x8c260320 in the rich tree); r1-r3, r5-r8 untouched; T=1 on every exit; only stack stores. |
| `0x8c0d5658` | `FUN_8c0d5658` | `notes/verifications/0x8c0d5658.json` — 503 oracle steps; Clears the flag word [r4+16*i+12] of eight 16-byte records, then for each record calls 0x8c0b4f34 (r4=record, r5=the saved r5 vector) so record[0..2] += vector[0..2] in place, and ORs classification bits into the flag word: x>1.0 sets 1/4/16 and -1.0>x sets 2/8/32 for the floats at offsets 0/4/8. Final r0=8, r1=r4_entry+116, r4=r4_entry+120, r5=r5_entry+8 (callee residue), r12=r5_entry, r13=8, r14/r15/pr restored, unrelated registers untouched; fr0=1.0 if the last record's offset-8 float was >1.0 else -1.0, fr1=that float; T=0; only record-array stores. |
| `0x8c0d357c` | `release_handle_maybe_free` | `notes/verifications/0x8c0d357c.json` — 277 oracle steps; if r4 != 0: saves r5 in r14, pushes r13/r14/pr, releases the handle via 0x8c018bd8 with r5 = -1 (delay slot at 0x8c0d3592), then frees r4 through the 0x8c01032c thunk when sext16(saved r5) > 0; returns r4 (0 on the null path); r13, r14, r15 and pr end at their entry values; callee register residue, T and heap stores pass through; T = 1 on the null path, 0 when the count is non-positive, free's T on the free path |
| `0x8c0d8ab8` | `FUN_8c0d8ab8` | `notes/verifications/0x8c0d8ab8.json` — 92 oracle steps; If r4 != 0: stores the literal-pool global *0x8c0d8adc (0x8c1ec360) at *r4 unconditionally (it is the bf.s delay slot), then when sext16(r5) > 0 calls the 0x8c01032c free thunk (r4 unchanged, pr=0x8c0d8ad2); returns r4 (0 on the null path). r1 holds the stored global; r14, r15 and pr end at their entry values; callee register residue, T and heap stores pass through; T=1 on the null path, T=0 when the guard declines, free's T on the free path. |
| `0x8c0d51c8` | `FUN_8c0d51c8` | `notes/verifications/0x8c0d51c8.json` — 104 oracle steps; If r4 (object) != 0: pushes r14/r13/pr, r14=r5, r13=r4, then calls 0x8c06cbc0 with r5=1 on the sub-objects at [r4+4] and [r4+8] (each stamps its type word 0x8c1e94e8 and frees it); then when sext16(r5) > 0 (exts.w r14) frees the object itself via the 0x8c01032c thunk with r4=object. Returns r0 = object (0 on the null path); r13, r14, r15 and pr end at their entry values; r1-r6/r4 residue and all heap stores pass through the callees. T = 1 on the null path, 0 when the sign-extended flag is non-positive, otherwise free's final T (0 in the main spec). |
| `0x8c0d21e4` | `FUN_8c0d21e4` | `notes/verifications/0x8c0d21e4.json` — 12 oracle steps; Releases a refcounted handle: pushes r14/r13/pr (12-byte frame). If r4==0 returns 0 in r0 with T=1 and no calls/stores. Else loads the release routine (0x8c018bd8) and calls it with r5=0xffffffff, which decrements [[r4]+8] and either clears *r4 (refcount still >0) or keeps it and frees [*r4+0xc], the [*r4+4] wrapper and *r4 itself. Then exts.w of the entry r5 low 16 bits decides whether the free thunk (0x8c01032c -> 0x8c159fc0) frees r4. r0 ends = handle (r4 entry); r13/r14/pr/r15 restored to entry values; r4 left at the last callee's value (entry r4 when the free thunk runs, otherwise callee residue); T is the last callee/cmp/pl flag (null path T=1). Stores outside the synthetic stack are exactly the callees' heap/refcount writes. |
| `0x8c07de58` | `FUN_8c07de58` | `notes/verifications/0x8c07de58.json` — 6 oracle steps; none |
| `0x8c07de4c` | `FUN_8c07de4c` | `notes/verifications/0x8c07de4c.json` — 9 oracle steps; Pure tail jump: loads literal 0x8c07de54 (=0x8c079064) into r0, then the jmp delay slot reloads r4 = mem32[r4+4]; jmp leaves pr untouched, so 0x8c079064 -> 0x8c05fa50 returns mem32[mem32[0x8c05fa58]] = mem32[0x8c23de98] in r0 (overwriting the loaded pointer). Final r0 comes from the callee chain; no stores, T unchanged, r1-r3 and r5-r15 untouched. |
| `0x8c07fd24` | `FUN_8c07fd24` | `notes/verifications/0x8c07fd24.json` — 9 oracle steps; Saves r14/pr, sets r14=r4; returns r0=r4. If r4 != 0 and the sign-extended low 16 of r5 is positive, loads 0x8c01032c and calls it (jsr @r0, pr=0x8c07fd3a, nop delay slot) with r4 unchanged; the thunk tail-jumps to free(0x8c159fc0), whole callee register/store residue passes through. epilogue restores pr/r14, r15 entry-equal; only stack stores. T=1 on the null-block exit (tst), T=0 on the non-positive-flag exit (cmp/pl), and free's final cmp/hi leaves T=0 on the call exit. |
| `0x8c06bde8` | `FUN_8c06bde8` | `notes/verifications/0x8c06bde8.json` — 34 oracle steps; writes [r14+0x54] (new head) and [new+0x28] (old head); callee 0x8c06c3b8 bumps the 0x8c23f438 array index |
| `0x8c09d338` | `FUN_8c09d338` | `notes/verifications/0x8c09d338.json` — 90 oracle steps; calls free (0x8c159fc0 via thunk 0x8c01032c) when r4 != 0 and (int16)r5 > 0; always returns r4 in r0; saves/restores r14 and pr |
| `0x8c07fd48` | `FUN_8c07fd48` | `notes/verifications/0x8c07fd48.json` — 75 oracle steps; calls free (0x8c159fc0 via thunk 0x8c01032c) when r4 != 0 and (int16)r5 > 0; always returns r4 in r0; saves/restores r14 and pr |
| `0x8c095758` | `FUN_8c095758` | `notes/verifications/0x8c095758.json` — 275 oracle steps; if r4 != 0, calls release(r4, -1) via 0x8c018bd8 (may destroy the pointed-at object when its refcount reaches zero); then if exts.w(r5) > 0 calls free(r4) via thunk 0x8c01032c; returns r4; pushes/restores r14, r13 and pr; r15 restored; T is the callee's then cmp/pl on the non-null paths, entry tst on the null path |
| `0x8c023be0` | `FUN_8c023be0` | `notes/verifications/0x8c023be0.json` — 347 oracle steps; If [r4+8] (array base) is NULL, returns after only the five frame pushes/pops with T left set by tst and r0..r10 untouched. Otherwise end = base + [r4+4]*0x20 computed with 32-bit wrap; while base < end (unsigned cmp/hi, so a zero or wrapped count skips the loop) it calls the container destructor 0x8c021d70(record, -1) via literal 0x8c023c24 on each 0x20-byte record and advances by 0x20; then it always calls the free thunk 0x8c01032c on the re-read [r4+8] via literal 0x8c023c28. r11..r14 and pr end at entry values (restored from the 20-byte frame), r15 entry-equal; r0..r7 and T are the last callee residue on the non-null paths; stores are the frame plus callee effects. |
| `0x8c024910` | `FUN_8c024910` | `notes/verifications/0x8c024910.json` — 316 oracle steps; NULL object: only the five frame pushes/pops, T=1 from tst, r0..r12 untouched. Otherwise stores the 0x8c1e88a0 sentinel at [r4] in the first jsr delay slot, calls 0x8c018bd8(&object->handle, -1) through literal 0x8c02494c — the release decrements the pointee refcount and at zero frees its +0xc inner and the pointee itself — then calls 0x8c05ad54(object, 0) through literal 0x8c024950, which overwrites [object] with the 0x8c1e8f88 sentinel via 0x8c056bcc and never frees because count 0 is not positive. Only exts.w's low half of r5 is then tested (cmp/pl): when positive, free(object) runs through the 0x8c01032c thunk, leaving free's register residue and T. Returns the object in r0; r13/r14/pr/r15 are restored from the 12-byte frame to their entry values; literals are read from the image. |
| `0x8c07e94c` | `release_handle_and_maybe_free` | `notes/verifications/0x8c07e94c.json` — 49 oracle steps; decrements the object refcount through 0x8c018bd8 with count -1, then frees the handle through 0x8c01032c when sext16(r5) > 0; returns the handle in r0 |
| `0x8c0d91e8` | `FUN_8c0d91e8` | `notes/verifications/0x8c0d91e8.json` — 247 oracle steps; stores 4B scroll at +0x58 and 4B selection at +0x48; walks +0x40 items calling 0x8c05d274 twice each (mask 2 then 1, r6 = i==sel) |
| `0x8c0ff5b8` | `FUN_8c0ff5b8` | `notes/verifications/0x8c0ff5b8.json` — 90 oracle steps; Returns param_1 (r4) in r0. If r4 != 0 and exts.w(r5) > 0, calls the free thunk 0x8c01032c via jsr with r4 = block and r5 untouched; on that path r1, r3-r6 and T end as free leaves them and r0 is then overwritten with r4. Otherwise no call: r0 = r4, and T = 1 when r4 == 0 (tst/bt) or T = 0 when exts.w(r5) <= 0 (cmp/pl/bf). Pushes r14 then pr to an 8-byte frame and restores both (r14 and pr end at entry values, r15 entry-equal); the only stores of its own are those two stack pushes plus free's heap writes. |
| `0x8c0ff594` | `free_block_if_len_positive` | `notes/verifications/0x8c0ff594.json` — 75 oracle steps; If r4 != 0 and (i16)(u16)r5 > 0, frees r4 through the 0x8c01032c thunk (0x8c159fc0), inheriting free's r0-r6 residue, T and heap stores; the jsr delay slot is a nop. Pushes r14 and pr (8-byte frame) and restores both, so r15 and r14 return to their entry values. r0 = r4 on every path; r1-r13 otherwise untouched. T = 1 on the r4 == 0 exit (tst), 0 on the length <= 0 exit (cmp/pl), free's T after a call (no flag-setting instruction follows it). |
| `0x8c0dfbec` | `FUN_8c0dfbec` | `notes/verifications/0x8c0dfbec.json` — 97 oracle steps; zeroes global 0x8c27a928 (bf.s delay slot) then calls free() at 0x8c159fc0 with r4 when exts.w(r5) > 0; returns r4 |
| `0x8c0e1f18` | `FUN_8c0e1f18` | `notes/verifications/0x8c0e1f18.json` — 25 oracle steps; pushes r14 and pr; r0 = 0x8c0eaef8(r5) then 0x8c227780(r4, 0, r0); restores pr and r14, r15 balanced; no flags set by the wrapper itself |
| `0x8c0ffa54` | `FUN_8c0ffa54` | `notes/verifications/0x8c0ffa54.json` — 66 oracle steps; Backward byte search over p = *(u32*)r4: count = *(u32*)p, clamps r6 to count-1 with unsigned cmp/hi, calls 0x8c01ec2c (three call sites, pr = 0x8c0ffa80/0x8c0ffa8c/0x8c0ffab2) for base = *(u32*)(p+0xc), scans base+min(r6,count-1) down to base for (i8)r5, returns index or 0xffffffff; r1 = last loaded byte sign-extended (the count on the zero-count exit), r4 = p, r11-r14 restored from the 20-byte frame, r15 entry-equal, T = 1 on every exit, reads only plus the five prologue stack stores. |
| `0x8c0e28b4` | `FUN_8c0e28b4` | `notes/verifications/0x8c0e28b4.json` — 131 oracle steps; Loads r0 = 0x8c0e8e08 and tail-jumps (jmp @r0) to FUN_8c0e8e08 after the delay slot sets r5 = 7; pr is untouched, so the callee's rts returns to our caller, and the callee restores r12-r14 from its frame. Verified path (entry r5=0, slot-7 table 0x8c1d6e54, list head 0x8c220000, cached [r4+0x74]=5, mode 1): stores [r4+0x74]=7; masks node word 0 with 0x0ff300c0 and +4 with 0x7ffc0000, ORs the slot-7 words 0xdeadbeef/0xcafebabe and sets bits 2 and 0x01000000 in +4; mode 1 multiplies the eight floats at r4+0x50 by 255.0 and ftrc-packs them into node+0x10/+0x14; then [r4+0x78]=1 and byte [r4+0x70]=0. Ends r0=0x70, r1=0, r2=0, r3=node+0x10, r5=0x003f0000, r6=0xfffffe02, r7=r4+0x60, fpul=0xfffffe02, T=1, r15 entry-equal, pr untouched. |
| `0x8c0db7dc` | `FUN_8c0db7dc` | `notes/verifications/0x8c0db7dc.json` — 90 oracle steps; Copies a source block into a record descriptor: [a+0x38]=[b+4], [a+0x3c]=[b+4] (replaced by 57600.0f, bits 0x47610000, when [[0x8c2ab468]+0x28]==3), [a+0x5b8]=[b+8], u8 [a+0x5c0]=u8 [b+0x71c], f32 [a+0x5bc]=f32 [b+0x720]; calls 0x8c066604([a+0x53c]) which writes u8 1 at [[a+0x53c]+0x9c]; zeroes [a+0xbc]; then calls init_record_a 0x8c1ccc18 on [a+0x5e4]+0xd0 when (u8)r6!=0, else init_record_b 0x8c1ccc3c. Final r0=0x178c, r1=0, r3=1 (a) or 0 (b), r4=[a+0x5e4]+0xd0, r5=b+0x720, r6=the byte, r7=0x5c0, r2=the 4-byte stack slot when the mode is 3, fr0=1.0; r8-r12 untouched, r13/r14/r15 entry-equal; T=1 iff (u8)r6==0. Stores only to the stack and those fields. |
| `0x8c10b7b8` | `FUN_8c10b7b8` | `notes/verifications/0x8c10b7b8.json` — 37 oracle steps; strcpy(r4+0x13c, r5); r4=dst past NUL; r5=src past NUL; r2=0; T=1 |
| `0x8c1044f4` | `FUN_8c1044f4` | `notes/verifications/0x8c1044f4.json` — 87 oracle steps; Recursive list search over a node's +0x0c child list (siblings linked by +8): r5==0 returns 0; [r5+0x14]!=0 returns r5; otherwise walks n=[r5+0x0c] and calls this same function (jsr through the literal pool at 0x8c104540, r4=entry r4, r5=n) until a call returns nonzero or n becomes 0. Returns the first accepting node (pointer that is its own call's r5) or 0. r4 stays the entry argument; r1=1 after a nonzero result, 0 after a zero-result call, entry r1 when no call ran; T=1 on the r5==0, empty-child and exhausted-list paths, T=0 on the [r5+0x14]!=0 and nonzero-result paths; no stores outside the stack. |
| `0x8c12d30c` | `call_two_helpers` | `notes/verifications/0x8c12d30c.json` — 74 oracle steps; no stores; pushes pr at r15-4 and restores it; calls 0x8c15a27c (double indirect global) then 0x8c15a23c (block-list walk) with r4 passed through; final r0/r1/r3/r4/r5 and sr T are 0x8c15a23c's residue; r2 and r6-r15 unchanged |
| `0x8c1261a8` | `FUN_8c1261a8` | `notes/verifications/0x8c1261a8.json` — 78 oracle steps; when r4 != 0 stores 0 to the global loaded from the pool at 0x8c1261cc (0x8c2ab5e0) via the bf.s delay slot, so the store happens whether or not the call is taken; when (int16)r5 > 0 also calls the free thunk 0x8c01032c (pr=0x8c1261c4) which tail-calls free 0x8c159fc0 with the unchanged r4; returns r0=r4, r14/pr restored from the 8-byte frame, r15 entry-equal; T comes from tst on the null exit, from the callee otherwise |
| `0x8c10ef64` | `FUN_8c10ef64` | `notes/verifications/0x8c10ef64.json` — 111 oracle steps; byte store 0 at r4+15 when r4 != r5; calls 0x8c011344 with r6=15; r0 = r4 |
| `0x8c10f2d4` | `FUN_8c10f2d4` | `notes/verifications/0x8c10f2d4.json` — 24 oracle steps; Pushes r14 then pr (stack only); calls 0x8c10e35c(*(r5+4)) then 0x8c10e35c(*(r6+4)) at pr=0x8c10f2e0/0x8c10f2ea with the first argument loaded in the jsr delay slot; r0 = signed (result2 > result1), sr T = that comparison; r4 is left at *(r6+4); r14/r15/pr restored to entry; no stores outside the stack frame. |
| `0x8c10ef88` | `FUN_8c10ef88` | `notes/verifications/0x8c10ef88.json` — 115 oracle steps; strncpy(r4, *(*(r5)+0xc), 15); r3/r5/r7 = callee exit; r1=0; T=1 |
| `0x8c12b430` | `FUN_8c12b430` | `notes/verifications/0x8c12b430.json` — 42 oracle steps; Pushes r14, r13 and pr (stack only); r14 = *(r4)+56 with the element array at [r14+8] and count at [r14+4]; walks r13 over the array while r13 != base+count*4, calling 0x8c10d7c8(*r13) at pr=0x8c12b444 with the element in the jsr delay slot, and stops at the first element whose extu.b(byte at elem+4) is 0, returning that element in r0; an exhausted scan returns r0=0. Residue r1 = last end pointer, r4 = last tested element (entry r4 when count is 0), r13/r14/r15/pr restored to entry; T=1 on every exit (tst on the found path, cmp/eq on the exhausted path); no stores outside the stack frame. |
| `0x8c10efa8` | `FUN_8c10efa8` | `notes/verifications/0x8c10efa8.json` — 109 oracle steps; calls 0x8c011344 with r6=15; byte store 0 at r4+15; r0 = r4; r14/pr restored |
| `0x8c027260` | `FUN_8c027260` | `notes/verifications/0x8c027260.json` — 208 oracle steps; Pushes r14 then pr, r14=r4 (container); calls 0x8c050fa8 twice, first with r4=*(container+0x2c) then r4=*(container+0x30) (both arg loads in the jsr delay slots, pr=0x8c02726c then 0x8c027272); no stores of its own outside the stack push pair; final r14=entry value, r15/pr restored, r0/r1/r3/r4/r5/r6/r7/fr0 as the second callee left them (r4=second walker), T=0 from the callee's trailing shll r0 over the zeroed *(p+0x20) |
| `0x8c146ec8` | `FUN_8c146ec8` | `notes/verifications/0x8c146ec8.json` — 68 oracle steps; init object via 0x8c14b1f4; store vtable 0x8c1ef1fc at [r4]; return object in r0 |
| `0x8c146428` | `FUN_8c146428` | `notes/verifications/0x8c146428.json` — 90 oracle steps; free(r4) via thunk 0x8c01032c -> 0x8c159fc0 when r4 != 0 and sext16(r5) > 0; r0 = r4 on every path; r14/pr pushed and restored; T = 1 on the null path, 0 when sext16(r5) <= 0, callee T after a call |
| `0x8c13ec90` | `FUN_8c13ec90` | `notes/verifications/0x8c13ec90.json` — 90 oracle steps; Pushes r14 then pr (8-byte frame), r14=r4; if r4!=0 and (i16)r5>0 calls the 0x8c01032c free thunk with r4/r5 unchanged (pr=0x8c13eca6, delay-slot nop), so the callee's heap stores and T appear on that path; epilogue restores pr/r14 from the frame (both entry-equal, r15 entry-equal) and returns the argument in r0; r1-r3, r6-r13 untouched; T is tst's 1 on the NULL path, cmp/pl's 0 on the non-positive path (exts.w tests only the low 16 bits). |
| `0x8c14ae28` | `FUN_8c14ae28` | `notes/verifications/0x8c14ae28.json` — 17 oracle steps; no calls, no stores; reads the literal at 0x8c14ae4c plus words at r4+0x1c/0x2c/8; writes MACH:MACL from the signed 0x570 multiply (MACL only is kept), sets T from bit 31 of *(r4+0x2c), leaves r0 = 48*field2c + lo32(product) + 4*field8, r1 = lo32(product) + 4*field8, r3 = 4*field8, r5 = lo32(product); r2, r4, r6-r15, pr and all other system registers keep their entry values |
| `0x8c13ec6c` | `FUN_8c13ec6c` | `notes/verifications/0x8c13ec6c.json` — 90 oracle steps; free(r4) through the 0x8c01032c thunk when r4 != NULL and sext16(r5) > 0 (jsr delay nop, pr=0x8c13ec82); r0 = r4 on every path; r14 and pr restored from the frame (seeds survive), r15 entry-equal; T = callee T after a call, tst set on the NULL path, cmp/pl clear on the non-positive path; callee's r1-r6 and heap-store residue pass through |
| `0x8c146684` | `FUN_8c146684` | `notes/verifications/0x8c146684.json` — 887 oracle steps; Walks the 40-word table at r4; calls 0x8c146d78 on each non-null word (callee keeps its counters in its own stack frame, so no net non-stack writes). |
| `0x8c145a04` | `FUN_8c145a04` | `notes/verifications/0x8c145a04.json` — 218 oracle steps; frees every node of the circular list at *(r4), starting at (*r4)->next and stopping back at the sentinel, through the 0x8c01032c thunk to free() (jsr delay slot loads the successor before the free, pr=0x8c145a1a; sentinel is not freed); then head->next = head and head->prev = head; r0 = head, r13/r14/pr restored from the frame (entry seeds), r15 entry-equal; callee register and heap-store residue passes through; final T = 1 (the exit cmp/eq is the last flag-setting instruction) |
| `0x8c0e28d0` | `FUN_8c0e28d0` | `notes/verifications/0x8c0e28d0.json` — 118 oracle steps; Pushes r14 then pr; r14=r5 (destination); calls 0x8c0e8e08 with r4 unchanged and r5=r6 (pr=0x8c0e28dc, delay slot mov r6,r5); then r0=[0x8c27a950] and copies the 16 words at *r0 to r14 (pairwise loads/stores, source read after the callee may have rewritten the list head). Final r0=src[15], r1=src[14], r3=src[12], r5/r6/r7 as the callee left them, r14/r15/pr restored to entry; T is the callee's last flag. Stores 64 bytes at the destination plus the callee's stores; no FPU use of its own. |
| `0x8c105f80` | `FUN_8c105f80` | `notes/verifications/0x8c105f80.json` — 53 oracle steps; only the prologue's stack pushes (r14, r13, pr); no global stores |
| `0x8c125630` | `FUN_8c125630` | `notes/verifications/0x8c125630.json` — 84 oracle steps; while *param != param->next, calls the 0x8c01032c free thunk with the node in r4 (pr=0x8c125646, successor carried in r14 by the jsr delay slot); then stores head at head->next and head->prev; r0 = head, T = 1 from the exit cmp/eq, pr/r13/r14/r15 restored, r1/r3/r4/r5/r6 keep the callee exit values on the free path |
| `0x8c144784` | `FUN_8c144784` | `notes/verifications/0x8c144784.json` — 70 oracle steps; reverse-scan of descriptor (*r4) buffer for any byte of r7-byte set r5; 3 calls to 0x8c01ec2c; returns match index or -1 |
| `0x8c145840` | `FUN_8c145840` | `notes/verifications/0x8c145840.json` — 100 oracle steps; Through the registry calls 0x8c145594 (local = *(*(u32*)r4)), 0x8c145580 (local = *(u32*)r4) and 0x8c145588 (cursor = *(u32*)cursor); when the flag byte 0x8c2abc20 is clear it writes fr0=0.0 to 0x8c2abc28/0x8c2abc2c/0x8c2abc30 and then 1 to the flag (that store order); if (u32)r5 < *(u32*)(r4+4) it walks at most r5 nodes from *(u32*)r4 looking for the node whose address equals *(u32*)r4 and returns *(u32*)(cursor+8), otherwise (and when the budget runs out) it returns the cursor payload, while a bigger r5 returns the default vector 0x8c2abc28 untouched; restores r11/r12/r13/r14/pr and r15, leaves r3=0x8c2abc20, r4=entry r4, r5=0 only if the advance call ran, T=1 on every exit |
| `0x8c144ad8` | `FUN_8c144ad8` | `notes/verifications/0x8c144ad8.json` — 682 oracle steps; recursively destroys a two-slot container: for each non-NULL container slot, the holder at +0/+4 has child[0] and child[1] destroyed by self-calls through the 0x8c144ad8 pool word, then the holder is freed through 0x8c01032c; the slot nodes are freed after both holders and the container is freed last; r4 is a dead context forwarded to every self-call; restores r11/r12/r13/r14/pr and r15; r0/r1/r3/r4/r5/r6 and T are the last free's residue (r2 and r7 untouched) |
| `0x8c165b08` | `unknown` | `notes/verifications/0x8c165b08.json` — 87 oracle steps |
| `0x8c161b50` | `unknown` | `notes/verifications/0x8c161b50.json` — 23 oracle steps |
| `0x8c15b228` | `FUN_8c15b228` | `notes/verifications/0x8c15b228.json` — 10 oracle steps; Loads r1 = dword at entry r4; the delay slot sets r4 = dword at r1+4, then tail-jumps (jmp, pr untouched) to 0x8c1c3a2c, which stores the float at r4+12 to (*r4)+8, leaves r4 advanced by 12 and r0 = that destination (*r4+8), fr0 = the stored float; T/SR keep their entry values (only mov.l/add/fmov, none flag-setting). r1 is the only thunk register besides the callee's r0/r4/fr0 residue. |
| `0x8c15b0c0` | `unknown` | `notes/verifications/0x8c15b0c0.json` — 20 oracle steps |
| `0x8c0855b0` | `FUN_8c0855b0` | `notes/verifications/0x8c0855b0.json` — 57 oracle steps; reads two 4-short arrays and returns fr0 ratio; clobbers r0,r1,r3-r7,mach,macl,fpul,fr0,fr1 |
| `0x8c07c56c` | `FUN_8c07c56c` | `notes/verifications/0x8c07c56c.json` — 159 oracle steps; writes [r4+4]=0x8c1e981c, *(*r4)=0x8c1e9850, *(*r4+24)=r4+36-*r4; releases handle slot [r4+24] with count -1 via 0x8c018bd8 (decrements [*[r4+24]+8], clears [r4+24] while refs remain); if exts.w r5!=0 stores [r4+36]=0x8c1e9ad4; if exts.w r5>0 frees r4 via 0x8c01032c |
| `0x8c07cdd4` | `FUN_8c07cdd4` | `notes/verifications/0x8c07cdd4.json` — 137 oracle steps; writes [r4+4]=0x8c1e98cc, *(*r4)=0x8c1e990c, *(*r4+24)=r4+48-*r4; frees [r4+32],[r4+36],[r4+40],[r4+44] via 0x8c0102cc; if exts.w r5!=0 stores [r4+48]=0x8c1e9ad4; if exts.w r5>0 frees r4 via 0x8c01032c |
| `0x8c07add4` | `FUN_8c07add4` | `notes/verifications/0x8c07add4.json` — 24 oracle steps; stores cursor at [r4+8] and index at [r4+4]; leaves r4 in r0, final index in r1, descriptor-or-count in r3, r6 = descriptor (append) or n / n*4 (rewind); sets T from the wrap compare |
| `0x8c05dfc0` | `FUN_8c05dfc0` | `notes/verifications/0x8c05dfc0.json` — 178 oracle steps; BST insertion-point walk: n=[r4+4]; while n: t=strcmp([[r5]],[[n+12]]); t<0 -> n=[n] else {r14=n; n=[n+4]}; if r14: t2=strcmp([[r14+12]],[[r5]]); t2>=0 -> *[entry r15]=r14 else *[entry r15]=r4+4; if r14==0 -> *[entry r15]=r4+4; writes the single out-slot store; restores r11-r14/pr; r0=out slot, r1=0 after any call, final r3-r5 callee residue; T=1 except t2<0 arm T=0 |
| `0x8c0802f0` | `FUN_8c0802f0` | `notes/verifications/0x8c0802f0.json` — 145 oracle steps; builds end/begin ring iterators via 0x8c079cac/0x8c079c9c, decrements end with 0x8c079c78, then frees (0x8c01032c->0x8c159fc0) count-1 items from start with pointer wrap at items+4*capacity; stores only in its frame |
| `0x8c04396c` | `FUN_8c04396c` | `notes/verifications/0x8c04396c.json` — 583 oracle steps; clears global 0x8c23dbf8 when it equals p; frees [p+0x1a0]; releases handle at p+0x190; destroys then frees list at p+0x184; releases handle at p+0x180; frees elements of array [p+0x17c] with count [p+0x178] (unsigned bound, reloaded per pass); releases block at p+0x174 then guarded-free; frees p when (i16)count > 0 |
| `0x8c09d204` | `FUN_8c09d204` | `notes/verifications/0x8c09d204.json` — 194 oracle steps; release refcounted handle: --refcount; at 0 keep *handle and free object->inner then object; at >0 clear *handle; free handle when sext16(r5)>0 |
| `0x8c07e47c` | `FUN_8c07e47c` | `notes/verifications/0x8c07e47c.json` — 22 oracle steps; moves cursor p={descriptor,index,element} by signed n: stores [p+8]=element+n*4 (or base+((capacity-n)<<2) / base+((descriptor[2]+index+|n|-capacity)<<2) on window wrap) then [p+4]=index-n; returns p; T=final cmp/hs (n>=0) or cmp/hi (n<0); r3/r6/r7 differ per sign path |
| `0x8c05cb48` | `FUN_8c05cb48` | `notes/verifications/0x8c05cb48.json` — 80 oracle steps; unlinks node p from old parent's list at [p+4] (scans [parent+0xc] via +8, rewrites the slot that held p, clears [p+8]); sets [p+4]=new parent; release (parent 0): clears [p+0x30], stamps depth 1 on children, calls 0x8c05caac(grandchild,2); attach: prepends to [parent+0xc] or appends to its tail when r6!=0, sets [p+0x30]=[parent+0x30]+1, children get +2, grandchildren get callee at +3; restores r12/r13/r14/pr, T=1 on exit |
| `0x8c18df08` | `FUN_8c18df08` | `notes/verifications/0x8c18df08.json` — 9 oracle steps; tail-call 0x8c1c3a2c(*(r4+4)); stores fr0 at *(ptr)+8 |
| `0x8c1862b4` | `unknown` | `notes/verifications/0x8c1862b4.json` — 75 oracle steps |
| `0x8c1862d8` | `FUN_8c1862d8` | `notes/verifications/0x8c1862d8.json` — 116 oracle steps; callee 0x8c1863e8 builds its template at r4+12; copies 3 words from global 0x8c1d3f68 into r4+0,+4,+8, zeroes byte r4+0x80, stores r5 at r4+0x84; returns r4 |
| `0x8c1842f0` | `FUN_8c1842f0` | `notes/verifications/0x8c1842f0.json` — 113 oracle steps; release 0x8c018bd8 on the handle at r4+0x54 with count -1 (clears the handle when a reference remains, destroys the object at refcount zero), then frees r4 through 0x8c01032c when the sign-extended low half of r5 is positive; returns r4 |
| `0x8c184260` | `unknown` | `notes/verifications/0x8c184260.json` — 20 oracle steps |
| `0x8c184324` | `FUN_8c184324` | `notes/verifications/0x8c184324.json` — 17 oracle steps; bounds-checked 8-byte record copy: count at [r4+0x138], records at r4+0x13c with stride 8; copies record r6 to r5 when r6 is in range, otherwise zeroes the 8 bytes at r5 through memset 0x8c010f88; returns via rts |
| `0x8c18db84` | `unknown` | `notes/verifications/0x8c18db84.json` — 48 oracle steps |
| `0x8c18bd70` | `copy_assign_record` | `notes/verifications/0x8c18bd70.json` — 139 oracle steps; callee 0x8c10ef64 recopies the +12 string field (r4=dstbuf, r5=15, r6=15, r7=dstbuf+15, r3=0); T=1; r0=destination; r13/r14 restored |
| `0x8c183e04` | `FUN_8c183e04` | `notes/verifications/0x8c183e04.json` — 179 oracle steps; scans count=[r4+4] elements at base=[r4+8]; per element calls 0x8c070b0c(name=[*[*p]+12], param_2), returns *p+4 when the compare is 0 else 0; r12-r14/pr restored from the four-word push; T=1 on both exits; no non-stack stores |
| `0x8c18b71c` | `FUN_8c18b71c` | `notes/verifications/0x8c18b71c.json` — 115 oracle steps; if r4!=0: clears u32 at 0x8c2b7578 (jsr delay slot), releases the refcounted handle r4 with count -1 via 0x8c018bd8, and when exts.w(r5) > 0 frees r4 via thunk 0x8c01032c -> 0x8c159fc0; returns r4 (0 on the null path); r13/r14/r15 and pr restored; T=1 on the null path, cmp/pl T=0 on the non-positive path, callee T on the free path |
| `0x8c1b67fc` | `FUN_8c1b67fc` | `notes/verifications/0x8c1b67fc.json` — 216 oracle steps; calls 0x8c16ee30 with fr4=0.0 for 7 words at [r4+0x1c..0x34] then 4 at [r4+0x38..0x44]; sets [r4]=1 |
| `0x8c1bb3f0` | `FUN_8c1bb3f0` | `notes/verifications/0x8c1bb3f0.json` — 9 oracle steps; calls 0x8c01032c with r4 unchanged when r4 != 0 and (int16)r5 > 0; returns r4 |
| `0x8c1bb480` | `FUN_8c1bb480` | `notes/verifications/0x8c1bb480.json` — 71 oracle steps; calls 0x8c1bb6a8 to init the 16-byte node at r4 and 0x8c1bb8d0 to init the node at r4+0x10, each fed the byte copied from the uninitialised frame at sp+8 / sp+6 down to sp+5 / sp+4; stores r4 to [0x8c2b82b0] |
| `0x8c1b9414` | `FUN_8c1b9414` | `notes/verifications/0x8c1b9414.json` — 67 oracle steps; +0 vtable 0x8c1f1b8c (after 0x8c1eb6a0 then 0x8c1f1ba4), +4..+12 copy of 0x8c2121e0 words 0..2, +16 u32 0, +20 u8 0, +24 = this, +28 arg2, +32 u8 1 and +36 *0x8c206c28 via 0x8c1261d4, +40/+42/+44/+46 u16 0, global 0x8c2ab5e0 = this+28; r0=this, r1=template word0, r3=this+4, r4=0x8c2121e0, r5=entry_sp-16, r6=this, r7=0, r15/pr/sr restored |
| `0x8c1b5c7c` | `FUN_8c1b5c7c` | `notes/verifications/0x8c1b5c7c.json` — 131 oracle steps; calls 0x8c018bd8 with r5=-1 to release a reference, then 0x8c01032c free(handle) when sext16(r5) > 0; returns the handle in r0 |
| `0x8c1bb2a8` | `FUN_8c1bb2a8` | `notes/verifications/0x8c1bb2a8.json` — 12 oracle steps; release(&p[1],-1); release(p,-1); free(p) when (int16)flag>0; r0=p, r13/r14/pr restored; T=1 iff p==NULL |
| `0x8c1bb264` | `FUN_8c1bb264` | `notes/verifications/0x8c1bb264.json` — 12 oracle steps; releases *handle through 0x8c018bd8(handle,-1); when sext16(r5) > 0 frees the handle through 0x8c01032c; writes the refcount, the handle slot and the dlmalloc block |
| `0x8c1af164` | `FUN_8c1af164` | `notes/verifications/0x8c1af164.json` — 97 oracle steps; free(mem[r4+8]) when non-NULL; walks ptr+mem[r4+4]*32 in 32-byte steps without using the result; the 0x8c01032c thunk tail-calls free (0x8c159fc0), which rewrites the block header, top chunk and bin pointers |
| `0x8c1b69ec` | `FUN_8c1b69ec` | `notes/verifications/0x8c1b69ec.json` — 416 oracle steps; submits 7 handles from [r4+28..52] then 4 from [r4+56..68] to 0x8c16eaec with fr4/fr5/fr6 read from the caller's stack words 0/1/2; each flagged handle gets the three floats staged at +60/64/68, copied to +72/76/80, and 0x01800000 ORed into +8; r12/r13/r14/pr/fr14/fr15 restored, r0=4, T=0 |
| `0x8c1af3b4` | `FUN_8c1af3b4` | `notes/verifications/0x8c1af3b4.json` — 85 oracle steps; frees the block at [r4+8] when non-zero via thunk 0x8c01032c -> free 0x8c159fc0; [r4+4] only feeds a dead 32-byte-stride walk into r3 |
| `0x8c21cda6` | `FUN_8c21cda6` | `notes/verifications/0x8c21cda6.json` — 17 oracle steps; calls 0x8c21a036 to read *0x8c2b8dcc; returns *(int*)(0x8c23c520+n*4) when n>0 else n; T=(n>0) signed; saves/restores pr; r3=0x8c21a036 when not taken |
| `0x8c222c3a` | `FUN_8c222c3a` | `notes/verifications/0x8c222c3a.json` — 18 oracle steps; clears record[8..12] and *(u32*)(record+4) at 0x8c2c02b8+0x148*r4 when record[2]!=0 and returns 0, else returns 0xffffff01; sets MACH:MACL from the signed 0x148*r4 product; T = (record[2]==0) |
| `0x8c220cd8` | `FUN_8c220cd8` | `notes/verifications/0x8c220cd8.json` — 38 oracle steps; raises IMASK to 15 around set_indexed_word_if_in_range(*(0x8c23cc68), 0), then restores the saved IMASK nibble |
| `0x8c21d398` | `FUN_8c21d398` | `notes/verifications/0x8c21d398.json` — 34543 oracle steps; memset(0x8c2bae60,0,0x1af8) via 0x8c010f88 then zero u32 at 0x8c2bc958, 0x8c2bc95c, 0x8c2bc960; r0 ends 0x8c2bc960, r1/r4/r5 0, r2 0x8c2bc958, r3 0x8c2bc95c, r6/r7 0x1af8; T set |
| `0x8c220ef2` | `FUN_8c220ef2` | `notes/verifications/0x8c220ef2.json` — 62 oracle steps; slot bring-up: first callee's +4 word nonzero -> r0 = 0xffffffff and no stores; else stores +8 = 1, +4 = 1 and zeros at +20..+40, and returns 0; T is left by the last callee's shll (deep) or by tst r0,r0 (busy); r13/r14/pr come back at entry values |
| `0x8c2205d0` | `FUN_8c2205d0` | `notes/verifications/0x8c2205d0.json` — 99 oracle steps; calls 0x8c2205b0 for each of the 4 records at *0x8c23d468 + 4 + 0x34*n (clears base[id]+28 with base = **0x8c23d468), then clears *(0x8c2bde5c); returns 0, restores r11-r14/pr, T ends set |
| `0x8c2206c0` | `FUN_8c2206c0` | `notes/verifications/0x8c2206c0.json` — 175 oracle steps; four calls to 0x8c220678 arm the records at [0x8c23d468]+4+{0,52,104,156} (each stores 1 at pool+id*32+28), then 1 -> 0x8c2bde5c and return 0; r1 ends 0xffffff0f, r2 1, r3 0x8c2bde5c, r4 = last pool target 0x8c250060, r5 = (sr>>4)&15; r11-r14/pr restored; T ends 1 from the exit cmp/hs |
| `0x8c221392` | `FUN_8c221392` | `notes/verifications/0x8c221392.json` — 471 oracle steps; rejects with -1 when 0x8c221da8 finds slot r4 occupied; else writes key 7 into the 128-byte slot at [0x8c2bde98], fills +20..+40 from r4/r5 and zero stack args via 0x8c221dc8, then copies up to 12 bytes from r5 into slot+44 and slot+60 via 0x8c221e18 |
| `0x8c21d6bc` | `FUN_8c21d6bc` | `notes/verifications/0x8c21d6bc.json` — 135 oracle steps; writes [base+0xd78]+0x14=[base+0x204]+1, a 152-byte saved CPU context at [base+0xd78]+0x18, and 1 to 0x8c23d890, then restores the base+4 frame (all GPRs, FR0-FR15, FPUL, FPSCR, MACH, MACL, GBR; r0=1, T clear) and returns via the frame's PR; calls 0x8c011168 then 0x8c010ddc |
| `0x8c21f9d8` | `FUN_8c21f9d8` | `notes/verifications/0x8c21f9d8.json` — 34 oracle steps; stores the low byte of r4 to [[0x8c2bdcb4]+12] (mode 0) or [[0x8c2bdcb0]+12] (mode 1), then masks status bits at +16; no calls |
| `0x8c0c9d90` | `FUN_8c0c9d90` | `notes/verifications/0x8c0c9d90.json` — 13 oracle steps; when [r4+4] != 0 calls 0x8c103f94 to store low byte of r5 at [r4+4]; T set on null path |
| `0x8c0c03a4` | `FUN_8c0c03a4` | `notes/verifications/0x8c0c03a4.json` — 199 oracle steps; frees header[1] through 0x8c0c0614, zeroes header[0..1], stores &header[1] at header[3] |
| `0x8c0bfb4c` | `FUN_8c0bfb4c` | `notes/verifications/0x8c0bfb4c.json` — 58 oracle steps; calls 0x8c0c06e4 to find the key node at [param_1+0x1c], stores param_1+0x20 through 0x8c0bef98, returns 0 when the found pointer equals param_1+0x20 else [node+0x10] |
| `0x8c0c5580` | `FUN_8c0c5580` | `notes/verifications/0x8c0c5580.json` — 504 oracle steps; calls 0x8c05d378 (vtable/fields init), 0x8c05d274(param_1,1,1) and 0x8c011344 (strncpy 64 bytes to param_1+72); stores 0x8c1eb940, 0x8c1e8b54 then 0x8c1eb798 at [param_1], copies param_4[0..1] to +56/+64 and param_3[0..3] to +88, returns param_1 |
| `0x8c07fe0c` | `FUN_8c07fe0c` | `notes/verifications/0x8c07fe0c.json` — 12 oracle steps; destroys the ring at container+4 through 0x8c0802f0 (frees count-1 elements and wraps), frees the item array at container+16 when non-NULL through the 0x8c01032c free thunk, calls 0x8c01e078(container+4, 0) which always early-exits, and frees the container itself when exts.w(flag) > 0; returns the container in r0 and leaves T = (exts.w(flag) > 0), or the entry tst T=1 for a NULL container |
| `0x8c0bdc0c` | `FUN_8c0bdc0c` | `notes/verifications/0x8c0bdc0c.json` — 110 oracle steps; loops over *(param_1+0xc) 108-byte records at *(param_1+0x10); each iteration calls 0x8c0c0328 with r4=record+8, r5=record+0x14, r6=param_2, r7=param_3 and two zeroed float out-slots pushed on the stack; returns the first successful index in r0 else 0xffffffff; callee advances r4..r7 by 4, mul.l leaves 108*i in MACH/MACL, r11..r14 end at entry values; T clear on both exits |
| `0x8c0bdcd0` | `FUN_8c0bdcd0` | `notes/verifications/0x8c0bdcd0.json` — 235 oracle steps; reads the descriptor at r4 (signed count at +12, record table at +16); for each i < count derives record table+i*108+32, reads [record+36] then [[record+36]+12] as a C string and compares needle r5 against the string tail starting at str+strlen(str)-strlen(needle); returns the first index whose tail matches, else -1 (also -1 when count <= 0); mul.l writes MACH/MACL = i*108 per tried record; exit r0 = index/-1, r3 = 0 on a match else the last wrapping unsigned byte difference, r4 = last needle byte, r5 = needle advanced past the last compared byte, r9-r14/pr restored from the frame; T=1 on a match, T=0 on -1 |
| `0x8c0c8cfc` | `FUN_8c0c8cfc` | `notes/verifications/0x8c0c8cfc.json` — 12 oracle steps; releases one reference on the refcounted handle through 0x8c018bd8 with count -1, then, when exts.w(low 16 bits of the flag) > 0, frees the handle slot itself through the 0x8c01032c free thunk; returns the handle in r0; r13, r14 and pr are restored from the frame; T is the entry tst result (1) on the NULL path, cmp/pl result when the free is skipped, and the free callee's T otherwise |
| `0x8c09d280` | `FUN_8c09d280` | `notes/verifications/0x8c09d280.json` — 442 oracle steps; releases handle[1] via 0x8c018bd8 with count -1 (decrements the pointee refcount; at zero frees inner then pointee through 0x8c01032c and keeps the slot, otherwise clears it), then decrements *handle's refcount the same way but destroys inner, (declined) guarded-free and the object through 0x8c018e14/0x8c01032c at zero, and finally frees the handle itself through 0x8c01032c only when sext16(r5)>0; stores refcounts, handle slots and dlmalloc chunk headers; returns the handle |
| `0x8c0bad3c` | `FUN_8c0bad3c` | `notes/verifications/0x8c0bad3c.json` — 176 oracle steps; frees the 0x8c279efc chain then the 0x8c279ef8 chain via 0x8c159fc0 and zeroes both heads |
| `0x8c02746c` | `FUN_8c02746c` | `notes/verifications/0x8c02746c.json` — 168 oracle steps; writes u32 at 0x178+idx and 0x174+idx, u8 at 0x170+idx, u32 at 0x168+idx; calls 0x8c0bfb4c twice |
| `0x8c0cd048` | `FUN_8c0cd048` | `notes/verifications/0x8c0cd048.json` — 343 oracle steps; writes u32 at [r4+4], [p+4], [q], [q+24], [p+36] where p=[r4], q=[p], and conditionally [r4+16]; frees [r4+8] and [r4+12], calls 0x8c07c100 on r4+44, conditionally frees r4 |
| `0x8c0c5f5c` | `FUN_8c0c5f5c` | `notes/verifications/0x8c0c5f5c.json` — 117 oracle steps; writes [r4+0x00], [r4+0x38..0x54]; calls 0x8c05d378 then 0x8c05d274 |
| `0x8c0d3258` | `FUN_8c0d3258` | `notes/verifications/0x8c0d3258.json` — 1335 oracle steps; unlinks/destroys nodes matching "instancetree" via 0x8c05cb48 (which stamps depths via 0x8c05caac); recurses into non-matching nodes' children |
| `0x8c0d43c0` | `FUN_8c0d43c0` | `notes/verifications/0x8c0d43c0.json` — 409 oracle steps; clears/tears down each 0x24-byte slot via 0x8c0d357c(-1), frees descriptor->array via 0x8c01032c |
| `0x8c0c5714` | `FUN_8c0c5714` | `notes/verifications/0x8c0c5714.json` — 125 oracle steps; initialises the object via 0x8c05d378, stamps 0x8c1eb940 then 0x8c1e8b54 then 0x8c1eb828/0x8c1eb7dc, sets bit 1 of [obj+16] via 0x8c05d274, copies two int pairs and three floats into obj+56..obj+84 |
| `0x8c0c5e84` | `FUN_8c0c5e84` | `notes/verifications/0x8c0c5e84.json` — 215 oracle steps; steps f32[r4+0x50] by ((fr14*(f32[+0x4c]-f32[+0x48]))/3.0)*fr15, weighted by gates 0x8c074ca0(*[0x8c264da8],8/2/3) and 0x8c074c54(,0,0); clamps to [f32[+0x48], f32[+0x4c]]; stores 3x f32 at r4+0x50 |
| `0x8c0d0274` | `reset_six_indexed_fields` | `notes/verifications/0x8c0d0274.json` — 91 oracle steps; if 0<=r5<4: r14 = fn_8c0c0ecc([0x8c27a238]); then six fn_8c16eadc stores of r14 through *(*r4+{0x4c,0xc0,0x134,0x1a8,0x21c,0x290}) + 0x5c; T at exit = (0<=r5<4) |
| `0x8c04d3dc` | `FUN_8c04d3dc` | `notes/verifications/0x8c04d3dc.json` — 243 oracle steps; calls 0x8c0bfb4c twice (key 1), 0x8c0c010c, 0x8c0c0100, then 0x8c0b4f94 (dst = frame) and 0x8c0b4ef0 (scale 0.5); writes 3 floats through the 5th-argument pointer |
| `0x8c0b90f4` | `FUN_8c0b90f4` | `notes/verifications/0x8c0b90f4.json` — 141 oracle steps; unlink matching list nodes and free them; recursive descent into kind=-1 nodes; patches list head, prev link and successor |
| `0x8c0ed80c` | `FUN_8c0ed80c` | `notes/verifications/0x8c0ed80c.json` — 51 oracle steps; zeros r4+0/4/8/12/16 and r4+44..+76; 0x8c0b4ff0 zeroes r4+20/24/28 and r4+32/36/40, fr0=0; r0=76, r1=0, r14/r15/pr restored |
| `0x8c0f87d8` | `FUN_8c0f87d8` | `notes/verifications/0x8c0f87d8.json` — 16 oracle steps; calls 0x8c14fe34 when (r5&0xff)!=0 else 0x8c14fe68, argument r4+0x2c; r0 = [[r4+0x30]+0x1c] or [[r4+0x30]+0x10]; r4 += 0x2c; T=(r5&0xff)==0; pr/r15 restored |
| `0x8c0f8828` | `FUN_8c0f8828` | `notes/verifications/0x8c0f8828.json` — 16 oracle steps; calls 0x8c14fe10 when (r5&0xff)!=0 else 0x8c14fe68 with argument r4+0x2c; r0 = [[arg+4]+0x1c] or [[arg+4]+0x10]; r4 += 0x2c; T=(r5&0xff)==0; pr/r15 restored |
| `0x8c0ed79c` | `FUN_8c0ed79c` | `notes/verifications/0x8c0ed79c.json` — 192 oracle steps; writes vtable 0x8c1ec934 then 0x8c1ec920 at *p; calls 0x8c010d8c with fr4=0.5 and stores 16.0/log10(0.5) = 0xc2549a78 at p+4 |
| `0x8c0ed764` | `FUN_8c0ed764` | `notes/verifications/0x8c0ed764.json` — 96 oracle steps; writes vtable word 0x8c1ec920 then 0x8c1ec934 at *p (literal pool read in order); if sign-extended low 16 bits of r5 > 0, calls 0x8c01032c -> free(p); returns p (or 0) |
| `0x8c0ccdd0` | `FUN_8c0ccdd0` | `notes/verifications/0x8c0ccdd0.json` — 293 oracle steps; writes [r4+8]=0x8c1ebe10; with p=[r4], q=[p], c=[q], d=[r4+4]: [p+4]=0x8c1ebee4, [q+4]=0x8c1ebec4, [c]=0x8c1ebe48, [d+24]=r4+12-d, [q+36]=r4+40-q, [p+16]=r4+80-p; (i16)r5!=0: teardown(r4+80,0) via 0x8c0cd048, destroy(r4+40,0) via 0x8c07c100, [r4+12]=0x8c1e9ad4; (i16)r5>0: free(r4) via 0x8c01032c; returns r4 |
| `0x8c09f00c` | `FUN_8c09f00c` | `notes/verifications/0x8c09f00c.json` — 51 oracle steps; writes class word 0x8c1eed70 at r4+8, lit+0x38/at [r4], lit+0xd4 at [[r4+4]+4], lit+0xb4 at [[[r4+4]]+4], relative offsets at [r4]+24 / [[[r4+4]]]+36 / [[r4+4]]+16; if (i16)r5!=0 calls 0x8c0cd048(r4+0x50,0) then 0x8c07c100(r4+0x28,0) and writes 0x8c1e9ad4 at r4+0xc; if (i16)r5>0 calls 0x8c01032c(r4); r0=r4; T=1 on r4==0 or 0 elsewhere (free residue on r5>0); r13/r14/pr restored |
| `0x8c0f3fd0` | `FUN_8c0f3fd0` | `notes/verifications/0x8c0f3fd0.json` — 17 oracle steps; frees every node from sentinel->next up to (not including) the sentinel at *r4 through 0x8c01032c -> free(0x8c159fc0), then writes sentinel->next = sentinel and sentinel->prev = sentinel; the successor is read in the jsr delay slot before free; r0 ends as the sentinel, T=1 at exit |
| `0x8c0ed6b8` | `FUN_8c0ed6b8` | `notes/verifications/0x8c0ed6b8.json` — 210 oracle steps; calls 0x8c010d8c (may store 0x44c/0x44d at 0x8c23d8b8); fr0 = clamp(f32[r4+4]*scale, 0, fr2); final r0=r6&0xff, r1=fpul=1 shld r5, T from the last fcmp, fr2/fr3/clamp arm as transcribed; r12-r14/pr restored by the frame |
| `0x8c0fe9fc` | `FUN_8c0fe9fc` | `notes/verifications/0x8c0fe9fc.json` — 21 oracle steps; calls 0x8c0b5e78 (stores 0x8c1eaaac to [r4], dead); then [r4]=0x8c1ed104 and zero [+4], [+28], [+36], [+48]; returns r4 in r0, r1=0; r14/pr restored, T untouched |
| `0x8c0fc940` | `FUN_8c0fc940` | `notes/verifications/0x8c0fc940.json` — 257 oracle steps; decrements refcount at [*handle+8] (lazily 1 if 0); when new count != 0 clears *handle to 0, else frees [*handle+12] via 0x8c01032c, calls 0x8c018e14(*handle+4,-1) which declines, then frees *handle via 0x8c01032c; if (i16)low16(r5) > 0 frees handle via 0x8c01032c; returns handle; T=cmp/pl result or callee T after final free |
| `0x8c104544` | `FUN_8c104544` | `notes/verifications/0x8c104544.json` — 54 oracle steps; calls 0x8c1044f4(r4,[r4+0x44]) via registry (pr=0x8c104552); when result r7 != 0 and [ [r7+0x14] ] == 1 writes [r6+4] to 0x8c2ab350, [r6+8] to 0x8c2ab354, [r6+12] to 0x8c2ab358 and f32 [r6+16] to 0x8c2ab35c, then f32 0x8c203da0 / f32 0x8c2ab35c to 0x8c203d98; returns r7, or 0 on either reject arm; T=1 on null arm (tst r7,r7), T=0 on header-mismatch arm (cmp/eq #1,r0), T=1 on success |
| `0x8c0f9aa4` | `thunk_FUN_8c0f9ab2` | `notes/verifications/0x8c0f9aa4.json` — 56 oracle steps; bra 0x8c0f9ab2 with nop delay slot; pr untouched, so the callee rts returns to the caller; target is the loop body entry — the 0x8c0f9aa8 fall-through block is only reached via bt.s with r3/T computed, entry r3/T unused |
| `0x8c0fee44` | `FUN_8c0fee44` | `notes/verifications/0x8c0fee44.json` — 275 oracle steps; calls 0x8c018bd8(handle,-1) via registry when handle != 0, then frees the handle via 0x8c01032c -> free(0x8c159fc0) when exts.w(r5) > 0; returns the handle in r0 and restores r13/r14/pr; T=1 on the null arm (tst r13,r13), T=0 on a non-positive count (cmp/pl), else free's residue (T=0 on every non-null free) |
| `0x8c0fc5ec` | `FUN_8c0fc5ec` | `notes/verifications/0x8c0fc5ec.json` — 199 oracle steps; decrements refcount [[r4+4]+8] (bumping a zero to 1 first), zeroes [r4+4] when the count remains, and when it reaches zero frees [[r4+4]+12], calls 0x8c018e14([r4+4]+4,-1) (declines), frees [r4+4]; then frees r4 when (i16)low16(r5) > 0; returns r4; T = cmp/pl except when the last free runs (callee T) |
| `0x8c23a800` | `FUN_8c23a800` | `notes/verifications/0x8c23a800.json` — 44 oracle steps; calls 0x8c237780(r4=param_1); if hit and param_2!=0 stores param_2 at [param_1+0x14] and returns 0, else returns 0x0000eeee; T=1 on both sentinel exits, 0 on store exit |
| `0x8c23a340` | `FUN_8c23a340` | `notes/verifications/0x8c23a340.json` — 57 oracle steps; writes fr4 to [p+0xc] where p = [[r4+0x88]] on a table hit; returns 0 on hit, 0xeeee on miss |
| `0x8c236df8` | `FUN_8c236df8` | `notes/verifications/0x8c236df8.json` — 51 oracle steps; calls 0x8c237780(r4=param_1); on a hit stores [param_1+0x3c] through param_2 and returns 0, on a miss returns 0x0000eeee and writes nothing; T=1 on miss, 0 on hit |
| `0x8c23b100` | `FUN_8c23b100` | `notes/verifications/0x8c23b100.json` — 12 oracle steps; reads *r4 into r6, then by r5 mode: 1 -> if r6&0x3000: r6=(r6&0xffffff4f)|0x40; 3 -> if r6&0x30: r6=(r6&0xffff4fff)|0x4000; 4 -> if r6&0x300: r6=(r6&0xfffcffff)|0x40000; always stores r6 back to *r4; r7=0x0000eeee, r0 returns 0x0000eeee on no-op paths and 0 on rewritten ones, r14 pushed/restored |
| `0x8c237680` | `FUN_8c237680` | `notes/verifications/0x8c237680.json` — 58 oracle steps; calls 0x8c237780 with r4=arg1; table miss returns 0xeeee; on hit reloads arg1 and, when arg2 != 0, copies words arg1+24/28/44/48/64/68 into arg2[0..6] and returns 0; null arg2 returns 0xeeee after the delay-slot arg1+24; r13/r14/pr/r15 restored |
| `0x8c239882` | `FUN_8c239882` | `notes/verifications/0x8c239882.json` — 85 oracle steps; claims the first free bit of the 8-bit mask at r4+0x248: sets it, increments the count at r4+0x24c, writes the index to r4+0x254+44*i, stores that slot address to *r5 and returns 0; a full table returns 0 with r4=8, r2=0x80, r3=mask and macl/mach untouched; r4==0 returns 0x0000eeee; T=1 on every exit; a claim sets macl=44*i and mach=0; r6=base, r7=slot address on claim / 1 on full; r13/r14 pushed and restored |
| `0x8c239a46` | `FUN_8c239a46` | `notes/verifications/0x8c239a46.json` — 59 oracle steps; calls 0x8c237780(r4=param_1); on a hit resolves the object at [param_1+0x88], sets bit 3 of [obj+4] when param_2!=0 and clears it when param_2==0, returns 0; on a miss returns 0x0000eeee and writes nothing; T=1 on miss, T=(param_2==0) on hit |
| `0x8c236da0` | `FUN_8c236da0` | `notes/verifications/0x8c236da0.json` — 64 oracle steps; calls 0x8c237780(arg1); on hit stores (mode<<24 | ([p+8] & 0xffffff)) to [p+8] where p=[arg1+0x88] and mode is built from arg2 bits 0x2000000/0x1000000/0x400000 (r14 starts 1, add #1 in each bt.s delay slot / fall-through, and #7), returns 0; on miss returns 0xeeee and stores nothing; T is the last tst the path executed (miss 1, hit with 0x2000000/0x400000 set 0, else 1) |
| `0x8c237700` | `FUN_8c237700` | `notes/verifications/0x8c237700.json` — 60 oracle steps; calls 0x8c237780(r4=param_1); on a hit with param_2 < [param_1+12] loads [param_1+0x48+param_2*8] (32-bit wrapping scale), stores it through param_3 raw and again masked with 0xefffffff, and returns 0; lookup miss or out-of-range index returns 0x0000eeee and writes nothing; T=1 on failure, bit 29 of param_2 on success |
| `0x8c239060` | `FUN_8c239060` | `notes/verifications/0x8c239060.json` — 68 oracle steps; stores p+0x30 and p+0x2c; returns 0 or 0xeeee; r4 = p+24 on the non-early paths |
| `0x8c106874` | `strcmp_thunk_2` | `notes/verifications/0x8c106874*.json` - 3 specs (17-56 steps) |
| `0x8c10eaea` | `strcmp_body_3` | `notes/verifications/0x8c10eaea*.json` - 3 specs (15-54 steps) |
| `0x8c06de78` | `zero_then_clear_pair` | `notes/verifications/0x8c06de78.json` - 76 oracle steps |
| `0x8c10b6fc` | `indexed_tail_call` | `notes/verifications/0x8c10b6fc.json` - 18 oracle steps |
| `0x8c06ee08` | `zero_clear_and_fldi0` | `notes/verifications/0x8c06ee08.json` - 83 oracle steps |
| `0x8c10cf0c` | `circular_list_clear` | `notes/verifications/0x8c10cf0c*.json` - 3 specs (17-166 steps) |
| `0x8c10cf3c` | `circular_list_clear_2` | `notes/verifications/0x8c10cf3c*.json` - 3 specs (17-166 steps) |
| `0x8c10cf6c` | `circular_list_clear_3` | `notes/verifications/0x8c10cf6c*.json` - 3 specs (17-166 steps) |
| `0x8c138494` | `release_handle_array` | `notes/verifications/0x8c138494*.json` - 3 specs (21-60 steps) |
| `0x8c06f830` | `init_tail_and_sentinel` | `notes/verifications/0x8c06f830.json` - 99 oracle steps; stores -300.0 |
| `0x8c10eadc` | `strcmp_thunk_3` | `notes/verifications/0x8c10eadc*.json` - 3 specs (17-56 steps) |
| `0x8c13d528` | `lookup_at_plus_92` | `notes/verifications/0x8c13d528*.json` - 2 specs (9 and 19 steps) |
| `0x8c13d8b4` | `free_stride8_array` | `notes/verifications/0x8c13d8b4*.json` - 3 specs (7-73 steps) |
| `0x8c13da10` | `free_stride8_array_2` | `notes/verifications/0x8c13da10*.json` - 3 specs (7-73 steps) |
| `0x8c13844c` | `release_then_free_array` | `notes/verifications/0x8c13844c*.json` - 3 specs (15-123 steps) |
| `0x8c146d60` | `call_with_r14_one` | `notes/verifications/0x8c146d60.json` - 27 oracle steps |
| `0x8c14aeb0` | `init_subobject_stamp_minus_one` | `notes/verifications/0x8c14aeb0*.json` - 2 specs (29/23 steps) |
| `0x8c14b2c0` | `free_stride4_array` | `notes/verifications/0x8c14b2c0*.json` - 2 specs (72/7 steps) |
| `0x8c14b664` | `free_stride570_array` | `notes/verifications/0x8c14b664*.json` - 2 specs (82/7 steps) |
| `0x8c146640` | `hash_cached_field` | `notes/verifications/0x8c146640.json` - 23663 oracle steps |
| `0x8c138614` | `release_stride8_then_free` | `notes/verifications/0x8c138614*.json` - 2 specs (142/15 steps) |
| `0x8c13a63c` | `scan_records_call_flagged` | `notes/verifications/0x8c13a63c*.json` - 2 specs (59/16 steps) |
| `0x8c13ca80` | `guarded_release_stride76_then_free` | `notes/verifications/0x8c13ca80*.json` - 2 specs (106/15 steps) |
| `0x8c14ce1c` | `release_stride48_then_zero_count` | `notes/verifications/0x8c14ce1c*.json` - 2 specs (44/24 steps) |
| `0x8c14aeec` | `object_init_two_resets` | `notes/verifications/0x8c14aeec.json` - 52 oracle steps |
| `0x8c14ae5c` | `reset_subarray_stamp` | `notes/verifications/0x8c14ae5c*.json` - 2 specs (38/58 steps) |
| `0x8c14d1e4` | `free_stride4_array_2` | `notes/verifications/0x8c14d1e4*.json` - 2 specs (72/7 steps) |
| `0x8c150818` | `clear_global_then_free_2` | `notes/verifications/0x8c150818*.json` - 3 specs (78/15/9 steps) |
| `0x8c151340` | `tail_thunk_0x8c0b4f34` | `notes/verifications/0x8c151340.json` - 22 oracle steps |
| `0x8c1514f8` | `stamp_vtable_then_free_2` | `notes/verifications/0x8c1514f8*.json` - 2 specs (77/14 steps) |
| `0x8c150844` | `free_inner_then_object` | `notes/verifications/0x8c150844*.json` - 3 specs (141/78/12 steps) |
| `0x8c151524` | `vtable_and_three_resets` | `notes/verifications/0x8c151524.json` - 51 oracle steps |
| `0x8c14d410` | `copy_records_48` | `notes/verifications/0x8c14d410*.json` - 2 specs (123/15 steps) |
| `0x8c14e708` | `copy_records_48_twin` | `notes/verifications/0x8c14e708*.json` - 2 specs (123/15 steps) |
| `0x8c14fe3c` | `indexed_field_12_load` | `notes/verifications/0x8c14fe3c*.json` - 2 specs (21/10 steps) |
| `0x8c151ae8` | `scale_vector_into_frame_merge` | `notes/verifications/0x8c151ae8.json` - 61 oracle steps |
| `0x8c152140` | `vtable_init_then_free` | `notes/verifications/0x8c152140*.json` - 2 specs (97/34 steps) |
| `0x8c154000` | `free_first_word_then_object` | `notes/verifications/0x8c154000*.json` - 2 specs (141/78 steps) |
| `0x8c0f87b0` | `dispatch_lookup_18_or_3c` | `notes/verifications/0x8c0f87b0*.json` - 2 specs (23/31 steps) |
| `0x8c0f8800` | `dispatch_lookup_f4_or_3c` | `notes/verifications/0x8c0f8800*.json` - 2 specs (23/31 steps) |
| `0x8c15d148` | `double_indirect_tail_thunk` | `notes/verifications/0x8c15d148.json` - 10 oracle steps |
| `0x8c154e94` | `free_stride12_array` | `notes/verifications/0x8c154e94*.json` - 2 specs (74/7 steps) |
| `0x8c155e20` | `clear_global_then_free_3` | `notes/verifications/0x8c155e20*.json` - 2 specs (78/15 steps) |
| `0x8c1508cc` | `release_stride16_then_free` | `notes/verifications/0x8c1508cc*.json` - 2 specs (171/15 steps) |
| `0x8c151be0` | `vtable_init2_then_free` | `notes/verifications/0x8c151be0*.json` - 3 specs (117/54/12 steps) |
| `0x8c15217c` | `compound_init_quaternion` | `notes/verifications/0x8c15217c.json` - 112 oracle steps |
| `0x8c155fbc` | `flag_store_then_bitset_tail` | `notes/verifications/0x8c155fbc*.json` - 3 specs (27/27/27 steps) |
| `0x8c15667c` | `release_then_free_twin` | `notes/verifications/0x8c15667c*.json` - 3 specs (112/49/12 steps) |
| `0x8c15e3c8` | `triple_indirect_tail_thunk` | `notes/verifications/0x8c15e3c8.json` - 11 oracle steps |
| `0x8c15fb30` | `double_indirect_thunk_2` | `notes/verifications/0x8c15fb30.json` - 10 oracle steps |
| `0x8c165b50` | `seed_then_restamp_vtable` | `notes/verifications/0x8c165b50.json` - 20 oracle steps |
| `0x8c16628c` | `store_byte_init_sub73` | `notes/verifications/0x8c16628c*.json` - 2 specs (27/9 steps) |
| `0x8c1662a8` | `store_byte_store_sub96` | `notes/verifications/0x8c1662a8*.json` - 2 specs (15/9 steps) |
| `0x8c150878` | `clear_global_drain_free` | `notes/verifications/0x8c150878*.json` - 3 specs (257/194/12 steps) |
| `0x8c152e28` | `vtable_init3_then_free` | `notes/verifications/0x8c152e28*.json` - 3 specs (137/74/12 steps) |
| `0x8c154de4` | `clear_global_drain_free_2` | `notes/verifications/0x8c154de4*.json` - 3 specs (174/111/12 steps) |
| `0x8c16c984` | `stamp_vtable_then_free_3` | `notes/verifications/0x8c16c984*.json` - 3 specs (77/14/9 steps) |
| `0x8c16e200` | `memset_then_clear_flag` | `notes/verifications/0x8c16e200.json` - 5122 oracle steps |
| `0x8c0708c8` | `vtable_init4_then_free` | `notes/verifications/0x8c0708c8*.json` - 3 specs (157/94/12 steps) |
| `0x8c16f54c` | `maxint_stamp_then_memset` | `notes/verifications/0x8c16f54c.json` - 352 oracle steps |
| `0x8c174600` | `memset_tail_thunk_fb8` | `notes/verifications/0x8c174600.json` - 20132 oracle steps |
| `0x8c17390c` | `slot_install_sr_merge` | `notes/verifications/0x8c17390c.json` - 58 oracle steps |
| `0x8c1725fc` | `bounded_strcpy_dispatch` | `notes/verifications/0x8c1725fc*.json` - 3 specs (70/105/3543 steps) |
| `0x8c172b1a` | `flag_store_or_error_emit` | `notes/verifications/0x8c172b1a*.json` - 2 specs (10/78 steps) |
| `0x8c172ea4` | `three_way_null_dispatch` | `notes/verifications/0x8c172ea4*.json` - 3 specs (393/360/17 steps) |
| `0x8c172f5a` | `three_way_dispatch_twin` | `notes/verifications/0x8c172f5a*.json` - 3 specs (368/360/16 steps) |
| `0x8c172b64` | `flag_store_or_emit_twin` | `notes/verifications/0x8c172b64*.json` - 2 specs (10/78 steps) |
| `0x8c172ee2` | `two_way_null_dispatch` | `notes/verifications/0x8c172ee2*.json` - 3 specs (382/357/14 steps) |
| `0x8c18052c` | `circular_list_drain_2` | `notes/verifications/0x8c18052c*.json` - 3 specs (17/99/166 steps) |
| `0x8c182bb4` | `minimal_guarded_free` | `notes/verifications/0x8c182bb4*.json` - 3 specs (75/12/9 steps) |
| `0x8c182bd8` | `minimal_guarded_free_twin` | `notes/verifications/0x8c182bd8*.json` - 3 specs (75/12/9 steps) |
| `0x8c1732b8` | `flag_call_or_slot_load` | `notes/verifications/0x8c1732b8*.json` - 2 specs (363/10 steps) |
| `0x8c172f96` | `null_dispatch_const64` | `notes/verifications/0x8c172f96*.json` - 3 specs (393/357/13 steps) |
| `0x8c1745ca` | `memset_store_thunk_2` | `notes/verifications/0x8c1745ca.json` - 1295 oracle steps |
| `0x8c172662` | `memset_store_tail_thunk` | `notes/verifications/0x8c172662.json` - 1295 oracle steps |
| `0x8c1702fc` | `movt_test_thunk_twin` | `notes/verifications/0x8c1702fc.json` - 183 oracle steps |
| `0x8c1702ec` | `movt_test_thunk` | `notes/verifications/0x8c1702ec.json` - 107 oracle steps |
| `0x8c173954` | `conditional_slot_clear_sr` | `notes/verifications/0x8c173954*.json` - 2 specs (53/60 steps) |
| `0x8c1730fe` | `countdown_counter_loop` | `notes/verifications/0x8c1730fe*.json` - 2 specs (40/5 steps) |
| `0x8c173580` | `store_then_memset_call` | `notes/verifications/0x8c173580.json` - 1304 oracle steps |
| `0x8c170efa` | `bit_set_dispatch` | `notes/verifications/0x8c170efa*.json` - 4 specs (346/295/22/20 steps) |
| `0x8c173084` | `range_checked_dispatch_copy` | `notes/verifications/0x8c173084*.json` - 3 specs (304/469/34 steps) |
| `0x8c151c1c` | `compound_float_init` | `notes/verifications/0x8c151c1c.json` - 158 oracle steps |
| `0x8c133f14` | `vtable_init_copy_twin` | `notes/verifications/0x8c133f14.json` - 55 oracle steps |
| `0x8c132bbc` | `vtable_init_struct_copy` | `notes/verifications/0x8c132bbc.json` - 55 oracle steps |
| `0x8c161a38` | `insertion_sort` | `notes/verifications/0x8c161a38*.json` - 2 specs (154/109 steps) |
| `0x8c1730f2` | `countdown_loop_thunk` | `notes/verifications/0x8c1730f2*.json` - 2 specs (42/7 steps) |
| `0x8c161a98` | `three_element_sort_network` | `notes/verifications/0x8c161a98*.json` - 2 specs (124/80 steps) |
| `0x8c10d5f0` | `insertion_sort_deref` | `notes/verifications/0x8c10d5f0*.json` - 2 specs (104/116 steps) |
| `0x8c1365cc` | `vtable_init_shifted_copy` | `notes/verifications/0x8c1365cc.json` - 63 oracle steps |
| `0x8c111868` | `insertion_sort_field68` | `notes/verifications/0x8c111868*.json` - 2 specs (116/86 steps) |
| `0x8c103604` | `multi_call_fanout` | `notes/verifications/0x8c103604*.json` - 2 specs (91/79 steps) |
| `0x8c111ca0` | `three_slot_sort_field68` | `notes/verifications/0x8c111ca0*.json` - 2 specs (81/52 steps) |
| `0x8c0326d4` | `five_callee_teardown` | `notes/verifications/0x8c0326d4*.json` - 3 specs (869/806/14 steps) |
| `0x8c0f9a30` | `list_search_strcmp` | `notes/verifications/0x8c0f9a30*.json` - 3 specs (169/109/23 steps) |
| `0x8c0c9d84` | `fanout_tail_thunk` | `notes/verifications/0x8c0c9d84.json` - 94 oracle steps |
| `0x8c10d65c` | `three_stage_swap_deref` | `notes/verifications/0x8c10d65c*.json` - 2 specs (75/48 steps) |
| `0x8c182bfc` | `stack_byte_copy_call` | `notes/verifications/0x8c182bfc.json` - 41 oracle steps |
| `0x8c111648` | `guarded_insertion_sort` | `notes/verifications/0x8c111648*.json` - 3 specs (137/100/105 steps) |
| `0x8c1825d8` | `float_pipeline_init` | `notes/verifications/0x8c1825d8*.json` - 2 specs (40/40 steps) |
| `0x8c152e64` | `compound_fanout_5resets` | `notes/verifications/0x8c152e64.json` - 309 oracle steps |
| `0x8c12d298` | `object_zero_init` | `notes/verifications/0x8c12d298.json` - 58 oracle steps |
| `0x8c174564` | `bounded_strcpy_dispatch_2` | `notes/verifications/0x8c174564*.json` - 3 specs (70/105/3543 steps) |
| `0x8c173868` | `masked_copy_sr_merge` | `notes/verifications/0x8c173868*.json` - 2 specs (122/46 steps) |
| `0x8c10cae8` | `fixpoint_search` | `notes/verifications/0x8c10cae8*.json` - 2 specs (85/97 steps) |
| `0x8c10cb7c` | `fixpoint_search_twin` | `notes/verifications/0x8c10cb7c*.json` - 2 specs (85/97 steps) |
| `0x8c162a10` | `keyed_insertion_sort` | `notes/verifications/0x8c162a10*.json` - 2 specs (148/105 steps) |
| `0x8c070904` | `vtable_restamp_global_copy` | `notes/verifications/0x8c070904.json` - 353 oracle steps |
| `0x8c175918` | `flag_gate_movt_store` | `notes/verifications/0x8c175918*.json` - 3 specs (264/266/15 steps) |
| `0x8c172b3c` | `flag_gated_global_swap` | `notes/verifications/0x8c172b3c*.json` - 2 specs (13/78 steps) |
| `0x8c17466e` | `nested_flag_gate` | `notes/verifications/0x8c17466e*.json` - 3 specs (304/262/15 steps) |
| `0x8c1759e2` | `flag_gate_slot_read` | `notes/verifications/0x8c1759e2*.json` - 3 specs (272/266/13 steps) |
| `0x8c1759ae` | `flag_gate_plain_load` | `notes/verifications/0x8c1759ae*.json` - 3 specs (340/288/13 steps) |
| `0x8c175a16` | `flag_gate_field12` | `notes/verifications/0x8c175a16*.json` - 3 specs (224/244/13 steps) |
| `0x8c175a4a` | `flag_gate_field44` | `notes/verifications/0x8c175a4a*.json` - 3 specs (311/294/13 steps) |
| `0x8c175a7e` | `flag_gate_deref_field60` | `notes/verifications/0x8c175a7e*.json` - 3 specs (293/275/14 steps) |
| `0x8c176512` | `flag_reporter_field3dc` | `notes/verifications/0x8c176512*.json` - 5 specs (258/243/16/18/19 steps) |
| `0x8c177fa0` | `bounded_table_store` | `notes/verifications/0x8c177fa0*.json` - 2 specs (12/374 steps) |
| `0x8c177fc6` | `clear_stamp_tail_thunk` | `notes/verifications/0x8c177fc6.json` - 6453 oracle steps |
| `0x8c17678e` | `null_guarded_field_store` | `notes/verifications/0x8c17678e*.json` - 3 specs (290/292/11 steps) |
| `0x8c176580` | `report_field_nonzero` | `notes/verifications/0x8c176580*.json` - 4 specs (258/243/14/14 steps) |
| `0x8c1765b0` | `report_field_eq2` | `notes/verifications/0x8c1765b0*.json` - 4 specs (272/272/14/14 steps) |
| `0x8c1765e0` | `aligned_store_or_report` | `notes/verifications/0x8c1765e0*.json` - 4 specs (366/368/14/389 steps) |
| `0x8c111794` | `guarded_sort_lexcmp` | `notes/verifications/0x8c111794*.json` - 2 specs (136/97 steps) |
| `0x8c14b070` | `four_subobject_teardown` | `notes/verifications/0x8c14b070*.json` - 3 specs (155/92/12 steps) |
| `0x8c10e3e4` | `nine_handle_record_teardown` | `notes/verifications/0x8c10e3e4*.json` - 4 specs (470/407/14/83 steps) |
| `0x8c14fe94` | `subrecord_teardown` | `notes/verifications/0x8c14fe94*.json` - 4 specs (85/22/14/252 steps) |
| `0x8c16d690` | `table_init_20_records` | `notes/verifications/0x8c16d690.json` - 1239 oracle steps |
| `0x8c0bdb94` | `vtable_teardown_wrapper` | `notes/verifications/0x8c0bdb94*.json` - 3 specs (175/112/12 steps) |
| `0x8c146e8c` | `vtable_teardown_wrapper_2` | `notes/verifications/0x8c146e8c*.json` - 3 specs (175/112/12 steps) |
| `0x8c183b6c` | `zero_five_reset_sub` | `notes/verifications/0x8c183b6c.json` - 25 oracle steps |
| `0x8c176df2` | `stride52_slot_store` | `notes/verifications/0x8c176df2*.json` - 4 specs (324/384/18/20 steps) |
| `0x8c184e88` | `release_stride88_array` | `notes/verifications/0x8c184e88*.json` - 2 specs (138/23 steps) |
| `0x8c1996b4` | `indexed_field_address` | `notes/verifications/0x8c1996b4*.json` - 2 specs (6/6 steps) |
| `0x8c199dd0` | `minimal_guarded_free_2` | `notes/verifications/0x8c199dd0*.json` - 3 specs (75/12/9 steps) |
| `0x8c199fac` | `round_up_even_then_unit` | `notes/verifications/0x8c199fac*.json` - 4 specs (6/20/25/31 steps) |
| `0x8c19aea0` | `free_stride12_array_2` | `notes/verifications/0x8c19aea0*.json` - 2 specs (82/7 steps) |
| `0x8c19b624` | `float_tail_thunk_16ee30` | `notes/verifications/0x8c19b624.json` - 12 oracle steps |
| `0x8c19781c` | `swap_subobject_60` | `notes/verifications/0x8c19781c*.json` - 2 specs (20/13 steps) |
| `0x8c19b634` | `cache_float_then_reset` | `notes/verifications/0x8c19b634.json` - 29 oracle steps |
| `0x8c19a4b0` | `release_stride_e4_array` | `notes/verifications/0x8c19a4b0*.json` - 2 specs (44/23 steps) |
| `0x8c19d258` | `circular_list_drain_3` | `notes/verifications/0x8c19d258*.json` - 2 specs (84/17 steps) |
| `0x8c19d370` | `stamp_then_free_2` | `notes/verifications/0x8c19d370*.json` - 3 specs (77/14/9 steps) |
| `0x8c1959e4` | `zero_fields_copy_frame` | `notes/verifications/0x8c1959e4.json` - 33 oracle steps |
| `0x8c19d45c` | `refcount_dec_clear_bits` | `notes/verifications/0x8c19d45c*.json` - 3 specs (12/12/38 steps) |
| `0x8c19d48c` | `refcount_inc_init_bits` | `notes/verifications/0x8c19d48c*.json` - 2 specs (36/12 steps) |
| `0x8c19ed28` | `circular_list_drain_4` | `notes/verifications/0x8c19ed28*.json` - 2 specs (84/17 steps) |
| `0x8c1a6748` | `minimal_guarded_free_3` | `notes/verifications/0x8c1a6748*.json` - 3 specs (75/12/9 steps) |
| `0x8c1ade24` | `minimal_guarded_free_4` | `notes/verifications/0x8c1ade24*.json` - 3 specs (75/12/9 steps) |
| `0x8c1ade48` | `minimal_guarded_free_5` | `notes/verifications/0x8c1ade48*.json` - 3 specs (75/12/9 steps) |
| `0x8c1ade78` | `minimal_guarded_free_6` | `notes/verifications/0x8c1ade78*.json` - 3 specs (75/12/9 steps) |
| `0x8c1ade9c` | `minimal_guarded_free_7` | `notes/verifications/0x8c1ade9c*.json` - 3 specs (75/12/9 steps) |
| `0x8c1ac718` | `clear_field_then_free` | `notes/verifications/0x8c1ac718*.json` - 3 specs (77/14/9 steps) |
| `0x8c1cc5dc` | `clear_global_then_free_4` | `notes/verifications/0x8c1cc5dc*.json` - 3 specs (78/15/9 steps) |
| `0x8c1beb34` | `free_stride92_array` | `notes/verifications/0x8c1beb34*.json` - 2 specs (82/7 steps) |
| `0x8c1cbb14` | `circular_list_drain_5` | `notes/verifications/0x8c1cbb14*.json` - 2 specs (84/17 steps) |
| `0x8c03ed5c` | `flag_then_compound_thunk` | `notes/verifications/0x8c03ed5c.json` - 39 oracle steps |
| `0x8c026e84` | `flag_then_thunk_through_field` | `notes/verifications/0x8c026e84.json` - 46 oracle steps |
| `0x8c184e3c` | `release_stride88_then_free` | `notes/verifications/0x8c184e3c*.json` - 2 specs (144/15 steps) |
| `0x8c175c28` | `three_exit_slot_fill` | `notes/verifications/0x8c175c28*.json` - 2 specs (278/401 steps) |
| `0x8c176d7e` | `stride52_pair_store` | `notes/verifications/0x8c176d7e*.json` - 4 specs (348/397/24/26 steps) |
| `0x8c177c3c` | `enum_to_code_switch` | `notes/verifications/0x8c177c3c*.json` - 5 specs (336/14/17/20/336 steps) |
| `0x8c146ca0` | `release_40_slots_then_free` | `notes/verifications/0x8c146ca0*.json` - 3 specs (629/566/18 steps) |
| `0x8c1831b8` | `table_entry_equality` | `notes/verifications/0x8c1831b8*.json` - 3 specs (24/24/32 steps) |
| `0x8c19ae24` | `double_stamp_subarray_free` | `notes/verifications/0x8c19ae24*.json` - 3 specs (100/37/12 steps) |
| `0x8c184c20` | `clear_field_subarray_free` | `notes/verifications/0x8c184c20*.json` - 3 specs (107/44/12 steps) |
| `0x8c19c95c` | `clear_pair_then_refcount` | `notes/verifications/0x8c19c95c.json` - 55 oracle steps |
| `0x8c1ae7d8` | `zero_three_publish_global` | `notes/verifications/0x8c1ae7d8.json` - 20 oracle steps |
| `0x8c193618` | `release_stride28_then_free` | `notes/verifications/0x8c193618*.json` - 2 specs (87/15 steps) |
| `0x8c19a45c` | `release_stride_e4_then_free` | `notes/verifications/0x8c19a45c*.json` - 2 specs (107/86 steps) |
| `0x8c193150` | `drain_subarray_then_free` | `notes/verifications/0x8c193150*.json` - 3 specs (170/107/12 steps) |
| `0x8c19a278` | `drain_subarray24_then_free` | `notes/verifications/0x8c19a278*.json` - 3 specs (173/110/12 steps) |
| `0x8c1b6088` | `release_stride68_then_free` | `notes/verifications/0x8c1b6088*.json` - 2 specs (87/15 steps) |
| `0x8c1bc940` | `three_subobject_teardown` | `notes/verifications/0x8c1bc940*.json` - 3 specs (250/187/12 steps) |
| `0x8c17797e` | `enum_to_code_switch_2` | `notes/verifications/0x8c17797e*.json` - 6 specs (12/15/18/21/340/373 steps) |
| `0x8c1bc318` | `clear_flag_helper_global_free` | `notes/verifications/0x8c1bc318*.json` - 4 specs (107/88/44/12 steps) |
| `0x8c1b5f74` | `clear_global_drain_free_3` | `notes/verifications/0x8c1b5f74*.json` - 3 specs (173/110/12 steps) |
| `0x8c17fc44` | `substring_search` | `notes/verifications/0x8c17fc44*.json` - 2 specs (452/224 steps) |
| `0x8c14409c` | `scan_records_by_name` | `notes/verifications/0x8c14409c*.json` - 2 specs (484/480 steps) |
| `0x8c18428c` | `scan_stride88_by_name` | `notes/verifications/0x8c18428c*.json` - 2 specs (160/152 steps) |
| `0x8c18c5c4` | `walk_cursor_set_flag` | `notes/verifications/0x8c18c5c4.json` - 21 oracle steps (empty walk only) |
| `0x8c18c608` | `walk_cursor_clear_flag` | `notes/verifications/0x8c18c608.json` - 21 oracle steps (empty walk only) |
| `0x8c216bee` | `fold_three_bytes` | `notes/verifications/0x8c216bee*.json` - 2 specs (66/66 steps) |
| `0x8c216dde` | `case_insensitive_strcmp` | `notes/verifications/0x8c216dde*.json` - 2 specs (170/165 steps) |
| `0x8c216d70` | `bounded_case_insensitive_cmp` | `notes/verifications/0x8c216d70*.json` - 4 specs (193/17/121/189 steps) |
| `0x8c216e3e` | `folded_byte_checksum` | `notes/verifications/0x8c216e3e*.json` - 2 specs (96/15 steps) |
| `0x8c21608e` | `validate_and_blank_block` | `notes/verifications/0x8c21608e*.json` - 3 specs (1449/1449/1773 steps) |
| `0x8c2199d8` | `call_wrapper_21aff2` | `notes/verifications/0x8c2199d8.json` - 14 oracle steps |
| `0x8c219ed6` | `lookup_split_two_bytes` | `notes/verifications/0x8c219ed6.json` - 263 oracle steps |
| `0x8c219cce` | `lookup_pack_be_word` | `notes/verifications/0x8c219cce.json` - 328 oracle steps |
| `0x8c219e8c` | `lookup_pack_be_word_32` | `notes/verifications/0x8c219e8c.json` - 302 oracle steps |
| `0x8c2188c0` | `lazy_descriptor_init` | `notes/verifications/0x8c2188c0*.json` - 2 specs (44/6 steps) |
| `0x8c1cc088` | `nine_free_clear_global` | `notes/verifications/0x8c1cc088*.json` - 3 specs (171/108/12 steps) |
| `0x8c176618` | `validate_three_fields_stamp` | `notes/verifications/0x8c176618*.json` - 3 specs (31/427/253 steps) |
| `0x8c1c97a8` | `vtable_init_table_copy` | `notes/verifications/0x8c1c97a8.json` - 58 oracle steps |
| `0x8c219d18` | `unpack_five_halfwords` | `notes/verifications/0x8c219d18.json` - 330 oracle steps |
| `0x8c219de6` | `unpack_two_words_two_bytes` | `notes/verifications/0x8c219de6.json` - 343 oracle steps |
| `0x8c1839f8` | `recursive_tree_match` | `notes/verifications/0x8c1839f8.json` - 99 oracle steps |
| `0x8c177e28` | `five_table_slot_clear` | `notes/verifications/0x8c177e28*.json` - 2 specs (72/304 steps) |
| `0x8c1a49ec` | `padded_slot_copy` | `notes/verifications/0x8c1a49ec.json` - 136 oracle steps |
| `0x8c1af0a0` | `record_release_chain` | `notes/verifications/0x8c1af0a0*.json` - 3 specs (218/155/12 steps): null, non-owning, owning; the owning path needs the allocator seed from `0x8c1af3b4.json` (block header at `r4-8`) |
| `0x8c170526` | `bitmap_set_bit` | `notes/verifications/0x8c170526*.json` - 4 specs (31/34/40/44 steps); two-word mode when `[0x8c2b52dc]==2`, four-word otherwise |
| `0x8c1bea5c` | `record_run_copy` | `notes/verifications/0x8c1bea5c*.json` - 3 specs (341/15/21 steps); the null-destination guard is per-step, not per-run |
| `0x8c1779fe` | `selector_to_constant` | `notes/verifications/0x8c1779fe*.json` - 5 specs (14/17/32/361/284 steps); `r4` 0 and -1..-8 map to constants, null `r5` and any other `r4` report through 0x8c174564 and return 0 |
| `0x8c172fcc` | `record_byte_match` | `notes/verifications/0x8c172fcc*.json` - 4 specs (485/311/84/74 steps); `r4` is a pointer, in `[0x8c2b5264, +0x8c2b5268)` it reports 0x1fb, otherwise 0x8c173084 decodes `r4+32` and the three bytes are compared against `r5`/`r6`/`r7`; the loads are `mov.b`, so they sign-extend - zero-extending passes every hand-written spec and fails the fuzzer |
| `0x8c1352a8` | `record_construct` | `notes/verifications/0x8c1352a8.json` - 86 oracle steps; the vtable at `+0` is written three times and only `0x8c1eeed8` survives; `+24` points at the record itself |
| `0x8c1ac828` | `table_seen_flags` | `notes/verifications/0x8c1ac828*.json` - 3 specs (39/45/13 steps); gated on `[0x8c2b79e0]`, two 0x8c074d58 calls (codes 6 and 7) and a walk of `[0x8c23dab0]` that flips each `+68` byte to 1 when it was 0 |
| `0x8c21afd0` | `slot_ptr_44` | `notes/verifications/0x8c21afd0*.json` - 2 specs (8/8 steps); `*[0x8c2ba634]+40+44*r4`, signed mul |
| `0x8c21afe0` | `slot_ptr_120` | `notes/verifications/0x8c21afe0*.json` - 2 specs (9/9 steps); `*[0x8c2ba634]+0x44c+120*r4`, signed mul |
| `0x8c19aacc` | `copy_records_e4` | `notes/verifications/0x8c19aacc*.json` - 2 specs (419/15 steps) |
## Blocked

A leaf whose behaviour depends on state the harness does not model cannot be
pinned by a spec. These are recorded, not retried: the oracle maps nothing at
the listed addresses, so any run dies with `data access ... outside mapped
memory` before a diff can mean anything.

| address | reason |
| --- | --- |
| `0x8c1739c0` | reads 0x8c004020 below LOAD_BASE (unmapped; oracle faults at step 2) |
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
| `0x8c21d89e` | hardware register access: `0xff800030` |
| `0x8c21e0f4` | hardware register access: `0xffd80000` |
| `0x8c21e11a` | hardware register access: `0xffd8****` |
| `0x8c21e298` | hardware register access: `0xffd80000` |
| `0x8c21e2a8` | hardware register access: `0xffd80004` |
| `0x8c21f4d8` | hardware register access: `0xffd00000` |
| `0x8c2201fa` | hardware register access: `0xf6000000` |
| `0x8c013248` | non-contiguous export (9 ranges, first ends 0x8c013327 with `bra` into the next range); the porting loop needs one linear body |
| `0x8c0cfaa4` | circular-list drain; its `free` calls walk the allocator's bin table, which the harness does not seed, so the oracle loops to the step limit before a diff can mean anything |
| `0x8c011470` | non-contiguous export (5 ranges spanning 0x8c011470-0x8c011869); the porting loop needs one linear body |
| `0x8c018244` | infinite spin loop (`bra` to self); exceeds any step budget — behaviour cannot be pinned by a spec |
| `0x8c21d89e` | hardware register access: `ff800030` |
| `0x8c21f90c` | hardware register access: `ffc00004` (leaf-worklist HW filter misses `ffc*`/`ffe*` prefixes) |
| `0x8c21fac4` | hardware register access: `ffc00004` (both branches converge on the tail touch) |
| `0x8c029028` | non-contiguous export (16 ranges) and calls unported 0x8c057348; caller-worklist only resolved its first range |
| `0x8c02c4d0` | non-contiguous export (16 ranges spanning 0x8c02c4d0-0x8c02c8d1); the porting loop needs one linear body |
| `0x8c044430` | `jsr @[(*e)+0x4c]` virtual dispatch: the call target is a runtime vtable slot into another subsystem, so the non-null-`e` arm cannot be pinned; a port covering only the `e == 0` arms would be incomplete |
| `0x8c04d948` | runtime vtable dispatch at `0x8c04d9a0` (`[*(r13)]+0x10`) and `0x8c04d9c2` (`[*(r13)]+0x40`), `r13 = [obj+0x24]`; targets are runtime data, unpinnable |
| `0x8c04d9f8` | runtime vtable dispatch at `0x8c04da4c` (`[*(r13)]+0x10`) and `0x8c04da6e` (`[*(r13)]+0x40`), `r13` reached through two pointer indirections; unpinnable |
| `0x8c02cd34` | runtime vtable dispatch at `0x8c02cd64` (`[(*r4)+0x24]`, base `[r4]` loaded at `0x8c02cd5c`); the other jsr (`0x8c02cd42`) is static (0x8c151b1c) |
| `0x8c08239c` | runtime call at `0x8c08248e` via `[r12+0x10]` (`mov.l @(16,r12),r0` at `0x8c08247a`); the two other jsr sites (`0x8c0823d4`→0x8c0b4e80, `0x8c08241c`→0x8c0b4e98) are static and ported. Slot targets (0x8c0822fc / 0x8c1c346c / 0x8c1a31cc) have no ports, so no spec can exercise the arm |
| `0x8c0b5734` | runtime vtable dispatch at `0x8c0b5754` (`[(*r4)+0x20]`, base `[r4]` loaded at `0x8c0b5750`); the other jsr (`0x8c0b574a`→0x8c165abc) is static and ported |
| `0x8c0b9e04` | two-level runtime call at `0x8c0b9e52`: `mov.l @(68,r4),r0` (0x8c0b9e4e) then `mov.l @(8,r0),r0` (0x8c0b9e50) then `jsr @r0`; target reachable only through heap data, unpinnable |

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

## Oracle: multi-bit shifts do not set T

Checked against the SH-4 Programming Manual (ADE-602-156D, Rev 5.0). The
operation listings for `SHLL2`/`SHLL8`/`SHLL16` and `SHLR2`/`SHLR8`/`SHLR16`
shift the register and advance PC; the T-bit column is "—". The description of
the right-shift forms says the bits shifted out are discarded. Only the
one-bit `SHLL`/`SHLR`/`SHAL`/`SHAR` transfer a bit into T.

A session briefly taught the interpreter the opposite (last bit out → T) and
rewrote 28 ports to match. That was wrong: the manual is the source of truth,
and the interpreter already matched it. Reverted. The fuzz cases that looked
like T mismatches were the rewritten ports disagreeing with the hardware, not
the hardware disagreeing with the ports.

## Verifier findings so far

Four ports, five spec runs, and every failure was a *hidden side effect* that the
decompiled C did not show:

| address | missed by the C |
| --- | --- |
| `0x8c175c72` | argument copied into `r4`; field left in `r3` |
| `0x8c18b9c8` | `base[1]` left in `r1`; T bit differs per branch |
| `0x8c1552e4` | inner pointer left in `r0` |
| `0x8c014de0` | `wrap_byte_accessor` | `notes/verifications/0x8c014de0.json` — verified |

The randomized differential audit (`scripts/fuzz-audit.py`, five pool fill modes,
random registers) added two more classes that hand-written specs had missed:

| address | missed by the port / the C |
| --- | --- |
| `0x8c0112f4` `0x8c010f44` `0x8c0112a4` `0x8c011344` `0x8c011374` | written against the *pre-fix* oracle: `mov.b` post-increment and `@(r0,Rm)` decoded as 32-bit, so stores wrote four bytes and the NUL test read a dword. Witness specs `.alias`, `.backbyte`, `.high1`, `.nul` fail on the old code. |
| `0x8c015194` | `neg r13,r3` executes before `cmp/pl r5`; the port only wrote `r3` inside the loop, so the skip branch kept the entry value. |

That is the argument for the oracle: register residue is invisible in pseudocode
and load-bearing for the caller. The audit is the argument for randomizing the
seed: every one of those ports had passing hand-written specs.

### The 53 ports the randomized audit broke, by root cause

Each row is a port that had passing hand-written specs and a failing random
case. The witness spec named in the ledger row reproduces the failure.

| class | addresses | what the hand specs missed |
| --- | --- | --- |
| byte load zero-extended instead of sign-extended | `0x8c0113b4` `0x8c011c50` `0x8c013226` `0x8c17eb18` `0x8c198b20` `0x8c215fac` `0x8c216dc0` `0x8c216e18` `0x8c217e3e` `0x8c222410` `0x8c225392` | `read_u8(..) as u32`; only visible when the byte is >= 0x80 and the register survives to exit |
| `mov.w` operand used before sign extension | `0x8c084f8c` `0x8c085628` `0x8c10fe08` `0x8c2275c0` `0x8c227620` `0x8c227720` `0x8c227440` | halfword arithmetic where the instruction writes `sext16`; masks like `0xfffff0ff` became `0x0000f0ff` |
| register written on one branch, modelled on the other | `0x8c015194` `0x8c015324` `0x8c1705ca` `0x8c23b94c` `0x8c1c13b2` | `neg r13,r3` before `cmp/pl`; `r2/r3` only inside the loop; `mov r6,r2` before an empty loop |
| T residue from `cmp/eq`, `cmp/pl`, `cmp/hs`, `tst`, `shll` | `0x8c14dda8` `0x8c14dd90` `0x8c164430` `0x8c2199b8` `0x8c226de0` `0x8c232f60` `0x8c21cad4` `0x8c093048` `0x8c216e18` `0x8c217e3e` `0x8c219c9e` | the last flag-setting instruction before an exit is part of the result; `shll` sets T from bit 31, `shll2` does not |
| signed compare done unsigned | `0x8c011c50` `0x8c085434` `0x8c085480` `0x8c1556d0` `0x8c18320c` `0x8c22817e` `0x8c22828e` | `cmp/gt`, `cmp/ge`, `cmp/pl` and `shad` treat the operand as i32 |
| delay slot applied on one path only | `0x8c093048` `0x8c21cad4` `0x8c22828e` `0x8c018edc` (found by its port) | a `bf.s`/`bt.s` delay slot runs on both outcomes |
| wrong field or dispatch offset | `0x8c1cbd88` `0x8c1cbda8` `0x8c1b7998` `0x8c017c0c` | the `r5==1/2/default` slots were shifted by one; the `bra` skips a shift the fall-through performs |
| inverted gates on the bit-insert family | `0x8c227280` `0x8c2273e0` `0x8c2274a0` `0x8c227500` `0x8c2276c0` | accept is `mask2==0 || r5!=1`, not `mask2==0 && r5==1` |
| missing store path / hardcoded literals | `0x8c010cf8` | the whole non-zero fall-through (four stores) was absent and the pool literals were baked in |
| loop shape | `0x8c15a23c` (callee, found by `0x8c018fb0`) | `mov r0,r1` runs once before the loop, not at its head |
| stack pops hardcoded | `0x8c0163e8` (callee, found by `0x8c016484` and the audit) | the exit pops four words although it pushes two; the extra two are the caller's |

The distribution is the lesson: 18 of 53 are one sign-extension mistake, 15 are
residue on a path the spec author did not take, and 12 are a flag the C never
mentions. Every one of them passed a hand-written spec first.

