# pubkey-cmp

Compares compute units used by [sol_memcmp](https://docs.rs/solana-program/latest/solana_program/program_memory/fn.sol_memcmp.html) syscall and the regular derived `PartialOrd` impl.

## Results

They're the exact same.

### Compute Units Consumed

**Payer Pubkey Success:**
- syscall: 327
- normal: 327

**Rand Pubkey Success:**
- syscall: 329
- normal: 329

**Zero Pubkey Failure:**
- syscall: 355
- normal: 355

### Binary Sizes

- syscall: 21_568
- normal: 21_568
