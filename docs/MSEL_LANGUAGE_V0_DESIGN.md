# MSEL — Diseño funcional v0 (borrador técnico)

**Nombre oficial del lenguaje:** MSEL (Matter Script Extension Language).  
**Componente Matter:** Matter-MSEL = integración de MSEL con Matter Ecosystem, equivalente en función al adaptador Matter-Rust. **No es un segundo lenguaje.**  
**Prioridad actual:** P0 = **hacer que el núcleo autónomo de MSEL funcione**. La integración de bots, sistemas y 32 tecnologías es posterior y depende de interfaces probadas.  
**Estado:** DISEÑO / NO IMPLEMENTADO. Ningún ejemplo aquí representa código que ya ejecuta un compilador.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 1. Alcance del lenguaje v0

MSEL se desarrollará como un lenguaje propio, no como un simple traductor de palabras a Python, Rust o JavaScript. El núcleo v0 debe proporcionar:

- Variables inmutables y mutables, literales, expresiones, operadores, tipos y funciones.
- Condiciones, bucles, retorno, errores recuperables, módulos, importaciones y una biblioteca estándar mínima.
- Un único AST semántico y una única MSEL IR tipada e independiente del idioma de origen.
- Diagnósticos con archivo, línea y columna; comandos para comprobar, ejecutar y probar programas.
- Frontends oficiales iniciales español (es) e inglés (en) con semántica idéntica.
- Runtime autónomo para CLI/local; el host de bots Matter no es requisito para usar MSEL.
- Extensión .msel independiente del idioma y sufijos explícitos .es.msel / .en.msel. Los idiomas adicionales se agregan por paquetes versionados.

**No incluir en el núcleo inicial:** motores completos de Python, C++, Rust, bases de datos, GUI, shaders, FFmpeg, WhatsApp, ni acceso privilegiado. Sus capacidades se integrarán más tarde por bibliotecas, FFI, backends y adaptadores.

## 2. Semántica canónica y frontends

Un **concepto semántico** tiene identificador estable independiente del idioma. Ejemplo de tabla propuesta:

| Concepto IR | Español | Inglés |
| --- | --- | --- |
| binding inmutable | `sea` | `let` |
| binding mutable | `variable` | `var` |
| función | `funcion` | `function` |
| condición | `si` | `if` |
| alternativa | `sino` | `else` |
| bucle | `mientras` | `while` |
| retorno | `retornar` | `return` |
| verdadero | `verdadero` | `true` |
| falso | `falso` | `false` |
| importación | `importar` | `import` |
| texto (tipo) | `Texto` | `Text` |
| sin retorno (tipo) | `Nada` | `Void` |
| entrada de programa | `principal` | `main` |
| salida en consola | `mostrar` | `print` |

**Decisión provisional:** usar delimitadores estructurales comunes (llaves, paréntesis, punto y coma opcional sólo si la gramática lo define de forma inequívoca). Las palabras reservadas pueden traducirse; la semántica y los operadores no. Se aceptará Unicode para identificadores, con normalización, diagnóstico de colisiones y política de caracteres confusables.

**Ejemplo de sintaxis propuesto en español:**

```msel
funcion saludar(nombre: Texto) -> Texto {
    retornar "Hola, " + nombre;
}

funcion principal() -> Nada {
    sea mensaje = saludar("Matter");
    mostrar(mensaje);
}
```

**Equivalente en inglés:**

```msel
function greet(name: Text) -> Text {
    return "Hola, " + name;
}

function main() -> Void {
    let message = greet("Matter");
    print(message);
}
```

**Importante:** el ejemplo muestra **comportamiento equivalente** y no una IR idéntica byte por byte: los identificadores elegidos por el autor y las posiciones de origen son diferentes. La comparación de IR debe normalizar nombres locales y metadatos no semánticos. El punto de entrada `principal`/`main` enlaza al mismo símbolo canónico; los nombres exportados restantes requieren una política explícita de enlace entre módulos. Las APIs de la biblioteca estándar podrán tener alias lingüísticos que enlacen al mismo identificador canónico.

## 3. Gramática inicial propuesta (resumen EBNF)

