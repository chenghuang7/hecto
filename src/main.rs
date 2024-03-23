#![warn(clippy::all)]
#[allow(clippy::unused_self)]
mod editor;

use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
