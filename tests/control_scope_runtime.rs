
/*
 * Base-Matter — live control no-argument state queries and scoped commands.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::{
    activation::SocketActivationStore,
    anti::AntiStore,
    control::{ControlDispatcher, DispatchResult},
    control_style::{ControlTheme, ControlThemes},
    menu::MenuPreferences,
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter,
};
use std::sync::Arc;

struct NoBot;
impl CommandAuthorizer for NoBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Bot-Matter unavailable")
    }
}

fn fixture() -> ControlDispatcher {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-test", "owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(NoBot)).unwrap());
    let menu = Arc::new(MenuPreferences::open(":memory:").unwrap());
    let themes = Arc::new(
        ControlThemes::new(ControlTheme::new("Base-Matter", "principal", "✦").unwrap())
            .with_dynamic_menu(menu),
    );
    ControlDispatcher::new(
        activation,
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    )
    .with_group_antis(Arc::new(AntiStore::open(":memory:").unwrap()))
    .with_public_menu(themes)
    .with_unified_live_controls()
    .with_silent_bot_commands()
}

fn call(control: &ControlDispatcher, source: &str, group: Option<&str>, message: &str) -> Result<DispatchResult> {
    control.dispatch("socket-test", "owner", source, group, message)
}

fn text(result: DispatchResult) -> String {
    match result {
        DispatchResult::Extension { output, .. } => output,
        other => panic!("expected a message, got {other:?}"),
    }
}

#[test]
fn no_argument_commands_show_current_effective_state() {
    let c = fixture();
    let dm = "5215512345678@s.whatsapp.net";
    assert!(matches!(
        call(&c, dm, None, "=/>bot").unwrap(),
        DispatchResult::Status { enabled: false, .. }
    ));
    assert!(text(call(&c, dm, None, "=/>anti").unwrap()).contains("link: off"));
    assert!(text(call(&c, dm, None, "=/>set").unwrap()).contains("Estilo: classic"));
    assert!(text(call(&c, dm, None, "=/>set style").unwrap()).contains("Estilo: classic"));
    assert!(text(call(&c, dm, None, "=/>set lang").unwrap()).contains("Idioma: es"));
    assert!(text(call(&c, dm, None, "=/>set anti").unwrap()).contains("status: off"));
    assert!(text(call(&c, dm, None, "=/>set anti link").unwrap()).contains("(lista vacía)"));

    let group = "120363555@g.us";
    assert!(matches!(
        call(&c, group, Some(group), "=/>bot").unwrap(),
        DispatchResult::GroupStatus { enabled: false, .. }
    ));
    assert!(text(call(&c, group, Some(group), "=/>anti").unwrap()).contains("chat actual"));
    assert!(text(call(&c, group, Some(group), "=/>anti link").unwrap()).contains("link: off"));
}

#[test]
fn socket_scope_never_mutates_global_defaults() {
    let c = fixture();
    let dm = "5215512345678@s.whatsapp.net";
    let group = "120363555@g.us";
    assert!(matches!(
        call(&c, dm, None, "=/>status --mod").unwrap(),
        DispatchResult::BotGlobalStatus { .. }
    ));
    assert!(text(call(&c, dm, None, "=/>status --global").unwrap()).contains("Predeterminado general"));
    assert!(text(call(&c, dm, None, "=/>bot --global").unwrap()).contains("Predeterminado general"));
    assert!(call(&c, dm, None, "=/>bot --global on").is_err());
    call(&c, dm, None, "=/>bot --mod on").unwrap();
    assert!(matches!(
        call(&c, dm, None, "=/>bot --mod").unwrap(),
        DispatchResult::BotGlobalStatus { default_policy: Some(true), .. }
    ));
    call(&c, group, Some(group), "=/>set anti --mod link on").unwrap();
    assert!(text(call(&c, group, Some(group), "=/>anti link").unwrap()).contains("link: on"));
    assert!(text(call(&c, group, Some(group), "=/>anti --mod link").unwrap()).contains("link: on"));
    assert!(text(call(&c, group, Some(group), "=/>anti --global link").unwrap()).contains("link: off"));
    assert!(call(&c, group, Some(group), "=/>set anti --global link on").is_err());
    assert!(call(&c, group, Some(group), "=/>anti link on").is_err());
}

#[test]
fn preferences_are_queried_and_mutated_only_by_authenticated_owner() {
    let c = fixture();
    let dm = "5215512345678@s.whatsapp.net";
    call(&c, dm, None, "=/>set lang en").unwrap();
    assert!(text(call(&c, dm, None, "=/>set --mod").unwrap()).contains("Idioma: en"));
    assert!(text(call(&c, dm, None, "=/>set --global").unwrap()).contains("idioma es"));
    assert!(text(call(&c, dm, None, "=/>set style --mod").unwrap()).contains("Estilo: classic"));
    assert!(text(call(&c, dm, None, "=/>set --mod style none").unwrap()).contains("none"));
    assert!(text(call(&c, dm, None, "=/>set style").unwrap()).contains("Estilo: none"));
    assert!(c
        .dispatch("socket-test", "stranger", dm, None, "=/>set")
        .is_err());
}
