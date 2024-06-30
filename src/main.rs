#[cfg(feature = "no-wasm")]
#[tokio::main]
async fn non_wasm_main() {
    drag_bt::server::server().await;
}

fn main() {
    #[cfg(feature = "no-wasm")]
    non_wasm_main();
}
