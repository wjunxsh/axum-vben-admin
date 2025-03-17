use serde::{Deserialize, Serialize};
use proc_macro2::{Ident, Span};
use std::collections::HashMap;
use heck::ToUpperCamelCase;
use quote::format_ident;
#[allow(unused_imports)]
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DataStruct, Fields, Lit, LitBool,
    LitStr, Meta, Type,
};
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Table {
    pub struct_name: String,
    pub attr: HashMap<String, String>,
    pub fileds: Vec<FieldInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct FieldInfo {
    pub filed: String,
    pub filed_type: String,
    pub is_opt: bool,
    pub serde_skip: bool,
    pub attr: HashMap<String, String>,
}

impl Table {
    pub fn alias_table_name(&self) -> String {
        let table_name = self.attr.get("table_name").unwrap_or(&self.struct_name);
        table_name.to_upper_camel_case()
    }
    pub fn parse_struct_info(ident: Ident, data: Data, attrs: Vec<Attribute>, idt: &str) -> Table {
        let mut ret = Table::default();
        ret.struct_name = ident.to_string();
        ret.attr = Table::parse_attrs_str(&attrs, idt);
        if let Data::Struct(DataStruct {
            fields: Fields::Named(named),
            ..
        }) = data
            {
                let fields = named.named.into_iter();//fields 迭代器
                for ele in fields.to_owned() {
                    //将字符拆机出来
                    let a: Ident = format_ident!("{}", ele.ident.unwrap());//获取字段名称
                    let mut tb_field = FieldInfo::default();//初始化字段信息
                    tb_field.filed = a.to_string();
                    tb_field.is_opt = true;
                    let serde_map = Table::parse_attrs_str(&ele.attrs, "serde");
                    if serde_map.contains_key("skip") {
                        tb_field.serde_skip = true;
                    }

                    //字段注解
                    let col_typ = match ele.ty {
                        Type::Path(p) => {
                            let f = p.path.segments.first().expect("col_type error").to_owned();
                            let idt = f.ident;
                            if idt == "Option" {
                                match f.arguments {
                                    syn::PathArguments::AngleBracketed(p) => {
                                        let args = p.args.first().expect("no args").to_owned();
                                        match args {
                                            syn::GenericArgument::Type(t) => match t {
                                                Type::Path(p) => {
                                                    let idt = p
                                                        .path
                                                        .to_owned()
                                                        .segments
                                                        .first()
                                                        .expect("err")
                                                        .to_owned()
                                                        .ident;
                                                    idt
                                                }
                                                _ => todo!(),
                                            },
                                            _ => todo!(),
                                        }
                                    }
                                    _ => todo!(),
                                }
                            } else {
                                tb_field.is_opt = false;
                                idt
                            }
                        }
                        _ => todo!(),
                    };
                    tb_field.filed_type = col_typ.to_string();
                    tb_field.attr = Table::parse_attrs_str(&ele.attrs, idt);
                    ret.fileds.push(tb_field);
                }
            }
            ret
    }
    //将属性转为kv
    pub fn parse_attrs_str(attrs: &Vec<Attribute>, idt: &str) -> HashMap<String, String> {
        let mut ret_map = HashMap::new();
        let parse_attrs = Table::parse_attrs(attrs, idt);
        for (k, v) in parse_attrs {
            ret_map.insert(k, Table::lit_str(&v));
        }
        ret_map
    }


    pub fn parse_attrs(attrs: &Vec<Attribute>, idt: &str) -> HashMap<String, Lit> {
        let mut ret_map = HashMap::new();
        attrs.iter().for_each(|attr| {
            if attr.path().get_ident().map(|i| i == idt) != Some(true) {
                return;
            }
            if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
                for meta in list.iter() {
                    match meta {
                        Meta::Path(path) => match path.get_ident() {
                            Some(ident) => {
                                let v = Lit::Bool(LitBool::new(true, Span::call_site()));
                                ret_map.insert(ident.to_string(), v);
                            }
                            None => {
                                //println!("path={path:#?}");
                            }
                        },
                        Meta::List(l) => {
                            println!("metalist={l:#?}");
                        }
                        Meta::NameValue(nv) => match nv.path.get_ident() {
                            Some(ident) => match &nv.value {
                                syn::Expr::Lit(lit) => {
                                    ret_map.insert(ident.to_string(), lit.lit.to_owned());
                                }
                                syn::Expr::Path(lit) => {
                                    if let Some(v) = lit.path.get_ident() {
                                        let v = v.to_string();
                                        ret_map.insert(
                                            ident.to_string(),
                                            Lit::Str(LitStr::new(&v, Span::call_site())),
                                        );
                                    }
                                }
                                _ => {
                                    //println!("ident={ident},{:?}", &nv.value);
                                }
                            },
                            None => {
                                //println!("NameValue={nv:#?}");
                            }
                        },
                    }
                }
            }
        });
        ret_map
    }


    pub fn lit_str(v: &Lit) -> String {
        Table::lit_str_def(v, "")
    }

    pub fn lit_str_def(v: &Lit, df: &str) -> String {
        match v {
            Lit::Str(c) => c.token().to_string().replace("\"", ""),
            Lit::Bool(c) => c.token().to_string(),
            Lit::Int(c) => c.token().to_string(),
            _ => {
                let ret = format!("{v:?}");
                if ret.is_empty() {
                    df.to_owned()
                } else {
                    ret
                }
            }
        }
    }
}


