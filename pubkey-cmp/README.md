# pubkey-cmp

Compares compute units used by [sol_memcmp](https://docs.rs/solana-program/latest/solana_program/program_memory/fn.sol_memcmp.html) syscall and the regular derived `PartialOrd` impl.

## Results

They use the exact same CUs and have the same binary sizes even though the code is different.

### Compute Units Consumed

**Payer Pubkey Success:**
- syscall: 313
- normal: 313

**Rand Pubkey Failure:**
- syscall: 331
- normal: 331

**Zero Pubkey Failure:**
- syscall: 330
- normal: 330

### Binary Sizes

- syscall: 21_440
- normal: 21_440

But `md5sum` shows they are actually different binaries
