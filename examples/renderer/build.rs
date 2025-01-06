use peacock::build::WidgetCollection;

use std::collections::HashMap;

fn main() {
    let libraries: WidgetCollection = HashMap::new();
    // libraries.insert("Ocelli", Box::new(peacock_ocelli::gen_library));
    peacock::build::build_with_libraries(libraries);
}
