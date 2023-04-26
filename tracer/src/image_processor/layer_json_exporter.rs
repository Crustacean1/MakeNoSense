use std::fs;

use crate::editor::image_selection::LayerInfo;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::triangle::Segmentation;

#[derive(Serialize, Deserialize)]
struct CocoInfo {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct CocoImages {
    id: usize,
    width: u32,
    height: u32,
    file_name: String,
}

#[derive(Serialize, Deserialize)]
struct CocoAnnotations {
    id: usize,
    iscrowd: usize,
    image_id: usize,
    category_id: usize,
    segmentation: [Vec<f32>; 1],
    bbox: [f32; 4],
    area: f32,
}

#[derive(Serialize, Deserialize)]
struct CocoCategory {
    id: usize,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct CocoStruct {
    info: CocoInfo,
    images: [CocoImages; 1],
    annotations: Vec<CocoAnnotations>,
    categories: Vec<CocoCategory>,
}

pub fn save_coco(
    image: &str,
    resolution: (u32, u32),
    segmentations: &[Segmentation],
    layer_info: &[LayerInfo],
) -> Result<()> {
    if let Some((filename, name, image_id)) = parse_image_path(image) {
        let coco_struct = CocoStruct {
            info: CocoInfo {
                description: String::from("my-project-name"), /*Yes, it needs to be hardcoded...*/
            },
            images: [create_images(&filename, image_id, resolution)],
            annotations: create_annotations(image_id, segmentations),
            categories: create_layer_info(layer_info),
        };
        let data = serde_json::to_string(&coco_struct)?;

        let output_filename = format!("{}.json", name);
        fs::write(&output_filename, data).expect("Failed to write");
    } else {
        println!("Failed to parse filepath data");
    }
    Ok(())
}

fn parse_image_path(filepath: &str) -> Option<(String, String, usize)> {
    let path_segments = filepath.split("/");
    match path_segments.last() {
        Some(filename) => {
            let filename_segments = filename.split(".");
            let mut filename_parts: Vec<_> = filename_segments.collect();
            filename_parts.pop();
            let name = filename_parts.join(".");
            match filename_parts.pop() {
                Some(image_id) => {
                    if let Ok(image_id) = image_id.parse::<usize>() {
                        Some((String::from(filename), name, image_id))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn create_images(image: &str, image_id: usize, resolution: (u32, u32)) -> CocoImages {
    CocoImages {
        id: image_id,
        width: resolution.0,
        height: resolution.1,
        file_name: String::from(image),
    }
}

fn create_annotations(image_id: usize, selections: &[Segmentation]) -> Vec<CocoAnnotations> {
    selections
        .iter()
        .enumerate()
        .map(|(i, selection)| {
            let area = selection.area();
            let bbox = selection.bounding_box();
            let bbox = [bbox.left, bbox.top, bbox.width(), bbox.height()];
            let segmentation = [selection.vertices().clone()];

            CocoAnnotations {
                id: i + 1,
                iscrowd: 0,
                image_id,
                category_id: selection.type_id,
                segmentation,
                bbox,
                area,
            }
        })
        .collect()
}

fn create_layer_info(layer_info: &[LayerInfo]) -> Vec<CocoCategory> {
    layer_info
        .iter()
        .map(|info| CocoCategory {
            id: info.id,
            name: info.layer_type.clone(),
        })
        .collect()
}
