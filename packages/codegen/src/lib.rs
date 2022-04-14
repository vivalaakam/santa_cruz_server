use std::collections::HashMap;
use std::fmt::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use prost::Message;
use prost_types::FileDescriptorSet;
use quote;

mod from_pg_row;
mod naive_snake_case;
mod queryable;

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
                use crate::query_builder::QueryBuilder;
            }
        ));

        let mut messages_keys = HashMap::new();

        for package in &self.packages {
            messages_keys.insert(package.message, package);
        }

        for f in &file_descriptor_set.file {
            for m in &f.message_type {
                let key = m.name.as_ref().unwrap();
                if messages_keys.contains_key(&key.as_str()) {
                    let package = *messages_keys.get(&key.as_str()).expect("oops");
                    let mod_name =
                        quote::format_ident!("{}", naive_snake_case::naive_snake_case(m.name()));
                    let message_name = quote::format_ident!("{}", m.name());

                    let from_pg_row_tokens = from_pg_row::from_pg_row(&m);
                    let queryable_tokens = queryable::queryable(&m, package);

                    let result = quote::quote! {
                        pub mod #mod_name {
                            use super::*;
                            use crate::proto::proto::santa_cruz::#message_name;

                            #from_pg_row_tokens

                            #queryable_tokens
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
