pub mod app;
pub mod cursor;
pub mod input;
pub mod nyanobj;
pub mod objects;

#[cfg(test)]
mod tests {
    use crate::{
        app::App,
        cursor::{self, Cursor},
        input::{NyanInput, NyanInputKey},
        nyanobj::NyanObj,
        objects::Objects,
    };

    #[test]
    fn test() {
        let mut nyan = App::new(60).clear().raw_mode().alternate_screen();

        let mut obj = NyanObj::new();

        obj.add_object("hello_world", Objects::Text("Hello world!"));
        let dbged = format!("{:?}", &nyan);
        obj.add_object("Test_NyanTerm_dbg", Objects::Text(&dbged));

        loop {
            nyan.draw(|| {
                obj.draw_object("hello_world").unwrap();
                cursor::Cursor::move_cursor(Cursor::Move(0, 1)).unwrap();
                obj.draw_object("Test_NyanTerm_dbg").unwrap();
            })
            .unwrap();

            let p = NyanInput::get_input();
            match p.unwrap() {
                NyanInput::Ctrl(NyanInputKey::C) => {
                    break;
                }

                NyanInput::Key(NyanInputKey::A) => {
                    obj.update_object("hello_world", Objects::Text("You Plessed \"A\"!"));
                }
                _ => {}
            }
        }

        nyan.exit().unwrap();
    }
}
