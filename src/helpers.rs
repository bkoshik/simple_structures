use syn::{Data, DataStruct, Field, Fields};
use syn::punctuated::Punctuated;
use syn::token::Comma;

pub fn get_fields(data: &Data) -> Punctuated<Field, Comma> {
    match data {
        Data::Struct(DataStruct { fields: Fields::Named(named), .. }) =>
            named.named.clone(),
        _ => unimplemented!("Only works for structs"),
    }
}
