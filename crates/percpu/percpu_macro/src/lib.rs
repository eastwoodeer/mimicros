mod arch;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};

#[proc_macro_attribute]
pub fn define_per_cpu(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(Span::call_site(), "no attribute needed.")
            .to_compile_error()
            .into();
    }

    let ast = syn::parse_macro_input!(item as syn::ItemStatic);
    let name = &ast.ident;
    let ty = &ast.ty;
    let attrs = &ast.attrs;
    let expr = &ast.expr;
    let visibility = &ast.vis;

    let inner_symbol_name = &format_ident!("__PER_CPU_{name}");
    let inner_struct_name = &format_ident!("__PER_CPU_STRUCT_{name}");
    let ty_str = quote!(#ty).to_string();
    let is_primitive_int = ["bool", "u8", "u16", "u32", "u64", "usize"].contains(&ty_str.as_str());

    let current_ptr_method = arch::gen_current_ptr(inner_symbol_name, ty);
    let offset_method = arch::gen_offset(inner_symbol_name);

    let read_write_method = if is_primitive_int {
        let read_method = arch::gen_read_current_raw(inner_symbol_name, ty);
        let write_method = arch::gen_write_current_raw(inner_symbol_name, ty, &format_ident!("v"));

        quote! {
            #[inline]
            pub unsafe fn read_current_raw(&self) -> #ty {
                #read_method
            }

            #[inline]
            pub unsafe fn write_current_raw(&self, v: #ty) {
                #write_method
            }

            pub fn read_current(&self) -> #ty {
                let _guard = kernel_guard::PreemptGuard::new();
                unsafe { self.read_current_raw() }
            }

            pub fn write_current(&self, v: #ty) {
                let _guard = kernel_guard::PreemptGuard::new();
                unsafe { self.write_current_raw(v) }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #[link_section = ".per_cpu"]
        #(#attrs)*
        static mut #inner_symbol_name: #ty = #expr;

        #visibility struct #inner_struct_name {}

        #(#attrs)*
        #visibility static #name: #inner_struct_name = #inner_struct_name {};

        impl #inner_struct_name {
            #[inline]
            pub fn offset(&self) -> usize {
                #offset_method
            }

            #[inline]
            pub fn current_ptr(&self) -> *const #ty {
                #current_ptr_method
            }

            #[inline]
            pub unsafe fn current_ref_raw(&self) -> &#ty {
                &*self.current_ptr()
            }

            #[inline]
            pub unsafe fn current_mut_raw(&self) -> &mut #ty {
                &mut *(self.current_ptr() as *mut #ty)
            }

            #read_write_method
        }
    }
    .into()
}

#[proc_macro]
pub fn percpu_symbol_offset(item: TokenStream) -> TokenStream {
    let symbol = format_ident!("{}", item.to_string());
    let offset = arch::gen_offset(&symbol);
    quote!(#offset).into()
}
