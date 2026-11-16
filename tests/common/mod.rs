/*
 * Matter-Base — Shared Integration Test Authorization
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::{LegalAction, LegalAuthorizationRequest, LegalAuthorizer, LegalPrincipal};

struct TestLegalAuthorizer;

impl LegalAuthorizer for TestLegalAuthorizer {
    fn authorize(&self, request: &LegalAuthorizationRequest<'_>) -> anyhow::Result<()> {
        match request.action {
            LegalAction::RegisterDocument
            | LegalAction::SetDocumentActive
            | LegalAction::ReadRequiredDocuments => {
                if request.principal_id == "test-legal-admin" {
                    Ok(())
                } else {
                    anyhow::bail!("test legal administration denied")
                }
            }
            LegalAction::RegisterUser
            | LegalAction::ReadUser
            | LegalAction::RecordDecision
            | LegalAction::ReadDecision
            | LegalAction::CheckAccess => {
                if request.user_id == Some(request.principal_id) {
                    Ok(())
                } else {
                    anyhow::bail!("test legal user authorization denied")
                }
            }
        }
    }
}

static TEST_LEGAL_AUTHORIZER: TestLegalAuthorizer = TestLegalAuthorizer;

pub fn legal_admin() -> LegalPrincipal<'static> {
    LegalPrincipal::new("test-legal-admin", &TEST_LEGAL_AUTHORIZER).unwrap()
}

pub fn legal_user(user_id: &str) -> LegalPrincipal<'_> {
    LegalPrincipal::new(user_id, &TEST_LEGAL_AUTHORIZER).unwrap()
}
