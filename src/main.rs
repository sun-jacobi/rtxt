use editor::Editor;

mod doc;
mod editor;
mod row;
mod terminal;
pub use doc::Doc;
pub use row::Row;

fn main() {
    Editor::default().run();
}
