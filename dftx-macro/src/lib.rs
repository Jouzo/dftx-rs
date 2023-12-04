use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, ItemFn};

#[proc_macro_attribute]
pub fn test_dftx_serialization(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let test_name = fn_name.to_string().replace("_", "");

    let path = format!("./tests/data/{}.txt", &test_name[4..].to_lowercase());

    let fn_body = &input_fn.block;

    let output = quote! {
        fn #fn_name() {
            #fn_body
            let s = std::fs::read_to_string(#path).unwrap();
            for line in s.lines() {
                let l = line.split(' ').next().unwrap();
                let hex = &hex::decode(l).unwrap();

                let offset = 1 + match hex[1] {
                    0x4c => 2,
                    0x4d => 3,
                    0x4e => 4,
                    _ => 1,
                };

                let raw_tx = &hex[offset..];

                let dftx = bitcoin::consensus::deserialize::<dftx_rs::DfTx>(&raw_tx).unwrap();
                let ser = bitcoin::consensus::serialize::<dftx_rs::DfTx>(&dftx);
                assert_eq!(ser, raw_tx);
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(ConsensusEncoding)]
pub fn consensus_encoding_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields
                .named
                .into_iter()
                .map(|f| f.ident)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let field_names = fields.iter().filter_map(|f| f.as_ref()).collect::<Vec<_>>();

    let expanded = quote! {
        impl_consensus_encoding!(#name, #(#field_names),*);
    };

    TokenStream::from(expanded)
}
