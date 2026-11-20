package registercard

import (
	"encoding/json"
	"testing"
)

func TestBuildUsesPublicVerifiedURL(t *testing.T) {
	if _, err := Build("http://matter.example/register", []string{"matter.example"}, false); err == nil {
		t.Fatal("http URL accepted")
	}
	if _, err := Build("https://matter.example/register?token=secret", []string{"matter.example"}, false); err == nil {
		t.Fatal("URL token accepted")
	}
	if _, err := Build("https://evil.example/register", []string{"matter.example"}, false); err == nil {
		t.Fatal("wrong host accepted")
	}
	if _, err := Build("https://matter.example/register", nil, false); err == nil {
		t.Fatal("missing allowlist accepted")
	}
	c, err := Build("https://matter.example/register", []string{"matter.example"}, false)
	if err != nil {
		t.Fatal(err)
	}
	if len(c.Buttons) != 2 || c.Buttons[0].Name != "cta_url" {
		t.Fatalf("unexpected button: %+v", c.Buttons)
	}
	var p map[string]any
	if err := json.Unmarshal([]byte(c.Buttons[0].ButtonParamsJSON), &p); err != nil {
		t.Fatal(err)
	}
	if p["url"] != "https://matter.example/register" {
		t.Fatalf("unexpected URL %q", p["url"])
	}
	v, err := Build("https://matter.example/register", []string{"matter.example"}, true)
	if err != nil || v.Buttons[0].Name != "open_webview" {
		t.Fatal("missing opt-in webview")
	}
}
