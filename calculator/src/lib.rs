use bindings::exports::eval::{Op, Eval};
use bindings::local::calculator::add;

struct Component;
impl Eval for Component {
    fn calc(op: Op, x: i32, y: i32, desc: String) -> (i32, String) {
        println!("{desc}");
        match op {
            Op::Add => (add::add(x, y), String::from("Addition applied")),
        }
    }
}
bindings::export!(Component);
