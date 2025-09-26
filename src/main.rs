// use std::fs;
// use std::path::Path;

// fn main() {
//     let current_dir = std::env::current_dir().expect("Cannot get current directory");
//     explore_dir(&current_dir);
// }

// fn explore_dir(path: &Path) {
//     if path.is_dir() {
//         for entry in fs::read_dir(path).expect("Cannot read directory") {
//             let entry = entry.expect("Invalid entry");
//             let path = entry.path();

//             if path.is_dir() {
//                 explore_dir(&path); // recursive call
//             } else if let Some(ext) = path.extension() {
//                 if ext == "rs" {
//                     println!("\n📂 File: {}\n", path.display());
//                     let content = fs::read_to_string(&path).unwrap_or_default();
//                     println!("{}", content);
//                 }
//             }
//         }
//     }
// }



// use std::fs;
// use std::path::Path;
// use std::collections::HashSet;
// use regex::Regex;

// fn main() {
//     let current_dir = std::env::current_dir().expect("Cannot get current directory");

//     // 1. Collect all classes from project
//     let mut classes = HashSet::new();
//     explore_dir(&current_dir, &mut classes);

//     println!("✅ Found {} unique classes", classes.len());

//     // 2. Load Tailwind full CSS
//     let full_css = fs::read_to_string("./src/full.css").expect("full.css missing");

//     // 3. Extract matched CSS
//     let mut final_css = String::new();
//     for cls in &classes {
//         let safe_cls = regex::escape(cls);
//         let pattern = format!(r"\.{}\s*\{{[^}}]*\}}", safe_cls);
//         let re = Regex::new(&pattern).unwrap();

//         for cap in re.find_iter(&full_css) {
//             final_css.push_str(cap.as_str());
//             final_css.push('\n');
//         }
//     }

//     // 4. Write output.css
//     fs::write("output.css", final_css).expect("Cannot write output.css");
// }

// fn explore_dir(path: &Path, classes: &mut HashSet<String>) {
//     if path.is_dir() {
//         for entry in fs::read_dir(path).expect("Cannot read directory") {
//             let entry = entry.expect("Invalid entry");
//             let path = entry.path();

//             if path.is_dir() {
//                 explore_dir(&path, classes); // recursive
//             } else if let Some(ext) = path.extension() {
//                 if ext == "rs" || ext == "html" || ext == "tsx" {
//                     let content = fs::read_to_string(&path).unwrap_or_default();
                    
//                     // Regex to extract class-like tokens
//                     let re = Regex::new(r"[a-z0-9\-:\/]+").unwrap();
//                     for cap in re.find_iter(&content) {
//                         classes.insert(cap.as_str().to_string());
//                     }
//                 }
//             }
//         }
//     }
// }


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

    fs::write("output.css", final_css).expect("Cannot write output.css");
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
                    let re = Regex::new(r"[a-z0-9\-:\/]+").unwrap();
                    for cap in re.find_iter(&content) {
                        classes.insert(cap.as_str().to_string());
                    }
                }
            }
        }
    }
}



// container
// mx-auto
// my-4
// mt-8
// mb-2



