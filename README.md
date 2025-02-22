Web4 Demo on NEAR protocol
==========================

[Web4](https://web4.near.page) is a gateway to the smart contracts deployed to NEAR protocol.

This demo just shows the minimum viable application that lives on NEAR chain, and thus anyone hosting the gateway can serve all the Web4 projects!

See it live on testnet: https://hello-web4.testnet.page

When you deploy on mainnet, you should be able to use https://ACCOUNT_ID.near.page as your gateway.

## Build

See [near-sdk docs](https://docs.near.org/build/smart-contracts/quickstart) for the prerequisites, and then build the contract using [`cargo-near` extension](https://github.com/near/cargo-near):

```
$ cargo near build
```

## Deploy

```
$ cargo near deploy
```
