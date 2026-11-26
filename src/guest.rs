/*
 * Base-Matter — guest access policy without personal registration.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

/// The access contract is independent of Vault's optional identity enrollment.
/// It never overrides socket power, group activation, bot authorization or
/// platform-specific restrictions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuestCapability {
    Status,
    RegistrationInfo,
    Menu,
    Help,
    UiDiagnostic,
    Credits,
    BotInfo,
    AdministrativeWrite,
    VaultProfile,
    IdentityVerification,
    CodeExecution,
}

/// Read-only demo capabilities remain available without registering.
/// Commands requiring an activated chat must still pass bot admission.
pub fn can_use_without_registration(capability: GuestCapability, chat_enabled: bool) -> bool {
    match capability {
        GuestCapability::Status => true,
        GuestCapability::RegistrationInfo
        | GuestCapability::Menu
        | GuestCapability::Help
        | GuestCapability::UiDiagnostic
        | GuestCapability::Credits
        | GuestCapability::BotInfo => chat_enabled,
        GuestCapability::AdministrativeWrite
        | GuestCapability::VaultProfile
        | GuestCapability::IdentityVerification
        | GuestCapability::CodeExecution => false,
    }
}

/// Informational only: no registration, identity claim, consent receipt,
/// Vault write, or new bot instance can be created by this command.
pub fn registration_notice(start_requested: bool) -> &'static str {
    if start_requested {
        "*Registro voluntario*\n> Todavía no disponible: falta el servicio seguro de Vault-Matter.\n> No se guardó ninguna solicitud ni se pidieron datos.\n> Puedes seguir usando el modo invitado en chats habilitados."
    } else {
        "*Matterssmith · Modo invitado (prueba)*\n> No necesitas registrarte para usar las funciones básicas disponibles.\n> El chat o socket debe estar habilitado por su propietario.\n> Registro voluntario para futuras funciones personales; no pide CURP ni biometría para probar.\n> Roles y acciones sensibles requieren autorización independiente.\n> Usa `=/>register start` cuando exista el registro seguro de Vault."
    }
}
