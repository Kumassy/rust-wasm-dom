extern crate webplatform;

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    body.html_set("<h1>Hello from Rust</h1> <button>Click me</button>");
    let button = document.element_query("button").unwrap();
    button.on("click", |_| webplatform::alert("Hello!"));
}
