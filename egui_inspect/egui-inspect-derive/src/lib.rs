use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Inspect)]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    let expanded = quote! {
        impl ::egui_inspect::Inspect for GameState {
            fn inspect_mut(&mut self, ui: &mut ::egui::Ui) {
                output_mut(|o| o.copied_text = format!(""));
                output_mut(|o| o.copied_text = format!("{:?}", self.tile_db));
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
