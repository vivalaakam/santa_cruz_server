use crate::naive_snake_case::naive_snake_case;
use prost_types::{DescriptorProto, MethodDescriptorProto};
use quote::__private::TokenStream;
use std::collections::HashMap;

pub fn proto_request_name(
    action: &MethodDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
) -> TokenStream {
    let snake = naive_snake_case(action.name());
    let service_name = quote::format_ident!("{}", snake);

    let MethodDescriptorProto {
        input_type,
        output_type,
        ..
    } = action.clone();

    let input_type = input_type.unwrap();
    let output_type = output_type.unwrap();

    let req = messages
        .get(input_type.split(".").collect::<Vec<_>>().last().unwrap())
        .expect("input message not found");

    let res = messages
        .get(output_type.split(".").collect::<Vec<_>>().last().unwrap())
        .expect("output not found");

    let req_name = quote::format_ident!("{}", req.name());
    let res_name = quote::format_ident!("{}", res.name());

    quote::quote! {
        #service_name( &self,request: Request<#req_name>) -> Result<Response<#res_name>, Status>
    }
}
