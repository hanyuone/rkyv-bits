extern crate console_error_panic_hook;

use local::Foo;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

#[wasm_bindgen(main)]
async fn main() {
    // Used so WASM doesn't output "panic: unreachable" for every error
    console_error_panic_hook::set_once();

    let archived_bytes = reqwest::get("http://localhost:8080/public/archived")
        .await
        .expect("Request successful")
        .text()
        .await
        .expect("Could fetch text");

    let foo = rkyv::check_archived_root::<Foo>(archived_bytes.as_bytes())
        .expect("rkyv unarchive successful");

    // Displays it on HTTP
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");

    let body = document.body().expect("Could not access document.body");

    let text_node = document.create_text_node(&foo.content.to_string());

    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}
