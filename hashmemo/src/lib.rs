use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, DeriveInput, Expr, FnArg, Ident, ItemFn, ReturnType, Type,
};
#[proc_macro_attribute]
pub fn memo(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let mut key_types = Vec::<Type>::new();
    let mut keys = Vec::<Expr>::new();
    input.sig.inputs.iter().for_each(|x| match x {
        FnArg::Typed(pat) => {
            key_types.push((*pat.ty).clone());
            let p = &pat.pat;
            keys.push(parse_quote!(#p));
        }
        _ => unimplemented!(),
    });

    let key_type: Type = parse_quote! {(#(#key_types),*)};
    let ret_type: Type = match &input.sig.output {
        ReturnType::Type(_, ty) => (**ty).clone(),
        _ => panic!("required: return type"),
    };
    let memo_type: Type = parse_quote!(std::collections::HashMap<#key_type, #ret_type>);
    let memo_name = format_ident!("{}_MEMO", format!("{}", input.sig.ident).to_uppercase());

    let fn_sig = input.sig;
    let fn_block = input.block;

    let expanded = quote! {
            thread_local!(
                static #memo_name: std::cell::RefCell<#memo_type> =
                    std::cell::RefCell::new(std::collections::HashMap::new())
            );

            #fn_sig {
                if let Some(ret) = #memo_name.with(|memo| memo.borrow().get(&(#(#keys),*)).cloned()) {
                    return ret.clone();
                }
                let ret: #ret_type = (||#fn_block)();
                #memo_name.with(|memo| {
                    memo.borrow_mut().insert((#(#keys),*), ret.clone());
                });
                ret
            }
    };

    expanded.into()
}
