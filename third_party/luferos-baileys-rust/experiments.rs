use crate::proto::ai_rich_response_code_metadata::AiRichResponseCodeBlock;
use crate::proto::ai_rich_response_latex_metadata::AiRichResponseLatexExpression;
use crate::proto::ai_rich_response_table_metadata::AiRichResponseTableRow;
use crate::proto::message::interactive_message::{
    self, native_flow_message::NativeFlowButton, Body, CarouselMessage, Footer, Header,
    NativeFlowMessage,
};
use crate::proto::message::{FutureProofMessage, InteractiveMessage};
use crate::proto::{
    AiRichResponseCodeMetadata, AiRichResponseLatexMetadata, AiRichResponseMessage,
    AiRichResponseSubMessage, AiRichResponseTableMetadata, Message, MessageContextInfo,
};
use crate::wa_binary::{BinaryNode, BinaryNodeAttributes, BinaryNodeData};
use rand::RngCore;
use serde_json::{json, Value};
use std::borrow::Borrow;

const AI_RICH_TYPE_STANDARD: i32 = 1;
const AI_RICH_SUB_TEXT: i32 = 2;
const AI_RICH_SUB_TABLE: i32 = 4;
const AI_RICH_SUB_CODE: i32 = 5;
const AI_RICH_SUB_LATEX: i32 = 8;
const CAROUSEL_HSCROLL_CARDS: i32 = 1;

#[derive(Debug, Clone)]
pub enum NativeButtonInput {
    QuickReply {
        display_text: String,
        id: String,
    },
    CtaUrl {
        display_text: String,
        url: String,
        merchant_url: Option<String>,
    },
    CtaCopy {
        display_text: String,
        copy_code: String,
    },
    CtaCall {
        display_text: String,
        phone_number: String,
    },
    WebView {
        display_text: String,
        url: String,
        in_app: bool,
    },
    SingleSelect {
        title: String,
        sections: Value,
    },
    Flow {
        flow_id: String,
        flow_token: String,
        cta_label: String,
        flow_action_payload: Option<Value>,
    },
    Custom {
        name: String,
        params_json: String,
    },
}

