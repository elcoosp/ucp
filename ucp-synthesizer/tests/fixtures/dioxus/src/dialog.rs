use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogContext {
    pub open: Memo<bool>,
    pub set_open: Callback<bool>,
    pub content_id: String,
    pub title_id: String,
    pub description_id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    #[props(default)]
    pub open: Option<Signal<bool>>,
    #[props(default = false)]
    pub default_open: bool,
    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,
    pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let mut internal_open = use_signal(|| props.default_open);
    let on_open_change = props.on_open_change;

    let get_open = use_memo(move || {
        props.open.map(|s| s()).unwrap_or_else(|| internal_open())
    });

    let set_open = use_callback(move |new_state: bool| {
        if let Some(mut controlled) = props.open {
            controlled.set(new_state);
        } else {
            internal_open.set(new_state);
        }
        if let Some(callback) = on_open_change {
            callback.call(new_state);
        }
    });

    let base_id = "dialog-0".to_string();
    let content_id = format!("{}-content", base_id);
    let title_id = format!("{}-title", base_id);
    let description_id = format!("{}-description", base_id);

    use_context_provider(|| DialogContext {
        open: get_open,
        set_open,
        content_id: content_id.clone(),
        title_id: title_id.clone(),
        description_id: description_id.clone(),
    });

    rsx! {
        {props.children}
    }
}
