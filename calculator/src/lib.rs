use bindings::exports::eval::{Op, Eval};
use bindings::local::calculator::add;

struct Component;

impl Eval for Component {
    /// Say hello!
    fn do_(op: Op, x: i32, y: i32) -> i32 {
        match op {
            Op::Add => add::add(x, y),
        }
    }
}

bindings::export!(Component);
