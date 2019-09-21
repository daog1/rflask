extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use core::convert::AsRef;
use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::ItemFn;
use syn::{ImplItem, ItemImpl};
/// Creates route handler with `GET` method guard.
///
/// Syntax: `#[route(GET,"/")]`
///
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut path = None;
    let mut reqtype = None;
    for arg in args {
        match arg {
            syn::NestedMeta::Lit(syn::Lit::Str(ref fname)) => {
                let fname = quote!(#fname).to_string();
                path = Some(fname.as_str()[1..fname.len() - 1].to_owned());
            }
            syn::NestedMeta::Meta(syn::Meta::Path(ident)) => {
                let identstr = quote!(#ident).to_string();
                reqtype = Some(identstr);
            }
            _ => {
                println!("error");
            }
        }
    }
    let psitem = parse_macro_input!(item as ItemFn);
    let name = psitem.sig.ident.clone();
    let classname = syn::Ident::new(&format!("{}{}", "class_", name), psitem.sig.ident.span());
    let rtypestr = reqtype.expect("error");
    let callprefix = format_ident!("web");
    #[allow(unused_assignments)]
    let mut method = None;
    match rtypestr.as_ref() {
        "GET" => method = Some(format_ident!("get")),
        "POST" => method = Some(format_ident!("post")),
        "PUT" => method = Some(format_ident!("put")),
        "PATCH" => method = Some(format_ident!("patch")),
        "DELETE" => method = Some(format_ident!("delete")),
        "HEAD" => method = Some(format_ident!("head")),
        _ => method = Some(format_ident!("")),
    }
    let func = quote! {
        #[allow(non_camel_case_types)]
        pub struct #classname;
        #psitem
        inventory::submit!(Box::new(#classname) as Box<dyn ServiceFactory>);
        impl ServiceFactory for #classname {
        fn register(&self, config: &mut web::ServiceConfig) {
            config.route(#path, #callprefix::#method().to(#name));
        }
    }
        };
    func.into()
}

#[proc_macro_attribute]
pub fn route_res(attr: TokenStream, item: TokenStream) -> TokenStream {
    //item
    let mut itimpl = parse_macro_input!(item as ItemImpl);
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut path = None;
    let mut reqtype = None;
    for arg in args {
        match arg {
            syn::NestedMeta::Lit(syn::Lit::Str(ref fname)) => {
                let fname = quote!(#fname).to_string();
                path = Some(fname.as_str()[1..fname.len() - 1].to_owned());
            }
            syn::NestedMeta::Meta(syn::Meta::Path(ident)) => {
                let identstr = quote!(#ident).to_string();
                reqtype = Some(identstr);
            }
            _ => {
                println!("error");
            }
        }
    }
    let self_ty = &itimpl.self_ty;
    let mut nameident = quote! {#self_ty};
    println!("ident {:?}", nameident.to_string());
    let classname = syn::Ident::new(&format!("{}{}", "class_", nameident), Span::call_site());
    println!("classname ident {:?}", classname.to_string());
    let callprefix = format_ident!("web");
    let method = format_ident!("get");
    let mut methods = vec![];
    for arg in itimpl.items.iter() {
        //println!("{:?}", arg);
        match arg {
            ImplItem::Method(m) => {
                //println!("{:?}", m.sig.ident.to_string());
                methods.push(m.sig.ident.to_string())
            }
            ImplItem::Type(t) => {
                println!("ImplItem::Type {:?}", t.ident.to_string());
            }
            ImplItem::Const(t) => {
                println!("ImplItem::Type {:?}", t.ident.to_string());
            }
            _ => {}
        }
    }
    println!("impl:{:?}", methods);
    let mut methodscode = vec![];
    for method in methods{
        let methodident = format_ident!("{}",method.to_string());
        let regcode = quote!{
            config.route(#path, #callprefix::#methodident().to(#nameident::#methodident));
        };
        methodscode.push(regcode);
    }

    let mut func = quote! {
        pub struct #classname;
        /*impl FromRequest for #nameident {
            type Error = Error;
            type Future = Result<Self, Self::Error>;
            type Config = ();
            fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
                return Ok(#nameident {
                });
            }
        }*/
        #itimpl
        inventory::submit!(Box::new(#classname) as Box<dyn ServiceFactory>);
        //inventory::submit!(Box::new(#classname) as Box<dyn ServiceFactory>);
        impl ServiceFactory for #classname {
            fn register(&self, config: &mut web::ServiceConfig) {
                #(#methodscode)*
                
            }
        }
    };
    println!("gen:{}", func.to_string());
    func.into()
}
