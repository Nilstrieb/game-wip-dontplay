use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Inspect)]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    let expanded = quote! {
        impl GameState {
            fn inspect_mut(&mut self) {
                output_mut(|o| o.copied_text = "".into());
                output_mut(|o| o.copied_text = format!("{:?}", self.tile_db));
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
