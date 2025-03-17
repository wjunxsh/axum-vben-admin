use proc_macro2::{Ident, TokenStream, Span};
use quote::quote;
#[allow(unused_imports)]
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DataStruct, Fields, Lit, LitBool,
    LitStr, Meta, Type,
};
use crate::table::Table;
use quote::format_ident;

pub fn expand_sea(ident: Ident, data: Data, attrs: Vec<Attribute>) -> TokenStream {
    let table = Table::parse_struct_info(ident, data, attrs, "sea_orm");
    //println!("table: {:#?}", table);
    let model = format_ident!("{}", table.struct_name);
    let table_name_ident = Ident::new(&table.alias_table_name(), Span::call_site());//format_ident!("{}", table.struct_name());
    quote! {
        pub type #table_name_ident = #model;
    }
}
