# GutTS → GutRS pinned upstream contract inventory

**Pinned upstream commit:** [GutTS 3e2cbc0990ba29830ce91c5d1a1d552675d5334c](https://github.com/dinilmusick/GutTS/tree/3e2cbc0990ba29830ce91c5d1a1d552675d5334c). **Rust target:** `dinilmusick/GutRS`. This inventory was assembled from the actual pinned `package.json`, root/subpath entrypoints and transitive source exports, not from README labels.

**Entry points:** `.`, `./browser`, `./worker`, `./server`, `./server/auth`, `./server/models`, `./server/storage`, `./json`, `./strings`, `./arrays`, `./objects`, `./numbers`, `./urls`, `./urls/safety`, `./api`, `./engineering`, `./auth`, `./models`, `./semantic-input`, `./storage`, `./storage/s3`, `./storage/google-drive`, `./gutts.manifest.json`.

The `rootExports/browserExports/workerExports/serverExports` row records **entrypoint composition**, not runtime functions. Nested SDK re-exports are portable Rust **adapter obligations**, not permission to claim false behavior. The inventory has **195 listed symbols/re-export contracts**; **85 have an explicit Rust mapping** (including partial equivalents), **110 remain unmapped**. Neither a mapped symbol nor passing compilation means upstream-equivalent behavior.

## Per-export traceability

| GutTS symbol | Rust equivalent | Source / Rust location | Test or fixture | Status |
|---|---|---|---|---|
| `JsonPrimitive` | `json_utils::JsonValue` | `src/json/json-concepts.ts` → `src/json_utils.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `JsonValue` | `json_utils::JsonValue` | `src/json/json-concepts.ts` → `src/json_utils.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `JsonObject` | `json_utils::JsonObject` | `src/json/json-concepts.ts` → `src/json_utils.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `isJsonValue` | `json_utils::is_json_value` | `src/json/json-concepts.ts` → `src/json_utils.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `parseJson` | `json_utils::parse_json` | `src/json/json-concepts.ts` → `src/json_utils.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `tryParseJson` | `json_utils::try_parse_json` | `src/json/json-concepts.ts` → `src/json_utils.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `stringifyJson` | `json_utils::stringify_json` | `src/json/json-concepts.ts` → `src/json_utils.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `isBlank` | `strings::is_blank` | `src/strings/string-concepts.ts` → `src/strings.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `isNonBlank` | `strings::is_non_blank` | `src/strings/string-concepts.ts` → `src/strings.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `isEmail` | `—` | `src/strings/string-concepts.ts` → `—` | — | ❌ missing |
| `startsWithAny` | `strings::starts_with_any` | `src/strings/string-concepts.ts` → `src/strings.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `endsWithAny` | `strings::ends_with_any` | `src/strings/string-concepts.ts` → `src/strings.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `capitalize` | `—` | `src/strings/string-concepts.ts` → `—` | — | ❌ missing |
| `camelCase` | `—` | `src/strings/string-concepts.ts` → `—` | — | ❌ missing |
| `kebabCase` | `—` | `src/strings/string-concepts.ts` → `—` | — | ❌ missing |
| `snakeCase` | `—` | `src/strings/string-concepts.ts` → `—` | — | ❌ missing |
| `truncate` | `truncate::truncate_numeric` | `src/strings/string-concepts.ts` → `src/truncate.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `truncateWords` | `strings::truncate_words` | `src/strings/string-concepts.ts` → `src/strings.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `compact` | `arrays::compact` | `src/arrays/array-concepts.ts` → `src/arrays.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `unique` | `arrays::unique` | `src/arrays/array-concepts.ts` → `src/arrays.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `chunk` | `arrays::chunk` | `src/arrays/array-concepts.ts` → `src/arrays.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `flatten` | `arrays::flatten` | `src/arrays/array-concepts.ts` → `src/arrays.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `last` | `arrays::last` | `src/arrays/array-concepts.ts` → `src/arrays.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `hasOwn` | `objects::has_own_field` | `src/objects/object-concepts.ts` → `src/objects.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `pick` | `objects::pick_fields` | `src/objects/object-concepts.ts` → `src/objects.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `omit` | `objects::omit_fields` | `src/objects/object-concepts.ts` → `src/objects.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `isPlainObject` | `objects::is_plain_object` | `src/objects/object-concepts.ts` → `src/objects.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `clamp` | `numbers::clamp` | `src/numbers/number-concepts.ts` → `src/numbers.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `inRange` | `numbers::in_range` | `src/numbers/number-concepts.ts` → `src/numbers.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `roundTo` | `numbers::round_to` | `src/numbers/number-concepts.ts` → `src/numbers.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `isFiniteNumber` | `numbers::is_finite_number` | `src/numbers/number-concepts.ts` → `src/numbers.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `sum` | `numbers::sum` | `src/numbers/number-concepts.ts` → `src/numbers.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `isUrl` | `url_operations::is_url` | `src/urls/url-concepts.ts` → `src/url_operations.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `joinUrl` | `urls::join_url` | `src/urls/url-concepts.ts` → `src/urls.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `getQueryParam` | `url_operations::get_query_param` | `src/urls/url-concepts.ts` → `src/url_operations.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `setQueryParam` | `url_operations::set_query_param` | `src/urls/url-concepts.ts` → `src/url_operations.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `normalizeStoragePath` | `path_safety::normalize_storage_path` | `src/urls/path-safety.ts` → `src/path_safety.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `encodeSafePathSegments` | `path_safety::encode_safe_path_segments` | `src/urls/path-safety.ts` → `src/path_safety.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `resolveTrustedApiUrl` | `trusted_api::resolve_trusted_api_url` | `src/urls/path-safety.ts` → `src/trusted_api.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `ApiDefinition` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `ApiExample` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `ApiParameter` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `ApiSemantics` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `JsonSchema` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `defineApi` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `guttsApiDefinitions` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `guttsApiById` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `guttsFodNodeDefinitions` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `guttsApiToFodNode` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `executeGuttsApi` | `—` | `src/api/index.ts` → `—` | — | ❌ missing |
| `Identifier` | `engineering::Identifier` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `ProviderKind` | `engineering::ProviderKind` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `RepositoryRef` | `engineering::RepositoryRef` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `WorkspaceRef` | `engineering::WorkspaceRef` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `ExecutionRequest` | `engineering::ExecutionRequest` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `ExecutionResult` | `engineering::ExecutionResult` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `VerificationCheck` | `engineering::VerificationCheck` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `VerificationReport` | `engineering::VerificationReport` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `ArtifactRef` | `engineering::ArtifactRef` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `EngineeringCapability` | `engineering::EngineeringCapability` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `CapabilityGrant` | `engineering::CapabilityGrant` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `EngineeringOperationContext` | `engineering::EngineeringOperationContext` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `ProvenanceRecord` | `engineering::ProvenanceRecord` | `src/engineering/engineering-types.ts` → `src/engineering.rs` | src/engineering.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `IDENTITY_KINDS` | `identity::IDENTITY_KINDS` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `IdentityKind` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `IdentityId` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `ApplicationId` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `DeploymentId` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `OAuthClientId` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `UserId` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `WorkspaceId` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `UiSessionId` | `—` | `src/auth/identityIds.ts` → `—` | — | ❌ missing |
| `IdentityRef` | `identity::IdentityRef` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `isIdentityKind` | `identity::is_identity_kind` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `parseIdentityId` | `identity::parse_identity_id` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `requireIdentityId` | `identity::require_identity_id` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `identityRef` | `identity::identity_ref` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `parseIdentityRef` | `identity::parse_identity_ref` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `parseLegacyApplicationId` | `identity::parse_legacy_application_id` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `ULTRST_APP_CONTRACT_VERSION` | `auth_contracts::ULTRST_APP_CONTRACT_VERSION` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `OAuthPlatform` | `—` | `src/auth/oauthClientContract.ts` → `—` | — | ❌ missing |
| `OAuthClientRegistrationV1` | `—` | `src/auth/oauthClientContract.ts` → `—` | — | ❌ missing |
| `ContractCheck` | `auth_contracts::ContractCheck` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `contractRecord` | `auth_contracts::contract_record` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `contractId` | `auth_contracts::contract_id` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `contractAppId` | `auth_contracts::contract_app_id` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `contractExactKeys` | `auth_contracts::contract_exact_keys` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `validateOAuthRedirectUri` | `auth_contracts::validate_oauth_redirect_uri` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `validateOAuthClientRegistration` | `auth_contracts::validate_oauth_client_registration` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `CapabilityParameterKind` | `—` | `src/auth/capabilityContract.ts` → `—` | — | ❌ missing |
| `CapabilityParameterV1` | `—` | `src/auth/capabilityContract.ts` → `—` | — | ❌ missing |
| `CapabilityBaseV1` | `—` | `src/auth/capabilityContract.ts` → `—` | — | ❌ missing |
| `StaticCapabilityV1` | `—` | `src/auth/capabilityContract.ts` → `—` | — | ❌ missing |
| `WorkflowPublicationV1` | `—` | `src/auth/capabilityContract.ts` → `—` | — | ❌ missing |
| `CapabilityDescriptorV1` | `—` | `src/auth/capabilityContract.ts` → `—` | — | ❌ missing |
| `validateCapabilityDescriptor` | `auth_contracts::validate_capability_descriptor` | `src/auth/capabilityContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `ScopedInvocationV1` | `—` | `src/auth/invocationContract.ts` → `—` | — | ❌ missing |
| `validateScopedInvocation` | `auth_contracts::validate_scoped_invocation` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `checkInvocationBudget` | `auth_contracts::check_invocation_budget` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `advanceInvocationBudget` | `auth_contracts::advance_invocation_budget` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `consumeInvocationRetry` | `auth_contracts::consume_invocation_retry` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `validateInvocationAgainstDescriptor` | `auth_contracts::validate_invocation_against_descriptor` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `DeploymentEnvironment` | `auth_types::DeploymentEnvironment` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthDeploymentRecord` | `auth_types::LuthDeploymentRecord` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthAuthClientConfig` | `—` | `src/auth/luthAuthTypes.ts` → `—` | — | ❌ missing |
| `LuthAuthTokenStore` | `—` | `src/auth/luthAuthTypes.ts` → `—` | — | ❌ missing |
| `RegisterApplicationResult` | `auth_types::RegisterApplicationResult` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthUser` | `auth_types::LuthUser` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthSession` | `auth_types::LuthSession` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthSessionResult` | `auth_types::LuthSessionResult` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `EmailSignInCredentials` | `auth_types::EmailSignInCredentials` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `EmailSignUpData` | `auth_types::EmailSignUpData` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `GoogleSignInOptions` | `auth_types::GoogleSignInOptions` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthAuthError` | `auth_types::LuthAuthError` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `resolveRuntimeAppUrl` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `resolveClientOrigin` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `resolveClientEnvironment` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `DEFAULT_LUTH_URL` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `hydrateLuthTokenFromUrl` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `LuthAuthClient` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `createLuthAuthClient` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `ApplicationAuthClientConfig` | `—` | `src/auth/applicationAuthClient.ts` → `—` | — | ❌ missing |
| `ApplicationAuthClient` | `—` | `src/auth/applicationAuthClient.ts` → `—` | — | ❌ missing |
| `createApplicationAuthClient` | `—` | `src/auth/applicationAuthClient.ts` → `—` | — | ❌ missing |
| `ModelMetadata` | `models::ModelMetadata` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `ModelLoadContext` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ModelRuntime` | `models::ModelRuntime` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `ModelLease` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ModelLoader` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ModelUnloader` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ModelResidency` | `models::ModelResidency` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `ModelLibrary` | `models::ModelLibrary` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `guttsModelDefinitions` | `models::gutts_model_definitions` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `guttsModelById` | `models::gutts_model_by_id` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `Needle3` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `needle3CompleteApi` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `needle3Metadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `SmolLM2` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `smolLM2GenerateApi` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `smolLM2Metadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `Whistle` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `whistleTranscribeApi` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `whistleMetadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `EttinReranker` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `EttinReranker17M` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `EttinReranker32M` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `EttinReranker68M` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `EttinReranker150M` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ettinRerankApi` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ettinReranker17MMetadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ettinReranker32MMetadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ettinReranker68MMetadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `ettinReranker150MMetadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `MiniLM` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `miniLMSimilarityApi` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `miniLML6Metadata` | `—` | `src/models/index.ts` → `—` | — | ❌ missing |
| `SemanticInputSchema` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputContext` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputErrorCode` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputRisk` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputProgress` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputEvent` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputModelPolicy` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputPolicy` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputProvenanceStore` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputProvenanceRecord` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputProvenance` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputCandidate` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputEvidence` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputClarification` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputResolution` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputCacheKey` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputCache` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputRequest` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `resolveDeterministicSemanticInput` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `rankSemanticInputCandidates` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `resolveSemanticInput` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `S3Client` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `GetObjectCommand` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `HeadObjectCommand` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `ListObjectsV2Command` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `PutObjectCommand` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `CopyObjectCommand` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `DeleteObjectCommand` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `DeleteObjectsCommand` | `—` | `src/storage/s3.ts` → `—` | — | ❌ missing |
| `GoogleApisModule` | `—` | `src/storage/google-drive.ts` → `—` | — | ❌ missing |
| `loadGoogleApis` | `—` | `src/storage/google-drive.ts` → `—` | — | ❌ missing |
| `createGoogleDriveClient` | `—` | `src/storage/google-drive.ts` → `—` | — | ❌ missing |
| `resolveServerRuntimeAppUrl` | `—` | `src/server/auth.ts` → `—` | — | ❌ missing |
| `nodeCactusNeedle3Runtime` | `—` | `src/server/models.ts` → `—` | — | ❌ missing |
| `nodeCactusWhistleRuntime` | `—` | `src/server/models.ts` → `—` | — | ❌ missing |
| `rootExports` | `—` | `src/worker.ts` → `—` | — | ❌ missing |
| `browserExports` | `—` | `src/worker.ts` → `—` | — | ❌ missing |
| `workerExports` | `—` | `src/worker.ts` → `—` | — | ❌ missing |
| `serverExports` | `—` | `src/worker.ts` → `—` | — | ❌ missing |

## Required semantics and outstanding differences

- **Rust types vs JavaScript dynamic domain:** `serde_json::Value` only represents JSON-safe values; JavaScript `undefined`, object identity, prototype getters, cycles, BigInt, functions and references are not representable at the typed Rust boundary. This is an explicit partial equivalence, not a full `isJsonValue`, `unique`, `compact` or object-path equivalent.
- **Object operations:** current native `pick_fields`, `omit_fields`, `has_own_field` operate on shallow own JSON fields. Upstream lodash `has`/`pick`/`omit` can navigate deep property paths. Full path handling is missing; special keys, arrays and inherited JS properties require documented differential tests.
- **Unicode strings:** Rust `truncate` currently uses Unicode extended grapheme clusters, whereas lodash has its own Unicode symbol segmentation; equivalent outcomes must be independently compared for combining marks and joined emoji. `isEmail` and case-conversion functions are absent.
- **Floating point:** `round_to` rejects precision above 308 as an explicit unsupported case. JavaScript NaN, signed zero, positive/negative infinity and decimal exponent boundaries require direct upstream differential fixtures.
- **URLs and safety:** Rust `url` implements URL parsing and query pairs, but WHATWG edge cases, canonical IPv6 host serialization, percent escapes and `URLSearchParams` quirks require direct upstream differential execution. Unsafe traversal, malformed escapes, origin credentials and multi-level encodings must fail closed.
- **OAuth/invocation:** wire validators do not authenticate a Luth session, prove OAuth client ownership, confirm grant/workspace membership, reauthorize a target, or prove live frontend consent. They check syntax/shape and execution budgets only; they must not be used as access-control decisions.
- **Models and semantic input:** runtime model inference, residency, API registry/execution and semantic input resolution are missing Rust implementations, **not platform exceptions**. They require explicit native runtime adapters and complete behavioral tests.
- **S3, Google Drive, browser, server:** dependency adapters are not portable utility logic. They need native feature-gated implementations to be marked equivalent. No SDK is mocked or silently replaced.
- **Error behavior:** Rust returns explicit `Result`/`Option` where TypeScript throws `RangeError`, `TypeError`, `SyntaxError` or returns `undefined`. Mapping must be tested per exported contract; do not conflate `null` and `undefined`.

## Fixture provenance / certification

`testdata/upstream-golden/portable.json` contains positive, negative and boundary examples derived by reading the pinned GutTS source and pinned TypeScript tests. `tests/upstream_golden.rs` executes the corresponding **public Rust exports** on those inputs. **The fixture has not yet been generated by actually executing pinned GutTS**; cross-language differential certification is therefore pending. `examples/external-consumer` is a distinct Cargo crate consuming GutRS via its public package boundary. The three-host CI includes format, lint, unit, docs and that consumer. A full parity claim is forbidden while any mapped symbol is partial or missing, any fixture lacks upstream verification, or CI/independent verification is absent.
