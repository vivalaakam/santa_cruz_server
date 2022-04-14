use prost_types::DescriptorProto;
use quote::__private::TokenStream;
use crate::CodegenPackage;
use crate::naive_snake_case::naive_snake_case;

pub fn service(message: &DescriptorProto, _package: &CodegenPackage) -> TokenStream {
    let snake = naive_snake_case(message.name());
    let message_name = quote::format_ident!("{}", message.name());
    let service_name = quote::format_ident!("{}Service", message.name());
    let get_by_id = quote::format_ident!("get_{}_by_id", snake);
    let return_by_id = quote::format_ident!("return_{}_by_id", snake);

    quote::quote! {
        pub struct #service_name {
            pool: PgPool,
        }

        impl #service_name {
            pub fn new(pool: &PgPool) -> Self {
                #service_name { pool: pool.clone() }
            }

            pub async fn #get_by_id (
                pool: &PgPool,
                id: i32,
                user_id: i32,
            ) -> Option<#message_name> {
                let mut query_builder = #message_name::query();
                query_builder.where_raw("((permissions ->> CAST(${index} as text))::integer > 0 OR (permissions ->> '0')::integer > 0)", user_id);
                query_builder.where_eq("id", id);

                let sql = query_builder.select_query();

                sqlx::query_with(sql.0.as_str(), sql.1)
                    .fetch_one(pool)
                    .await
                    .map(|r| r.into())
                    .ok()
            }

            pub async fn #return_by_id(
                &self,
                id: i32,
                user_id: i32,
            ) -> Result<Response<#message_name>, Status> {
                #service_name::#get_by_id(&self.pool, id, user_id)
                    .await
                    .map(|reply| Response::new(reply))
                    .ok_or(Status::not_found(format!(
                        "object #{} not found",
                        id.to_string()
                    )))
            }
        }
    }
}
