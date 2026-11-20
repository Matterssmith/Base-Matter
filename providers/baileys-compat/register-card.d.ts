/**
 * Base-Matter optional Baileys transport contract.
 * No identity proof and no Vault write occurs in this module.
 * Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
export type RegisterTransport = "native-flow" | "text-fallback";
export type ButtonRenderStatus = "unconfirmed" | "text";

export interface RegisterCardOptions {
  allowedHosts: string[];
  /** Experimental in-app WhatsApp WebView; verify device support. */
  preferInAppWebview?: boolean;
}
export interface BaileysLikeSocket {
  relayMessage(jid: string, message: unknown, options: { messageId: string }): Promise<unknown>;
  sendMessage(jid: string, content: { text: string }): Promise<unknown>;
}
export interface RegisterCardSendOptions extends RegisterCardOptions {
  socket: BaileysLikeSocket;
  jid: string;
  httpsUrl: string;
  generateWAMessageFromContent(jid: string, card: unknown, options: object): {
    message?: unknown; key?: { id?: string };
  };
}
export declare function createMatterRegisterCard(
  httpsUrl: string,
  options: RegisterCardOptions
): object;
export declare function sendMatterRegisterCard(
  options: RegisterCardSendOptions
): Promise<{ transport: RegisterTransport; delivered: true; rendered: ButtonRenderStatus }>;
