// wit_bindgen::generate!("http-trigger" in "../wit/deps/spin");

// use exports::fermyon::spin::inbound_http::{self, Request, Response};

// // mod cal {
// //     wit_bindgen::generate!("calculator" in "../wit");
// // }
use bindings::calculator_eval::{calc, Op};

// struct SpinHttp;
// export_http_trigger!(SpinHttp);

// impl inbound_http::InboundHttp for SpinHttp {
//     fn handle_request(req: Request) -> Response {
//         // println!("{}", calc(Op::Add, 1, 2));

//         Response {
//             status: 200,
//             headers: None,
//             body: Some(String::from("Great success!").into_bytes()),
//         }
//     }
// }
#[no_mangle]
pub extern "C" fn foo() {
    println!("{}", calc(Op::Add, 1, 2));
    println!("Hello, world!");
}