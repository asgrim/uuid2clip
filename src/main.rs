use arboard::Clipboard;
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4().as_hyphenated().to_string();
    println!("uuid - {}", id);

    Clipboard::new().unwrap().set_text(id).unwrap();
}
