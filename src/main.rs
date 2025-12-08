use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

const FULL_CSS: &str = include_str!("full.css"); // embedded CSS

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let watch_mode = args.iter().any(|arg| arg == "--watch");

    if watch_mode {
        println!("👀 Watching for file changes...");
        watch_and_generate();
    } else {
        generate_css();
    }
}

fn generate_css() {
    let current_dir = std::env::current_dir().expect("Cannot get current directory");
    let mut classes = HashSet::new();
    explore_dir(&current_dir, &mut classes);

    println!("✅ Found {} unique classes", classes.len());

    let mut final_css = String::new();
    for cls in &classes {
        let safe_cls = regex::escape(cls);
        // let pattern = format!(r"\.{}\s*\{{[^}}]*\}}", safe_cls);
        let pattern = format!(
            r"\.{}(?:\:hover|\:focus|\:active|\:disabled)?[^\{{]*\{{[^}}]*\}}",
            safe_cls
        );

        let re = Regex::new(&pattern).unwrap();

        for cap in re.find_iter(FULL_CSS) {
            final_css.push_str(cap.as_str());
            final_css.push('\n');
        }
    }

    if !final_css.is_empty() {
        fs::write("output.css", final_css).expect("Cannot write output.css");
        println!("💾 output.css updated!");
    } else {
        println!("[X] Not a single class found in this directory");
    }
}

fn watch_and_generate() {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        RecommendedWatcher::new(tx, Config::default()).expect("Failed to create watcher");

    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .expect("Cannot watch directory");

    println!("👀 Watching for changes...");

    // First run
    generate_css();

    let mut last_run = Instant::now();

    for res in rx {
        match res {
            Ok(Event { paths, .. }) => {
                // Check if relevant source file changed
                let relevant = paths.iter().any(|p| {
                    if let Some(ext) = p.extension() {
                        ext == "rs" || ext == "html" || ext == "tsx"
                    } else {
                        false
                    }
                });

                // Ignore output.css and target dir
                let ignore = paths.iter().any(|p| {
                    p.to_string_lossy().ends_with("output.css")
                        || p.to_string_lossy().contains("target")
                });

                if !relevant || ignore {
                    continue;
                }

                // Debounce 500ms
                if last_run.elapsed() > Duration::from_millis(500) {
                    println!("🔄 Source changed, regenerating...");
                    generate_css();
                    last_run = Instant::now();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
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

fn extract_classes(content: &str, classes: &mut HashSet<String>) {
    let no_comments = Regex::new(r"//.*|/\*[\s\S]*?\*/")
        .unwrap()
        .replace_all(content, "");

    let re = Regex::new(r#"class\s*[:=]\s*["']([^"']+)["']"#).unwrap();
    let class_filter = Regex::new(r"^[a-z0-9\-\:\/]+$").unwrap();

    for cap in re.captures_iter(&no_comments) {
        let mut class_str = cap[1].to_string();

        class_str = Regex::new(r"/\*.*?\*/")
            .unwrap()
            .replace_all(&class_str, "")
            .to_string();
        class_str = Regex::new(r"//.*")
            .unwrap()
            .replace_all(&class_str, "")
            .to_string();

        for class_name in class_str.split_whitespace() {
            if class_filter.is_match(class_name) {
                classes.insert(class_name.to_string());
            }
        }
    }
}
