# Matter-Bot — diseño original de ping

Plantilla definida por el propietario el 25 de noviembre de 2026. Reservada para el futuro `#ping` del bot, no para habilitar rutas Bot-Matter en Base-Matter.

```text
╭─💠 𝐏𝐨𝐧𝐠
│·♪·╠[ ⴵ · 𝐒𝐨𝐜𝐤𝐞𝐭 ···· *0 ms*
│·♪·╠[ ⚙︎  𝐏𝐫𝐨𝐜𝐞𝐬𝐬 · *0 ms*
│·♪·╚[ ✧  𝐓𝐨𝐭𝐚𝐥 ········ *0 ms*
╰───────────⟨ 🌀
```

Los ceros son marcadores, nunca mediciones inventadas. Semántica propuesta pendiente de validar: Socket = RTT de prueba autenticada del transporte; Process = tiempo monotónico local excluyendo esa prueba; Total = tiempo hasta construir respuesta. Envío real se registra después, no puede incluirse en esa misma respuesta sin otra operación. Si no hay medición, mostrar N/D. `=/>ping` conserva RTT IPC local. No implementado aún en Matter-Bot.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
