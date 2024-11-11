/// The `Optionalize` macro generates a new struct with optional fields.
///
/// For any struct that derives `Optionalize`, the macro will generate a new struct
/// with the same name appended by `Optional`. Each field in the original struct
/// is transformed:
///
/// - If a field is of type `Option<T>`, it remains `Option<T>`.
/// - If a field is of type `T`, it becomes `Option<T>`.
///
/// # Example
///
/// ```rust
/// use optionalize_macro::Optionalize;
///
/// #[derive(Optionalize)]
/// pub struct MyStruct {
///     pub id: i32,
///     pub name: String,
///     pub description: Option<String>,
/// }
///
/// // The generated struct will look like:
/// let test = MyStructOptional {
///     id: Some(1i32),
///     name: Some("Name".to_string()),
///     description: Some("Test Description".to_string())
/// };
/// ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Type, TypePath};

#[proc_macro_derive(Optionalize)]
pub fn derive_optionalize(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct name
    let struct_name = input.ident.clone();

    // Generate a new name for the "optionalized" struct
    let optional_struct_name = syn::Ident::new(&format!("{}Optional", struct_name), struct_name.span());

    // Build the fields for the new struct
    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        // Only work with structs
        return syn::Error::new_spanned(input, "Optionalize can only be used on structs")
            .to_compile_error()
            .into();
    };

    // Create fields with Option types
    let optional_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        // Check if the field is already an Option<T>
        if let Type::Path(TypePath { path, .. }) = field_type {
            if path.segments.last().map(|s| s.ident == "Option").unwrap_or(false) {
                // Field is already an Option<T>, keep it as is
                quote! { #field_name: #field_type }
            } else {
                // Wrap the field type in Option<T>
                quote! { #field_name: Option<#field_type> }
            }
        } else {
            // Wrap non-path types (like tuples) in Option<T>
            quote! { #field_name: Option<#field_type> }
        }
    });

    // Generate the output tokens
    let expanded = quote! {
        // Define the new struct with optionalized fields
        pub struct #optional_struct_name {
            #( #optional_fields, )*
        }
    };

    TokenStream::from(expanded)
}

