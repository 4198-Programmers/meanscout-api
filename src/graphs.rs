use std::f64::consts::PI;

use std::collections::HashMap;

use serde_json::value::{to_value, Value};
use std::error::Error;
use tera::{Context, Result, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut teras = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        teras.autoescape_on(vec![".html", ".sql", ".svg"]);
        teras.register_filter("do_nothing", do_nothing_filter);
        teras
    };
}

pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(s).unwrap())
}

fn get_coords(percentages: Vec<f32>) -> Vec<Vec<f64>> {
    let center_x = 0.0;  // X-coordinate of the circle's center
    let center_y = 0.0;  // Y-coordinate of the circle's center
    let radius = 100.0;    // Radius of the circle

    // let percentages = vec![25, 35, 40];  // Percentages of the points on the circle

    let total_percentage: f64 = percentages.iter().sum::<f32>() as f64;
    let angle_increment = (2.0 * PI) / (total_percentage / 100.0);

    let mut current_angle: f64 = 0.0;
    let mut points: Vec<Vec<f64>> = Vec::new();

    for &percentage in percentages.iter() {
        let point_x = center_x + radius * current_angle.cos();
        let point_y = center_y + radius * current_angle.sin();

        points.push([point_x, point_y].into());

        current_angle += (percentage as f64 / 100.0) * angle_increment;
    }
    points
}

#[derive(Clone, Debug)]
pub struct LineGraph {
    pub name: String,
    pub points: Vec<Point>,
    pub colour: String
}

impl LineGraph {
    pub fn new(name: String, colour: String) -> Self {
        LineGraph {
            name,
            points: Vec::new(),
            colour,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point { x, y });
    }

    pub fn draw_svg(&self, width: usize, height: usize, lines: usize) -> std::result::Result<String, Box<dyn Error>> {

        let mut context = Context::new();
    
        //hardset the padding around the graph
        let padding = 50;
    
        //ensure the viewbox is as per input
        let width = width - padding * 2;
        let height = height - padding * 2;
        

        let max_x = self
            .points
            .iter()
            .map(|point| point.x)
            .fold(0. / 0., f64::max);

        let max_y = self
            .points
            .iter()
            .map(|point| point.y)
            .fold(0. / 0., f64::max);

        println!("{} {}", max_x.round(), max_y.round());
    
        let path = self
            .points
            .iter()
            .map(|val| Point {
                x: (val.x / max_x * width as f64) + padding as f64,
                y: (val.y / max_y * (height as f64 * -1.0)) + (padding + height) as f64,
            })
            .enumerate()
            .map(|(i, point)| {
                if i == 0 {
                    format!("M {} {}", point.x, point.y)
                } else {
                    format!("L {} {}", point.x, point.y)
                }
            })
            .collect::<Vec<String>>().join(" ");

        context.insert("name", &self.name);
        context.insert("width", &width);
        context.insert("height", &height);
        context.insert("padding", &padding);
        context.insert("path", &path);
        context.insert("colour", &self.colour);
        context.insert("lines", &lines);
        context.insert("max_x", &max_x.round());
        context.insert("max_y", &max_y.round());
        context.insert("x_label", "This is an X label");
        context.insert("y_label", "This is a Y label");
    
        // Tera::one_off(, &context, true).expect("Could not draw graph")
        let graph = TEMPLATES.render("graphs/line.svg", &context)?;
        Ok(graph)
    
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Slice {
    pub amount: f64,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct PieGraph {
    pub name: String,
    pub points: Vec<Slice>,
    pub colour: String
}

impl PieGraph {
    pub fn new(name: String, colour: String) -> Self {
        PieGraph {
            name,
            points: Vec::new(),
            colour,
        }
    }

    pub fn add_slice(&mut self, amount: f64, name: String) {
        self.points.push(Slice { amount, name });
    }

    pub fn draw_svg(&self, height: i64, width: i64, background: String) -> std::result::Result<String, Box<dyn Error>> {

        let mut context = Context::new();
    
        //hardset the padding around the graph
        let padding = 50;

        let total: f64 = self.points
            .iter()
            .map(|point| point.amount)
            .sum();

        let percentages: Vec<f32> = self.points.iter().map(|point| (point.amount * 100.0 / total) as f32).collect();

        let points_on_graph = get_coords(percentages.clone());

        let all_points: Vec<f64> = self.points.iter().map(|point| point.amount).collect();

        let names: Vec<String> = self.points.iter().map(|point| point.clone().name).collect();

        let colors: Vec<String> = ["#fab387".into(), "#f38ba8".to_string(), "#74c7ec".into(), "#b4befe".into(), "#94e2d5".into(), "#f9e2af".into(), "#a6e3a1".into(), "#209fb5".into(), "#7287fd".into(), "#eb6f92".into(), "#9ccfd8".into()].into();

        // let rotation_list: Vec<f64> = self.points.iter().map(|point| point.amount).collect();
        // rotations.push(0.0);
        
        // unsafe {prepend_slice(&mut rotations, &[0.0])}
        // rotations.swap(1, 2);
        // rotations.swap(3, 1);

        let length = points_on_graph.len();

        context.insert("name", &self.name);
        context.insert("padding", &padding);
        context.insert("total", &total);
        context.insert("length", &length);
        context.insert("points_in_graph", &points_on_graph);
        context.insert("percentages", &percentages);
        context.insert("height", &height);
        context.insert("width", &width);
        context.insert("background", &background);
        // context.insert("path", &path);
        context.insert("colour", &colors);
        // context.insert("lines", &lines);
        context.insert("all_points", &all_points);
        context.insert("names", &names);
        context.insert("x_label", "This is an X label");
        context.insert("y_label", "This is a Y label");
    
        // Tera::one_off(, &context, true).expect("Could not draw graph")
        let graph = TEMPLATES.render("graphs/pie.svg", &context)?;
        Ok(graph)
    
    }
}