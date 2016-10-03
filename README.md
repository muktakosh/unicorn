# unicorn

[![Build Status](https://travis-ci.org/muktakosh/unicorn.svg?branch=master)](https://travis-ci.org/muktakosh/unicorn) [![API Docs](https://img.shields.io/badge/docs-API-blue.svg)](https://muktakosh.github.io/unicorn) ![Project Status: Experiment](https://img.shields.io/badge/status-experiment-red.svg)

**unicorn is a decentralized, context-aware pipeline for real-time data.**

"unicorn" is our sweet little acronym for "Unified Communications Over
Real-time Networks".

Eventually, unicorn aims to:

- bridge different communication APIs
- connect devices (or humans) across network transport protocols or
communication services
- support audio and video (media) streams

If a data type can be streamed, unicorn would be able to support it.

## Feature set

This is the list of currently targeted broad feature set for unicorn:

- [ ] **Logger**
  - [x] Console
  - [ ] File logger
- [x] **CLI**: Command line arguments parsing
- [ ] **Configuration**
 - [ ] Config parser
 - [ ] Config creator
- [ ] **Network layer**
  - [x] JSON-RPC over WebSockets
  - [ ] Internal communication (possibly `nanomsg`)
- [ ] **Data transmission features**
  - [ ] Channels (a.k.a Rooms/Topics)
  - [ ] Publishers
  - [ ] Subscribers
- [ ] **Adapters (Data access)**
  - [ ] Modifiers (Write/Operate on data)
  - [ ] Readers (Readonly access for data streams)
- [ ] **DataStore (Data persistency)**
  - [ ] Write to memory
  - [ ] Write to disk
- [ ] **Decentralization features**
 - [ ] Service discovery
 - [ ] Ledger of known services
 - [ ] Pipelining: Connect to their unicorn instances

## About this repository

This repository provides unicorn's core features as a Rust library
(`libunicorn`) and as a binary.

## Contribute

unicorn and its components are written in the Rust programming language.

The project is still in its early days and contributions are very much
welcome!

For build instructions, setup information and general contribution
requirements, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Community

- [**Specifications (RFCs)**](https://github.com/muktakosh/unicorn/labels/rfc)
- [**Bug reports/issues**](https://github.com/muktakosh/unicorn/issues)
- [**Feature requests**](https://github.com/muktakosh/unicorn/labels/feature-request)
- [**Telegram**](https://telegram.me/mk_unicorn)

## License

This is free and unencumbered software released into the public
domain.

unicorn uses several other software. The entire list of those, along
with the licenses they are used under, can be found in
[ATTRIBUTIONS](ATTRIBUTIONS).

unicorn is a project made possible by
[Muktakosh](https://muktakosh.org) and
[contributors](CREDITS). [No rights reserved](LICENSE).
