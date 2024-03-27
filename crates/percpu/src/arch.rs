use quote::{format_ident, quote};
use syn::{Ident, Type};

pub fn gen_offset(symbol: &Ident) -> proc_macro2::TokenStream {
    quote! {
        let offset: usize;

        core::arch::asm!("movz {0}, :abs_g0_nc:{VAR}", out(reg) offset, VAR = sym #symbol);

        offset
    }
}

pub fn gen_current_ptr(symbol: &Ident, ty: &Type) -> proc_macro2::TokenStream {
    quote! {
        let base: usize;
        core::arch::asm!("mrs {}, TPIDR_EL1", out(reg) base);

        (base + self.offset(symbol)) as *const #ty
    }
}

pub fn gen_read_current_raw(symbol: &Ident, ty: &Type) -> proc_macro2::TokenStream {
    quote! {
        *self.current_ptr()
    }
}

pub fn gen_write_current_raw(symbol: &Ident, ty: &Type, v: &Ident) -> proc_macro2::TokenStream {
    quote! {
        *(self.current_ptr() as *mut #ty) = #v;
    }
}
