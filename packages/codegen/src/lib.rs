use std::{fs};
use std::fmt::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

use prost::Message;
use prost_types::FileDescriptorSet;
use quote;

#[derive(Copy, Clone)]
pub struct CodegenPackage {
    pub service: &'static str,
    pub message: &'static str,
    pub table: &'static str,
}

#[derive(Default)]
pub struct Codegen {
    proto_descriptior: PathBuf,
    packages: Vec<CodegenPackage>,
}

impl Codegen {
    pub fn new(proto_descriptior: impl AsRef<Path>) -> Self {
        Codegen {
            proto_descriptior: proto_descriptior.as_ref().to_path_buf(),
            ..Codegen::default()
        }
    }

    pub fn add(&mut self, package: CodegenPackage) {
        self.packages.push(package);
    }

    pub fn build(&self, target: impl AsRef<Path>) -> Result<(), Error> {
        let buf = fs::read(&self.proto_descriptior).unwrap();
        let file_descriptor_set = FileDescriptorSet::decode(&*buf).unwrap();

        let mut results = vec![];

        results.push(format!(
            "{}",
            quote::quote! {
                use chrono::{DateTime, Utc};

                use sqlx::Row;
                use sqlx::postgres::PgRow;

                use crate::Queryable;
            }
        ));

        let messages_keys = &self
            .packages
            .to_vec()
            .into_iter()
            .map(|p| p.message)
            .collect::<Vec<_>>();

        for f in &file_descriptor_set.file {
            for m in &f.message_type {
                let key = m.name.as_ref().unwrap();
                if messages_keys.contains(&key.as_str()) {
                    let message_name = quote::format_ident!("{}", m.name());

                    let fields = &m
                        .field
                        .clone()
                        .into_iter()
                        .map(|field| {
                            let name = quote::format_ident!("{}", field.name());
                            let formatted = format!("{}", field.name());
                            let data_type = match field.name() {
                                "id" => quote::quote! { i32 },
                                "created_at" | "updated_at" => quote::quote! { DateTime<Utc> },
                                _ => match field.r#type.unwrap() {
                                    9 => quote::quote! { String },
                                    _ => quote::quote! { unknown },
                                },
                            };

                            let into = match field.name() {
                                "created_at" | "updated_at" => quote::quote! { .to_rfc3339() },
                                _ => quote::quote! {},
                            };

                            quote::quote! { #name: row.get::<#data_type, _>(#formatted)#into, }
                        })
                        .collect::<Vec<_>>();

                    let result = quote::quote! {
                        use crate::proto::proto::santa_cruz::#message_name;

                        impl From<PgRow> for #message_name {
                            fn from(row: PgRow) -> Self {
                                #message_name {
                                    #(#fields)*
                                }
                            }
                        }
                    };

                    results.push(format!("{}", result));

                    let queryable_fields = &m
                        .field
                        .clone()
                        .into_iter()
                        .map(|field| format!("{}", field.name()))
                        .collect::<Vec<_>>();

                    let result = quote::quote! {
                        impl Queryable for #message_name {
                            fn fields() -> Vec<&'static str> {
                               vec![ #(#queryable_fields ,)* ]
                            }
                        }
                    };

                    results.push(format!("{}", result));
                }
            }
        }

        let output_path = target.as_ref().join("services.rs");

        fs::write(&output_path, results.join("\n")).unwrap();

        Command::new("rustfmt")
            .arg("--")
            .arg(&output_path.as_path())
            .output()
            .expect("failed to execute process");

        Ok(())
    }
}
