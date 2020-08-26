extern crate proc_macro;

use proc_macro::TokenStream;

use quote::format_ident;
use quote::quote;

#[proc_macro_derive(UGenMacro)]
pub fn ugen_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_ugen_macro(&ast)
}

fn impl_ugen_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_lowercase = name.to_string().to_lowercase();
    let create_fn = format_ident!("sp_{}_create", name_lowercase);
    let init_fn = format_ident!("sp_{}_init", name_lowercase);
    let destroy_fn = format_ident!("sp_{}_destroy", name_lowercase);

    let gen = quote! {
        impl #name {
            pub fn new(sp: Soundpipe) -> Self {
                println!("Create {}", stringify!(#name));
                let mut result = Self {
                    sp,
                    ffi: std::ptr::null_mut(),
                };
                unsafe {
                    #create_fn(&mut result.ffi);
                    #init_fn(*result.sp.sp_ffi, result.ffi);
                }
                result
            }
        }

        unsafe impl Send for #name {}

        impl crate::ugens::ugen::UGen for #name {}

        impl Drop for #name {
             fn drop(&mut self) {
                unsafe {
                    println!("Drop {}", stringify!(#name));
                    #destroy_fn(&mut self.ffi);
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(OscillatorMacro)]
pub fn oscillator_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_oscillator_macro(&ast)
}

fn impl_oscillator_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_lowercase = name.to_string().to_lowercase();
    let compute_fn = format_ident!("sp_{}_compute", name_lowercase);

    let gen = quote! {
        impl crate::ugens::oscillators::common::MonoOscInternal for #name {
                fn compute_internal(&self) -> f32 {
                    let mut out: f32 = 0.0;
                    let null = null_mut();
                    unsafe {
                        #compute_fn(*self.sp.sp_ffi, self.ffi, null, &mut out);
                    }
                    out
                }
        }
    };
    gen.into()
}
