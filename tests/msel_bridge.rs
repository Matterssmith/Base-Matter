/*
 * Base-Matter — offline IPC contract tests for original-MSEL integration.
 * Stub Python modules verify subprocess I/O only, never MSEL language syntax.
 * Actual MSEL ABI conformance runs in private MSEL repository.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::msel_bridge::{validate_replies, MselCommandRunner, MselMessageEvent};
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

    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
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

    let runner = MselCommandRunner::new(
        PathBuf::from("python3"),
        folder.clone(),
        script,
        native,
    );
    let event = MselMessageEvent {
        provider: "whatsapp",
        chat: "group@g.us",
        sender: "test@s.whatsapp.net",
        message_id: "id_1",
        text: "#ping",
    };
    let read_only = runner.execute(&event).unwrap();
    assert!(read_only.is_empty());
    let write_enabled = runner.clone().allow_message_reply().execute(&event).unwrap();
    assert_eq!(write_enabled, vec!["pong"]);

    let invalid_timeout = MselCommandRunner {
        timeout: Duration::ZERO,
        ..runner
    };
    assert!(invalid_timeout.execute(&event).is_err());
    fs::remove_dir_all(folder).unwrap();
}

// Fin: Hecho por Lournyx Matterssmith [Lou-Nyx].
