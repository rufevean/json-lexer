extern crate minify;
use minify::json::minify as json_minimize;

pub fn minify(contents :&String) -> String{
    let minified_json = json_minimize(&contents);
    println!("{}", minified_json);
    minified_json
}
