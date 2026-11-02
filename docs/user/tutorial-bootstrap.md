# Tutorial: Bootstrapping a Component Library

Learn how to extract a UCP spec from a real component library.

## Prerequisites

- UCP CLI installed (`ucp --version`)
- A directory of UI component source files (Rust with Dioxus/Leptos/GPUI,
  or TypeScript/TSX with React)

## Step 1: Prepare Source Directory

For this tutorial, we'll use a mock Dioxus codebase. Create a small test
project:

``` bash
mkdir -p my-lib/src
cat > my-lib/src/button.rs << 'SRC'
use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps {
    #[props(default)]
    disabled: bool,
    label: String,
    variant: ButtonVariant,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! { button { disabled: props.disabled, "{props.label}" } }
}
SRC
```

## Step 2: Run Bootstrap

``` bash
ucp bootstrap --source-dir my-lib --output-dir ./spec
```

Expected output:

```
🔍 Scanning my-lib...
   📁 Files scanned:   1
   📄 Files parsed:    1
   🧩 Components:     1
   ✓ Spec written to spec/ucp-spec.json
✅ Synthesis complete!
```

## Step 3: Examine the Spec

``` bash
ucp components spec/ucp-spec.json --verbose
```

You should see:

```
  🧩 Button
     - disabled: ControlFlag
     - label: StaticValue(Any)
     - variant: StaticValue(Any)
```

## Step 4: Machine-Readable Output

For CI, add `--json` to get a one-line machine-readable summary:

``` bash
ucp bootstrap --source-dir my-lib --output-dir ./spec --json
# {"components":12,"ok":true,"output":"./spec/ucp-spec.json","stats":{...}}
```

Bootstrap is fully offline and deterministic: the same sources always produce the
same spec bytes, so the spec can live in version control.

## Next Steps

- [Merge specs from multiple codebases](tutorial-merge.md)
- [Learn about the Canonical Abstract Model](../concepts/cam.md)