impl NativeButtonInput {
    pub fn to_proto(&self) -> NativeFlowButton {
        match self {
            Self::QuickReply { display_text, id } => NativeFlowButton {
                name: Some("quick_reply".to_string()),
                button_params_json: Some(
                    json!({
                        "display_text": display_text,
                        "id": id
                    })
                    .to_string(),
                ),
            },
            Self::CtaUrl {
                display_text,
                url,
                merchant_url,
            } => NativeFlowButton {
                name: Some("cta_url".to_string()),
                button_params_json: Some(
                    json!({
                        "display_text": display_text,
                        "url": url,
                        "merchant_url": merchant_url.as_deref().unwrap_or(url.as_str())
                    })
                    .to_string(),
                ),
            },
            Self::CtaCopy {
                display_text,
                copy_code,
            } => NativeFlowButton {
                name: Some("cta_copy".to_string()),
                button_params_json: Some(
                    json!({
                        "display_text": display_text,
                        "copy_code": copy_code
                    })
                    .to_string(),
                ),
            },
            Self::CtaCall {
                display_text,
                phone_number,
            } => NativeFlowButton {
                name: Some("cta_call".to_string()),
                button_params_json: Some(
                    json!({
                        "display_text": display_text,
                        "phone_number": phone_number
                    })
                    .to_string(),
                ),
            },
            Self::WebView {
                display_text,
                url,
                in_app,
            } => NativeFlowButton {
                name: Some("open_webview".to_string()),
                button_params_json: Some(
                    json!({
                        "title": display_text,
                        "link": {
                            "in_app_webview": in_app,
                            "url": url
                        }
                    })
                    .to_string(),
                ),
            },
            Self::SingleSelect { title, sections } => NativeFlowButton {
                name: Some("single_select".to_string()),
                button_params_json: Some(
                    json!({
                        "title": title,
                        "sections": sections
                    })
                    .to_string(),
                ),
            },
            Self::Flow {
                flow_id,
                flow_token,
                cta_label,
                flow_action_payload,
            } => NativeFlowButton {
                name: Some("galaxy_message".to_string()),
                button_params_json: Some(
                    json!({
                        "flow_message_version": "3",
                        "flow_token": flow_token,
                        "flow_id": flow_id,
                        "flow_cta": cta_label,
                        "flow_action": "navigate",
                        "flow_action_payload": flow_action_payload.clone().unwrap_or_else(|| json!({}))
                    })
                    .to_string(),
                ),
            },
            Self::Custom { name, params_json } => NativeFlowButton {
                name: Some(name.clone()),
                button_params_json: Some(params_json.clone()),
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InteractiveInput {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub body: Option<String>,
    pub footer: Option<String>,
    pub buttons: Vec<NativeButtonInput>,
    pub message_params_json: Option<String>,
    pub wrap_view_once: bool,
}

#[derive(Debug, Clone)]
pub enum RichBlock {
    Text(String),
    Code {
        language: String,
        content: String,
    },
    Table {
        title: Option<String>,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Latex {
        text: String,
        expressions: Vec<String>,
    },
}

#[derive(Debug, Clone, Default)]
pub struct RichResponseInput {
    pub blocks: Vec<RichBlock>,
}

fn generate_message_secret() -> Vec<u8> {
    let mut secret = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    secret
}

pub fn attach_experimental_context(mut msg: Message) -> Message {
    if msg.message_context_info.is_none() {
        msg.message_context_info = Some(
            MessageContextInfo {
                device_list_metadata_version: Some(2),
                message_secret: Some(generate_message_secret()),
                ..Default::default()
            }
            .into(),
        );
    }
    msg
}

pub fn wrap_view_once_message(inner: Message) -> Message {
    attach_experimental_context(Message {
        view_once_message: Some(
            FutureProofMessage {
                message: Some(inner.into()),
            }
            .into(),
        ),
        ..Default::default()
    })
}

pub fn build_interactive_proto(input: &InteractiveInput) -> InteractiveMessage {
    let header = if input.title.is_some() || input.subtitle.is_some() {
        Some(Box::new(Header {
            title: input.title.clone(),
            subtitle: input.subtitle.clone(),
            has_media_attachment: Some(false),
            media: None,
        }))
    } else {
        None
    };

    let body = input.body.as_ref().map(|t| Body {
        text: Some(t.clone()),
    });

    let footer = input.footer.as_ref().map(|t| Box::new(Footer {
        text: Some(t.clone()),
        has_media_attachment: Some(false),
        media: None,
    }));

    let buttons = input.buttons.iter().map(NativeButtonInput::to_proto).collect();

    InteractiveMessage {
        header,
        body,
        footer,
        interactive_message: Some(interactive_message::InteractiveMessage::NativeFlowMessage(
            NativeFlowMessage {
                buttons,
                message_params_json: input.message_params_json.clone(),
                message_version: Some(1),
            }
            .into(),
        )),
        ..Default::default()
    }
}

pub fn build_interactive_message(input: &InteractiveInput) -> Message {
    let interactive = build_interactive_proto(input);
    let inner = attach_experimental_context(Message {
        interactive_message: Some(interactive.into()),
        ..Default::default()
    });
    if input.wrap_view_once {
        wrap_view_once_message(inner)
    } else {
        inner
    }
}

pub fn build_webview_message(
    title: impl Into<String>,
    body: impl Into<String>,
    footer: Option<String>,
    button_label: impl Into<String>,
    url: impl Into<String>,
    in_app: bool,
) -> Message {
    let input = InteractiveInput {
        title: Some(title.into()),
        subtitle: None,
        body: Some(body.into()),
        footer,
        buttons: vec![NativeButtonInput::WebView {
            display_text: button_label.into(),
            url: url.into(),
            in_app,
        }],
        message_params_json: None,
        wrap_view_once: true,
    };
    build_interactive_message(&input)
}

pub fn build_carousel_message(
    body: Option<String>,
    footer: Option<String>,
    cards: Vec<InteractiveInput>,
    wrap_view_once: bool,
) -> Message {
    let card_protos: Vec<InteractiveMessage> =
        cards.iter().map(build_interactive_proto).collect();
    let interactive = InteractiveMessage {
        body: body.map(|t| Body { text: Some(t) }),
        footer: footer.map(|t| Box::new(Footer {
            text: Some(t),
            has_media_attachment: Some(false),
            media: None,
        })),
        interactive_message: Some(interactive_message::InteractiveMessage::CarouselMessage(
            CarouselMessage {
                cards: card_protos,
                message_version: Some(1),
                carousel_card_type: Some(CAROUSEL_HSCROLL_CARDS),
            }
            .into(),
        )),
        ..Default::default()
    };

    let inner = attach_experimental_context(Message {
        interactive_message: Some(interactive.into()),
        ..Default::default()
    });

    if wrap_view_once {
        wrap_view_once_message(inner)
    } else {
        inner
    }
}

pub fn build_rich_response_message(input: &RichResponseInput) -> Message {
    let mut submessages = Vec::with_capacity(input.blocks.len());

    for block in &input.blocks {
        match block {
            RichBlock::Text(text) => {
                submessages.push(AiRichResponseSubMessage {
                    message_type: Some(AI_RICH_SUB_TEXT),
                    message_text: Some(text.clone()),
                    ..Default::default()
                });
            }
            RichBlock::Code { language, content } => {
                submessages.push(AiRichResponseSubMessage {
                    message_type: Some(AI_RICH_SUB_CODE),
                    code_metadata: Some(AiRichResponseCodeMetadata {
                        code_language: Some(language.clone()),
                        code_blocks: vec![AiRichResponseCodeBlock {
                            highlight_type: Some(0),
                            code_content: Some(content.clone()),
                        }],
                    }),
                    ..Default::default()
                });
            }
            RichBlock::Table {
                title,
                headers,
                rows,
            } => {
                let mut proto_rows = Vec::with_capacity(rows.len() + 1);
                if !headers.is_empty() {
                    proto_rows.push(AiRichResponseTableRow {
                        items: headers.clone(),
                        is_heading: Some(true),
                    });
                }
                for row in rows {
                    proto_rows.push(AiRichResponseTableRow {
                        items: row.clone(),
                        is_heading: Some(false),
                    });
                }
                submessages.push(AiRichResponseSubMessage {
                    message_type: Some(AI_RICH_SUB_TABLE),
                    table_metadata: Some(AiRichResponseTableMetadata {
                        title: title.clone(),
                        rows: proto_rows,
                    }),
                    ..Default::default()
                });
            }
            RichBlock::Latex { text, expressions } => {
                let exprs = expressions
                    .iter()
                    .map(|e| AiRichResponseLatexExpression {
                        latex_expression: Some(e.clone()),
                        ..Default::default()
                    })
                    .collect();
                submessages.push(AiRichResponseSubMessage {
                    message_type: Some(AI_RICH_SUB_LATEX),
                    latex_metadata: Some(AiRichResponseLatexMetadata {
                        text: Some(text.clone()),
                        expressions: exprs,
                    }),
                    ..Default::default()
                });
            }
        }
    }

    let rich = AiRichResponseMessage {
        message_type: Some(AI_RICH_TYPE_STANDARD),
        submessages,
        ..Default::default()
    };

    attach_experimental_context(Message {
        rich_response_message: Some(rich.into()),
        ..Default::default()
    })
}

fn unwrap_future_proof<'a>(msg: &'a Message) -> &'a Message {
    if let Some(ref vo) = msg.view_once_message {
        let inner: &FutureProofMessage = Borrow::borrow(vo);
        if let Some(ref m) = inner.message {
            return Borrow::borrow(m);
        }
    }
    if let Some(ref vo) = msg.view_once_message_v2 {
        let inner: &FutureProofMessage = Borrow::borrow(vo);
        if let Some(ref m) = inner.message {
            return Borrow::borrow(m);
        }
    }
    if let Some(ref vo) = msg.view_once_message_v2_extension {
        let inner: &FutureProofMessage = Borrow::borrow(vo);
        if let Some(ref m) = inner.message {
            return Borrow::borrow(m);
        }
    }
    msg
}

pub fn extract_experimental_nodes(msg: &Message) -> Vec<BinaryNode> {
    let mut nodes = Vec::new();
    let target = unwrap_future_proof(msg);

    if let Some(ref interactive_box) = target.interactive_message {
        let interactive: &InteractiveMessage = Borrow::borrow(interactive_box);
        let flow_name = match &interactive.interactive_message {
            Some(interactive_message::InteractiveMessage::NativeFlowMessage(nf_box)) => {
                let nf: &NativeFlowMessage = Borrow::borrow(nf_box);
                let first_name = nf
                    .buttons
                    .first()
                    .and_then(|b| b.name.as_deref())
                    .unwrap_or("mixed");
                match first_name {
                    "galaxy_message" => "galaxy_message",
                    "open_webview" => "open_webview",
                    "review_and_pay" => "order_details",
                    _ => "mixed",
                }
            }
            Some(interactive_message::InteractiveMessage::CarouselMessage(_)) => "mixed",
            _ => "mixed",
        };

        nodes.push(BinaryNode {
            tag: "biz".to_string(),
            attrs: BinaryNodeAttributes::new(),
            content: Some(BinaryNodeData::List(vec![BinaryNode {
                tag: "interactive".to_string(),
                attrs: BinaryNodeAttributes::from([
                    ("type".to_string(), "native_flow".to_string()),
                    ("v".to_string(), "1".to_string()),
                ]),
                content: Some(BinaryNodeData::List(vec![BinaryNode {
                    tag: "native_flow".to_string(),
                    attrs: BinaryNodeAttributes::from([
                        ("v".to_string(), "9".to_string()),
                        ("name".to_string(), flow_name.to_string()),
                    ]),
                    content: None,
                }])),
            }])),
        });
    }

    if target.rich_response_message.is_some() {
        nodes.push(BinaryNode {
            tag: "bot".to_string(),
            attrs: BinaryNodeAttributes::from([("biz_bot".to_string(), "1".to_string())]),
            content: None,
        });
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webview_and_biz_node_generation() {
        let msg = build_webview_message(
            "Panel",
            "Abre el panel embebido",
            Some("Baileys-Rust".to_string()),
            "Abrir WebView",
            "https://example.com",
            true,
        );
        let nodes = extract_experimental_nodes(&msg);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].tag, "biz");
    }

    #[test]
    fn test_rich_response_blocks_and_bot_node() {
        let msg = build_rich_response_message(&RichResponseInput {
            blocks: vec![
                RichBlock::Text("Diagnostico".to_string()),
                RichBlock::Code {
                    language: "rust".to_string(),
                    content: "fn main() {}".to_string(),
                },
                RichBlock::Table {
                    title: Some("Metricas".to_string()),
                    headers: vec!["Modulo".to_string(), "ms".to_string()],
                    rows: vec![vec!["Signal".to_string(), "0.4".to_string()]],
                },
                RichBlock::Latex {
                    text: "Formula".to_string(),
                    expressions: vec!["E = mc^2".to_string()],
                },
            ],
        });
        let nodes = extract_experimental_nodes(&msg);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].tag, "bot");
    }
}
