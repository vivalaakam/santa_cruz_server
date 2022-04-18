use std::collections::HashMap;

use prost_types::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, ServiceDescriptorProto,
};
use quote::__private::TokenStream;

use convert_case::{Case, Casing};

use crate::CodegenPackage;

pub fn enums(
    _service: &ServiceDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
    enums: &HashMap<&str, EnumDescriptorProto>,
    package: &CodegenPackage,
) -> Vec<TokenStream> {
    let mut results = vec![];

    if let Some(message) = messages.get(package.message) {
        for field in &message.field {
            if let Some(type_name) = &field.type_name {
                let enum_name = type_name
                    .split(".")
                    .collect::<Vec<_>>()
                    .last()
                    .unwrap()
                    .clone();

                if let Some(msg) = enums.get(enum_name) {
                    let enum_name = quote::format_ident!("{}", msg.name());

                    if msg.value.len() == 0 {
                        continue;
                    }

                    let EnumValueDescriptorProto { name, .. } = msg.value.first().unwrap().clone();

                    let default_value = name.unwrap();

                    let default_value_name = format!("{}", default_value.to_case(Case::Camel));
                    let default_value_key =
                        quote::format_ident!("{}", default_value.to_case(Case::Pascal));

                    let enum_values = &msg
                        .value
                        .to_vec()
                        .into_iter()
                        .map(|f| {
                            let EnumValueDescriptorProto { name, .. } = f.clone();
                            let name = name.unwrap();
                            let value_name = format!("{}", name.to_case(Case::Camel));
                            let value_key = quote::format_ident!("{}", name.to_case(Case::Pascal));

                            quote::quote! {
                                #value_name => #enum_name::#value_key,
                            }
                        })
                        .collect::<Vec<_>>();

                    let result = quote::quote! {
                        impl sqlx::Type<sqlx::Postgres> for #enum_name {
                            fn type_info() -> PgTypeInfo {
                                PgTypeInfo::with_oid(1043)
                            }
                        }

                        impl<'r, DB: Database> Decode<'r, DB> for #enum_name
                        where
                            &'r str: Decode<'r, DB>,
                        {
                            fn decode(
                                value: <DB as HasValueRef<'r>>::ValueRef,
                            ) -> Result<WorkoutStatus, Box<dyn Error + 'static + Send + Sync>> {
                                let result = match <&str as Decode<DB>>::decode(value).unwrap_or(#default_value_name) {
                                    #(#enum_values)*
                                    &_ => #enum_name::#default_value_key,
                                };

                                Ok(result)
                            }
                        }
                    };

                    results.push(result);
                }
            }
        }
    }

    results
}
