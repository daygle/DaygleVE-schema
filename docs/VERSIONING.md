# API Versioning

DaygleVE uses a single, node-local REST API whose contract lives entirely in
this repository. Versioning has two layers.

## 1. API major version (URL path)

The API is served under a major-version path segment: `/api/v1`. The current
value is exported as `API_VERSION` (Rust) and embedded in the backend router.

- A **breaking** change (removing a field, renaming a variant, changing a
  type, tightening validation) requires a new major version: `/api/v2`. The
  previous version stays available until formally deprecated.
- **Additive** changes (new optional fields, new enum variants, new
  endpoints) are made in place within the current major version.

## 2. Crate / package SemVer

- The Rust crate `daygleve-schema` and the npm package `@daygleve/schema`
  share the same version number.
- **MAJOR** — a breaking change to any existing type (implies a new API major
  version too).
- **MINOR** — additive, backward-compatible changes.
- **PATCH** — documentation or comment-only changes; no shape change.

## Compatibility rules for consumers

- **Clients must tolerate unknown enum variants and unknown object fields.**
  New variants/fields may be added in a MINOR release; a client pinned to an
  older MINOR must not crash on them.
- Optional fields (`Option<T>` / `field?:`) may be absent. Never assume
  presence unless the field is non-optional.
- Both backend and frontend pin an exact `daygleve-schema` / `@daygleve/schema`
  version and upgrade deliberately.

## Regenerating bindings

The TypeScript bindings in `generated/typescript/` are produced from the
annotated Rust types:

```sh
./scripts/generate.sh
```

Regenerate and commit whenever a type changes. CI verifies the checked-in
bindings match the Rust source.
