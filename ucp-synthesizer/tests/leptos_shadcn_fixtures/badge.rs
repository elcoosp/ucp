use leptos::prelude::*;

pub struct BadgeProps { pub label: Option<String>, pub variant: Option<String> }
pub struct Badge;
impl Badge { pub fn render(props: BadgeProps) -> impl IntoView { view! { <span>{props.label}</span> } } }
