use proc_macro::{Ident, TokenStream, TokenTree};

/// A simple macro that can be used to annotate `async fn main` so that it
/// executes and waits for the future to resolve.
#[proc_macro_attribute]
pub fn nakama_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let result: TokenStream = format!(
        "
        use futures::executor::block_on;
        fn main() {{
           {}
           let result = async {{
                main().await
           }};
           block_on(result);
        }} 
    ",
        item,
    )
    .parse()
    .unwrap();

    result
}
