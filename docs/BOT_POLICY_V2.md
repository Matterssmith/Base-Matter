# Base-Matter — bot policy and command surfaces (alpha trial)

**Status:** owner/group offline implementation in alpha; not yet qualified
for third-party WhatsApp private chats, channels, Telegram or Discord.
This is Base-Matter control infrastructure. Bot-Matter is still blocked.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Independent dimensions

1. **Socket power** remains an internal operational gate, no longer a
   separate WhatsApp command. `=/>bot on` and scoped variants wake it
   atomically when needed. `=/>bot off` disables only its selected
   chat/scope and never switches off other chats.
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
Chat-kind default (group / external private / channel / community notices)
       ↓ otherwise
Socket-wide default (--global)
       ↓ otherwise
Trusted global fallback, if provisioned
       ↓ otherwise
Off (deny)
```

The current alpha implementation represents chat-kind defaults as
internal, reserved `kind:group`, `kind:private`, `kind:channel`, `kind:community`
keys inside the existing audited activation store. These keys are
never accepted from a chat as authenticated JIDs. Group overrides
remain scoped to verified `@g.us` identities.

## Current signed WhatsApp commands in alpha

| Command | Effect |
| --- | --- |
| `=/>status`, `=/>estado` | Effective state of current chat, always readable |
| `=/>status --global` | Owner-only socket defaults and configured chat types |
| `=/>bot on/off` | Enable/disable current authorized chat; power gate wakes atomically |
| `=/>bot inherit` | Clear current chat exception and inherit its type |
| `=/>bot --global on/off/inherit` | Owner-only socket fallback |
| `=/>bot group/private/channel/community on/off/inherit` | Owner-only type defaults from self-chat |
| `=/>bot comunidad/avisos encender/apagar/heredar` | Explicit Spanish aliases for announcement-group policy |
| `=/>bot` | Guide to available policy scopes when chat is active |
| `=/>menu`, `=/>menú` | Single compact global control menu, no categories |
| `=/>set` | Owner-only configuration guidance |
| `=/>set role` | Information only, actual grants denied without Vault |
| `--debug` final modifier | Redacted owner-only technical diagnostics |

**Removed from real WhatsApp ingress:** `=/>on`, `=/>off`, `=/>help`,
`=/>h`, and `=/>menu <category>`. The core retains legacy
in-process power APIs for compatibility tests; the verified WhatsApp
provider/bridge reject obsolete chat commands. In particular,
`=/>group on/off` is not another activation switch in live control.
Use `=/>bot on/off` inside the group instead.

**Scope safety:** exact group overrides > verified kind defaults >
socket fallback where applicable. External groups and private chats
still **default off** even if the socket fallback is on. `community`
stores a separate policy for *announcement groups* but does not yet
recognize WhatsApp announcement metadata: it must never activate
unverified ordinary groups. Likewise channel settings require
authenticated channel identity support before live application.

**Styling:** all IPC replies use the same renderer from
`src/control_style.rs`, including status, role refusals, audit and
the compact menu. The external default files are
`templates/matterssmith-control.txt` (frame and credits) and
`templates/matterssmith-menu.txt` (single command list).
The separate Rust provider reuses the **same** renderer for its
direct REGISTER and community recovery previews. The host may
select a bounded, validated replacement template per community;
this is currently in-memory configuration, not yet an operator
command or Vault-backed persistent setting.

### Silent off

When the bot policy is disabled for a chat, its ordinary menu and
nonessential commands **do not reply**. `=/>status` still reports the
current effective decision, including the separate socket power gate.
Authenticated owner power/policy configuration remains possible so an
off state can be recovered without opening a backdoor to Bot-Matter.

The HMAC bridge has separate owner-control and read-only ingress
paths. Verified third-party group/private messages can query
`status`, `menu` and `bot` help only, never change bot power or
Vault roles. Channel and community-type detection remain unverified
and must not be inferred from plain group JIDs.

### Debug

`--debug` is an opt-in final flag that does not change the command,
scope or actor permission. Debug summaries show mode, authenticated
origin, policy source and redacted Vault state. They must never show
pairing codes, client JIDs, group identifiers, private credentials,
keys, message bodies or sensitive user records. A denied command
does not become authorized because `--debug` was supplied.

## Styles and automatic imports — not yet enabled

The default Matterssmith theme is a Unicode heading, short command
list and credits. No categories. Both the control frame and menu
are separate editable files, not embedded in bot handlers.
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

## Default-deny de ámbitos externos

La herencia no debe habilitar superficies externas accidentalmente:
`group`, `private` externo y `channel` **deniegan** si no
existe una asignación explícita de tipo o grupo. La política
`--global on` NO las convierte en encendidas por omisión.
Se requiere `=/>bot group on`, `=/>bot private on`,
`=/>bot channel on` o excepción puntual firmada y aprobada.
El estado `inherit` elimina la excepción y vuelve al tipo;
si ese tipo no está habilitado, permanece apagado.

El chat propio del propietario usa el ámbito especial
`kind:self`, que puede heredar la política base y jamás
es gobernado por `kind:private`. Esto preserva los
comandos de recuperación de socket. La política de encendido
no autoriza ninguna operación administrativa de Vault.

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
