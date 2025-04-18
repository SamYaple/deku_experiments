use anyhow::Result;
use pci_ids::{Class, PciIds, SubClass};
use proc_macro2::TokenStream;
use quote::quote;
use std::{env, fs, path::Path};
use syn::{Ident, LitInt};

fn main() -> Result<()> {
    // 1) Load and parse your pci.ids into PciIds
    let pci_ids = pci_ids::load_from_file().expect("failed to load pci.ids");

    // 2) Generate tokens
    let tokens = generate_tokens(&pci_ids);

    // 3) Write out
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("pci_generated.rs");
    fs::write(&dest, tokens.to_string())?;
    Ok(())
}

fn generate_tokens(pci: &PciIds) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
    });

    // 1) all ProgIf enums
    for class in &pci.classes {
        for sc in &class.subclasses {
            if !sc.prog_ifs.is_empty() {
                ts.extend(gen_progif_enum(class, sc));
            }
        }
    }

    // 2) all SubClass enums
    for class in &pci.classes {
        if !class.subclasses.is_empty() {
            ts.extend(gen_subclass_enum(class));
        }
    }

    // 3) the top‑level PciDeviceClass
    ts.extend(gen_class_enum(pci));

    ts
}

fn gen_progif_enum(class: &Class, sc: &SubClass) -> TokenStream {
    let name = format!("{}{}ProgIf", sanitize(&class.name), sanitize(&sc.name));
    let ident = Ident::new(&name, proc_macro2::Span::call_site());

    let variants = sc.prog_ifs.iter().map(|pif| {
        let mut v = sanitize(&pif.name);
        if v.chars().next().unwrap().is_numeric() {
            v = "_".to_owned() + &v;
        }

        let vid = Ident::new(&v, proc_macro2::Span::call_site());
        let lit = LitInt::new(&format!("0x{:02X}", pif.id), proc_macro2::Span::call_site());
        quote! {
            #[deku(id = #lit)]
            #vid
        }
    });

    quote! {
        #[derive(Debug, DekuRead, DekuWrite)]
        #[deku(id = "prog_if", ctx = "prog_if: u8")]
        pub enum #ident {
            #(#variants,)*
        }
    }
}

fn gen_subclass_enum(class: &Class) -> TokenStream {
    // 1) don’t emit an empty enum
    if class.subclasses.is_empty() {
        return TokenStream::new();
    }

    // 2) do any of the subclasses have ProgIFs?
    let has_progif = class.subclasses.iter().any(|sc| !sc.prog_ifs.is_empty());

    // 3) pick the right ctx attribute
    let ctx = if has_progif {
        "subclass: u8, prog_if: u8"
    } else {
        "subclass: u8"
    };

    let name = format!("{}Subtype", sanitize(&class.name));
    let ident = Ident::new(&name, proc_macro2::Span::call_site());

    // 4) build each variant
    let variants = class.subclasses.iter().map(|sc| {
        // sanitize and avoid a leading digit
        let mut v = sanitize(&sc.name);
        if v.chars().next().unwrap().is_numeric() {
            v = "_".to_string() + &v;
        }
        let vid = Ident::new(&v, proc_macro2::Span::call_site());
        let lit = LitInt::new(&format!("0x{:02X}", sc.id), proc_macro2::Span::call_site());

        if sc.prog_ifs.is_empty() {
            // plain unit variant
            quote! {
                #[deku(id = #lit)]
                #vid
            }
        } else {
            // nested ProgIf enum
            let progif_name = format!("{}{}ProgIf", sanitize(&class.name), sanitize(&sc.name));
            let progif_ident = Ident::new(&progif_name, proc_macro2::Span::call_site());
            quote! {
                #[deku(id = #lit)]
                #vid(#[deku(ctx = "prog_if")] #progif_ident)
            }
        }
    });

    quote! {
        #[derive(Debug, DekuRead, DekuWrite)]
        #[deku(id = "subclass", ctx = #ctx)]
        pub enum #ident {
            #(#variants,)*
        }
    }
}

fn gen_class_enum(pci: &PciIds) -> TokenStream {
    let variants = pci.classes.iter().map(|class| {
        let lit = LitInt::new(
            &format!("0x{:02X}", class.id),
            proc_macro2::Span::call_site(),
        );
        let vid = Ident::new(&sanitize(&class.name), proc_macro2::Span::call_site());

        let has_subclasses = !class.subclasses.is_empty();
        let has_progif = class.subclasses.iter().any(|sc| !sc.prog_ifs.is_empty());

        // pre‐compute the Subtype ident in case we need it
        let subtype_name = format!("{}Subtype", sanitize(&class.name));
        let subtype_ident = Ident::new(&subtype_name, proc_macro2::Span::call_site());

        if has_progif {
            // 1) subclass+prog_if
            quote! {
                #[deku(id = #lit)]
                #vid(#[deku(ctx = "subclass, prog_if")] #subtype_ident)
            }
        } else if has_subclasses {
            // 2) subclass only
            quote! {
                #[deku(id = #lit)]
                #vid(#[deku(ctx = "subclass")] #subtype_ident)
            }
        } else {
            // 3) no subclass
            quote! {
                #[deku(id = #lit)]
                #vid
            }
        }
    });

    quote! {
        #[derive(Debug, DekuRead, DekuWrite)]
        #[deku(id = "class_code", ctx = "class_code: u8, subclass: u8, prog_if: u8")]
        pub enum PciDeviceClass {
            #(#variants,)*
        }
        impl Default for PciDeviceClass {
            fn default() -> Self {
                PciDeviceClass::UnassignedClass
            }
        }
    }
}

/// Turn “Mass Storage” or “non-volatile” → CamelCase Rust identifiers
fn sanitize(input: &str) -> String {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            let first = chars.next().unwrap();
            first.to_uppercase().collect::<String>() + chars.as_str()
        })
        .collect()
}
