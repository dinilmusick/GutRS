# GutTS → GutRS pinned upstream contract inventory

**Pinned upstream commit:** [GutTS 3e2cbc0990ba29830ce91c5d1a1d552675d5334c](https://github.com/dinilmusick/GutTS/tree/3e2cbc0990ba29830ce91c5d1a1d552675d5334c). **Rust target:** `dinilmusick/GutRS`. This inventory was assembled from the actual pinned `package.json`, root/subpath entrypoints and transitive source exports, not from README labels.

**Entry points:** `.`, `./browser`, `./worker`, `./server`, `./server/auth`, `./server/models`, `./server/storage`, `./json`, `./strings`, `./arrays`, `./objects`, `./numbers`, `./urls`, `./urls/safety`, `./api`, `./engineering`, `./auth`, `./models`, `./semantic-input`, `./storage`, `./storage/s3`, `./storage/google-drive`, `./gutts.manifest.json`.

The `rootExports/browserExports/workerExports/serverExports` row records **entrypoint composition**, not runtime functions. Nested SDK re-exports are portable Rust **adapter obligations**, not permission to claim false behavior. The inventory has **195 listed symbols/re-export contracts**; **122 have an explicit Rust mapping** (including partial equivalents), **73 remain unmapped**. Neither a mapped symbol nor passing compilation means upstream-equivalent behavior.

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
| `isEmail` | `string_ext::is_email` | `src/strings/string-concepts.ts` → `src/string_ext.rs` | tests/upstream_golden.rs + upstream JS | △ policy/Unicode differentials pending |
| `startsWithAny` | `strings::starts_with_any` | `src/strings/string-concepts.ts` → `src/strings.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `endsWithAny` | `strings::ends_with_any` | `src/strings/string-concepts.ts` → `src/strings.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `capitalize` | `string_ext::capitalize` | `src/strings/string-concepts.ts` → `src/string_ext.rs` | tests/upstream_golden.rs + upstream JS | △ policy/Unicode differentials pending |
| `camelCase` | `string_ext::camel_case` | `src/strings/string-concepts.ts` → `src/string_ext.rs` | tests/upstream_golden.rs + upstream JS | △ policy/Unicode differentials pending |
| `kebabCase` | `string_ext::kebab_case` | `src/strings/string-concepts.ts` → `src/string_ext.rs` | tests/upstream_golden.rs + upstream JS | △ policy/Unicode differentials pending |
| `snakeCase` | `string_ext::snake_case` | `src/strings/string-concepts.ts` → `src/string_ext.rs` | tests/upstream_golden.rs + upstream JS | △ policy/Unicode differentials pending |
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
| `ApiDefinition` | `api::ApiDefinition` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `ApiExample` | `api::ApiExample` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `ApiParameter` | `api::ApiParameter` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `ApiSemantics` | `api::ApiSemantics` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `JsonSchema` | `api::JsonSchema` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `defineApi` | `api::define_api` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `guttsApiDefinitions` | `api::gutts_api_definitions` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `guttsApiById` | `api::gutts_api_by_id` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `guttsFodNodeDefinitions` | `api::gutts_fod_node_definitions` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `guttsApiToFodNode` | `api::gutts_api_to_fod_node` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
| `executeGuttsApi` | `api::execute_gutts_api` | `src/api/index.ts` → `src/api.rs` | src/api.rs (unit tests); pinned manifest | ◐ native metadata / partial runtime dispatch |
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
| `resolveRuntimeAppUrl` | `auth_host::resolve_runtime_app_url` | `src/auth/luthAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `resolveClientOrigin` | `auth_host::resolve_client_origin` | `src/auth/luthAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `resolveClientEnvironment` | `auth_host::resolve_client_environment` | `src/auth/luthAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `DEFAULT_LUTH_URL` | `auth_host::DEFAULT_LUTH_URL` | `src/auth/luthAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `hydrateLuthTokenFromUrl` | `auth_host::extract_luth_token_and_clean_url` | `src/auth/luthAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `LuthAuthClient` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `createLuthAuthClient` | `—` | `src/auth/luthAuthClient.ts` → `—` | — | ❌ missing |
| `ApplicationAuthClientConfig` | `auth_host::ApplicationAuthClientConfig` | `src/auth/applicationAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `ApplicationAuthClient` | `auth_host::ApplicationAuthClient` | `src/auth/applicationAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `createApplicationAuthClient` | `auth_host::create_application_auth_client` | `src/auth/applicationAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
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
| `SemanticInputSchema` | `semantic_input::SemanticInputSchema` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputContext` | `semantic_input::SemanticInputContext` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputErrorCode` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputRisk` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputProgress` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputEvent` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputModelPolicy` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputPolicy` | `semantic_input::SemanticInputPolicy` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputProvenanceStore` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputProvenanceRecord` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputProvenance` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputCandidate` | `semantic_input::SemanticInputCandidate` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputEvidence` | `—` | `src/semantic-input/index.ts` → `—` | — | ❌ missing |
| `SemanticInputClarification` | `semantic_input::SemanticInputClarification` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputResolution` | `semantic_input::SemanticInputResolution` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputCacheKey` | `semantic_input::SemanticInputCacheKey` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputCache` | `semantic_input::SemanticInputCache` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputRequest` | `semantic_input::SemanticInputRequest` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `resolveDeterministicSemanticInput` | `semantic_input::resolve_deterministic_semantic_input` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `rankSemanticInputCandidates` | `semantic_input::rank_semantic_input_candidates` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `resolveSemanticInput` | `semantic_input::resolve_semantic_input` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
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
| `resolveServerRuntimeAppUrl` | `auth_host::resolve_server_runtime_app_url` | `src/server/auth.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
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
- **Models and semantic input:** actual inference runtimes and semantic input resolution are still missing. Model metadata and thread-safe residency now exist; the API registry reproduces pinned definitions, with explicit errors for unported runtime operations. They require explicit native runtime adapters and complete behavioral tests.
- **S3, Google Drive, browser, server:** dependency adapters are not portable utility logic. They need native feature-gated implementations to be marked equivalent. No SDK is mocked or silently replaced.
- **Error behavior:** Rust returns explicit `Result`/`Option` where TypeScript throws `RangeError`, `TypeError`, `SyntaxError` or returns `undefined`. Mapping must be tested per exported contract; do not conflate `null` and `undefined`.

## Fixture provenance / certification

`testdata/upstream-golden/portable.json` contains positive, negative and boundary examples derived by reading the pinned GutTS source and pinned TypeScript tests. `tests/upstream_golden.rs` executes the corresponding **public Rust exports** on those inputs. **The fixture has not yet been generated by actually executing pinned GutTS**; cross-language differential certification is therefore pending. `examples/external-consumer` is a distinct Cargo crate consuming GutRS via its public package boundary. The three-host CI includes format, lint, unit, docs and that consumer. A full parity claim is forbidden while any mapped symbol is partial or missing, any fixture lacks upstream verification, or CI/independent verification is absent.
