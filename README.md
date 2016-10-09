# unicorn

[![Build Status](https://travis-ci.org/muktakosh/unicorn.svg?branch=master)](https://travis-ci.org/muktakosh/unicorn) [![API Docs](https://img.shields.io/badge/docs-API-blue.svg)](https://muktakosh.github.io/unicorn) ![Project Status: Experiment](https://img.shields.io/badge/status-experiment-red.svg)

**`unicorn` is a decentralized, context-aware pipeline for real-time data.**

"unicorn" is our sweet little acronym for "Unified Communications Over
Real-time Networks".

## Capabilities

`unicorn` is being designed to have these key capabilities:

**Messaging:** It will let clients or groups of clients exchange
streams of data. Some clients publish data and some clients subscribe
to those data. In this aspect, `unicorn` is similar to a messaging
system.

**Streaming:** It will be tuned for exchanging data in real-time. This
includes storing, processing and transporting data streams as they are
created. This will include any data type that can be serialized to
binary, eventually, including audio/video streams as well.

**Bridge:** It will let clients exchange streams of data where the
publishers and subscribers use different formats or speak different
languages. In this aspect, `unicorn` works like a bridge. In the long
run, `unicorn` can connect peers across different network transport
protocols or communication services.

**Context-aware:** Even though `unicorn` is content agnostic at the
core, it will provide APIs for adding context to data through
adapters. These adapters can read, write, modify or take action on
data streams. This also forms the basis of building a bridge across
data formats.

**Decentralized:** It will be decentralized in the sense of a
federated network with fault-tolerance, as in:

- Individual `unicorn` instances logically behave as a centralized
  network where clients directly connect to them.
- `unicorn` instances can be formed either as a single node or as a
  cluster of multiple nodes.
- When an instance is formed of multiple nodes, `unicorn` can share
  data and responsibilities among the nodes to allow
  fault-tolerance. So, at an operational level, a multi-node instance
  will behave like a distributed network, but logically it will still
  behave like a centralized network where clients will connect to
  pre-defined gateways.
- `unicorn` instances can connect to other `unicorn` instances and
  exchange data. Clients still connect to their specific `unicorn`
  instances, essentially forming a `client-server-server-client`
  topology. This creates a distributed network of eventually
  centralized networks, a.k.a, a federated network.

**Pipeline:** The federated nature of `unicorn` ends up creating data
pipelines where data streams can flow in real-time between servers.

**Resource efficiency:** It should be highly resource efficient and
should be able to pack in as much power in as less resource as
possible.

**Adaptable:** It should run on an embedded device and still be able
to scale across data centres. `unicorn` should adapt to the nature of
the usage.

**Ease of use:** Nothing, absolutely nothing, will compromise on a
simple API and ease of usage, whether it is about installation or
configuration or deployment.

## Feature set

This is the list of currently targeted broad feature set for `unicorn`:

- [ ] **Logger**
  - [x] Console
  - [ ] File logger
- [x] **CLI**: Command line arguments parsing
- [ ] **Configuration**
 - [x] Config parser
 - [x] Config creator
 - [ ] Multi-mode config (single instance vs distributed)
- [ ] **Network layer**
  - [x] JSON-RPC over WebSockets
  - [ ] Internal communication (Asynchronous TCP)
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
- [ ] **Fault tolerance**
  - [ ] Data replication across nodes
  - [ ] Data partitioning
- [ ] **Federation features**
 - [ ] Service discovery
 - [ ] Ledger of known services
 - [ ] Pipelining: Connect to other `unicorn` instances

## About this repository

This repository provides `unicorn`'s core features as a Rust library
(`libunicorn`) and as a binary.

## Contribute

`unicorn` and its components are written in the Rust programming language.

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

`unicorn` uses several other software. The entire list of those, along
with the licenses they are used under, can be found in
[ATTRIBUTIONS](ATTRIBUTIONS).

`unicorn` is a project made possible by
[Muktakosh](https://muktakosh.org) and
[contributors](CREDITS). [No rights reserved](LICENSE).
