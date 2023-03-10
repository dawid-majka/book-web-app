use serde_json::{Map, Value};

use crate::state::write_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        write_file("./state.json", state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}
