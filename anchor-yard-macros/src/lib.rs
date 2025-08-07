use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, GenericArgument, Ident, ItemFn, LitInt, PatType, PathArguments, Token, Type, TypePath,
    parse::Parse, parse_macro_input,
};

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

fn extract_component_types(
    inputs: &syn::punctuated::Punctuated<FnArg, Token![,]>,
) -> Vec<TypePath> {
    let mut component_types = Vec::new();

    for input in inputs {
        if let FnArg::Typed(PatType { ty, .. }) = input {
            if let Type::Path(type_path) = &**ty {
                if let Some(last_segment) = type_path.path.segments.last() {
                    if last_segment.ident == "View" || last_segment.ident == "ViewMut" {
                        if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                            if let Some(GenericArgument::Type(Type::Path(component_type))) =
                                args.args.first()
                            {
                                component_types.push(component_type.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    component_types
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
    let component_types = extract_component_types(fn_inputs);

    let register_components = component_types.iter().map(|ty| {
        quote! {
            anchor_yard::REGISTRY.lock().unwrap().register::<#ty>();
        }
    });

    let expanded = quote! {
        #vis fn #fn_name(#fn_inputs) #fn_output {
            {
                #(#register_components)*
            }

            use std::time::Instant;

            let snapshot = anchor_yard_core::with_current_world(|world| {
                anchor_yard_core::SystemSnapshot::capture_world(
                world,
                stringify!(#fn_name),
                #threshold_ms,
                )
            }).flatten();

            let start = Instant::now();
            let result = (|| #fn_block)();
            let elapsed = start.elapsed();

            if elapsed.as_millis() as u64 > #threshold_ms && let Some(snapshot) = snapshot {
                #[cfg(feature = "tracing")]
                tracing::info!("System '{}' took {}ms (threshold: {}ms). Saving snapshot...", stringify!(#fn_name), elapsed.as_millis(), #threshold_ms);
                let _ = snapshot.save_to_file();
                #[cfg(feature = "tracing")]
                tracing::info!("Snapshot saved!");
            }

            result
        }
    };

    TokenStream::from(expanded)
}
