use std::{env};


use santa_cruz_codegen::{Codegen, CodegenPackage};

fn main() {
    let mut builder = Codegen::new(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    builder.add(CodegenPackage {
        service: "",
        message: "Exercise",
        table: "exercises",
    });

   let _ = builder.build("src");
}
