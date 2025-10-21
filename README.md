# ⚡ Sol Counter

A simple Solana on-chain counter program built with **Anchor**, developed as part of the **Turbin3 Pre-Builders Bootcamp (Q4 2025)**.

This program demonstrates:
- Incrementing and decrementing a stored counter by `1`
- Incrementing and decrementing by any **custom value** e.g `20`


## ⚙️ Setup & Build

Make sure you have **Anchor**, **Rust**, and **Solana CLI** installed.

```bash
# 1. Install dependencies
yarn install

# 2. Build the Anchor program
anchor build

# 4. Run tests
anchor test
```

> ⚠️ Note: If you skip yarn install, your tests may fail due to missing dependencies.