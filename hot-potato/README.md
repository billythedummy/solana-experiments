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

- **YES**, you can create a hot potato
- The key is to **Create the hot potato account with lamports less than rent-exempt minimum e.g. 1 lamport**:
    - This causes `TransactionError(InsufficientFundsForRent { account_index: 1 })` to be thrown at end of transaction if the potato is created in the tx and has non-zero but insufficient balance at the end of it
- Failed attempts:
    - Creating a hot potato with 0 lamports results in the account not being created in the first place
    - Creating a hot potato with lamports >= rent exempt min and then removing all of it at the end of create instruction
    - Both results in the `fail_create_without_consuming` tx succeeding because the potato account has 0 balance and is gc-ed at end of tx, so the tx is valid (bad)