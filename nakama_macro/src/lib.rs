use proc_macro::{TokenStream};

/// A simple macro that can be used to annotate `async fn main` so that it
/// executes and waits for the future to resolve.
#[proc_macro_attribute]
pub fn nakama_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let result: TokenStream = format!(
        "
        use simple_logger::SimpleLogger;
        use futures::executor::block_on;
        use log::LevelFilter;


        fn main() {{
            SimpleLogger::new()
                .with_level(LevelFilter::Debug)
                .with_module_level(\"#nakama_rs\", LevelFilter::Trace)
                .init()
                .unwrap();

           {}

           block_on(main());
        }} 
    ",
        item,
    )
    .parse()
    .unwrap();

    result
}
