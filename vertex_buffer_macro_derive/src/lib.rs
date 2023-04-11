use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Field};

#[proc_macro_derive(VertexBuffer)]
pub fn vertex_layout(struct_stream: TokenStream) -> TokenStream {
    let struct_ast: syn::DeriveInput = syn::parse(struct_stream).unwrap();

    let struct_name = struct_ast.ident;

    let struct_ast = match struct_ast.data {
        Data::Struct(strct) => strct,
        _ => panic!("Failed to derive 'VertexBuffer' for '{}'", struct_name),
    };

    let fields = struct_ast.fields;
    let field_sizes: Vec<_> = fields
        .iter()
        .map(|Field { ty, .. }| quote! {#ty::get_size()})
        .collect();

    let layout_declarations = fields.iter().enumerate().map(|(i, Field { ty, .. })| {
        let stride = field_sizes.iter();
        let offset = field_sizes.iter().take(i);
        let field_count = quote! {#ty::get_field_count()};

        quote! {
            glad_gl::gl::VertexAttribPointer(#i as u32,
                #field_count as i32,
                glad_gl::gl::FLOAT,
                glad_gl::gl::FALSE,
                (#(#stride)+*) as i32,
                (0 #(+#offset)*) as *const std::ffi::c_void);

            glad_gl::gl::EnableVertexAttribArray(#i as u32);
        }
    });

    quote! {
        impl VertexBuffer for #struct_name
        {
            fn declare_layout(){
                unsafe{
                    #(#layout_declarations)*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(VertexAttribute)]
pub fn derive_vertex_attribute(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let Data::Struct(ast_struct) = ast.data else{
        panic!("Cannot derive 'VertexAttribute', type '{}', must be a struct", stringify!(#name));
    };

    let field_count = ast_struct.fields.iter().count();
    let total_size = ast_struct.fields.iter().map(|Field { ty, .. }| {
        quote! {std::mem::size_of::<#ty>()}
    });

    quote! {
        impl VertexAttribute for #name{
            fn get_size()-> usize {
                #(#total_size)+*
            }

            fn get_field_count() -> usize {
                #field_count
            }
        }
    }
    .into()
}
