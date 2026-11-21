/*
 * Base-Matter — offline IPC contract tests for original-MSEL integration.
 * Stub Python modules verify subprocess I/O only, never MSEL language syntax.
 * Actual MSEL ABI conformance runs in private MSEL repository.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::msel_bridge::{validate_replies, MselMessageEvent};
#[cfg(target_os = "linux")]
use matter_base::msel_bridge::MselCommandRunner;
#[cfg(target_os = "linux")]
use std::path::PathBuf;

#[test]
fn message_schema_and_permissions_are_fail_closed() {
    let event = MselMessageEvent {
        provider: "whatsapp",
        chat: "group@g.us",
        sender: "test@s.whatsapp.net",
        message_id: "id_1",
        text: "#ping",
    };
    event.validate().unwrap();
    let value = serde_json::to_value(&event).unwrap();
    assert_eq!(value["text"], "#ping");
    assert_eq!(value.as_object().unwrap().len(), 5);
    assert!(validate_replies(br#"["pong"]"#, false).is_err());
    assert!(validate_replies(br#"["pong"]"#, true).is_ok());
    assert!(validate_replies(br#"["\u0000"]"#, true).is_err());
}

#[cfg(target_os = "linux")]
#[test]
fn subprocess_streams_normalized_event_without_any_credentials() {
    use std::fs;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let folder = std::env::temp_dir().join(format!("matter-msel-test-{}-{id}", std::process::id()));
    let module = folder.join("msel");
    fs::create_dir_all(&module).unwrap();
    fs::write(module.join("__init__.py"), "").unwrap();
    fs::write(
        module.join("__main__.py"),
        "import json,sys\nrequest=json.load(sys.stdin)\nassert sorted(request)==['chat','message_id','provider','sender','text']\nassert request['text']=='#ping'\nprint(json.dumps([] if '--read-only' in sys.argv else ['pong']))\n",
    )
    .unwrap();
    let script = folder.join("bot.msel");
    let native = folder.join("libmsel_embed.so");
    fs::write(&script, "fn main(){}").unwrap();
    fs::write(&native, "").unwrap();

    let runner = MselCommandRunner::new(PathBuf::from("python3"), folder.clone(), script, native);
    let event = MselMessageEvent {
        provider: "whatsapp",
        chat: "group@g.us",
        sender: "test@s.whatsapp.net",
        message_id: "id_1",
        text: "#ping",
    };
    let read_only = runner.execute(&event).unwrap();
    assert!(read_only.is_empty());
    let write_enabled = runner
        .clone()
        .allow_message_reply()
        .execute(&event)
        .unwrap();
    assert_eq!(write_enabled, vec!["pong"]);

    let mut invalid_timeout = runner;
    invalid_timeout.timeout = Duration::ZERO;
    assert!(invalid_timeout.execute(&event).is_err());
    fs::remove_dir_all(folder).unwrap();
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "Requires authenticated access to private MSEL and its compiled C++ runtime"]
fn original_msel_checkout_e2e() {
    let checkout = PathBuf::from(
        std::env::var_os("MATTER_MSEL_CHECKOUT")
            .expect("Set MATTER_MSEL_CHECKOUT to authorized private MSEL clone"),
    );
    let runner = MselCommandRunner::new(
        PathBuf::from("python3"),
        checkout.clone(),
        checkout.join("examples/bots/multiplatform.msel"),
        checkout.join("build/libmsel_embed.so"),
    )
    .allow_message_reply();
    let event = MselMessageEvent {
        provider: "whatsapp",
        chat: "test@g.us",
        sender: "test@s.whatsapp.net",
        message_id: "offline-e2e-1",
        text: "#ping",
    };
    assert_eq!(runner.execute(&event).unwrap(), vec!["pong"]);
}

// Fin: Hecho por Lournyx Matterssmith [Lou-Nyx].
