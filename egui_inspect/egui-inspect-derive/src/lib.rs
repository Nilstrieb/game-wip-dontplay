use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Member};
enum FieldInspectKind {
    /// Auto-inspected (it is assumed that the field implements Inspect)
    Auto,
    /// A function named by the token stream is called to inspect the field.
    /// The function takes (thing: &mut T, ui: &mut Ui, id_source: u64)
    WithFn(syn::Ident),
    /// Not visited, left alone.
    /// Useful when you want to skip a field that doesn't implement Inspect.
    Opaque,
}

fn inspect_kind(attrs: &[Attribute]) -> FieldInspectKind {
    for attr in attrs {
        if attr.path().is_ident("opaque") {
            return FieldInspectKind::Opaque;
        } else if attr.path().is_ident("inspect_with") {
            let fun: syn::Ident = attr.parse_args().expect("Failed to parse ident");
            return FieldInspectKind::WithFn(fun);
        }
    }
    FieldInspectKind::Auto
}

#[proc_macro_derive(Inspect, attributes(opaque, inspect_with))]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty_ident = input.ident;
    let ts = match input.data {
        Data::Struct(s) => {
            let mut exprs = Vec::new();
            for (i, f) in s.fields.iter().enumerate() {
                let memb = match &f.ident {
                    Some(ident) => Member::from(ident.clone()),
                    None => Member::from(i),
                };
                match inspect_kind(&f.attrs) {
                    FieldInspectKind::Auto => {
                        exprs.push(quote! {
                            ui.horizontal(|ui| {
                                if ui.add(::egui::Label::new(stringify!(#f)).sense(::egui::Sense::click())).clicked() {
                                    ui.output_mut(|o| o.copied_text = format!("{:?}", self.#memb));
                                }
                            });
                        });
                    }
                    FieldInspectKind::Opaque => {}
                    FieldInspectKind::WithFn(_) => {}
                }
            }
            quote! {
                ::egui::CollapsingHeader::new(stringify!(#ty_ident)).id_source(id_source).show(ui, |ui| {
                    #(#exprs)*
                });
            }
        }
        _ => panic!(),
    };
    let (intro_generics, forward_generics, where_clauses) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #intro_generics ::egui_inspect::Inspect for #ty_ident #forward_generics #where_clauses {
            fn inspect_mut(&mut self, ui: &mut ::egui::Ui, id_source: u64) {
                #ts
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
