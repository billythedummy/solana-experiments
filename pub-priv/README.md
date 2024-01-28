# pub-priv

Compare the effects of making functions public vs private on solana program binary size

## Setup

`pub-program` is a simple program that finds `gcd(69, arg)` and exports this function by making it `pub`.

`priv-program` is the exact same program, but does not export the function.

## Result

Program sizes are the same. Huh. I could've sworn I ran into an instance where making a function private in the program actually reduced binary size. Will need to dig deeper.
