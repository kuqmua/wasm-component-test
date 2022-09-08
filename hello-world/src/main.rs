#[no_mangle]
pub extern "C" fn print_hello() {
    println!("Hello, world!");
}

#[no_mangle]
pub fn sum(x: i32, y: i32) -> i32 {
    println!("kekw");
    // using_a_macro();
    // use web_sys::console;
    // console_log!("Hello {}!", "world");
    x + y
}