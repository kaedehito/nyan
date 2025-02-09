pub mod init;
pub mod nyanterm;
pub mod objects;

#[cfg(test)]
mod tests {
    use crate::{init::NyanTerminal, nyanterm::NyanTermObjs, objects::Objects};

    #[test]
    fn test() {
        let nyan = NyanTerminal::new(10).clear().alternate_screen();

        let mut obj: NyanTermObjs<String> = NyanTermObjs::new();

        obj.add_object("hello_world".into(), Objects::Text("Hello world!"));
        let dbged = format!("{:?}", &nyan);
        obj.add_object("Test_NyanTerm_dbg".into(), Objects::Text(&dbged));

        loop {
            nyan.draw(|| {
                obj.draw_object("hello_world".into());
                obj.draw_object("Test_NyanTerm_dbg".into());
            })
            .unwrap();
        }
    }
}
