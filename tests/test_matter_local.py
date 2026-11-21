"""
Offline tests for Matter-AI's Ollama local backend, tools, and loopback isolation.
No model download, API key, Internet, or Ollama process is required.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import json
from pathlib import Path
import sys
import os
import tempfile
import unittest
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parents[1] / "tools"
sys.path.insert(0, str(TOOLS))
import matter_ai
import matter_local


class MatterLocalTests(unittest.TestCase):
    def test_endpoint_is_loopback_only(self):
        with self.assertRaises(ValueError):
            matter_local.local_request("https://cloud.ollama.com/api/chat")
        with self.assertRaises(ValueError):
            matter_local.local_request("http://0.0.0.0:11434/api/chat")

    def test_ollama_tool_conversion(self):
        schemas = matter_local.function_schemas()
        names = {item["function"]["name"] for item in schemas}
        self.assertIn("read_file", names)
        self.assertIn("write_file", names)
        self.assertIn("list_workstreams", names)
        self.assertTrue(all("parameters" in item["function"] for item in schemas))
        old = matter_ai.AUTOPILOT_MODE
        try:
            matter_ai.AUTOPILOT_MODE = True
            restricted = {item["function"]["name"] for item in matter_local.function_schemas()}
            self.assertNotIn("publish_workstream", restricted)
            self.assertNotIn("save_project_progress", restricted)
            self.assertIn("read_file", restricted)
        finally:
            matter_ai.AUTOPILOT_MODE = old

    def test_local_text_response(self):
        with patch.object(matter_local, "local_request", return_value={
            "message": {"role": "assistant", "content": "Matter listo."}
        }) as mock:
            answer = matter_local.ask("Continúa", [])
        self.assertEqual(answer, "Matter listo.")
        payload = mock.call_args.args[1]
        self.assertEqual(payload["model"], "qwen3:0.6b")
        self.assertEqual(payload["stream"], False)
        self.assertEqual(payload["messages"][-1]["role"], "user")

    def test_tool_call_dispatch_one_round(self):
        calls = [
            {"message": {"role": "assistant", "content": "",
                         "tool_calls": [{"type": "function", "function": {
                             "name": "list_workstreams", "arguments": {}
                         }}]}},
            {"message": {"role": "assistant", "content": "Proyectos encontrados."}}
        ]
        with patch.object(matter_local, "local_request", side_effect=calls) as mock:
            answer = matter_local.ask("Lista Matter", [])
        self.assertIn("encontrados", answer)
        second = mock.call_args.args[1]
        self.assertTrue(any(x.get("role") == "tool" for x in second["messages"]))
        self.assertIn("workstreams", json.dumps(second["messages"]))

    def test_configurable_local_model(self):
        with patch.object(matter_ai, "config", return_value={
            "backend": "local", "local_model": "qwen3:1.7b",
            "local_context": 2048
        }):
            self.assertEqual(matter_local.local_configuration()["model"], "qwen3:1.7b")

    def test_local_availability_checks_tags(self):
        with patch.object(matter_local, "local_request", return_value={
            "models": [{"name": "qwen3:0.6b"}, {"name": "qwen3:1.7b"}]
        }):
            report = matter_local.availability()
        self.assertTrue(report["running"])
        self.assertTrue(report["installed"])
        self.assertEqual(report["installed_count"], 2)

    @unittest.skipUnless(os.name == "posix", "POSIX file mode 0600 is Linux-specific")
    def test_chat_memory_is_local_private_and_filters_secrets(self):
        with tempfile.TemporaryDirectory() as td:
            history_file = Path(td) / "chat-history.json"
            with patch.object(matter_ai, "CHAT_HISTORY_PATH", history_file):
                matter_ai.save_local_chat_history([
                    {"user": "continúa Matter-Rust", "assistant": "pendiente revisar pruebas"},
                    {"user": "OPENAI_API_KEY=sk-" + "A" * 30, "assistant": "no guardar"}
                ])
                self.assertEqual(history_file.stat().st_mode & 0o077, 0)
                loaded = matter_ai.load_local_chat_history()
                self.assertEqual(len(loaded), 1)
                self.assertIn("Matter-Rust", loaded[0]["user"])

    def test_local_backend_does_not_require_openai_key(self):
        with patch.object(matter_ai, "config", return_value={"backend": "local"}), \
             patch.object(matter_local, "available", return_value=True), \
             patch.object(matter_ai, "api_key", side_effect=AssertionError("must not read API key")):
            self.assertTrue(matter_ai.backend_available())


if __name__ == "__main__":
    unittest.main()
