use std::env;

use santa_cruz_codegen::{Codegen, CodegenPackage};

fn main() {
    let mut builder = Codegen::new(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    builder.add(CodegenPackage {
        service: "ExerciseService",
        message: "Exercise",
        table: "exercises",
        list: Some("GetExercises"),
        get: Some("GetExercise"),
        create: Some("CreateExercise"),
        update: Some("UpdateExercise"),
        delete: Some("DeleteExercise"),
        ..CodegenPackage::default()
    });

    let _ = builder.build("src");
}
