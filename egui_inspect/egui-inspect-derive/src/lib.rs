use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Member};

#[proc_macro_derive(Inspect)]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ts = match input.data {
        Data::Struct(s) => {
            let mut exprs = Vec::new();
            for (i, f) in s.fields.iter().enumerate() {
                let ident = &f.ident;
                exprs.push(quote! {
                            if ui.add(::egui::Label::new(stringify!(#f)).sense(::egui::Sense::click())).clicked() {
                                ui.output_mut(|o| o.copied_text = format!("{:?}", self.#ident));
                            }
                });
            }
            quote! {
                ::egui::CollapsingHeader::new("").id_source(id_source).show(ui, |ui| {
                    #(#exprs)*
                });
            }
        }
        _ => panic!(),
    };

    let expanded = quote! {
        impl ::egui_inspect::Inspect for GameState {
            fn inspect_mut(&mut self, ui: &mut ::egui::Ui, id_source: u64) {
                #ts
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
