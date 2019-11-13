use stdweb::js;

fn main() {
    stdweb::initialize();

    let message = "Hello, world!";
    js! {
        alert( @{message} );
    }

    stdweb::event_loop();
}
