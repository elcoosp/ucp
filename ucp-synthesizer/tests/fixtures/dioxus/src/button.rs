use dioxus::prelude::*;
use crate::cn;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90",
            Self::Destructive => "bg-destructive shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 text-white",
            Self::Outline => "bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border",
            Self::Secondary => "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80",
            Self::Ghost => "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
            Self::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

impl ButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            Self::Sm => "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5",
            Self::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
            Self::Icon => "size-9",
            Self::IconSm => "size-8",
            Self::IconLg => "size-10",
        }
    }
}

const BASE_CLASSES: &str = "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium outline-none transition-all focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0";

pub fn button_variants(variant: ButtonVariant, size: ButtonSize) -> String {
    format!("{} {} {}", BASE_CLASSES, variant.class(), size.class())
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub size: ButtonSize,
    #[props(into, default)]
    pub class: String,
    #[props(default = "button".to_string())]
    pub button_type: String,
    #[props(default = false)]
    pub disabled: bool,
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            "data-slot": "button",
            class: "{props.class}",
            disabled: props.disabled,
            {props.children}
        }
    }
}
