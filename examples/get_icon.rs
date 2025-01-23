//

use freedesktop_icons::{list_themes, lookup};
use linicon::lookup_icon;

fn main() {
    let res = lookup("input-keyboard-symbolic")
        .with_size(24)
        .with_theme("breeze-dark")
        .find();
    // println!("{:?}", list_themes());
    println!("{res:?}");

    lookup_icon("input-keyboard-symbolic")
        // .from_theme("Adwaita")
        .from_theme("breeze-dark")
        .for_each(|f| {
            println!("{f:?}");
        });
}
