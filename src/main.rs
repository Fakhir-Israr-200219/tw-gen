use std::collections::HashSet;
use std::fs;
use std::path::Path;
use regex::Regex;

const FULL_CSS: &str = include_str!("full.css"); // embedded CSS

fn main() {
    let current_dir = std::env::current_dir().expect("Cannot get current directory");
    let mut classes = HashSet::new();
    explore_dir(&current_dir, &mut classes);

    println!("✅ Found {} unique classes", classes.len());

    // extract CSS rules from embedded full.css
    let mut final_css = String::new();
    for cls in &classes {
        let safe_cls = regex::escape(cls);
        let pattern = format!(r"\.{}\s*\{{[^}}]*\}}", safe_cls);
        let re = Regex::new(&pattern).unwrap();

        for cap in re.find_iter(FULL_CSS) {
            final_css.push_str(cap.as_str());
            final_css.push('\n');
        }
    }
    if !final_css.is_empty() {
        fs::write("output.css", final_css).expect("Cannot write output.css");
    }else{
        println!("[X] not a single class found in this directory");
    }
}

fn explore_dir(path: &Path, classes: &mut HashSet<String>) {
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("Cannot read directory") {
            let entry = entry.expect("Invalid entry");
            let path = entry.path();

            if path.is_dir() {
                explore_dir(&path, classes);
            } else if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "html" || ext == "tsx" {
                    let content = fs::read_to_string(&path).unwrap_or_default();
                    extract_classes(&content, classes);
                }
            }
        }
    }
}

/// Extract classes from a source file (removes Rust comments + inline comments in strings)
fn extract_classes(content: &str, classes: &mut HashSet<String>) {
    // 1. Remove top-level Rust comments (// ... or /* ... */)
    let no_comments = Regex::new(r"//.*|/\*[\s\S]*?\*/")
        .unwrap()
        .replace_all(content, "");

    // 2. Capture `class = "..."` or `class: "..."` 
    let re = Regex::new(r#"class\s*[:=]\s*["']([^"']+)["']"#).unwrap();
    let class_filter = Regex::new(r"^[a-z0-9\-\:\/]+$").unwrap();

    for cap in re.captures_iter(&no_comments) {
        let mut class_str = cap[1].to_string();

        // 3. Remove inline comments inside strings
        class_str = Regex::new(r"/\*.*?\*/").unwrap().replace_all(&class_str, "").to_string();
        class_str = Regex::new(r"//.*").unwrap().replace_all(&class_str, "").to_string();

        // 4. Split and filter only valid class tokens
        for class_name in class_str.split_whitespace() {
            if class_filter.is_match(class_name) {
                classes.insert(class_name.to_string());
            }
        }
    }
}
