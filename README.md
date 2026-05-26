# AidTrack
**Transparent, on-chain disaster relief distribution using Stellar.**

## Problem & Solution
NGO relief coordinators in disaster-struck regions struggle with cash misappropriation and logistical delays. AidTrack solves this by utilizing Soroban smart contracts to conditionally release USDC directly to verified typhoon victims' mobile wallets once they are registered by an NGO, guaranteeing transparent and instant fund distribution.

## Timeline
Built for the Stellar Bootcamp 2026.

## Stellar Features Used
* USDC transfers
* Soroban smart contracts
* Stellar Trustlines

## Vision and Purpose
To eliminate friction, fraud, and delay in humanitarian aid, ensuring that 100% of donated emergency funds reach the hands of the displaced individuals who need them most.

## Prerequisites
* Rust toolchain (edition 2021)
* Soroban CLI (`soroban-cli v20.0.0` or higher)

## How to Build
Compile the smart contract to WebAssembly:
```bash
soroban contract build

Contract ID: CD6IKGKAV3PP677BOHLAXRYDLWURWPIBL5ADFCEUCJDBSESBLPUJXKH4

Link Stellar Expert
https://stellar.expert/explorer/testnet/contract/CD6IKGKAV3PP677BOHLAXRYDLWURWPIBL5ADFCEUCJDBSESBLPUJXKH4

screenshot
![alt text](image-1.png)