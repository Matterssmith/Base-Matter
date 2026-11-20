// Package registercard builds an optional WhatsApp NativeFlow card.
// The calling Go transport must authenticate the inbound/outbound chat
// and map this abstract card to its pinned WhatsApp SDK.
// Credits: Hecho por Lournyx Matterssmith [Lou-Nyx].
package registercard

import (
	"encoding/json"
	"errors"
	"net/url"
	"strings"
)

type Button struct {
	Name             string `json:"name"`
	ButtonParamsJSON string `json:"buttonParamsJson"`
}

type Card struct {
	Title   string   `json:"title"`
	Body    string   `json:"body"`
	Footer  string   `json:"footer"`
	Buttons []Button `json:"buttons"`
}

func allowedURL(value string, hosts []string) (string, error) {
	if len(hosts) == 0 || len(value) == 0 || len(value) > 2048 {
		return "", errors.New("missing_public_url_or_allowlist")
	}
	u, err := url.Parse(value)
	if err != nil || u.Scheme != "https" || u.Hostname() == "" || u.User != nil || u.RawQuery != "" || u.Fragment != "" {
		return "", errors.New("unsafe_registration_url")
	}
	for _, host := range hosts {
		if strings.EqualFold(u.Hostname(), host) {
			return u.String(), nil
		}
	}
	return "", errors.New("unapproved_registration_host")
}

// Build creates a CTA_URL or opt-in OPEN_WEBVIEW and a guest reply,
// but does not send any message or grant any capability.
func Build(publicURL string, hosts []string, preferInAppWebview bool) (Card, error) {
	target, err := allowedURL(publicURL, hosts)
	if err != nil {
		return Card{}, err
	}
	var buttonName string
	var params any
	if preferInAppWebview {
		buttonName = "open_webview"
		params = map[string]any{
			"title": "Abrir Matter Register",
			"link": map[string]any{"in_app_webview": true, "url": target},
		}
	} else {
		buttonName = "cta_url"
		params = map[string]any{"display_text": "Abrir registro", "url": target}
	}
	mainParams, err := json.Marshal(params)
	if err != nil {
		return Card{}, err
	}
	return Card{
		Title: "✦ Matter Register",
		Body: "Regístrate en la web oficial o continúa como invitado. No compartas documentos en el chat.",
		Footer: "Matterssmith · Registro voluntario",
		Buttons: []Button{
			{Name: buttonName, ButtonParamsJSON: string(mainParams)},
			{Name: "quick_reply", ButtonParamsJSON: `{"display_text":"Soy invitado","id":"matter.register.guest"}`},
		},
	}, nil
}

// A reply is untrusted until the caller verifies sender, socket/chat,
// unique message ID and authorization. No civil identity is proven here.
