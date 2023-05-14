use obj::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

fn main() {
    let polyline_buf = BufReader::new(File::open("assets/track-polyline.obj").unwrap());
    let model = raw::parse_obj(polyline_buf).unwrap();
    let pretty_config = ron::ser::PrettyConfig::default()
        .indentor("  ".to_string())
        .new_line("\n".to_string());
    let pos_ron = ron::ser::to_string_pretty(&model.positions, pretty_config).unwrap();
    File::create(format!("assets/track-positions.ron"))
        .and_then(|mut file| file.write(pos_ron.as_bytes()))
        .expect("Error while writing scene to file");
    // let positions = model.positions;
}

// let obj_path = "assets/track-polyline.obj";
// let polyline_buf = BufReader::new(File::open(obj_path).unwrap());
// let model = raw::parse_obj(polyline_buf).unwrap();
// let positions = model.positions;
