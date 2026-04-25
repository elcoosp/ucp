use leptos::prelude::*;

pub struct StandardizedButtonProps {
    pub id: Option<String>,
    pub disabled: Option<bool>,
    pub variant: Option<StandardVariant>,
    pub size: Option<StandardSize>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub children: Option<Children>,
}

pub enum StandardVariant { Default, Destructive, Outline, Secondary, Ghost, Link }
pub enum StandardSize { Default, Sm, Lg, Icon }

pub struct StandardizedButton;
impl StandardizedButton {
    pub fn render(props: StandardizedButtonProps) -> impl IntoView {
        view! { <button disabled={props.disabled.unwrap_or(false)}>{props.children}</button> }
    }
}
