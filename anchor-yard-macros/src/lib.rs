use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, LitInt, Token, parse::Parse, parse_macro_input};

struct SnapshotArgs {
    threshold_ms: u64,
}

impl Parse for SnapshotArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut threshold_ms = 1u64;

        if !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "threshold_ms" {
                input.parse::<Token![=]>()?;
                let lit: LitInt = input.parse()?;
                threshold_ms = lit.base10_parse()?;
            }
        }

        Ok(SnapshotArgs { threshold_ms })
    }
}

#[proc_macro_attribute]
pub fn snapshot_system(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SnapshotArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    let threshold_ms = args.threshold_ms;
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let vis = &input_fn.vis;

    let expanded = quote! {
        #vis fn #fn_name(#fn_inputs) #fn_output {
            use std::time::Instant;

            let start = Instant::now();
            let result = (|| #fn_block)();
            let elapsed = start.elapsed();

            if elapsed.as_millis() as u64 > #threshold_ms {
                if let Some(snapshot) = anchor_yard::SystemSnapshot::capture_current_world(
                    stringify!(#fn_name),
                    elapsed.as_millis() as u64
                ) {
                    let _ = snapshot.save_to_file();
                }
            }

            result
        }
    };

    TokenStream::from(expanded)
}
