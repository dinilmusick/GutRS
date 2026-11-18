# GutTS to GutRS contract coverage

Pinned authoritative upstream: GutTS commit d14ba93ef203135ad7ac536c8ad1ec1251b5c0b5, npm package version 2.2.2.

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
- urls.joinUrl: exact slash trimming and empty-segment filtering.

Also implemented with regression coverage: strings.isBlank, strings.isNonBlank, strings.startsWithAny, strings.endsWithAny, strings.truncateWords, including ECMAScript whitespace and non-integer/NaN word counts. Full upstream differential execution is still pending.

Known uncopied or unverified GutTS public contracts:
- numbers.roundTo (lodash exponent-aware precision).
- urls.isUrl, getQueryParam and setQueryParam (WHATWG URL behavior).
- Remaining strings (isEmail, case transforms, truncate), objects, JSON, API definitions, auth, semantic-input, storage, model,
  server, worker and engineering contracts.
- Arrays.unique on reference-identity object values; Rust identity semantics must be explicit.
- Dynamic JavaScript coercion/error mapping across typed Rust API boundaries.
- Independent cross-language execution against upstream to validate the golden examples.
- Consumer integration, CI status and downstream release compatibility.

These are precise remaining gaps, not approvals to exclude public behavior. Parity is incomplete
until all exported portable contracts are mapped and verified. Native/UI-only equivalence must be
reviewed on the FrnlRS and FloksRS workstreams rather than mocked.
