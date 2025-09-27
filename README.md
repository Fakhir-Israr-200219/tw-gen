# 🌀 tw-gen

**tw-gen** is a lightweight Rust CLI tool that generates a custom CSS file from Tailwind-like utility classes found in your project files (`.rs`, `.html`, `.tsx`). It allows you to use Tailwind-style classes without needing Node.js or a full Tailwind installation. Ideal for Rust frontend frameworks like Leptos, Dioxus, or Yew.

## Features

- 🚀 **Rust-native:** No Node.js, npm, or Node modules required.  
- 🔍 **Automatic class extraction:** Scans your project directory for `.rs`, `.html`, and `.tsx` files to find all utility classes.  
- 🎨 **Custom CSS output:** Generates `output.css` with definitions only for classes actually used in your project.  
- 🛡️ **Comment-safe parsing:** Ignores commented-out classes or inline debug notes.  
- 📂 **Recursive scanning:** Automatically explores subdirectories.  
- 📦 **CLI installable globally:** `cargo install --path .` lets you run `tw-gen` anywhere.  

## Installation

```bash
# Clone the repository
git clone https://github.com/Fakhir-Israr-200219/tw-gen
cd tw-gen

# Install globally
cargo install --path .
```

Usage

Navigate to your project directory and run:

```bash
tw-gen
```

This will:

Scan the current directory and all subdirectories for .rs, .html, .tsx files.

Extract all unique Tailwind-like classes.

Generate an output.css containing only the CSS rules for the classes found.

✅ If no classes are found, it will print a message instead of creating an empty file.

Example

Input (.rs file with RSX):
```bash
rsx! {
    div {
        class: "container flex-col w-full /* not-a-class */ gap-4", 
        "Hello"
    }
}
```

Generated output.css:
```bash
.container { max-width: 100%; margin-left: auto; margin-right: auto; }
.flex-col { flex-direction: column; }
.w-full { width: 100%; }
.gap-4 { gap: 1rem; }
```
Embedded CSS

tw-gen uses an embedded full.css containing the complete Tailwind-like definitions. No external files or dependencies are required at runtime.

Contribution

Feel free to add more utility classes to full.css.

Add features like dynamic class parsing or custom class injection.
