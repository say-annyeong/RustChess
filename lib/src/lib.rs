use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, GenericParam, Error};

#[proc_macro_derive(Dimension)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut life_params = Vec::new();
    let mut type_params = Vec::new();
    let mut const_params = Vec::new();

    for param in &ast.generics.params {
        match param {
            GenericParam::Lifetime(lifetime) => life_params.push(lifetime),
            GenericParam::Type(type_param) => type_params.push(type_param),
            GenericParam::Const(const_param) => const_params.push(const_param)
        }
    }

    let gen = {
        let target_const_params: Vec<_> = const_params.iter().filter_map(|x| {
            let const_param_name = x.ident.to_string();
            let const_param_type = x. 
            if const_param_name == "D".to_string() {
                Some(x)
            } else {
                None
            }
        }).collect();
        match target_const_params.len() {
            0 => return Error::new_spanned(&ast, "const D: usize가 없음 만들어 시키야").to_compile_error().into(),
            1 => {
                macro_rules! in_ref {
                    ($input:expr) => {
                        $input.into_iter().map(|x| x.clone()).collect()
                    };
                }
                let (life_params, type_params, const_params): (Vec<_>, Vec<_>, Vec<_>) = (in_ref!(life_params), in_ref!(type_params), in_ref!(const_params.clone()));
                let target_param_ident = &target_const_params[0].ident;
                let life_params_ident: Vec<_> = life_params.iter().map(|x| &x.lifetime).collect();
                let type_params_ident: Vec<_> = type_params.iter().map(|x| &x.ident).collect();
                let const_params_ident: Vec<_> = const_params.iter().map(|x| &x.ident).collect();
                quote! {
                    impl<#(#life_params,)* #(#type_params,)* #(#const_params),*> Dimension<#target_param_ident> for #name<#(#life_params_ident,)* #(#type_params_ident,)* #(#const_params_ident),*> {}
                }
            }
            _ => return Error::new_spanned(&ast, "const D: usize가 2개 이상인데 뭐함;;").to_compile_error().into()
        }
    };

    gen.into()
}