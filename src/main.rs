use arboard::Clipboard;
use uuid::Uuid;
use notify_rust::{Hint, Notification};

fn main() {
    let id = Uuid::new_v4().as_hyphenated().to_string();
    println!("{}", id);

    Clipboard::new().unwrap().set_text(id.to_owned()).unwrap();

    Notification::new()
        .summary("Generated UUID in clipboard")
        .body(id.as_str())
        .hint(Hint::Transient(true))
        .show().unwrap();
}
