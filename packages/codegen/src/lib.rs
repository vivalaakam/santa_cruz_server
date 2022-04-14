use std::collections::HashMap;
use std::fmt::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use prost::Message;
use prost_types::{DescriptorProto, FileDescriptorSet, ServiceDescriptorProto};
use proto_service::messages::proto_service_messages;
use quote;

mod from_pg_row;
mod naive_snake_case;
mod proto_request_name;
mod proto_request_params;
mod proto_service;
mod queryable;
mod service;

#[derive(Copy, Clone, Default)]
pub struct CodegenPackage {
    pub service: &'static str,
    pub message: &'static str,
    pub table: &'static str,
    pub list: Option<&'static str>,
    pub get: Option<&'static str>,
    pub create: Option<&'static str>,
    pub update: Option<&'static str>,
    pub delete: Option<&'static str>,
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
                use std::collections::HashMap;

                use chrono::{DateTime, Utc};

                use sqlx::{PgPool, Row};
                use sqlx::postgres::PgRow;
                use sqlx::types::Json;

                use tonic::{Request, Response, Status};

                use crate::proto::proto;
                use crate::Queryable;
                use crate::query_builder::QueryBuilder;
                use crate::me_extension::MeExtension;
            }
        ));

        let mut messages: HashMap<&str, DescriptorProto> = HashMap::new();
        let mut services: HashMap<&str, ServiceDescriptorProto> = HashMap::new();

        for f in &file_descriptor_set.file {
            for s in &f.service {
                services.insert(s.name(), s.clone());
            }

            for m in &f.message_type {
                messages.insert(m.name(), m.clone());
            }
        }

        for package in &self.packages {
            let message = messages.get(package.message).unwrap();
            let service = services.get(package.service).unwrap();

            let mod_name =
                quote::format_ident!("{}", naive_snake_case::naive_snake_case(message.name()));

            let from_pg_row_tokens = from_pg_row::from_pg_row(&message);
            let queryable_tokens = queryable::queryable(&message, package);
            let service_tokens = service::service(&message, package);

            let proto_service_tokens = proto_service::proto_service(&service, &messages, package);

            let message_names = proto_service_messages(&service, &messages, package)
                .into_iter()
                .collect::<Vec<_>>();

            let result = quote::quote! {
                pub mod #mod_name {
                    use super::*;
                    use crate::proto::proto::santa_cruz::{
                        #(#message_names ,)*
                    };

                    #from_pg_row_tokens

                    #queryable_tokens

                    #service_tokens

                    #proto_service_tokens
                }
            };

            results.push(format!("{}", result));
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