```ebnf
programa       = { declaracion } ;
declaracion    = funcion | variable | importacion ;
funcion        = KW_FUNCION , IDENT , "(" , [ parametros ] , ")" ,
                 [ "->" , tipo ] , bloque ;
parametros     = parametro , { "," , parametro } ;
parametro      = IDENT , ":" , tipo ;
variable       = ( KW_SEA | KW_VARIABLE ) , IDENT , [ ":" , tipo ] ,
                 "=" , expresion , ";" ;
importacion    = KW_IMPORTAR , STRING , ";" ;
bloque         = "{" , { sentencia } , "}" ;
sentencia      = variable | bloque | condicional | bucle | retorno |
                 expresion , ";" ;
condicional    = KW_SI , "(" , expresion , ")" , bloque ,
                 [ KW_SINO , bloque ] ;
bucle          = KW_MIENTRAS , "(" , expresion , ")" , bloque ;
retorno        = KW_RETORNAR , [ expresion ] , ";" ;
expresion      = asignacion ;
asignacion     = logica , [ "=" , asignacion ] ;
logica         = comparacion , { ("&&" | "||") , comparacion } ;
comparacion    = suma , { ("==" | "!=" | "<" | ">" | "<=" | ">=") , suma } ;
suma           = producto , { ("+" | "-") , producto } ;
producto       = unario , { ("*" | "/" | "%") , unario } ;
unario         = [ "!" | "-" ] , primario ;
primario       = literal | IDENT | "(" , expresion , ")" | llamada ;
llamada        = IDENT , "(" , [ argumentos ] , ")" ;
argumentos     = expresion , { "," , expresion } ;
tipo           = IDENT ;
```

Este es un **borrador que necesita pruebas del parser**. El AST no se debe inferir de expresiones regulares ni traducir código fuente mediante reemplazo textual. La gramática final debe resolver precedencia de acceso a miembros, llamadas encadenadas, métodos, inferencia de tipos y salto de línea.

## 4. Tipos y comportamiento mínimo

- Primitivos propuestos: Entero (entero con ancho especificado), Decimal, Booleano, Texto y Nada. Definir semántica precisa para overflow, Unicode y redondeo antes de declarar estabilidad.
- Ámbito léxico, sombreado de nombres y mutabilidad explícitos. Los valores son inmutables salvo declaración mutable.
- Sin conversiones implícitas de texto a números ni de booleanos a enteros.
- Errores de tipo y símbolos desconocidos se diagnostican antes de ejecutar.
- E/S de consola mediante biblioteca estándar; no abrir red/procesos/archivos ni secretos sin capacidades autorizadas.
- Importaciones resueltas por rutas controladas, sin descarga automática de paquetes durante la compilación.

## 5. Resolución del idioma

Precedencia propuesta: sufijo explícito de archivo > metadatos explícitos del archivo (si no hay sufijo) > configuración del proyecto > configuración del entorno > preferencia del usuario > detección de tokens > idioma predeterminado del proyecto.

**Regla crítica:** dos indicaciones explícitas contradictorias generan error, no sustitución silenciosa. El idioma de los mensajes de error se configura aparte del idioma del código.

Los códigos canónicos seguirán BCP 47. Para japonés usar `ja`, coreano `ko` y portugués brasileño `pt-BR`; `jp`, `kr`, `br` pueden mantenerse como alias documentados.

## 6. MSEL IR v0

El frontend convierte el código local al mismo conjunto de operaciones conceptuales:

- Module, Import, Function, Parameter, Binding, Assignment, Block.
- Constant, Call, ConditionalBranch, Loop, Return, BinaryOp, UnaryOp.
- TypeRef, SourceSpan, CapabilityRequirement, ErrorDiagnostic.

La IR será **versionada**, validable, con tabla de símbolos y metadatos de localización separados del núcleo semántico. Una optimización no puede cambiar el comportamiento observable. Las transformaciones de idioma deben pasar por AST/IR y no por reemplazos de texto.

## 7. CLI objetivo

```text
msel check ruta/programa.es.msel
msel run ruta/programa.es.msel
msel test
msel fmt ruta/programa.es.msel
msel translate ruta/programa.es.msel --to en
```

Son **comandos previstos**. Sólo `check`, `run` y `test` constituyen la primera puerta de ejecución; formato y traducción se completan después.

## 8. Plan verificable para cerrar P0

| Etapa | Comprobación |
| --- | --- |
| L0 | Gramática y diagnóstico formal con ejemplos válidos e inválidos |
| L1 | Lexer/parser Rust con suites es/en, Unicode y errores |
| L2 | AST, símbolos, comprobación de tipos y equivalencia semántica IR entre idiomas |
| L3 | Ejecución de funciones, variables, bloques, condicionales, bucles y consola |
| L4 | CLI reproducible en Linux/Windows con pruebas de módulos, errores, permisos y programas de extremo a extremo |

**Criterio de aceptación:** dos programas MSEL funcionalmente equivalentes, uno español y otro inglés, pasan los mismos tests de comportamiento; una CLI ejecuta programas reales sin depender de Base-Matter, Matter-MSEL ni servicios remotos.

**Sólo después:** comenzar Matter-MSEL para integrar MSEL con bots/sistemas; interoperar con la lista de 32 entradas tecnológicas de [MATTER_MSEL_MASTER_SPEC.md](MATTER_MSEL_MASTER_SPEC.md) según prioridad, compatibilidad y pruebas. No etiquetar una entrada como implementada si sólo existe el diseño.

## 9. Continuidad

Cada avance deberá registrar: resumen, archivos modificados, tests con resultado reproducible, pendientes, siguiente paso y SHA remoto. Revisar los README afectados antes de cada commit. Mantener créditos y no guardar secretos en repositorios.

---

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
