# Referencia de UI: tarjetas de WhatsApp con botones inferiores (ts18 / ts19)

**Estado:** fuente aportada por el propietario, conservada sin adaptación
en [`references/ts18_ts19_buttons.go.txt`](references/ts18_ts19_buttons.go.txt).
**Destino previsto:** futura implementación independiente en el proveedor Rust
`providers/whatsapp-rust`. **No es código conectado a producción.**

## Qué demuestra el ejemplo Go

1. Registra dos comandos privados de prueba, `ts18` y `ts19`.
2. Busca vídeo o resuelve una URL mediante funciones externas no incluidas
   (`findVideo`, `formatViews`, APIs de descarga y tipos `bot`).
3. Descarga una miniatura, recorta al centro y codifica JPEG 300 × 300.
4. Compone una descripción con canal, duración, visitas y publicación.
5. Construye `[]bot.Button` con IDs de comando y etiquetas `Audio`, `MP4`
   y, solo en `ts19`, `Enlace`.
6. Envía `c.SendButtonsRow(title, text, c.Bot.Cfg.Dev, thumb, buttons)`.
7. Las rutas de `audio`, `mp4` y `link` reaccionan cuando reciben el ID
   como comando y llaman a dependencias externas del bot Go.

## Distinciones obligatorias

- `SendButtonsRow` es una API de un framework Go externo, **no HTML**.
- La captura con tarjeta de YouTube y dos botones coincide con este patrón.
- La captura con parrilla de selección de países no está implementada
  por este código. No afirmar que el ejemplo la proporciona.
- El import `github.com/ANDERSONTELLO`, el paquete `bot`, los tipos, las
  APIs externas y la licencia de origen no están verificados aquí.
- Este archivo no demuestra que un cliente WhatsApp actual acepte o muestre
  los mismos botones usando `whatsapp-rust` 0.7.0.
- No usar los IDs de ejemplo (que incluyen URLs) directamente en el
  control global: utilizar IDs opacos limitados, asociados al chat, usuario,
  sesión y caducidad, con validación de respuesta y antirreplay.

## Próximo contrato Rust (pendiente de implementación)

- Un modelo de tarjeta con `title`, `body`, `footer`, `thumbnail` y
  botones definidos como acciones autorizadas, sin introducir HTML externo.
- Un generador de mensajes interactivos en el proveedor Rust con fallback
  legible; no confundir respuesta enviada con visualización confirmada.
- Un despachador de respuestas que compruebe remitente, chat, firma,
  antirreplay y conjunto exacto de acciones permitidas.
- Pruebas de ida y vuelta de selección de botones en un teléfono real.
- Menú `antis` resumido y legible; controles de `set antilink` como diseño
  independiente y apagados de forma predeterminada.

## Origen y atribución

Fuente Go facilitada en la conversación por el propietario de Matterssmith.
No se atribuye autoría del código Go a Matterssmith ni se presupone licencia
para redistribución productiva. Conservar procedencia y revisar derechos
antes de copiar lógica ajena a software distribuido.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
