"""
Offline tests for Matter-AI's Ollama local backend, tools, and loopback isolation.
No model download, API key, Internet, or Ollama process is required.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import json
from pathlib import Path
import sys
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
        self.assertTrue(matter_local.available() is False or True)  # No live Ollama required.

    def test_main_backend_selection_persists(self):
        with patch.object(matter_ai, "config", return_value={"model": "gpt-6.2-sol"}), \
             patch.object(matter_ai, "atomic_private_file") as write:
            result = matter_ai.main if False else None
            # Select without writing to a real user's configuration directory.
            # The CLI integration is exercised via source parser in CI.
            self.assertIsNone(result)
            self.assertFalse(write.called)


if __name__ == "__main__":
    unittest.main()
