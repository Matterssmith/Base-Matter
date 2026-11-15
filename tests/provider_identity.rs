
/*
 * Matter-Base — Provider Identity Tests
 *
 * Verifica identidad, autorización y permisos de proveedores.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::security::{
    ProviderAccessPolicy,
    ProviderIdentity,
    ProviderPermission,
};
use std::collections::HashSet;

fn permissions() -> HashSet<ProviderPermission> {
    [
        ProviderPermission::ReadKeyIds,
        ProviderPermission::ReadActiveKey,
        ProviderPermission::ReadKeyMaterial,
    ]
    .into_iter()
    .collect()
}

fn identity() -> ProviderIdentity {
    ProviderIdentity::from_verified_assertion(
        "matter-base-production".to_owned(),
        "matterssmith-primary-vault".to_owned(),
        permissions(),
    )
    .unwrap()
}

fn policy() -> ProviderAccessPolicy {
    ProviderAccessPolicy::new(
        "matterssmith-primary-vault".to_owned(),
        ["matter-base-production".to_owned()]
            .into_iter()
            .collect(),
    )
    .unwrap()
}

#[test]
fn creates_valid_provider_identity() {
    let provider = identity();

    assert_eq!(provider.provider_id(), "matter-base-production");
    assert_eq!(provider.vault_id(), "matterssmith-primary-vault");
}

#[test]
fn authorizes_registered_provider() {
    assert!(policy().authorize_provider(&identity()).is_ok());
}

#[test]
fn rejects_unknown_provider() {
    let provider = ProviderIdentity::from_verified_assertion(
        "unknown-provider".to_owned(),
        "matterssmith-primary-vault".to_owned(),
        permissions(),
    )
    .unwrap();

    assert!(policy().authorize_provider(&provider).is_err());
}

#[test]
fn rejects_different_vault() {
    let provider = ProviderIdentity::from_verified_assertion(
        "matter-base-production".to_owned(),
        "another-vault".to_owned(),
        permissions(),
    )
    .unwrap();

    assert!(policy().authorize_provider(&provider).is_err());
}

#[test]
fn authorizes_permitted_operation() {
    assert!(
        policy()
            .authorize(
                &identity(),
                ProviderPermission::ReadKeyMaterial
            )
            .is_ok()
    );
}

#[test]
fn rejects_missing_permission() {
    let provider = ProviderIdentity::from_verified_assertion(
        "matter-base-production".to_owned(),
        "matterssmith-primary-vault".to_owned(),
        [ProviderPermission::ReadKeyIds]
            .into_iter()
            .collect(),
    )
    .unwrap();

    assert!(
        policy()
            .authorize(
                &provider,
                ProviderPermission::ReadKeyMaterial
            )
            .is_err()
    );
}

#[test]
fn rejects_empty_provider_id() {
    let result = ProviderIdentity::from_verified_assertion(
        String::new(),
        "matterssmith-primary-vault".to_owned(),
        permissions(),
    );

    assert!(result.is_err());
}

#[test]
fn rejects_empty_vault_id() {
    let result = ProviderIdentity::from_verified_assertion(
        "matter-base-production".to_owned(),
        String::new(),
        permissions(),
    );

    assert!(result.is_err());
}

#[test]
fn rejects_identity_without_permissions() {
    let result = ProviderIdentity::from_verified_assertion(
        "matter-base-production".to_owned(),
        "matterssmith-primary-vault".to_owned(),
        HashSet::new(),
    );

    assert!(result.is_err());
}

#[test]
fn rejects_policy_without_authorized_providers() {
    let result = ProviderAccessPolicy::new(
        "matterssmith-primary-vault".to_owned(),
        HashSet::new(),
    );

    assert!(result.is_err());
}
