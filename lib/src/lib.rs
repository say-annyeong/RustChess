use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, LitInt, Data, Fields};

#[proc_macro_derive(Board, attributes(board_dimension))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let param_value = board_dimension(&ast).unwrap_or(0);

    let name = &ast.ident;

    let gen = quote! {
        impl Board for #name {
            fn new(size: Vec<usize>) -> BoardXD {
                if !size.len() == #param_value { panic!("Board{}D is not {}D!", #param_value, #param_value) }
                BoardXD::new(size)
            }
        }
    };

    gen.into()
}

fn board_dimension(ast: &DeriveInput) -> Option<usize> {
    for attr in &ast.attrs {
        if attr.path().is_ident("board_dimension") {
            let lit_int: LitInt = attr.parse_args().ok()?;
            return lit_int.base10_parse().ok();
        }
    }
    None
}