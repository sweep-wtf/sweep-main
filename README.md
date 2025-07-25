```
   ______      _____  ___  ____ 
  / ___/ | /| / / _ \/ _ \/ __ \
 (__  )| |/ |/ /  __/  __/ /_/ /
/____/ |__/|__/\___/\___/ .___/ 
                       /_/
```
# sweep-program

On-chain Solana program for Sweep. It:
- detects and aggregates dust tokens
- executes swaps or burns
- records rewards for $SWEEP distribution
- exposes program instructions for the SDK and frontend

## Stack
- Solana
- Anchor
- Rust

## Quick start

```bash
anchor build
anchor test
anchor deploy
