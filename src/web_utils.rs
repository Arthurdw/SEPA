use leptos::window;

pub fn copy_to_clipboard(text: &str) {
    let clipboard = window().navigator().clipboard();
    let _ = clipboard.write_text(text);
}
