# Base-Matter — bot policy and command surfaces (alpha trial)

**Status:** owner/group offline implementation in alpha; not yet qualified
for third-party WhatsApp private chats, channels, Telegram or Discord.
This is Base-Matter control infrastructure. Bot-Matter is still blocked.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Independent dimensions

1. **Socket power** is operational. Only its authenticated session owner
   can use `=/>on` and `=/>off` in their private self-chat. This does
   not automatically enable bot messages.
2. **Chat bot policy** is a Boolean decision with inheritance. If the
   socket is off, the result is off regardless of configuration.
3. **Bot binding/runtime** is independent. A policy can be enabled even
   when no separate bot executable is registered; a policy status must
   never be mistaken for proof that Bot-Matter is running.
4. **Role authorization** belongs exclusively to Vault-Matter. No local
   role grants, and no offline fallback. A power setting does not grant
   a Vault rank.

After the power gate, effective bot policy uses this precedence:

```text
Exact chat override (verified group JID)
       ↓ otherwise
Chat-kind default (group / private / channel)
       ↓ otherwise
Socket-wide default (--global)
       ↓ otherwise
Trusted global fallback, if provisioned
       ↓ otherwise
Off (deny)
```

The current alpha implementation represents chat-kind defaults as
internal, reserved `kind:group`, `kind:private`, `kind:channel`
keys inside the existing audited activation store. These keys are
never accepted from a chat as authenticated JIDs. Group overrides
remain scoped to verified `@g.us` identities.

## Implemented reserved commands in alpha

| Command | Effect |
| --- | --- |
| `=/>status`, `=/>estado` | Chat-specific effective bot policy; responds even when off |
| `=/>status --global` | Raw socket power and kind/default policies, independently of socket power |
| `=/>on`, `=/>off` | Power on/off socket, self-chat owner only |
| `=/>bot` | Bot policy usage when that chat is enabled |
| `=/>bot on/off/inherit` | Exact group override or private-kind setting in self-chat |
| `=/>bot --global on/off/inherit` | Socket-wide default, owner only |
| `=/>bot group/private/channel on/off/inherit` | Defaults for that kind, owner in self-chat |
| `=/>bot grupo/privado/canal encender/apagar/heredar` | Explicit Spanish aliases for kinds and actions |
| `=/>set` | Restricted configuration guide, self-chat owner |
| `=/>set role` | Information only; actual rank changes fail closed until Vault |
| `=/>menu`, `=/>menú` | Category menu when policy is enabled; uses Matterssmith default |
| `=/>menu <category>` | Context-independent category selection, policy-gated |
| `--debug` last argument | Owner-only redacted diagnostic after an authorized command |

`=/>help` and `=/>h` are retired and no longer provided as menu
aliases. Socket-wide `roles` and `audit` remain owner-private read-only
control functions. Legacy `=/>group status/on/off/inherit` is
temporarily retained for migration; it does **not** power on the socket.
The existing REGISTER and recovery commands are still bounded to
previously defined preview/read-only contracts.

### Silent off

When the bot policy is disabled for a chat, its ordinary menu and
nonessential commands **do not reply**. `=/>status` still reports the
current effective decision, including the separate socket power gate.
Authenticated owner power/policy configuration remains possible so an
off state can be recovered without opening a backdoor to Bot-Matter.

At present the Rust WhatsApp HMAC bridge admits **only events verified
as authored by the linked socket owner**. The `menu` and `status`
protocol supports private self-chat and groups. Third-party private
chat and channel identity assertions must be added as a new
authenticated ingress contract before asserting that all people and
chat kinds can access these commands. Do not forge `from_me` or
extend the v1 owner signing identity to arbitrary actors.

### Debug

`--debug` is an opt-in final flag that does not change the command,
scope or actor permission. Debug summaries show mode, authenticated
origin, policy source and redacted Vault state. They must never show
pairing codes, client JIDs, group identifiers, private credentials,
keys, message bodies or sensitive user records. A denied command
does not become authorized because `--debug` was supplied.

## Styles and automatic imports — not yet enabled

The separate default Matterssmith theme is a Unicode heading, category
tree and credits. It may be used in every admitted chat; categories
identify their implemented commands and pending integrations.
`=/>set style` and `=/>set theme` should provide authenticated,
versioned, scoped style persistence. Manual preview should come before
publishing a style. Automatic adaptation to another bot's `#menu`
must require a trusted, authorized message source, an explicit
preview, approved attribution and a size/content sanitizer. It
must not import commands, identities, third-party links, privileges
or any credentials from that menu. An AI-backed `--auto` is a
future optional provider, never a silent fallback and never capable
of bypassing authorization. It is **not implemented** by this change.

## `mod`, `self`, `selfmod` and `--global`

A *scope* is where a configuration applies, and an *authorization
requirement* is who may execute it. These are separate dimensions:

- `--global`: select the current socket's fallback across its
  connected chats. It is **not** an administrative rank.
- `mod`: require independently proven group moderation and a fresh
  Vault-Matter capability for that operation.
- `self`: permit only the verified socket owner, and later explicitly
  authorized Vault-Matter delegates for that socket.
- `selfmod`: require **both** the above socket capability and
  proven group moderator status; it is not an alias for `--global`.
  In a non-group context this check fails closed.

The exact grammar and authorized handlers for `self`, `selfmod`
and other historical commands must be integrated with a *real*
Vault decision verifier and the platform's verified admin metadata.
They are **not live actions** in this checkpoint.

## Implementación posterior: lectura pública limitada (alpha, prueba real pendiente)

El controlador puede ahora firmar y responder **únicamente**
`status/estado`, `menu/menú [categoría]` y `bot` sin
argumentos desde otros participantes. Esta ruta requiere
evento de WhatsApp autenticado, remitente numérico
`@lid/@s.whatsapp.net`, JID grupal real `@g.us` o
chat privado cuyo JID sea el del remitente, HMAC válido,
marca temporal reciente y rechazo de replay. El socket
emparejado conserva su identidad PN/LID verificada.

Solo consultas sin efectos: no se ejecutan handlers del bot,
extensiones arbitrarias, cambios de activación, rangos ni
escritura a Vault. La consulta `status` responde cuando
está apagado y `menu/bot` es silencioso. Las identidades de
canales y vinculación PN/LID cruzada para terceros siguen
pendientes. `private` aplica únicamente a privados
externos; `self` identifica el chat propio del propietario.

La prueba de unidad confirma el contrato de código, **no**
una prueba real de entrega para usuarios externos. Ejecutar
el binario en Debian y probar contra una cuenta ajena antes
de declarar listo este ingreso.

## Mandatory verification and promotion

Use the owner's **single cloned checkout** and shared Cargo target.
Never reset private sessions or keys to test policies. Run Cargo
format/check/test on the root and standalone WhatsApp provider.
Perform real tests of self-chat, two distinct groups, silent-off,
raw global defaults, replay denial, forged sender denial and
socket-owner refusal. Build the future channel/third-party ingress
separately with an authenticated new protocol and verify it before
promoting to beta. Promote independently validated changes by PR
to beta, then to stable, without bulk-merging incomplete work.
