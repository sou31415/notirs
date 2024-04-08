use notify_rust::Notification;
fn main() {
    let _ = Notification::new()
        .summary("Firefox news")
        .body("This will almost look like a real firefox notification.")
        .icon("GitHub-Mark.png")
        .show();
}
