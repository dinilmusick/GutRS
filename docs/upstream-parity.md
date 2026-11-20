# GutTS → GutRS pinned upstream contract inventory

**Pinned upstream commit:** [GutTS 3e2cbc0990ba29830ce91c5d1a1d552675d5334c](https://github.com/dinilmusick/GutTS/tree/3e2cbc0990ba29830ce91c5d1a1d552675d5334c). **Rust target:** `dinilmusick/GutRS`. This inventory was assembled from the actual pinned `package.json`, root/subpath entrypoints and transitive source exports, not from README labels.

**Entry points:** `.`, `./browser`, `./worker`, `./server`, `./server/auth`, `./server/models`, `./server/storage`, `./json`, `./strings`, `./arrays`, `./objects`, `./numbers`, `./urls`, `./urls/safety`, `./api`, `./engineering`, `./auth`, `./models`, `./semantic-input`, `./storage`, `./storage/s3`, `./storage/google-drive`, `./gutts.manifest.json`.

The `rootExports/browserExports/workerExports/serverExports` row records **entrypoint composition**, not runtime functions. Nested SDK re-exports are portable Rust **adapter obligations**, not permission to claim false behavior. The inventory has **195 listed symbols/re-export contracts**; **195 have an explicit Rust mapping** (including partial equivalents), **0 remain unmapped**. Neither a mapped symbol nor passing compilation means upstream-equivalent behavior.

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
| `IdentityKind` | `auth_wires::IdentityKind` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `IdentityId` | `auth_wires::IdentityId` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `ApplicationId` | `auth_wires::ApplicationId` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `DeploymentId` | `auth_wires::DeploymentId` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `OAuthClientId` | `auth_wires::OAuthClientId` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `UserId` | `auth_wires::UserId` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `WorkspaceId` | `auth_wires::WorkspaceId` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `UiSessionId` | `auth_wires::UiSessionId` | `src/auth/identityIds.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `IdentityRef` | `identity::IdentityRef` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `isIdentityKind` | `identity::is_identity_kind` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `parseIdentityId` | `identity::parse_identity_id` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `requireIdentityId` | `identity::require_identity_id` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `identityRef` | `identity::identity_ref` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `parseIdentityRef` | `identity::parse_identity_ref` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `parseLegacyApplicationId` | `identity::parse_legacy_application_id` | `src/auth/identityIds.ts` → `src/identity.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `ULTRST_APP_CONTRACT_VERSION` | `auth_contracts::ULTRST_APP_CONTRACT_VERSION` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `OAuthPlatform` | `auth_wires::OAuthPlatform` | `src/auth/oauthClientContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `OAuthClientRegistrationV1` | `auth_wires::OAuthClientRegistrationV1` | `src/auth/oauthClientContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `ContractCheck` | `auth_contracts::ContractCheck` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `contractRecord` | `auth_contracts::contract_record` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `contractId` | `auth_contracts::contract_id` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `contractAppId` | `auth_contracts::contract_app_id` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `contractExactKeys` | `auth_contracts::contract_exact_keys` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `validateOAuthRedirectUri` | `auth_contracts::validate_oauth_redirect_uri` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | △ partial / differential pending |
| `validateOAuthClientRegistration` | `auth_contracts::validate_oauth_client_registration` | `src/auth/oauthClientContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `CapabilityParameterKind` | `auth_wires::CapabilityParameterKind` | `src/auth/capabilityContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `CapabilityParameterV1` | `auth_wires::CapabilityParameterV1` | `src/auth/capabilityContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `CapabilityBaseV1` | `auth_wires::CapabilityBaseV1` | `src/auth/capabilityContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `StaticCapabilityV1` | `auth_wires::StaticCapabilityV1` | `src/auth/capabilityContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `WorkflowPublicationV1` | `auth_wires::WorkflowPublicationV1` | `src/auth/capabilityContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `CapabilityDescriptorV1` | `auth_wires::CapabilityDescriptorV1` | `src/auth/capabilityContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `validateCapabilityDescriptor` | `auth_contracts::validate_capability_descriptor` | `src/auth/capabilityContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `ScopedInvocationV1` | `auth_wires::ScopedInvocationV1` | `src/auth/invocationContract.ts` → `src/auth_wires.rs` | src/auth_wires.rs (unit tests) | ◐ checked typed wire implementation |
| `validateScopedInvocation` | `auth_contracts::validate_scoped_invocation` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `checkInvocationBudget` | `auth_contracts::check_invocation_budget` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `advanceInvocationBudget` | `auth_contracts::advance_invocation_budget` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `consumeInvocationRetry` | `auth_contracts::consume_invocation_retry` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `validateInvocationAgainstDescriptor` | `auth_contracts::validate_invocation_against_descriptor` | `src/auth/invocationContract.ts` → `src/auth_contracts.rs` | tests/upstream_golden.rs; module unit tests | ◐ implemented / golden certification pending |
| `DeploymentEnvironment` | `auth_types::DeploymentEnvironment` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthDeploymentRecord` | `auth_types::LuthDeploymentRecord` | `src/auth/luthAuthTypes.ts` → `src/auth_types.rs` | src/auth_types.rs (unit tests) | ◐ typed wire mapping / runtime pending |
| `LuthAuthClientConfig` | `luth_client::LuthAuthClientConfig` | `src/auth/luthAuthTypes.ts` → `src/luth_client.rs` | src/luth_client.rs (unit tests) | ◐ host HTTP transport required |
| `LuthAuthTokenStore` | `luth_client::LuthAuthTokenStore` | `src/auth/luthAuthTypes.ts` → `src/luth_client.rs` | src/luth_client.rs (unit tests) | ◐ host HTTP transport required |
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
| `LuthAuthClient` | `luth_client::LuthAuthClient` | `src/auth/luthAuthClient.ts` → `src/luth_client.rs` | src/luth_client.rs (unit tests) | ◐ host HTTP transport required |
| `createLuthAuthClient` | `luth_client::create_luth_auth_client` | `src/auth/luthAuthClient.ts` → `src/luth_client.rs` | src/luth_client.rs (unit tests) | ◐ host HTTP transport required |
| `ApplicationAuthClientConfig` | `auth_host::ApplicationAuthClientConfig` | `src/auth/applicationAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `ApplicationAuthClient` | `auth_host::ApplicationAuthClient` | `src/auth/applicationAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `createApplicationAuthClient` | `auth_host::create_application_auth_client` | `src/auth/applicationAuthClient.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `ModelMetadata` | `models::ModelMetadata` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `ModelLoadContext` | `model_operations::ModelLoadContext` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ModelRuntime` | `models::ModelRuntime` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `ModelLease` | `model_operations::ModelLease` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ModelLoader` | `model_operations::ModelLoader` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ModelUnloader` | `model_operations::ModelUnloader` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ModelResidency` | `models::ModelResidency` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `ModelLibrary` | `models::ModelLibrary` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `guttsModelDefinitions` | `models::gutts_model_definitions` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `guttsModelById` | `models::gutts_model_by_id` | `src/models/index.ts` → `src/models.rs` | src/models.rs (unit tests) | ◐ native contract / model inference host pending |
| `Needle3` | `model_operations::Needle3` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `needle3CompleteApi` | `model_operations::needle3_complete_api` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `needle3Metadata` | `model_operations::needle3_metadata` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `SmolLM2` | `model_operations::SmolLM2` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `smolLM2GenerateApi` | `model_operations::smollm2_generate_api` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `smolLM2Metadata` | `model_operations::smollm2_metadata` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `Whistle` | `model_operations::Whistle` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `whistleTranscribeApi` | `model_operations::whistle_transcribe_api` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `whistleMetadata` | `model_operations::whistle_metadata` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `EttinReranker` | `model_operations::EttinReranker` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `EttinReranker17M` | `model_operations::EttinReranker17M` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `EttinReranker32M` | `model_operations::EttinReranker32M` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `EttinReranker68M` | `model_operations::EttinReranker68M` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `EttinReranker150M` | `model_operations::EttinReranker150M` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ettinRerankApi` | `model_operations::ettin_rerank_api` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ettinReranker17MMetadata` | `model_operations::ettin_metadata(17)` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ettinReranker32MMetadata` | `model_operations::ettin_metadata(32)` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ettinReranker68MMetadata` | `model_operations::ettin_metadata(68)` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `ettinReranker150MMetadata` | `model_operations::ettin_metadata(150)` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `MiniLM` | `model_operations::MiniLM` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `miniLMSimilarityApi` | `model_operations::minilm_similarity_api` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `miniLML6Metadata` | `model_operations::minilm_l6_metadata` | `src/models/index.ts` → `src/model_operations.rs` | src/model_operations.rs (unit tests) | △ real inference requires host runtime |
| `SemanticInputSchema` | `semantic_input::SemanticInputSchema` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputContext` | `semantic_input::SemanticInputContext` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputErrorCode` | `semantic_types::SemanticInputErrorCode` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputRisk` | `semantic_types::SemanticInputRisk` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputProgress` | `semantic_types::SemanticInputProgress` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputEvent` | `semantic_types::SemanticInputEvent` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputModelPolicy` | `semantic_types::SemanticInputModelPolicy` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputPolicy` | `semantic_input::SemanticInputPolicy` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputProvenanceStore` | `semantic_types::SemanticInputProvenanceStore` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputProvenanceRecord` | `semantic_types::SemanticInputProvenanceRecord` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputProvenance` | `semantic_types::SemanticInputProvenance` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputCandidate` | `semantic_input::SemanticInputCandidate` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputEvidence` | `semantic_types::SemanticInputEvidence` | `src/semantic-input/index.ts` → `src/semantic_types.rs` | src/semantic_types.rs (unit tests) | ◐ typed provenance/events; host callbacks pending |
| `SemanticInputClarification` | `semantic_input::SemanticInputClarification` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputResolution` | `semantic_input::SemanticInputResolution` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputCacheKey` | `semantic_input::SemanticInputCacheKey` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputCache` | `semantic_input::SemanticInputCache` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `SemanticInputRequest` | `semantic_input::SemanticInputRequest` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `resolveDeterministicSemanticInput` | `semantic_input::resolve_deterministic_semantic_input` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `rankSemanticInputCandidates` | `semantic_input::rank_semantic_input_candidates` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `resolveSemanticInput` | `semantic_input::resolve_semantic_input` | `src/semantic-input/index.ts` → `src/semantic_input.rs` | src/semantic_input.rs (unit tests) | ◐ portable core; native model host required |
| `S3Client` | `storage_adapters::s3::S3Client` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `GetObjectCommand` | `storage_adapters::s3::GetObjectCommand` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `HeadObjectCommand` | `storage_adapters::s3::HeadObjectCommand` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `ListObjectsV2Command` | `storage_adapters::s3::ListObjectsV2Command` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `PutObjectCommand` | `storage_adapters::s3::PutObjectCommand` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `CopyObjectCommand` | `storage_adapters::s3::CopyObjectCommand` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `DeleteObjectCommand` | `storage_adapters::s3::DeleteObjectCommand` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `DeleteObjectsCommand` | `storage_adapters::s3::DeleteObjectsCommand` | `src/storage/s3.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `GoogleApisModule` | `storage_adapters::google_drive::GoogleApisModule` | `src/storage/google-drive.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `loadGoogleApis` | `storage_adapters::google_drive::GoogleApisModule` | `src/storage/google-drive.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | △ Native compile-time import replaces JavaScript dynamic import |
| `createGoogleDriveClient` | `storage_adapters::google_drive::create_google_drive_client` | `src/storage/google-drive.ts` → `src/storage_adapters.rs` | feature-enabled SDK compile test | ◐ Real SDK feature-gated; cloud integration unverified |
| `resolveServerRuntimeAppUrl` | `auth_host::resolve_server_runtime_app_url` | `src/server/auth.ts` → `src/auth_host.rs` | src/auth_host.rs (unit tests) | ◐ host-injected implementation |
| `nodeCactusNeedle3Runtime` | `cactus_native::native::NodeCactusNeedle3Runtime` | `src/server/models.ts` → `src/cactus_native.rs` | feature-gated native Cactus SDK type test | △ native inference platform/weights required |
| `nodeCactusWhistleRuntime` | `cactus_native::native::NodeCactusWhistleRuntime` | `src/server/models.ts` → `src/cactus_native.rs` | feature-gated native Cactus SDK type test | △ native inference platform/weights required |
| `rootExports` | `src/lib.rs` | `src/worker.ts` → `src/lib.rs` | cargo public consumer + module exports | △ entrypoint composition; platform-specific |
| `browserExports` | `src/lib.rs (native counterpart, not WASM DOM bindings)` | `src/worker.ts` → `src/lib.rs` | cargo public consumer + module exports | △ entrypoint composition; platform-specific |
| `workerExports` | `src/lib.rs (std hosted only; no Cloudflare JS Worker runtime)` | `src/worker.ts` → `src/lib.rs` | cargo public consumer + module exports | △ entrypoint composition; platform-specific |
| `serverExports` | `src/lib.rs + auth_host/luth_client/storage_adapters` | `src/worker.ts` → `src/lib.rs` | cargo public consumer + module exports | △ entrypoint composition; platform-specific |

## Required semantics and outstanding differences

- **Rust types vs JavaScript dynamic domain:** `serde_json::Value` only represents JSON-safe values; JavaScript `undefined`, object identity, prototype getters, cycles, BigInt, functions and references are not representable at the typed Rust boundary. This is an explicit partial equivalence, not a full `isJsonValue`, `unique`, `compact` or object-path equivalent.
- **Object operations:** `has_property_path`, `pick_property_paths` and `omit_property_paths` now handle nested JSON object/array paths, quoted bracket notation and literal-key precedence. They have independent GutTS golden cases. Limitations remain: JavaScript inherited properties, symbolic property keys, unusual escaping and non-JSON object identity are not representable.
- **Unicode strings:** Rust `truncate` currently uses Unicode extended grapheme clusters, whereas lodash has its own Unicode symbol segmentation; equivalent outcomes must be independently compared for combining marks and joined emoji. `isEmail` and case conversions are implemented with Rust `email_address` and `heck`, but their Unicode and validator.js-specific policy differences require broader differential fixtures.
- **Floating point:** `round_to` rejects precision above 308 as an explicit unsupported case. JavaScript NaN, signed zero, positive/negative infinity and decimal exponent boundaries require direct upstream differential fixtures.
- **URLs and safety:** Rust `url` implements URL parsing and query pairs, but WHATWG edge cases, canonical IPv6 host serialization, percent escapes and `URLSearchParams` quirks require direct upstream differential execution. Unsafe traversal, malformed escapes, origin credentials and multi-level encodings must fail closed.
- **OAuth/invocation:** wire validators do not authenticate a Luth session, prove OAuth client ownership, confirm grant/workspace membership, reauthorize a target, or prove live frontend consent. They check syntax/shape and execution budgets only; they must not be used as access-control decisions.
- **Models and semantic input:** model metadata, thread-safe residency, typed Cactus/Transformers model interfaces, deterministic semantic resolution, candidate-ranking and host-injected model operation adapters are implemented. Real Needle3/Whistle model weights and executable engine backends are **not** bundled or integration-certified. API registry model operations return explicit no-adapter errors; no fake model output is returned.
- **S3, Google Drive, browser, server:** actual AWS Rust S3 and Google Drive v3 SDKs are optional features, with real request builders and a Drive hub. Credentialed live access remains unverified. Browser/worker/server JS entrypoints are mapped to Rust modules and hosted adapters rather than falsely emulating the JavaScript runtime.
- **Error behavior:** Rust returns explicit `Result`/`Option` where TypeScript throws `RangeError`, `TypeError`, `SyntaxError` or returns `undefined`. Mapping must be tested per exported contract; do not conflate `null` and `undefined`.

## Fixture provenance / certification

`testdata/upstream-golden/portable.json` contains positive, negative and boundary examples derived by reading the pinned GutTS source and pinned TypeScript tests. `tests/upstream_golden.rs` executes the corresponding **public Rust exports** on those inputs. The independent `upstream-differential` CI executes **52 same source-derived cases against actual pinned GutTS**, and compares all 38 API definitions, 38 FOD definitions and eight model metadata records with the native registry. This certifies those tested cases, **not exhaustive export parity**. `examples/external-consumer` is a distinct Cargo crate consuming GutRS via its public package boundary. The three-host CI includes format, lint, unit, docs and that consumer. Full parity remains unverified for JavaScript-only behaviors, host-specific inference engines and credentialed cloud integrations; mapping/type parity is not full behavioral parity.

## Hosted storage SDK adapters

Enable `--features aws-s3` to expose the real `aws-sdk-s3` client and native request builders. Enable `--features google-drive` for the actual `google-drive3` v3 hub and an authenticated factory. These are **optional host-specific equivalents**, not native JavaScript `@aws-sdk/client-s3` / `googleapis` reexports; credentials and signed OAuth grants are supplied by the consumer. Tests prove builder construction and API types without cloud credentials; they do **not** prove end-to-end access to live storage.

## Packaging and platform qualification

The native Rust crate is a single entry graph (`src/lib.rs`). The TypeScript `./browser` and `./worker` entrypoints intentionally **do not** imply a JavaScript Fetch/DOM runtime inside native Rust. Core algorithms are portable on hosted Linux, macOS and Windows. `no_std`, browser WebAssembly DOM integration and Cloudflare Workers target are not yet supported and must not be advertised. Native server adapters are explicitly typed. SDK features use caller-supplied credentials, and live cloud integration testing needs separately provisioned external accounts.

## Native Cactus inference target boundaries

Feature `cactus-native` exposes real `cactus-rs` 0.2.1 Needle3 and Whistle CPU engine types on `x86_64-unknown-linux-gnu` and `aarch64-apple-darwin`. This is **conditional native SDK equivalence**, not a claim that the JavaScript `cactus-needle` runtime can be used unmodified. The engines require valid model weights, and the host integration must handle Cactus's process-global model lifetime and thread-safety rules. Upstream native archives do not support Intel macOS or Windows MSVC; the feature compiles without native Cactus on those targets. A live on-device inference/transcription test is pending authenticated model weights and real supported hardware. The third-party binding is independent of Cactus Compute; review its licensing and static-link requirements before distribution.

## Model execution by exact API ID

`model_dispatch::execute_gutts_api_with_models` implements dispatch for all five model API metadata IDs through `GuttsModelDispatch`. Caller-supplied inference engines must do the actual work; missing engines never produce success. Whistle audio uses an explicit RFC4648 base64 native wire encoding. Model quality and actual weights still require independent live-engine verification.

### Cactus native execution adapters

With `cactus-native` on a supported host, `cactus_native::native::CactusNeedle3` implements the actual `Needle3Inference` contract and `CactusWhistle` implements `WhistleInference`. Callers must load real weights using the pinned `cactus-rs` `NeedleBuilder` and `WhistleBuilder` before constructing these adapters. Cactus has process-global lifetime semantics, unlike GutTS model residency unloading; this native adapter keeps a mutex around the real engine. Whistle native audio bytes must be signed little-endian PCM16 mono at 16 kHz and no more than 30 seconds, explicitly decoded into the Cactus `f32` sample input. Dynamic tool/schema installation is rejected with a typed error rather than silently omitted. Tests validate PCM conversion and API type correctness; full real-weight inference accuracy is not yet proven.

## Composing model runtimes

`model_host::ModelHost::new(needle, smollm, whistle, ettin, minilm)` accepts five *actual implementations* of the model-family interfaces and itself implements `model_dispatch::GuttsModelDispatch`. The fully typed public model API dispatcher can then execute native calls through the supplied engines. The included test engines are unit-test doubles, not production inferencers. The optional Cactus native adapter implements real Needle3 and Whistle. Live SmolLM2, Ettin and MiniLM backends are still caller-provided, and model-weight inference requires separate end-to-end verification.

## Actual engine verification

The Linux CI job `native-cactus-live` downloads and checksum-verifies public Cactus Needle3 and Whistle weight files, then executes an actual Needle3 inference and embedding plus Whistle 16-kHz speech transcription on the native Cactus CPU engine. It is intentionally separate from deterministic unit tests because model archives are network-fetched and CPU execution is host-specific. Results must be checked on the exact PR SHA; a workflow passing utility checks alone does **not** establish live-model verification.

## Executable complete utility registry differential

`testdata/upstream-golden/api-utility-cases.json` covers all 33 portable API registry IDs at the pinned upstream SHA. GitHub CI compiles and executes the native Rust public package exporter, separately loads and executes the unmodified pinned TypeScript functions, then compares JSON results for each ID and input. The 52 hand-curated portable cases remain as a separate negative/boundary suite. Model endpoints are verified through injected real host dispatch and the separately gated Cactus live inference job. One case per utility ID is not proof of all edge-case parity.

## Cross-language undefined representation

Pinned GutTS `tryParseJson` returns JavaScript `undefined` for malformed JSON, whereas the native Rust `Option<Value>` serializes absence to JSON `null`. The portable differential normalizes only JavaScript `undefined` to `null` to make JSON-wire comparison possible; this is a documented platform-specific exception, not full JavaScript value-domain parity.

## Real MiniLM ONNX backend

Optional feature `minilm-native` uses FastEmbed 7.7.1 to load the exact `sentence-transformers/all-MiniLM-L6-v2` model and its ONNX weights. `minilm_native::native::NativeMiniLM` implements `MiniLMInference` with an authenticated caller's own runtime environment, a synchronized session and bounded cosine-similarity output. The separate `native-minilm-live` CI job downloads weights and executes actual self-similarity inference. Success is contingent on exact-head GitHub CI; unit vector tests alone do not prove model inference.

## Ettin reranker scoring head

`ettin_head::EttinHeadWeights::from_directory` loads the exact 2_Dense / 3_LayerNorm / 4_Dense model safetensors files from an authorized local model download. `compute_ettin_head_scores` implements the pinned TypeScript GELU → LayerNorm → sigmoid scoring transform with explicit f32 intermediate rounding. CI compares output numerically with the actual pinned JavaScript `computeEttinHeadScores`. The transformer backbone supplying real `last_hidden_state` remains a separate native inference integration requirement; passing this numerical differential alone does not establish end-to-end Ettin inference.

## Exact Ettin reranker native inference

`ettin-native` uses the published `cross-encoder/ettin-reranker-{17,32,68,150}m-v1` ONNX archives with FastEmbed's real cross-encoder runtime. `NativeEttin::load(size)` resolves the exact Hugging Face repository and caches its model, tokenizer and configuration; it never falls back to a different size. `EttinInference::rerank` preserves the caller's original candidate ordering and checks result count, score finiteness and indices. A separate Linux CI job evaluates the actual 17M model. Other sizes use the same code path but need their own live-weights certification. ONNX scores are genuine native inference, not synthetic keyword matching.
