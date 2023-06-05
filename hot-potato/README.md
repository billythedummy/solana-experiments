# hot-potato

Tests if it's possible to implement a [hot potato](https://examples.sui.io/patterns/hot-potato.html) account, an account that is enforced to be deleted at the end of a transaction. 

## Setup

- `2gt7fw22RH8g8Q12JrzrRvEMBNLKB4QnLwQiUXnhGPb6` hot-potato program with 2 instructions:
    - `create_potato()` creates an account with allocated space but 0 balance
    - `consume_potato()` deletes the same account by realloc-ing to 0
- tests check if a transaction with `create_potato()` followed by `consume_potato()` is possible and if `create_potato(`) without `consume_potato()` fails

## Running

```
cargo-test-sbf
```

## Result

- `solana-program-test`'s bank client fails silently when an account is below rent-exempt minimum at the end of an instruction, and the account is simply deleted. This causes the `fail_create_without_consuming` test to succeed and `create_then_consume` to fail when attempting to read the data stored in the potato.
