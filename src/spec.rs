//! Immutable Fanwaave identity and provider-operation contract.

use ore_mcp_org_server::OrgSpec;

const DEPENDENCIES: &[&str] = &[
    "ORESoftware/mcp-rust-libs",
    "ORESoftware/ores-interfaces",
    "ORESoftware/api-docs",
    "ORESoftware/typespec-json-schema-validator",
    "ores-otel/ores-mcp-server-core-libs.rs",
    "shared-auth/shared-auth-clients",
    "shared-auth/shared-auth-interfaces",
    "shared-auth/shared-auth-lib",
    "zed-pkg/zed-cli",
];

/// Operations inherited from the pinned shared provider implementation.
pub const PROVIDER_OPERATIONS: &[(&str, &[&str])] = &[
    ("github", &["read_organization", "read_latest_workflow_run"]),
    ("aws", &["read_caller_identity", "read_eks_clusters"]),
    ("gcp", &["read_project", "read_enabled_services"]),
    ("supabase", &["read_auth_settings", "read_data_api_schema"]),
    ("neon", &["read_projects", "read_project_branches"]),
    ("cloudflare", &["read_zone", "read_dns_records"]),
    ("k8s_cluster", &["read_deployments", "read_pods"]),
    (
        "nats",
        &["read_service_snapshot", "read_dependency_snapshot"],
    ),
];

/// Returns the exact organization, repository, service, and Zed dependency identity.
#[must_use]
pub fn org_spec() -> OrgSpec {
    debug_assert_eq!(PROVIDER_OPERATIONS.len(), 8);
    OrgSpec {
        organization: "fanwaave",
        repository: "fanwaave/fanwaave-mcp-server.rs",
        service_name: "fanwaave-mcp-server",
        package_name: "fanwaave-mcp-server",
        dependencies: DEPENDENCIES,
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_and_dependency_catalogs_have_no_ambiguous_entries() {
        use std::collections::BTreeSet;

        let mut dependency_set = BTreeSet::new();
        for dependency in DEPENDENCIES {
            assert!(dependency_set.insert(*dependency), "duplicate dependency");
            assert_eq!(dependency.split('/').count(), 2, "unqualified dependency");
        }

        let mut provider_set = BTreeSet::new();
        let mut operation_set = BTreeSet::new();
        for (provider, operations) in PROVIDER_OPERATIONS {
            assert!(provider_set.insert(*provider), "duplicate provider");
            assert!(!provider.is_empty() && !provider.contains('*'));
            for operation in *operations {
                assert!(
                    operation_set.insert((*provider, *operation)),
                    "duplicate provider operation"
                );
                assert!(operation.starts_with("read_"), "write operation in read-only contract");
                assert!(!operation.contains('*'), "wildcard operation");
            }
        }
        assert_eq!(provider_set.len(), 8);
        assert_eq!(operation_set.len(), 16);
    }

    #[test]
    fn scope_and_provider_contract_are_exact() {
        let spec = org_spec();
        assert_eq!(spec.organization, "fanwaave");
        assert_eq!(spec.repository, "fanwaave/fanwaave-mcp-server.rs");
        assert_eq!(PROVIDER_OPERATIONS.len(), 8);
        assert!(DEPENDENCIES.contains(&"ORESoftware/ores-interfaces"));
        assert!(DEPENDENCIES.contains(&"ORESoftware/api-docs"));
        assert!(DEPENDENCIES.contains(&"ORESoftware/typespec-json-schema-validator"));
        assert!(PROVIDER_OPERATIONS.iter().all(|(provider, operations)| {
            !provider.contains('*')
                && operations.len() == 2
                && operations
                    .iter()
                    .all(|operation| operation.starts_with("read_"))
        }));
    }
}
