# cpi-by-ref

Test if it's possible to CPI instructions without allocating `Vec`s. 

## Result

- Yes, you can invoke the `sol_invoke_signed_rust` syscall with slices on the stack instead of `Vec`s
- Idk how to get the syscall stubs to work with regular `cargo-test`, so you'll just get `Program log: SyscallStubs: sol_invoke_signed() not available` when running `cargo-test` and the test will fail but `cargo-test-sbf` will work.