// Base-Matter — Matter AI programming assistant through Vault's broker.
// Never receives plaintext provider keys. Never executes, writes or grants roles.
use anyhow::{bail, Result};
use crate::vault_ai;

pub fn command(socket_id: &str, actor_id: &str, args: &[String]) -> Result<String> {
    if args.is_empty() || args == ["help"] {
        return Ok(
            "Matter AI: asistente de programación. Comandos: matter status,              matter ask <pregunta>, matter plan <tarea>.              El proveedor lo selecciona Vault-Matter (Claude, Venice,              OpenAI, Alya u otros autorizados). Sin ejecución ni cambios automáticos."
                .to_owned(),
        );
    }
    if args == ["status"] {
        return Ok(if vault_ai::configured() {
            "Matter AI: broker Vault configurado. Estado remoto no verificado.              Las credenciales permanecen en Vault; solo se autoriza por operación."
                .to_owned()
        } else {
            "Matter AI: broker Vault no configurado.              No se cargan claves de API desde Base-Matter."
                .to_owned()
        });
    }
    let [operation, parameters @ ..] = args else {
        bail!("Uso: matter [status|ask <pregunta>|plan <tarea>]");
    };
    if !matches!(operation.as_str(), "ask" | "plan") || parameters.is_empty() {
        bail!("Uso: matter [status|ask <pregunta>|plan <tarea>]");
    }
    let question = parameters.join(" ");
    if question.trim().is_empty() || question.len() > 2000 || question.contains('\0') {
        bail!("Matter AI question rejected");
    }
    let prompt = if operation == "plan" {
        format!(
            "Produce un plan y pruebas de programación. No ejecutes ni declares cambios realizados. {}",
            question
        )
    } else {
        question
    };
    let (provider, answer) =
        vault_ai::ask("coding-assistant", socket_id, actor_id, &prompt)?;
    Ok(format!("Matter [{provider}] — asesoría, sin ejecución:\n{answer}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_credentials_loaded_in_base() {
        let help = command("socket-1", "owner-1", &[]).unwrap();
        assert!(help.contains("Vault-Matter"));
        let status = command("socket-1", "owner-1", &["status".to_owned()]).unwrap();
        assert!(status.contains("Vault"));
        assert!(command(
            "socket-1",
            "owner-1",
            &["console".to_owned(), "ls".to_owned()]
        ).is_err());
    }
}
