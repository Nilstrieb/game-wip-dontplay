use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn expand(_: TokenStream) -> TokenStream {
    quote! {
        output_mut(|o| o.copied_text = "".into());
    }
    .into()
}
