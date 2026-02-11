// https://rustwasm.github.io/book/game-of-life/debugging.html
#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        if cfg!(target_arch = "wasm32") {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        } else {
            println!("{}", &format!( $( $t )* ));
        }
    }
}

pub fn set_panic_hook() {
    #[cfg(all(target_arch = "wasm32", feature = "debug-panic-hook"))]
    console_error_panic_hook::set_once();
}
