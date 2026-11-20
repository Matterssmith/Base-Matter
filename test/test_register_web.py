"""
Matter Register — minimal HTML loader and JavaScript-driven web verification.
Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
import pathlib
import re
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
WEB = ROOT / "web" / "matter-register"
HTML = (WEB / "index.html").read_text(encoding="utf-8")
JS = (WEB / "app.js").read_text(encoding="utf-8")


class RegisterJavaScriptArchitectureTests(unittest.TestCase):
    def test_html_is_only_the_javascript_loader(self):
        self.assertIn('<script src="./app.js" defer></script>', HTML)
        self.assertRegex(HTML, r"<body>\s*</body>")
        self.assertIn("script-src 'self'", HTML)
        self.assertEqual(len(re.findall(r"<script\b", HTML)), 1)
        for forbidden in ("<style", "onclick=", "onload=", "<section", "<form", "<h1", "<main", "srcdoc="):
            with self.subTest(forbidden=forbidden):
                self.assertNotIn(forbidden, HTML.lower())

    def test_javascript_owns_all_presentation_and_behavior(self):
        for feature in (
            "document.createElement",
            "document.body.innerHTML = TEMPLATE",
            "sheet.textContent = CSS",
            "requestFullscreen",
            "AudioContext",
            "prefers-reduced-motion",
            'audience:"guest"',
            "verification_provider_ready===true",
            "authenticated_session===true",
            "legal_documents_published===true",
            "ready===true",
            'data-theme-choice="ideas"',
            'data-theme-choice="kids"',
            'data-theme-choice="sfw"',
            'data-theme-choice="testers"',
            'data-theme-choice="channel"',
        ):
            with self.subTest(feature=feature):
                self.assertIn(feature, JS)

    def test_no_capture_or_secret_in_public_bootstrap(self):
        for source in (HTML, JS):
            self.assertNotRegex(source, r"gh[pousr]_[A-Za-z0-9]{25,}")
            self.assertNotIn("5215643778440", source)
            self.assertNotIn('type="file"', source)
            self.assertNotIn("localStorage", source)
            self.assertNotIn("sessionStorage", source)
        self.assertIn("SIN ENVÍO DE DATOS", JS)
        self.assertIn("No se pudo confirmar una sesión segura con Vault", JS)

    def test_optional_native_flow_is_independent_of_page(self):
        cards = (ROOT / "providers" / "baileys-compat" / "register-card.mjs").read_text()
        self.assertIn("viewOnceMessage", cards)
        self.assertIn("cta_url", cards)
        self.assertIn("open_webview", cards)
        self.assertNotIn("baileys", HTML.lower())


if __name__ == "__main__":
    unittest.main()
