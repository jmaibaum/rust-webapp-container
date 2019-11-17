use stdweb::{js, web::{document, IParentNode}};

fn main() {
    stdweb::initialize();

    let main_content_p = document().query_selector("#main_content").unwrap().unwrap();
    js!(@{main_content_p.as_ref()}.innerHTML = "Hello, world");

    stdweb::event_loop();
}
