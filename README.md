# DaygleVE-schema

Shared API type definitions for the [DaygleVE](https://github.com/daygle)
single-node virtualization platform. This repository is the **single source of
truth** for every request/response body, enum and error shape that crosses the
API boundary between the backend and the frontend.

```
┌────────────────────┐         imports Rust crate          ┌────────────────────┐
│  DaygleVE-backend  │  ◀───────────────────────────────   │  DaygleVE-schema   │
│      (Rust)        │                                      │  (types only)      │
└────────────────────┘         imports @daygleve/schema     └────────────────────┘
┌────────────────────┐  ◀───────────────────────────────         ▲
│  DaygleVE-frontend │      (generated TypeScript)                │
│    (SvelteKit)     │────────────────────────────────────────────┘
└────────────────────┘
```

## Rules

- **Types only.** No business logic, no I/O, no framework or UI code.
- Every public type derives `Serialize` + `Deserialize` and is annotated with
  `#[typeshare]`.
- Backend and frontend both import from here and never redefine these shapes.

## Layout

| Path                          | Purpose                                         |
| ----------------------------- | ----------------------------------------------- |
| `src/`                        | Canonical Rust type definitions.                |
| `generated/typescript/`       | Generated TS bindings + `@daygleve/schema` pkg. |
| `openapi/daygleve.v1.yaml`    | REST endpoint surface (paths, methods, auth).   |
| `scripts/generate.sh`         | Regenerates the TypeScript bindings.            |
| `docs/VERSIONING.md`          | API/SemVer versioning and compatibility rules.  |

## Consuming from the backend (Rust)

```toml
# DaygleVE-backend/Cargo.toml
[dependencies]
daygleve-schema = { git = "https://github.com/daygle/DaygleVE-schema", branch = "main" }
```

## Consuming from the frontend (TypeScript)

```jsonc
// DaygleVE-frontend/package.json
"dependencies": {
  "@daygleve/schema": "github:daygle/DaygleVE-schema#main"
}
```

```ts
import type { Vm, CreateVmRequest, ApiError } from "@daygleve/schema";
```

## Modules

`common` · `auth` · `vm` · `lxc` · `storage` · `network` · `gpu` · `metrics`

## Regenerating bindings

```sh
cargo build           # type-check the Rust definitions
./scripts/generate.sh # regenerate generated/typescript/index.ts
```

Commit the regenerated bindings alongside the Rust change.

## License

Apache-2.0.
