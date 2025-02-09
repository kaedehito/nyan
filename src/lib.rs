pub mod app;
pub mod nyanobj;
pub mod objects;

#[cfg(test)]
mod tests {
    use crate::{app::App, nyanobj::NyanObj, objects::Objects};

    #[test]
    fn test() {
        let nyan = App::new(10).clear().alternate_screen();

        let mut obj = NyanObj::new();

        obj.add_object("hello_world", Objects::Text("Hello world!"));
        let dbged = format!("{:?}", &nyan);
        obj.add_object("Test_NyanTerm_dbg", Objects::Text(&dbged));

        loop {
            nyan.draw(|| {
                obj.draw_object("hello_world").unwrap();
                obj.draw_object("Test_NyanTerm_dbg").unwrap();
            })
            .unwrap();
        }
    }
}
