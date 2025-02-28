use crate::domain::dto::EditorCurrent;
use salvo::prelude::*;

pub fn get_current(depot: &Depot) -> EditorCurrent {
    let res = depot.get::<EditorCurrent>("current_editor");
    if res.is_err() {
        return EditorCurrent::empty();
    }
    let opt = res.unwrap();
    let editor = opt.to_owned();
    editor
}
