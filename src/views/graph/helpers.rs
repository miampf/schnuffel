use crate::views::graph::{constants, GraphState, GraphStateUpdate};
use crate::views::ViewState;
use crate::Message;
use iced::mouse::ScrollDelta;
use iced::widget::{column, text, Column};
use iced::{Element, Theme};
use schnuffel_types::graph::{DNSRecord, Node};

pub fn update_graph(state: &mut GraphState, message: Message) {
    match message {
        Message::MouseClick(position) => {
            for node in &mut state.graph.nodes {
                // clear selections
                node.is_selected = false;

                if (position.x - node.x).powf(2.0) + (position.y - node.y).powf(2.0)
                    < (node.radius * state.zoom_factor).powf(2.0)
                {
                    node.is_dragged = true;
                    node.is_selected = true;
                }
            }
        }
        Message::MouseDrag(position) => {
            for node in &mut state.graph.nodes {
                if node.is_dragged {
                    node.x = position.x;
                    node.y = position.y;
                }

                if state.is_panning {
                    let pan_delta = state.panning_start_point - position;
                    node.x -= pan_delta.x * constants::PAN_MULTIPLIER;
                    node.y -= pan_delta.y * constants::PAN_MULTIPLIER;
                }
            }
            if state.is_panning {
                state.panning_start_point = position;
            }
        }
        Message::MouseRelease => {
            state.is_panning = false;
            for node in &mut state.graph.nodes {
                node.is_dragged = false;
            }
        }
        Message::MouseScroll(delta) => {
            if let ScrollDelta::Lines { x: _, y } = delta {
                state.zoom_factor += y * constants::ZOOM_MULTIPLIER;
                state.zoom_factor = state
                    .zoom_factor
                    .clamp(constants::MIN_ZOOM, constants::MAX_ZOOM);

                for node in &mut state.graph.nodes {
                    // update the node positions to reflect the zoom
                    if y < 0.0 && state.zoom_factor != constants::MIN_ZOOM {
                        // zoom out
                        node.x += state.zoom_factor * constants::NODE_ZOOM_POSITION_CHANGE;
                        node.y += state.zoom_factor * constants::NODE_ZOOM_POSITION_CHANGE;
                    } else if y > 0.0 && state.zoom_factor != constants::MAX_ZOOM {
                        // zoom in
                        node.x -= state.zoom_factor * constants::NODE_ZOOM_POSITION_CHANGE;
                        node.y -= state.zoom_factor * constants::NODE_ZOOM_POSITION_CHANGE;
                    }
                }
            }
        }
        Message::MiddleMouseClick(position) => {
            state.is_panning = true;
            state.panning_start_point = position;
        }
    };
    state.update_state(GraphStateUpdate {
        graph: state.graph.clone(),
        zoom_factor: state.zoom_factor,
    });
}

pub fn build_info_column(state: &GraphState) -> Column<Message, Theme, iced::Renderer> {
    match state.graph.nodes.iter().find(|n| n.is_selected) {
        Some(node) => match &node.node {
            Node::SocialMedia {
                social_media_url,
                account_url,
            } => {
                column!(
                    text("Type: Social Media"),
                    text(format!("Network URL: {}", social_media_url.as_str())),
                    text(format!("Account URL: {}", account_url.as_str())),
                )
            }
            Node::IP(ip) => {
                column!(text("Type: IP Address"), text(format!("IP: {}", ip)))
            }
            Node::Person(name) => {
                column!(text("Type: Person"), text(format!("Name: {name}")))
            }
            Node::Domain(domain) => {
                column!(
                    text("Type: Domain"),
                    text(format!("domain: {}", domain.domain))
                )
            }
            Node::Website { url } => {
                column!(text("Type: Website"), text(format!("URL: {url}")))
            }
            Node::DNSEntry { nameserver, record } => {
                let mut entry: Vec<Element<'_, Message, Theme, iced::Renderer>> = vec![
                    text("Type: DNS Entry").into(),
                    text(format!("NS: {}", nameserver.domain)).into(),
                ];
                let mut record: Vec<Element<'_, Message, Theme, iced::Renderer>> = match record {
                    DNSRecord::A(addr) => {
                        vec![
                            text("Record Type: A").into(),
                            text(format!("Address: {addr}")).into(),
                        ]
                    }
                    DNSRecord::MX(domain) => vec![
                        text("Record Type: MX").into(),
                        text(format!("Domain: {}", domain.domain)).into(),
                    ],
                    DNSRecord::TXT(txt) => {
                        vec![
                            text("Record Type: TXT").into(),
                            text(format!("Text: {txt}")).into(),
                        ]
                    }
                    DNSRecord::AAAA(addr) => {
                        vec![
                            text("Record Type: AAAA").into(),
                            text(format!("Address: {addr}")).into(),
                        ]
                    }
                    DNSRecord::SRV {
                        service,
                        protocol,
                        from,
                        to,
                        to_port,
                    } => vec![
                        text("Record Type: SRV").into(),
                        text(format!("Service: {service}")).into(),
                        text(format!("Protocol: {protocol}")).into(),
                        text(format!("From: {from}")).into(),
                        text(format!("To: {to}")).into(),
                        text(format!("To port: {to_port}")).into(),
                    ],
                    DNSRecord::CNAME { from, to } => vec![
                        text("Record Type: CNAME").into(),
                        text(format!("From: {from}")).into(),
                        text(format!("To: {}", to.domain)).into(),
                    ],
                };
                entry.append(&mut record);
                Column::from_vec(entry)
            }
            Node::PhoneNumber(number) => column!(
                text("Type: Phone Number"),
                text(format!("Number: {}", number.number))
            ),
            Node::EmailAddress(email) => column!(
                text("Type: Email Address"),
                text(format!("Address: {}", email.email))
            ),
            Node::Organization(org) => {
                column!(text("Type: Organization"), text(format!("Name: {org}")))
            }
        },
        None => column!().padding(10),
    }
}
