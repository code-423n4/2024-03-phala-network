# Pink runtime audit details
- Total Prize Pool: 60,500 in USDC
  - HM awards: 46,250 in USDC
  - Analysis awards: 2,500 in USDC
  - QA awards: 1,250 in USDC
  - Judge awards: 6,000 in USDC
  - Lookout awards: 4,000 in USDC
  - Scout awards: $500 in USDC
- Join [C4 Discord](https://discord.gg/code4rena) to register
- Submit findings [using the C4 form](https://code4rena.com/contests/2024-03-phala-network/submit)
- [Read our guidelines for more details](https://docs.code4rena.com/roles/wardens)
- Starts March 1, 2024 20:00 UTC
- Ends March 22, 2024 20:00 UTC

# Overview

## About Pink Runtime
`pink-runtime` is the `ink!` contract execution engine for Phala Network, built on Substrate's `pallet-contracts` with custom chain extensions, written in Rust.
It executes smart contracts on the Phala Network. It is compiled to a Linux shared object, `libpink.so`. It is loaded and runs in the Phala Network's off-chain TEE workers.

See the [README](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/README.md) for more information.

## Links

- **Previous audits:** 
- **Documentation:** https://docs.phala.network/developers/phat-contract
- **Website:** https://phala.network/
- **Github:** https://github.com/Phala-Network/phala-blockchain
- **Twitter:** https://twitter.com/PhalaNetwork
- **Discord:** https://discord.com/invite/phala


# Scope

The codes in this repository are copied from Phala Network's [repository](https://github.com/Phala-Network/phala-blockchain), keeping the directory structure while removing unnecessary files and directories.

## Files in scope

| File | SLOC | Purpose |
| ----------- | ----------- | ----------- |
| [pink/runtime/src/runtime.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/runtime.rs) | 209 | Construct and configure the substrate runtime |
| [pink/runtime/src/contract.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/contract.rs) | 199 | The contracts call/instantiation API |
| [pink/runtime/src/storage/mod.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/storage/mod.rs) | 119 | Abstract storage provider for the runtime |
| [pink/runtime/src/storage/external_backend.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/storage/external_backend.rs) | 18 | The storage provider for the runtime |
| [pink/runtime/src/runtime/pallet_pink.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/runtime/pallet_pink.rs) | 168 | The pallet used to store some costom configuration of the runtime |
| [pink/runtime/src/capi/mod.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/capi/mod.rs) | 53 | The entrypoint of the libpink.so, FFI helpers bridge between the cross lib boundary |
| [pink/runtime/src/capi/ecall_impl.rs  ](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/capi/ecall_impl.rs  ) | 273 | The enterward cross boundary call support |
| [pink/runtime/src/capi/ocall_impl.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/capi/ocall_impl.rs) | 92 | The low level outward cross boundary call support |
| [pink/runtime/src/runtime/extension.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/runtime/src/runtime/extension.rs) | 440 | The chain extension implementation of the runtime, inheriting pink-chain-extension with some overwrites |
| [pink/capi/src/v1/mod.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/capi/src/v1/mod.rs) | 225 | Definition of the cross host-lib function calls (ecalls and ocalls) |
| [pink/capi/src/types.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/capi/src/types.rs) | 86 | Types exported to host |
| [pink/chain-extension/src/lib.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/chain-extension/src/lib.rs) | 458 | The chain extension feature implementation |
| [pink/chain-extension/src/local_cache.rs](https://github.com/code-423n4/2024-03-phala-network/blob/main/phala-blockchain/crates/pink/chain-extension/src/local_cache.rs) | 371 | Implementation of the feature worker local cache |
| Total | 2711 | |


## Out of scope

These files are some direct or indirect dependencies of the runtime but most of the codes are not included
in the final runtime shared object. They are not in the scope of this audit.

```
crates/sgx-attestation/src/lib.rs
crates/sgx-attestation/src/dcap.rs
crates/sgx-attestation/src/ias.rs
crates/sgx-attestation/src/dcap/constants.rs
crates/sgx-attestation/src/dcap/quote.rs
crates/sgx-attestation/src/dcap/tcb_info.rs
crates/sgx-attestation/src/dcap/report.rs
crates/sgx-attestation/src/dcap/utils.rs
crates/sgx-attestation/src/ias/report.rs
crates/sgx-attestation/src/gramine.rs
crates/sgx-attestation/build.rs
crates/this-crate/src/lib.rs
crates/phala-crypto/src/lib.rs
crates/phala-crypto/src/ecdh.rs
crates/phala-crypto/src/aead/stream.rs
crates/phala-crypto/src/key_share.rs
crates/phala-crypto/src/sr25519.rs
crates/phala-crypto/src/aead.rs
crates/pink/capi/src/lib.rs
crates/pink/capi/src/helper.rs
crates/pink/capi/build.rs
crates/pink/pink/tests/test_chain_extensions.rs
crates/pink/pink/src/lib.rs
crates/pink/pink/src/logger.rs
crates/pink/pink/src/allocator_dlmalloc.rs
crates/pink/pink/src/system.rs
crates/pink/pink/src/topic.rs
crates/pink/pink/src/chain_extension/http_request.rs
crates/pink/pink/src/chain_extension/test.rs
crates/pink/pink/src/chain_extension/signing.rs
crates/pink/pink/macro/src/lib.rs
crates/pink/pink/macro/src/chain_extension.rs
crates/pink/pink/macro/src/contract.rs
crates/pink/pink/macro/src/driver_system.rs
crates/pink/macro/src/lib.rs
crates/pink/macro/src/macro_xcall.rs
crates/pink/macro/src/tests.rs
crates/pink/runtime/tests/helpers/ink_helpers.rs
crates/pink/runtime/tests/helpers/mod.rs
crates/pink/runtime/tests/helpers/xcalls.rs
crates/pink/runtime/tests/helpers/test_cluster.rs
crates/pink/runtime/tests/helpers/storage.rs
crates/pink/runtime/tests/test_pink_contract.rs
crates/pink/runtime/src/export_fixtures.rs
crates/pink/runtime/src/storage/in_memory_backend.rs
crates/pink/loader/src/lib.rs
crates/pink/loader/src/runtimes.rs
crates/pink/loader/src/storage.rs
crates/pink/loader/src/runtimes/v1.rs
crates/pink/chain-extension/src/mock_ext.rs
crates/pink/pink-types/src/lib.rs
crates/pink/pink-types/src/result.rs
crates/pink/pink-types/src/js.rs
crates/pink/pink-types/src/sgx.rs
crates/phala-git-revision/src/lib.rs
crates/phala-git-revision/build.rs
crates/phala-types/src/lib.rs
crates/phala-types/src/contract.rs
crates/type-info-stringify/src/lib.rs
crates/phala-serde-more/src/lib.rs
crates/phala-serde-more/src/option_key_bytes.rs
crates/phala-serde-more/src/scale_bytes.rs
crates/phala-serde-more/src/key_bytes.rs
crates/phala-serde-more/src/pubkey_bytes.rs
crates/reqwest-env-proxy/src/lib.rs
crates/phala-mq/tests/tests.rs
crates/phala-mq/src/lib.rs
crates/phala-mq/src/types.rs
crates/phala-mq/src/send_queue.rs
crates/phala-mq/src/simple_mpsc.rs
crates/phala-mq/src/checkpoint_helper.rs
crates/phala-mq/src/dispatcher.rs
crates/phala-mq/src/signer/mod.rs
crates/phala-trie-storage/tests/test_state_root.rs
crates/phala-trie-storage/src/lib.rs
crates/phala-trie-storage/src/memdb.rs
crates/phala-trie-storage/src/ser.rs
crates/phala-sanitized-logger/src/lib.rs
crates/phala-sanitized-logger/src/logger.rs
crates/phala-sanitized-logger/src/test.rs
crates/phala-sanitized-logger/src/subscriber.rs
crates/prpc/src/lib.rs
crates/phala-wasm-checker/src/lib.rs
crates/phala-wasm-checker/src/error.rs
```

## Scoping Details 

```
- If you have a public code repo, please share it here:  https://github.com/Phala-Network/phala-blockchain/blob/master/crates/pink/runtime/, https://github.com/Phala-Network/phala-blockchain/blob/master/crates/pink/capi/  
- How many contracts are in scope?: 1   
- Total SLoC for these contracts?: 2711
- How many external imports are there?: 28  
- How many separate interfaces and struct definitions are there for the contracts within scope?: struct: 24, trait: 10  
- Does most of your code generally use composition or inheritance?: Composition   
- How many external calls?: 0   
- What is the overall line coverage percentage provided by your tests?: 90
- Is this an upgrade of an existing system?: No
- On which Parachain will the contracts be deployed?: Phala Network
- Check all that apply (e.g. timelock, NFT, AMM, ERC20, rollups, etc.): ERC-20 Token, Non ERC-20 Token, Timelock function 
- Is there a need to understand a separate part of the codebase / get context in order to audit this part of the protocol?: Yes   
- Please describe required context: The understanding of Substrate runtime development and pallet-contracts (the ink contract pallet)  
- Does it use an oracle?: No  
- Describe any novel or unique curve logic or mathematical models your code uses: 
- Is this either a fork of or an alternate implementation of another project?: True   
- Does it use a side-chain?:
- Describe any specific areas you would like addressed:
```

# Tests

```sh
git clone https://github.com/code-423n4/2024-03-phala-network
cd 2024-03-phala-network/phala-blockchain/crates/pink/runtime
cargo test  # for test only
./cov.sh # for coverage report
```

## Miscellaneous

Employees of Phala Network and employees' family members are ineligible to participate in this audit.
