use cursive::Cursive;
use cursive::views::*;

// To Do: Look into using &str instead of String
pub fn extract_edit(tui: &mut Cursive, id: &str) -> String {
    tui.call_on_id(id, |v: &mut EditView| v.get_content())
        .unwrap()
        .to_string()
}

pub fn select_enum(s: &mut SelectView) {}