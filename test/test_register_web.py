"""
Base-Matter — static security checks for the registration preview.
Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import pathlib
import re
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
HTML = (ROOT / "web" / "matter-register" / "index.html").read_text(encoding="utf-8")

class RegistrationWebPreviewTests(unittest.TestCase):
    def test_safe_initial_state(self):
        self.assertIn('audience:"guest"', HTML)
        self.assertIn('ready===true', HTML)
        self.assertIn('verification_provider_ready===true', HTML)
        self.assertIn('authenticated_session===true', HTML)
        self.assertIn('legal_documents_published===true', HTML)
        self.assertIn('Sin datos', HTML.replace("SIN ENVÍO DE DATOS", "Sin datos"))
        self.assertNotIn('type="file"', HTML)
        self.assertNotIn("localStorage", HTML)
        self.assertNotIn("sessionStorage", HTML)

    def test_interactions_and_fallback(self):
        self.assertIn("requestFullscreen", HTML)
        self.assertIn("AudioContext", HTML)
        self.assertIn("prefers-reduced-motion", HTML)
        self.assertIn("viewOnceMessage", (ROOT / "providers" / "baileys-compat" / "register-card.mjs").read_text())
        for theme in ("ideas", "kids", "sfw", "testers", "channel"):
            self.assertIn(f'data-theme-choice="{theme}"', HTML)

    def test_no_embedded_credentials(self):
        self.assertNotRegex(HTML, r"gh[pousr]_[A-Za-z0-9]{25,}")
        self.assertNotIn("5215643778440", HTML)

if __name__ == "__main__":
    unittest.main()
