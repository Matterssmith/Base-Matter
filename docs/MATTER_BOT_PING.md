# Matter-Bot — diseño original de ping

Plantilla definida por el propietario el 25 de noviembre de 2026. El formato se reutiliza en el control `=/>ping` de WhatsApp; el futuro `#ping` del bot sigue separado y no habilita rutas Bot-Matter en Base-Matter.

```text
╭─ 💠 𝐏𝐨𝐧𝐠
│·♪·╠[ ⴵ · 𝐒𝐨𝐜𝐤𝐞𝐭 ···· *0 ms*
│·♪·╠[ ⚙︎  𝐏𝐫𝐨𝐜𝐞𝐬𝐬 · *0 ms*
│·♪·╚[ ✧  𝐓𝐨𝐭𝐚𝐥 ········ *0 ms*
╰───────────⟨ 🌀
```

Los ceros de la plantilla son marcadores, nunca mediciones inventadas. En `=/>ping` (proveedor WhatsApp): Socket = RTT medido del IPC local autenticado, no latencia remota de WhatsApp; Process = suma del tiempo monotónico de verificación local del comando y del procesamiento posterior al ACK hasta antes de formatear Pong; Total = Socket + Process. Se excluyen la consulta de metadatos diagnósticos, el formateo final y el envío posterior. Los valores se muestran con tres decimales de milisegundos. Solo se construye Pong tras un ACK aplicado y se intenta enviar una única respuesta, sin marco Control adicional. El bot apagado sigue sin responder a ping. El envío se registra posteriormente en consola y no prueba visualización en el cliente. `#ping` de Matter-Bot sigue pendiente.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

La plantilla ejecutable está en `templates/pong.txt`, con símbolos comunes de `templates/menu/theme.json`.
