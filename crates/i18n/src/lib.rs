extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use serde_json::Value;
use std::fs::read_to_string;
use std::path::PathBuf;

fn get_locale(locale: &str) -> Result<Value, anyhow::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("locale");
    path.push(format!("{locale}.json"));

    let contents = read_to_string(path)?;

    Ok(serde_json::from_str::<Value>(&contents)?)
}

#[proc_macro]
pub fn t(input: TokenStream) -> TokenStream {
    let mut args = Vec::new();

    for i in input {
        if let TokenTree::Literal(s) = i {
            args.push(s.to_string())
        } else {
            continue;
        }
    }

    let key = args
        .get(0)
        .expect("Expected at least one argument")
        .trim_matches('"');
    let locale = match args.get(1) {
        Some(s) => s.trim_matches('"'),
        None => "en",
    };

    let mut locale_json =
        get_locale(locale).unwrap_or_else(|_| panic!("Could not get locale {locale}"));
    for part in key.split('.') {
        locale_json = locale_json[part].clone();
    }
    let value = match locale_json {
        Value::String(s) => s,
        Value::Null => {
            println!("Warning: missing locale key \"{key}\" in {locale}");
            key.to_owned()
        }
        v => v.to_string(),
    };

    quote!(
        #value
    )
    .into()
}
