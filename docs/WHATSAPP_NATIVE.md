## WhatsAppNativeIngress: contrato del proveedor Rust

El proceso whatsapp-rust debe proporcionar exclusivamente un MessageContext confiable. El modulo Rust comprueba que la cuenta escribio el mensaje y que chat y emisor corresponden al JID privado propio. Requiere prefijo =/>, mensaje reciente e ID no repetido en SQLite antes de aplicar ControlDispatcher. No es un receptor de datos de red y NO autentica JSON arbitrario. El proceso no debe exponer QR; whatsapp-rust upstream puede generar un flujo interno QR concurrente con pair-code aun cuando el host no lo muestre. No hay emparejamiento real validado en este cambio.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
