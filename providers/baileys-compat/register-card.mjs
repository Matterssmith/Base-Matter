/*
 * Base-Matter — optional Baileys transport for Matter Register.
 * This transport does NOT authenticate a user or create a Vault account.
 * It must never embed PII, JWTs, session identifiers or raw documents in URLs.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
function validatedRegistrationUrl(input, allowedHosts = []) {
  if (!Array.isArray(allowedHosts) || allowedHosts.length === 0) throw new TypeError("required_host_allowlist");
  if (typeof input !== "string" || input.length > 2048) throw new TypeError("invalid_url");
  const url = new URL(input);
  if (url.protocol !== "https:" || !url.hostname || url.username || url.password || url.hash || url.search) {
    throw new TypeError("unsafe_registration_url");
  }
  if (allowedHosts.length > 0 && !allowedHosts.includes(url.hostname)) throw new TypeError("unapproved_host");
  return url.toString();
}

export function createMatterRegisterCard(httpsUrl, options = {}) {
  const url = validatedRegistrationUrl(httpsUrl, options.allowedHosts ?? []);
  const summary = "Registra tu cuenta Matter o continúa como invitado. La identidad y los documentos solo se verifican desde el portal seguro.";
  const urlButton = options.preferInAppWebview === true
    ? { name: "open_webview", buttonParamsJson: JSON.stringify({ title: "Abrir Matter Register", link: { in_app_webview: true, url } }) }
    : { name: "cta_url", buttonParamsJson: JSON.stringify({ display_text: "Abrir registro", url }) };
  return {
    viewOnceMessage: {
      message: {
        messageContextInfo: {
          deviceListMetadata: {},
          deviceListMetadataVersion: 2
        },
        interactiveMessage: {
          header: { title: "✦ Matter Register", hasMediaAttachment: false },
          body: { text: summary },
          footer: { text: "Matterssmith · Registro voluntario" },
          nativeFlowMessage: {
            buttons: [
              urlButton,
              { name: "quick_reply", buttonParamsJson: JSON.stringify({ display_text: "Soy invitado", id: "matter.register.guest" }) }
            ],
            messageParamsJson: ""
          }
        }
      }
    }
  };
}

/**
 * Dependency-injected Baileys adapter.
 * The caller must provide the authorized socket, JID, and compatible
 * generateWAMessageFromContent from its pinned Baileys version.
 * WhatsApp native-flow rendering is client/version dependent.
 */
export async function sendMatterRegisterCard({ socket, jid, httpsUrl, generateWAMessageFromContent, allowedHosts = [], preferInAppWebview = false }) {
  if (!socket || typeof socket.relayMessage !== "function" || typeof socket.sendMessage !== "function") {
    throw new TypeError("invalid_socket");
  }
  if (typeof jid !== "string" || !/^[0-9]+@(s\.whatsapp\.net|g\.us)$/.test(jid)) {
    throw new TypeError("invalid_jid");
  }
  if (typeof generateWAMessageFromContent !== "function") {
    throw new TypeError("missing_message_builder");
  }
  const url = validatedRegistrationUrl(httpsUrl, allowedHosts);
  try {
    const card = createMatterRegisterCard(url, { allowedHosts, preferInAppWebview });
    const generated = generateWAMessageFromContent(jid, card, {});
    if (!generated?.message || !generated?.key?.id) throw new TypeError("invalid_generated_message");
    await socket.relayMessage(jid, generated.message, { messageId: generated.key.id });
    return { transport: "native-flow", delivered: true, rendered: "unconfirmed" };
  } catch (error) {
    // Never include error strings or request payloads in WhatsApp messages.
    await socket.sendMessage(jid, {
      text: "✦ Matter Register\nPortal de inscripción: " + url + "\nO continúa como invitado con =/>register guest."
    });
    return { transport: "text-fallback", delivered: true, rendered: "text" };
  }
}

// Responses such as "matter.register.guest" are UNTRUSTED input until the
// adapter validates the actual sender, chat, socket, and unique message ID.
