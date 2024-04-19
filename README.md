# Cedar Server

/!\ This is incomplete - work in progress.

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## What is Cedar-Server?

Cedar-Server is a server for [Cedar](https://www.cedarpolicy.com/en) policies and data.  Written in Rust, with a GRPC 
API and PostgreSQL backend.

Another implementation of such server is [Cedar-Agent](https://github.com/permitio/cedar-agent/tree/main) a fast, 
non-persistent, in-memory store, requiring additional software (e.g. OPAL) to manage data and policies. Which I
would recommend you take a look at if you're looking at large scale cloud deployments.

This implementation aims to be a battery-included alternative for those who do not require such scale and do not
want to have to manage the complexities associated with it.

## APIs

Cedar's core [SDK](https://github.com/cedar-policy/cedar) is written in Rust, hence the choice of language for this
project, however the use of Protobuf for the APIs should make this a non-issue to integrate with other stacks.

The resulting schema and client for other languages are available on [Buf.build](https://buf.build/crashdump/cedar-server)


### Management API

__Stores__
- [ ] CreatePolicyStore
- [ ] DeletePolicyStore

__Policies__
- [x] CreatePolicy
- [ ] GetPolicy
- [ ] ListPolicies
- [ ] DeletePolicy

__Schemas__
- [ ] CreateSchema
- [ ] GetSchema
- [ ] ListSchemas
- [ ] DeleteSchema

### Verification API
- [x] IsAuthorized
- [ ] BatchIsAuthorized


## Roadmap

[ ] Use Postgres partitions for individual stores (https://www.postgresql.org/docs/current/ddl-partitioning.html)