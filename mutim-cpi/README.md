# mutim-cpi

[Original tweet](https://twitter.com/armaniferrante/status/1668910262488014848):

```
Lazy @solana
 question:

- suppose you have a transaction with som mutable account and programs A, B, C
- suppose program A changes that account to be readonly in the AccountMeta before CPI invoking to program B
- suppose, in the same tx, program B changes the account to be mutable again before CPI

What happens on that last CPI? Does it fail or succeed?
```

## Running

```
cargo-test-sbf
```

## Result

**Program B fails with writable privilege escalated when it attempts to CPI program C with account reset to mutable**

This confirms [@enzoampil's reply](https://twitter.com/AND__SO/status/1668917116211453952)

```
[2023-06-14T10:20:43.943342358Z DEBUG solana_runtime::message_processor::stable_log] Program 22222222222222222222222222222222222222222222 invoke [1]
[2023-06-14T10:20:43.943430922Z DEBUG solana_runtime::message_processor::stable_log] Program 22222222222222222222222222222222222222222222 invoke [1]
[2023-06-14T10:20:43.944006845Z DEBUG solana_runtime::message_processor::stable_log] Program log: payer: 75Rmp1byeEsvnYrwUcYX18gfRDzmkLVhndWERTGGJRZM
[2023-06-14T10:20:43.944027058Z DEBUG solana_runtime::message_processor::stable_log] Program log: dst: 3djtp3eaGArZKYeC671jx6D9yNqh8MaYWJ8yQepppGbm
[2023-06-14T10:20:43.944042470Z DEBUG solana_runtime::message_processor::stable_log] Program 33333333333333333333333333333333333333333333 invoke [1]
[2023-06-14T10:20:43.944112221Z DEBUG solana_runtime::message_processor::stable_log] Program 33333333333333333333333333333333333333333333 invoke [2]
[2023-06-14T10:20:43.944135941Z DEBUG solana_runtime::message_processor::stable_log] Program 33333333333333333333333333333333333333333333 invoke [2]
[2023-06-14T10:20:43.944593379Z DEBUG solana_runtime::message_processor::stable_log] Program 44444444444444444444444444444444444444444444 invoke [2]
[2023-06-14T10:20:43.944619544Z DEBUG solana_runtime::message_processor::stable_log] 75Rmp1byeEsvnYrwUcYX18gfRDzmkLVhndWERTGGJRZM's writable privilege escalated
```
