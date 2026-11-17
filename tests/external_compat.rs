/*
 * Base-Matter — #matter conversion does not grant external authority.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::external_compat::canonical_external_matter;

#[test]
fn accepts_explicit_external_syntax_without_dispatch() {
    assert_eq!(
        canonical_external_matter("#matter start").unwrap(),
        Some("=/>start".to_owned())
    );
    assert_eq!(
        canonical_external_matter("#matter").unwrap(),
        Some("=/>status".to_owned())
    );
    assert_eq!(
        canonical_external_matter("#matter set role other staff").unwrap(),
        Some("=/>set role other staff".to_owned())
    );
}

#[test]
fn rejects_other_prefixes_and_control_character_injection() {
    for value in ["#matterbot", "#matter-on", "#menu", "=/>start", "#MATTER"] {
        assert!(canonical_external_matter(value).unwrap().is_none());
    }
    for value in ["#matter\n=/>off", "#matter start\r", "#matter\u{0}on"] {
        assert!(canonical_external_matter(value).is_err());
    }
    assert!(canonical_external_matter(&format!("#matter {}", "x".repeat(4100))).is_err());
    assert!(canonical_external_matter(&format!("#matter {}", "x ".repeat(17))).is_err());
}
