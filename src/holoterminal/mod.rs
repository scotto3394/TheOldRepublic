//! The `holoterminal` module
//!
//! This module will control the animations and graphics built into the 
//! application.

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::*;
use cursive::align::HAlign;

use super::force::drinks_served;

//To Do: Add a ton of documentation to all the `holoterminal` functions
pub fn startup(tui: &mut Cursive) {
	// Intro Screen
	tui.add_fullscreen_layer(Dialog::text("Welcome to the Hall of the Tauntaun King!")
		.h_align(HAlign::Center)
		.button("Start Game", character_select)
		.button("Quit", shutdown)
		.full_screen());
}

fn character_select(tui: &mut Cursive) {
	// Character Select & Login
	let select = SelectView::<String>::new()
        .on_submit(stronghold)
        .with_id("select")
        .fixed_size((10, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", build_character))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    tui.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        	.title("Select your hero"));
}

fn build_character(tui: &mut Cursive) {
    // fn ok(tui: &mut Cursive, name: &str) {
    //     tui.call_on_id("select", |view: &mut SelectView<String>| {
    //         view.add_item_str(name);
    //     });
    //     tui.pop_layer();
    // }

    // tui.add_layer(Dialog::around(EditView::new()
    //         .on_submit(ok)
    //         .with_id("name")
    //         .fixed_width(10))
    //     .title("Enter a new name")
    //     .button("Ok", |s| {
    //         let name = extract_edit(s, "name");
    //         ok(s, &name);
    //     })
    //     .button("Cancel", |s| s.pop_layer()));

	fn fill_entity(t: &mut Cursive) {

	}

    tui.add_layer(Dialog::new()
        .title("Create your loving Alter Ego!")
        .button("Ok", fill_entity)
        .content(ListView::new()
            .child("Name", EditView::new().fixed_width(10))
            .delimiter()
            .child("Age",
                   SelectView::new()
                       .popup()
                       .item_str("0-18")
                       .item_str("19-30")
                       .item_str("31-40")
                       .item_str("41+"))
            .with(|list| for i in 0..50 {
                list.add_child(&format!("Item {}", i), EditView::new());
            })));
}

//To Do: Assign Admin privileges for character deletion
fn delete_name(s: &mut Cursive) {
    let mut select = s.find_id::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn cantina(tui: &mut Cursive) {
	tui.add_layer(Dialog::around(EditView::new()
			.with_id("cantina")
			.fixed_width(10))
		.title("What would you like to order?")
		.button("Ok", |t| {
			let drink = extract_edit(t,"cantina");
			drinks_served(&drink);
			t.pop_layer();
		})
	);
}

// To Do: Look into TrackedView and easy callbacks to return to `Home` page
fn stronghold(tui: &mut Cursive, name: &String) {
	// Load in necessary callbacks
	tui.add_global_callback('/', cantina);

	// Home Screen
    tui.clear();
    tui.add_fullscreen_layer(Dialog::text(
			format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit)
		.full_screen());
}

fn extract_edit(tui: &mut Cursive, id: &str) -> String {
	tui.call_on_id(id, |v: &mut EditView| {
		v.get_content()
	})
		.unwrap()
		.to_string()
}

pub fn shutdown(tui: &mut Cursive) {
	tui.quit();
}