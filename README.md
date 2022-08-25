Small freestanding ARM binary calling function already in memory

```
> cargo +nightly-2022-06-18 build --release -Zbuild-std --target thumbv7em-none-eabi`
> rizin ./target/release/thumbv7em-none-eabi/freestanding
> pd 11
            ;-- entry0:
            ;-- section..text:
            ;-- segment.LOAD1:
            0x000200e4      push  {r5, r6, r7, lr}                     ; [01] -r-x section size 30 named .text
            0x000200e6      add   r7, sp, 8
            0x000200e8      movs  r0, 0xcc
            0x000200ea      movw  r2, 0x1234
            0x000200ee      strb.w r0, [sp, 6]
            0x000200f2      movw  r0, 0xbbaa
            0x000200f6      strh.w r0, [sp, 4]
            0x000200fa      add   r0, sp, 4
            0x000200fc      movs  r1, 3
            0x000200fe      blx   r2
            0x00020100      pop   {r2, r3, r7, pc}
```
