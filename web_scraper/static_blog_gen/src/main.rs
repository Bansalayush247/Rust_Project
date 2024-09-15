use pulldown_cmark::{html, Parser};
use std::fs::File;
use std::io::Write;

fn markdown_to_html(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn generate_blog_post(title: &str, content: &str) -> String {
    format!("<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>{}</title>
</head>
<body>
    <article>{}</article>
</body>
</html>", title, markdown_to_html(content))
}

fn main() {
    let markdown_content = "www.google.com";
    let title = "Your Blog Post Title";
    let html_content = generate_blog_post(title, &markdown_content);
    
    // Code to write `html_content` to an HTML file
    let mut file = File::create("blog_post.html").expect("Could not create file");
    file.write_all(html_content.as_bytes()).expect("Could not write to file");
}
