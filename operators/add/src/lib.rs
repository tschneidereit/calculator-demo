use bindings::exports::local::calculator::add::Add;

struct Component;
impl Add for Component {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
bindings::export!(Component);
