/*
 * Base-Matter — parser regression: syntax never causes group mutation.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::add_commands::{parse_add_text, AddIntent, Destination, Resource, User};

#[test]
fn parses_user_and_reply_targets() {
    assert_eq!(
        parse_add_text("=/>add user +5215512345678").unwrap(),
        AddIntent::User { user: User::Number("5215512345678".into()), destination: None }
    );
    assert_eq!(
        parse_add_text("=/>add user @sakura group official --register").unwrap(),
        AddIntent::User {
            user: User::Username("sakura".into()),
            destination: Some((Destination::Group, Resource::Official(Some("register".into()))))
        }
    );
    assert_eq!(
        parse_add_text("=/>add user community 120363000123456@g.us").unwrap(),
        AddIntent::User {
            user: User::Reply,
            destination: Some((
                Destination::Community,
                Resource::Id("120363000123456@g.us".into())
            ))
        }
    );
    assert_eq!(
        parse_add_text("=/>add user reply").unwrap(),
        AddIntent::User { user: User::Reply, destination: None }
    );
}

#[test]
fn parses_group_and_backup_targets() {
    assert_eq!(
        parse_add_text("=/>add group --hide community official").unwrap(),
        AddIntent::Group { community: Resource::Official(None), hide: true }
    );
    assert_eq!(
        parse_add_text("=/>add group community 123456@g.us --hide").unwrap(),
        AddIntent::Group {
            community: Resource::Id("123456@g.us".into()),
            hide: true
        }
    );
    assert_eq!(
        parse_add_text("=/>add community backup official to new --hide").unwrap(),
        AddIntent::Backup {
            source: Resource::Official(None),
            destination: Resource::New,
            hide: true
        }
    );
    assert_eq!(
        parse_add_text("=/>add community backup 123456@g.us to official --register").unwrap(),
        AddIntent::Backup {
            source: Resource::Id("123456@g.us".into()),
            destination: Resource::Official(Some("register".into())),
            hide: false
        }
    );
}

#[test]
fn parses_new_community_with_multitoken_name() {
    assert_eq!(
        parse_add_text("=/>add community new --Mi Comunidad Matter").unwrap(),
        AddIntent::Community {
            community: Resource::New, name: Some("Mi Comunidad Matter".into())
        }
    );
    assert_eq!(
        parse_add_text("=/>add community official").unwrap(),
        AddIntent::Community {
            community: Resource::Official(None), name: None
        }
    );
}

#[test]
fn rejects_invalid_syntax_and_injection() {
    for input in [
        "=/>add",
        "=/>add user +abc",
        "=/>add user @bad:name",
        "=/>add user 123",
        "=/>add user +5215512345678 group",
        "=/>add group",
        "=/>add group community new",
        "=/>add group community official --hide --hide",
        "=/>add community new",
        "=/>add community new --hide",
        "=/>add community backup official to",
        "=/>add community backup new to official",
        "=/>add community backup official to new --hide --hide",
        "=/>add user 5215512345678\n=/>off",
        "=/>add user 5215512345678\0",
        "=/>add user 5215512345678 group ../unsafe",
    ] {
        assert!(parse_add_text(input).is_err(), "Expected failure: {input:?}");
    }
}
