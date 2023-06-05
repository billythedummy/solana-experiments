# top-level-return-data

Tests if return data can be set and get from transaction top-level instructions in sequence, and not just in CPI calls.

## Setup

- Setter program `EvnFQCWwzDbkaGEPuyFE75VBhZHb9eYhfrMAsjJgiBvx` sets return data
- Getter program `2gt7fw22RH8g8Q12JrzrRvEMBNLKB4QnLwQiUXnhGPb6` gets return data
- Make transaction that calls setter program followed by getter program, check if return data is accessible in getter program


## Running

```
cd test
cargo-test-sbf
```

## Result

Nope, `return_data` is cleared when a top-level instruction returns.

```
running 1 test
[2023-06-05T06:49:40.978779101Z INFO  solana_program_test] "setter" builtin program
[2023-06-05T06:49:40.978870889Z INFO  solana_program_test] "getter" builtin program
[2023-06-05T06:49:40.990323317Z DEBUG solana_runtime::message_processor::stable_log] Program EvnFQCWwzDbkaGEPuyFE75VBhZHb9eYhfrMAsjJgiBvx invoke [1]
[2023-06-05T06:49:40.990397892Z DEBUG solana_runtime::message_processor::stable_log] Program EvnFQCWwzDbkaGEPuyFE75VBhZHb9eYhfrMAsjJgiBvx invoke [1]
[2023-06-05T06:49:40.990445578Z DEBUG solana_runtime::message_processor::stable_log] Program EvnFQCWwzDbkaGEPuyFE75VBhZHb9eYhfrMAsjJgiBvx success
[2023-06-05T06:49:40.990470249Z DEBUG solana_runtime::message_processor::stable_log] Program EvnFQCWwzDbkaGEPuyFE75VBhZHb9eYhfrMAsjJgiBvx success
[2023-06-05T06:49:40.990501884Z DEBUG solana_runtime::message_processor::stable_log] Program 2gt7fw22RH8g8Q12JrzrRvEMBNLKB4QnLwQiUXnhGPb6 invoke [1]
[2023-06-05T06:49:40.990518205Z DEBUG solana_runtime::message_processor::stable_log] Program 2gt7fw22RH8g8Q12JrzrRvEMBNLKB4QnLwQiUXnhGPb6 invoke [1]
[2023-06-05T06:49:40.990546458Z DEBUG solana_runtime::message_processor::stable_log] Program log: setter: 2gt7fw22RH8g8Q12JrzrRvEMBNLKB4QnLwQiUXnhGPb6
[2023-06-05T06:49:40.990551731Z DEBUG solana_runtime::message_processor::stable_log] Program log: len: 0
```
