extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const TEMPLATE: &str = "/*
 * [{{ problem_id }}] {{ problem_title }}
 */

struct Solution;

impl Solution {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::add(2, 3), 5);
    }
}
";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let problem_id = &args[1];
    let problem_title = &args[2];
    let filename = format!("src/a{}_{}.rs", problem_id, problem_title.replace("-", "_"));
    println!("{}", filename);

    let path = Path::new(&filename);
    if path.exists() {
        panic!("{} is already exists!", path.to_str().unwrap());
    }

    let rendered = Handlebars::new()
        .render_template(
            TEMPLATE,
            &json!({
                "problem_id": problem_id,
                "problem_title": problem_title,
            }),
        )
        .unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
    drop(file);
}
