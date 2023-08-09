# account-data-alignment

Experiment to see what bad things would happen to `bytemuck::Pod` account data if the data pointed to did not match the account data struct's alignment, but this seems unlikely since the account data is 8-byte aligned when sent to BPF: https://github.com/solana-labs/solana/blob/d5faa6e8aaa87166f11e9ad8bda7339fb2f5bd36/programs/bpf_loader/src/serialization.rs#L396.

But lets try anyway...

## Serialized data layout

1. 8 bytes: n instruction accounts

2. Individual accounts. For each account, Either:
    - 8 bytes for duplicate account, or
    - (88 bytes for non-duplicate account excluding data) + (n >= 10240, divisible by 16, bytes for account data) + (8 bytes for rent epoch)

## Test

What happens if we pass:
- empty account
- the same empty account
- an account with 16-byte aligned account data

Byte offset end of first empty account = 10344

Byte offset end of second account (duplicate of first) = 10352

Start of third account's account data = 10440, not divisible by 16.

## Running the tests

Since `prefer_bpf=true`, be sure to recompile the `.so` with `cargo build-sbf` after every change to program.

## Results

`bytemuck::try_from_bytes()` fails with `TargetAlignmentGreaterAndInputNotAligned` when data is not aligned, as expected.

## Conclusion

- don't overalign your bytemuck structs
- be very worried if you ever find a bytemuck struct with align > 8 in your account data
