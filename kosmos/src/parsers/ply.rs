use std::collections::HashMap;

use ilios_types::geometry::Point;

#[derive(Debug)]
pub struct Element {
    name: String,
    count: usize,
    properties: Vec<PropertyDefinition>,
}

#[derive(Debug)]
pub enum PropertyDefinition {
    None,
    Scalar {
        value_type: String,
        name: String,
    },
    List {
        size_type: String,
        element_type: String,
        name: String,
    },
}

#[derive(Default, Debug)]
pub struct Header {
    format: String,
    version: String,
    comments: Vec<String>,
    elements: Vec<Element>,
}

#[derive(PartialEq)]
pub enum ParsingState {
    HeaderStart,
    ValidFileType,
    Definitions,
    HeaderEnd,
}

#[derive(Debug, Default)]
pub enum Number {
    #[default]
    None,
    Float(f32),
    Int(i32),
}

impl Number {
    pub fn as_i32(&self) -> i32 {
        match self {
            Number::Int(i) => *i,
            Number::Float(f) => *f as i32,
            _ => 0,
        }
    }
    pub fn as_f32(&self) -> f32 {
        match self {
            Number::Float(f) => *f,
            Number::Int(i) => *i as f32,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Component(Vec<Number>);

#[derive(Debug, Default)]
pub struct PlyFile {
    header: Header,
    element_components: HashMap<String, Vec<Component>>,
}

impl PlyFile {
    pub fn faces(&self) -> impl Iterator<Item = Vec<Point>> {
        let faces = self.element_components.get("face").unwrap();
        let vertex = self.element_components.get("vertex").unwrap();

        (0..faces.len()).map(move |idx| {
            let fc: &Component = &faces[idx];
            let v_idx =
                fc.0.iter()
                    .map(|v| v.as_i32() as usize)
                    .collect::<Vec<usize>>();
            let v1 = vertex[v_idx[0]]
                .0
                .iter()
                .map(|v| v.as_f32())
                .collect::<Vec<f32>>();
            let v2 = vertex[v_idx[1]]
                .0
                .iter()
                .map(|v| v.as_f32())
                .collect::<Vec<f32>>();
            let v3 = vertex[v_idx[2]]
                .0
                .iter()
                .map(|v| v.as_f32())
                .collect::<Vec<f32>>();

            vec![
                Point::new(v1[0], v1[1], v1[2]),
                Point::new(v2[0], v2[1], v2[2]),
                Point::new(v3[0], v3[1], v3[2]),
            ]
        })
    }
}

pub fn parse(file_content: &str) -> PlyFile {
    let mut lines = file_content.lines();
    let mut header = Header::default();
    let mut parsing_state = ParsingState::HeaderStart;

    while let Some(line) = lines.next() {
        match parsing_state {
            ParsingState::HeaderStart => {
                if line == "ply" {
                    parsing_state = ParsingState::ValidFileType;
                } else {
                    panic!("Unsupported file type");
                }
            }
            ParsingState::ValidFileType => {
                if line == "format ascii 1.0" {
                    parsing_state = ParsingState::Definitions;
                    header.format = String::from("ascii");
                    header.version = String::from("1.0");
                } else {
                    panic!("Invalid file format");
                }
            }
            ParsingState::Definitions => {
                if line.starts_with("end_header") {
                    parsing_state = ParsingState::HeaderEnd;
                } else if line.starts_with("comment") {
                    header.comments.push(line.to_string());
                } else if line.starts_with("element") {
                    header.elements.push(parse_element(&line));
                } else if line.starts_with("property") {
                    let mut el = header
                        .elements
                        .pop()
                        .expect("No element available for property");
                    el.properties.push(parse_property(&line));
                    header.elements.push(el);
                }
            }
            _ => panic!("Invalid state"),
        }
        if parsing_state == ParsingState::HeaderEnd {
            break;
        }
    }

    let mut ply_file = PlyFile::default();
    let mut element_components: HashMap<String, Vec<Component>> = HashMap::default();

    for element in header.elements.iter() {
        let count = element.count;
        let name = &element.name;

        if !element_components.contains_key(name) {
            element_components.insert(name.to_string(), vec![]);
        }
        let entries = element_components.get_mut(name).unwrap();

        for _i in 0..count {
            if let Some(line) = lines.next() {
                let value = parse_element_line(element, &line);
                entries.push(Component(value));
            }
        }
    }

    ply_file.header = header;
    ply_file.element_components = element_components;
    ply_file
}

fn parse_element_line(element: &Element, line: &str) -> Vec<Number> {
    let mut values = line.split(' ');
    let mut result: Vec<Number> = vec![];
    for prop in element.properties.iter() {
        match prop {
            PropertyDefinition::None => {}
            PropertyDefinition::Scalar { value_type, .. } => {
                if value_type == "float" {
                    let value: f32 = values.next().unwrap().parse().unwrap();
                    result.push(Number::Float(value));
                } else {
                    panic!("Unsupported Scalar value type");
                }
            }
            PropertyDefinition::List { .. } => {
                let size: usize = values.next().unwrap().parse().unwrap();
                for _i in 0..size {
                    let value: i32 = values.next().unwrap().parse().unwrap();
                    result.push(Number::Int(value));
                }
            }
        }
    }
    result
}

fn parse_property(line: &str) -> PropertyDefinition {
    let mut it = line.split(' ').skip(1);
    let tp = it.next().unwrap();
    if tp == "float" {
        PropertyDefinition::Scalar {
            value_type: tp.to_string(),
            name: it.next().unwrap().to_string(),
        }
    } else if tp == "list" {
        PropertyDefinition::List {
            size_type: it.next().unwrap().to_string(),
            element_type: it.next().unwrap().to_string(),
            name: it.next().unwrap().to_string(),
        }
    } else {
        PropertyDefinition::None
    }
}

fn parse_element(line: &str) -> Element {
    let mut it = line.split(' ').skip(1);
    let name = it.next().unwrap().to_string();
    let count: usize = it.next().unwrap().parse().unwrap();
    Element {
        name,
        count,
        properties: vec![],
    }
}
