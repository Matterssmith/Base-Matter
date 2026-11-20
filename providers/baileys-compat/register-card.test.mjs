/*
 * Base-Matter — Baileys card unit tests (no WhatsApp network).
 * Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
import test from "node:test";
import assert from "node:assert/strict";
import { createMatterRegisterCard, sendMatterRegisterCard } from "./register-card.mjs";

test("URL button only accepts safe public HTTPS destinations", () => {
  assert.throws(() => createMatterRegisterCard("http://example.com/register"));
  assert.throws(() => createMatterRegisterCard("https://user:pass@example.com/"));
  assert.throws(() => createMatterRegisterCard("https://example.com/register?token=abc"));
  assert.throws(() => createMatterRegisterCard("https://example.com/register#secret"));
  assert.throws(() => createMatterRegisterCard("https://example.com/", { allowedHosts: ["safe.example"] }));
  const card = createMatterRegisterCard("https://matter.example/register", { allowedHosts: ["matter.example"] });
  const btns = card.viewOnceMessage.message.interactiveMessage.nativeFlowMessage.buttons;
  assert.equal(btns[0].name, "cta_url");
  assert.equal(JSON.parse(btns[0].buttonParamsJson).url, "https://matter.example/register");
  assert.equal(btns[1].name, "quick_reply");
  const webview = createMatterRegisterCard("https://matter.example/register", { allowedHosts: ["matter.example"], preferInAppWebview: true });
  const webBtn = webview.viewOnceMessage.message.interactiveMessage.nativeFlowMessage.buttons[0];
  assert.equal(webBtn.name, "open_webview");
  assert.equal(JSON.parse(webBtn.buttonParamsJson).link.in_app_webview, true);
});

test("native flow uses verified sender and falls back to safe text", async () => {
  const sent = [];
  const socket = {
    async relayMessage(...args) { sent.push(["native", ...args]); },
    async sendMessage(...args) { sent.push(["text", ...args]); }
  };
  const generateWAMessageFromContent = () => ({ key: { id: "synthetic-id" }, message: { interactiveMessage: {} } });
  const input = {
    socket, jid: "5215550000000@s.whatsapp.net",
    httpsUrl: "https://matter.example/register",
    allowedHosts: ["matter.example"],
    generateWAMessageFromContent
  };
  assert.equal((await sendMatterRegisterCard(input)).transport, "native-flow");
  assert.equal(sent[0][0], "native");
  socket.relayMessage = async () => { throw new Error("transport failure"); };
  assert.equal((await sendMatterRegisterCard(input)).transport, "text-fallback");
  assert.match(sent[1][2].text, /https:\/\/matter.example\/register/);
  await assert.rejects(() => sendMatterRegisterCard({ ...input, jid: "attacker@invalid" }));
});
