extern crate proc_macro;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Unit)]
pub fn unit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => &fields.unnamed,
        _ => panic!("expected a struct with named fields"),
    };
    let field_type = fields.iter().map(|field| &field.ty);
    let field_type_h = field_type.clone();
    let field_type_d = field_type.clone();
    let field_type_m = field_type.clone();
    let field_type_mu = field_type.clone();
    let field_type_n = field_type.clone();
    let field_type_c = field_type.clone();

    let field_vis = fields.iter().map(|field| &field.vis);
    let field_vis_h = fields.iter().map(|field| &field.vis);
    let field_vis_m = field_vis.clone();
    let field_vis_d = field_vis.clone();
    let field_vis_mu = field_vis.clone();
    let field_vis_n = field_vis.clone();
    let field_vis_c = field_vis.clone();

    let struct_name = &input.ident;
    let mili = Ident::new(&format!("M{}", struct_name), Span::call_site());
    let nano = Ident::new(&format!("N{}", struct_name), Span::call_site());
    let micro = Ident::new(&format!("Mu{}", struct_name), Span::call_site());
    let kilo = Ident::new(&format!("K{}", struct_name), Span::call_site());
    let deci = Ident::new(&format!("Deci{}", struct_name), Span::call_site());
    let hecta = Ident::new(&format!("H{}", struct_name), Span::call_site());
    let centi = Ident::new(&format!("C{}", struct_name), Span::call_site());

    let output: TokenStream = quote! {
    // Preserve the input struct unchanged in the output.
    impl Unit for #struct_name {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self {
            self
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            base
        }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub struct #mili (
            #(
                #field_vis_m #field_type_m
            )*
    );
    impl Unit for #mili {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self::BaseUnit {
            #struct_name (self.0 / 1000.)
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            Self(base.0 * 1000.)
        }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub struct #nano (
            #(
                #field_vis_n #field_type_n
            )*
    );
    impl Unit for #nano {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self::BaseUnit {
            #struct_name (self.0 / 1E9)
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            Self(base.0 * 1E9)
        }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub struct #micro (
            #(
                #field_vis_mu #field_type_mu
            )*
    );
    impl Unit for #micro {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self::BaseUnit {
            #struct_name (self.0 / 1E6)
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            Self(base.0 * 1E6) }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub struct #kilo (
            #(
                #field_vis #field_type
            )*
    );
    impl Unit for #kilo {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self::BaseUnit {
            #struct_name (self.0 * 100.)
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            Self(base.0 * 1E-3)
        }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub struct #deci (
            #(
                #field_vis_d #field_type_d
            )*
    );
    impl Unit for #deci {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self::BaseUnit {
            #struct_name (self.0 * 0.1)
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            Self(base.0 * 10.)
        }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub struct #centi (
            #(
                #field_vis_c #field_type_c
            )*
    );
    impl Unit for #centi {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self::BaseUnit {
            #struct_name (self.0 / 100.)
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            Self(base.0 * 100.)
        }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub struct #hecta (
            #(
                #field_vis_h #field_type_h
            )*
    );
    impl Unit for #hecta {
        type BaseUnit = #struct_name;
        fn to_base(self) -> Self::BaseUnit {
            #struct_name (self.0 * 100.)
        }
        fn from_base(base: Self::BaseUnit) -> Self {
            Self(base.0 * 1E-2)
        }
    }

    };
    output.into()
}
