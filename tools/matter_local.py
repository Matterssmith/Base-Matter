#!/usr/bin/env python3
"""
Matter Local — Ollama loopback chat/tool backend for Matter-AI.
Uses the official /api/chat function calling protocol, without any cloud key.
Does not install, run, or expose Ollama to the internet.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from __future__ import annotations

import json
import re
import urllib.error
import urllib.request
import matter_ai

OLLAMA_HOST = "http://127.0.0.1:11434"
CHAT_URL = OLLAMA_HOST + "/api/chat"
TAGS_URL = OLLAMA_HOST + "/api/tags"


def local_configuration() -> dict:
    cfg = matter_ai.config()
    model = cfg.get("local_model", "qwen3:0.6b")
    if not isinstance(model, str) or not re.fullmatch(r"[a-zA-Z0-9_.:/-]{1,80}", model):
        raise ValueError("Modelo local no válido")
    context = cfg.get("local_context", 4096)
    if type(context) is not int or not 1024 <= context <= 16384:
        raise ValueError("Contexto local fuera de límites")
    return {"model": model, "num_ctx": context}


def local_request(url: str, payload: dict | None = None, timeout: int = 180) -> dict:
    """No proxy; 127.0.0.1 only. No remote or user-defined URLs."""
    if url not in {CHAT_URL, TAGS_URL}:
        raise ValueError("Ruta Ollama prohibida")
    headers = {"Accept": "application/json"}
    data = None
    if payload is not None:
        data = json.dumps(payload, ensure_ascii=False).encode("utf-8")
        headers["Content-Type"] = "application/json"
        if len(data) > 700_000:
            raise ValueError("Entrada local demasiado grande")
    request = urllib.request.Request(url, data=data, headers=headers,
                                     method="POST" if data is not None else "GET")
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    try:
        with opener.open(request, timeout=timeout) as reply:
            result = reply.read(2_000_001)
            if len(result) > 2_000_000:
                raise RuntimeError("Respuesta Ollama demasiado grande")
            return json.loads(result)
    except urllib.error.HTTPError as error:
        raise RuntimeError(f"Ollama devolvió HTTP {error.code}; revisa modelo y RAM.") from None
    except (urllib.error.URLError, TimeoutError):
        raise RuntimeError("Ollama no responde en 127.0.0.1:11434; comprueba systemctl status ollama") from None


def availability() -> dict:
    model = local_configuration()["model"]
    try:
        tags = local_request(TAGS_URL, timeout=5)
    except (RuntimeError, ValueError, json.JSONDecodeError):
        return {"running": False, "model": model, "installed": False}
    models = tags.get("models", []) if isinstance(tags, dict) else []
    names = {entry.get("name") for entry in models if isinstance(entry, dict)}
    return {"running": True, "model": model, "installed": model in names,
            "installed_count": len(names)}


def available() -> bool:
    info = availability()
    return info["running"] and info["installed"]


def function_schemas() -> list[dict]:
    allowed = {"list_workstreams", "list_files", "read_file", "write_file", "run_command"}
    selected = [tool for tool in matter_ai.TOOLS
                if not matter_ai.AUTOPILOT_MODE or tool["name"] in allowed]
    return [{
        "type": "function",
        "function": {
            "name": tool["name"],
            "description": tool.get("description", ""),
            "parameters": tool["parameters"]
        }
    } for tool in selected]


def local_messages(prompt: str, history: list[dict]) -> list[dict]:
    instructions = matter_ai.SYSTEM
    if matter_ai.AUTOPILOT_MODE:
        instructions += "\n" + matter_ai.autonomy_policy()
    messages = [{"role": "system", "content": instructions}]
    if matter_ai.HANDOFF_PATH.is_file() and not matter_ai.HANDOFF_PATH.is_symlink():
        document = matter_ai.HANDOFF_PATH.read_text(encoding="utf-8")[:4200]
        messages.append({"role": "system",
                         "content": "Traspaso del proyecto (verificar contra código):\n" + document})
    for turn in history[-2:]:
        messages.extend([
            {"role": "user", "content": turn["user"][:1200]},
            {"role": "assistant", "content": turn["assistant"][:1200]},
        ])
    messages.append({"role": "user", "content": prompt[:6500]})
    return messages


def ask(prompt: str, history: list[dict]) -> str:
    """Use local model, loop through allowed tool calls as long as bounded."""
    settings = local_configuration()
    messages = local_messages(prompt, history)
    tools = function_schemas()
    for _ in range(matter_ai.MAX_ACTIONS):
        result = local_request(CHAT_URL, {
            "model": settings["model"],
            "messages": messages,
            "stream": False,
            "tools": tools,
            "options": {
                "num_ctx": settings["num_ctx"],
                "num_predict": 480,
                "temperature": 0.15,
            },
            "keep_alive": "10m"
        }, timeout=300)
        message = result.get("message", {})
        if not isinstance(message, dict):
            raise RuntimeError("Respuesta Ollama inválida")
        text = message.get("content", "")
        calls = message.get("tool_calls", []) or []
        if not isinstance(calls, list):
            raise RuntimeError("Tool calls locales no válidos")
        if not calls:
            return text.strip() if isinstance(text, str) and text.strip() else "Sin respuesta local. Prueba otro modelo."
        if len(calls) > 5:
            raise RuntimeError("El modelo local pidió demasiadas acciones")
        messages.append({
            "role": "assistant",
            "content": text if isinstance(text, str) else "",
            "tool_calls": calls
        })
        for tool_call in calls:
            function = tool_call.get("function", {}) if isinstance(tool_call, dict) else {}
            name = function.get("name", "")
            args = function.get("arguments", {})
            if isinstance(args, str):
                try:
                    args = json.loads(args)
                except json.JSONDecodeError:
                    args = {}
            if not isinstance(args, dict) or not isinstance(name, str):
                data = {"error": "argumentos_del_modelo_invalidos"}
            else:
                data = matter_ai.dispatch(name, args)
            print(f"  • {name}: {data.get('error', 'ok')}", flush=True)
            messages.append({
                "role": "tool", "tool_name": name,
                "content": json.dumps(data, ensure_ascii=False)[:26000]
            })
    return "Límite de pasos del modelo local. Los cambios guardados permanecen."


if __name__ == "__main__":
    print(json.dumps(availability(), indent=2))
