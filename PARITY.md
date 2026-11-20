# GutTS to GutRS contract coverage

Previously pinned baseline: GutTS commit d14ba93ef203135ad7ac536c8ad1ec1251b5c0b5, npm package version 2.2.2. Latest inspected main: d5706af3484d496d665a9c73b4bc9dcac7a64560 (cross-app identity contracts; not yet ported).

Implemented and regression-tested in this PR:
- arrays.chunk: integer sizing, invalid-size rejection, ordering and empty arrays.
- arrays.flatten: exactly one level for scalar/list inputs.
- arrays.last: empty input maps to Option::None.
- arrays.unique: SameValueZero for declared scalar types (NaN and signed zero).
- arrays.compact: nullable scalar JS falsy rules (false, 0, empty string, NaN).
- numbers.clamp: normal bounds and invalid ordering.
- numbers.inRange: inclusive lower and exclusive upper bound.
- numbers.isFiniteNumber: finite f64 values.
- numbers.sum: sequential IEEE-754 accumulation.
- numbers.roundTo: exponent-shifted decimal rounding, JavaScript negative half ties, NaN/Infinity and invalid-precision rejection (verified fixture slice; deliberately rejects precision >308 rather than claiming full edge-case parity).
- urls.joinUrl: exact slash trimming and empty-segment filtering.

Also implemented with regression coverage: strings.isBlank, strings.isNonBlank, strings.startsWithAny, strings.endsWithAny, strings.truncateWords, including ECMAScript whitespace and non-integer/NaN word counts. Full upstream differential execution is still pending.

Known uncopied or unverified GutTS public contracts:
- urls.isUrl, getQueryParam and setQueryParam now implemented in url_operations.rs with regression fixtures; exact-head Cargo and differential WHATWG verification remain pending.
- Remaining strings (isEmail, case transforms, truncate), objects, JSON, API definitions, auth, semantic-input, storage, model,
  server, worker and engineering contracts.
- Arrays.unique on reference-identity object values; Rust identity semantics must be explicit.
- Dynamic JavaScript coercion/error mapping across typed Rust API boundaries.
- Independent cross-language execution against upstream to validate the golden examples.
- Consumer integration, CI status and downstream release compatibility.

These are precise remaining gaps, not approvals to exclude public behavior. Parity is incomplete
until all exported portable contracts are mapped and verified. Native/UI-only equivalence must be
reviewed on the FrnlRS and FloksRS workstreams rather than mocked.
