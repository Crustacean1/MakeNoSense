/* this is shit, but here we go*/

use core::fmt;
use std::fs;

use crate::editor::{image_selection::LayerInfo, ui_layer::UiLayer};

pub struct ImageInfo {
    pub filename: String,
    pub resolution: (u32, u32),
}

pub fn export_to_json(
    filename: &str,
    image_info: &ImageInfo,
    vertices: &[(f32, f32)],
    layer_types: &[LayerInfo],
    layers: &[UiLayer],
) {
    let mut result = String::new();
    result.push_str("{");
    save_meta_info(&mut result);
    save_image_info(image_info, &mut result);
    save_image_annotations(layers, vertices, &mut result);

    result.push_str("}");

    save_string_to_file(filename, &result);
}

fn save_string_to_file(filename: &str, content: &str) {
    match fs::write(filename, content) {
        Ok(_) => {
            println!("File saved successfully");
        }
        Err(e) => {
            println!("I fucked up: {}", e)
        }
    }
}

fn save_meta_info(output: &mut String) {
    let raw = [&mut String::new()];
    add_property("description", "\"project-name\"", raw[0]);
    output.push_str(&wrap_in_obj("info", raw[0]));
}

fn save_image_info(image_info: &ImageInfo, output: &mut String) {
    let mut raw = String::new();
    raw.push_str("{");
    add_property("id", "1", &mut raw);
    add_property("width", image_info.resolution.0, &mut raw);
    add_property("height", image_info.resolution.1, &mut raw);
    add_property(
        "file_name",
        format!("\"{}\"", image_info.filename),
        &mut raw,
    );
    raw.push_str("}");
    let arr: [&str; 1] = [&raw];

    output.push_str(&wrap_in_arr("images", &arr));
}

fn save_image_annotations(
    id: &mut usize,
    image_id: usize,
    layers: &[UiLayer],
    vertices: &[(f32, f32)],
    output: &mut String,
) {
    for layer in layers {
        output.push_str("{");
        add_property("id", id, output);
        add_property("iscrowd", 0, output);
        add_property("image_id", 0, output);
        add_property("category_id", layer.layer_info().id(), output);
        output.push_str("\"segmentation\" : [[");
        for &point in layer.indices() {
            output.push_str(&format!(
                "{}, {}, ",
                vertices[point as usize].0, vertices[point as usize].1
            ));
        }
        output.push_str("]],");
        add_property("bbox", 0, output);
        add_property("area", 0, output);
        output.push_str("},");
        *id += 1;
    }
}

fn save_image_layers(layers: &[UiLayer], output: &mut String) {}

fn wrap_in_obj(name: &str, content: &str) -> String {
    format!("\"{}\" : {{ {} }},", name, content)
}

fn wrap_in_arr(name: &str, content: &[&str]) -> String {
    let mut inner = String::new();
    for element in content {
        inner.push_str(element);
        inner.push(',');
    }
    inner.pop();

    format!("\"{}\" : [{}],", name, inner)
}

fn add_property<T: fmt::Display>(name: &str, content: T, output: &mut String) {
    output.push_str(&format!("\"{}\" : {}, ", name, content))
}
