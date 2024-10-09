use egui::Galley;
use rsc::lexer::tokenize;
use rsc::parser::{Expr, parse};
use rsc::computer::Computer;
use rsc::computer::ComputeError;


pub struct PointsChache{
    pub(crate) points : Vec<GraphPoint>,
    pub(crate) d_points : Vec<GraphPoint>,
    pub(crate) input : String,
    pub(crate) draw : bool
}

pub type GraphPoint = (f32, f32, bool);

impl Differentiable for GraphPoint{
    fn gradiet(&self, other: &Self) -> f32 {
        (self.1 - other.1).abs() / (self.0-other.0).abs()
    }
}

trait Differentiable {
    fn gradiet(&self, other: &Self) -> f32;
}

impl PointsChache{
    pub fn clear(&mut self){
        self.draw = false;
        self.points.clear();
        self.d_points.clear();
    }

    pub fn differentiate(&mut self, step : f32) {
        let mut points : Vec<GraphPoint> = Vec::new();

        for i in 0..(self.points.len() - (2 as usize)){
            let y = self.points.get(i).unwrap().gradiet(self.points.get(i+1).unwrap());
            points.push((step * i as f32, y, true));
        }

        self.d_points = points;
    }
}

pub fn get_points(start_x : f32, end_x : f32, step : f32,input: &String) -> PointsChache{
    let mut points = Vec::new();

    let computer = match parse_input(input.to_string()){
        Ok(expression) => expression,
        Err(_e) => {
            let pc = PointsChache{
                points : {
                    let mut p = Vec::new();
                    p.push((0.0,0.0,false));
                    p
                },
                d_points : Vec::new(),
                input : "".to_string(),
                draw : false
            };
            return pc;
        }
    };

    let mut x = start_x;

    while x <= end_x{
        let point : GraphPoint = compute_point(computer.clone(), x);
        points.push(point);
        x += step;
    }

    let mut pc = PointsChache{
        points : points,
        d_points : Vec::new(),
        input : input.clone(),
        draw : true
    };
    pc.differentiate(step);

    pc
}


fn parse_input(expr: String) -> Result<Expr, String> {
    let tokens = tokenize(&expr).map_err(|_e| format!("Tokenize error:"))?;
    let ast = parse(&tokens).map_err(|_e| format!("Parse error:"))?;

    Ok(ast)
}

fn compute_point(mut ast: Expr, x: f32) -> GraphPoint{ 
    let mut draw = true; 
    let mut computer = Computer::new();

    ast.replace(&Expr::Identifier("x".to_string()), &Expr::Constant(x as f64), false);
    let result: Result<f64, ComputeError> = computer.compute(&ast);

    let y : f64 = result.unwrap_or_else(|_| {
        draw = false;
        0.0
    });
    let point : GraphPoint = (x,(y as f32)*-1.0, draw);
    point
}

fn compute_point2(mut ast: Expr, x: f32, last_point: GraphPoint) -> GraphPoint{ 
    let mut draw = true; 
    let mut computer = Computer::new();

    ast.replace(&Expr::Identifier("x".to_string()), &Expr::Constant(x as f64), false);
    let result: Result<f64, ComputeError> = computer.compute(&ast);

    let y : f64 = result.unwrap_or_else(|_| {
        draw = false;
        0.0
    });

    if (y.is_sign_negative() != last_point.1.is_sign_negative()) && (y > 100.0 || y < -100.0){
        draw = false;
    }
    
    let point : GraphPoint = (x,(y as f32)*-1.0, draw);
    point
}

pub fn round_floor(i : f32, round : i32) -> f32{
    let a = 10f32.powi(round as i32);
    (i * a).round() / a
}

pub fn get_y_intercept(expr: String) -> f32{
    let ast = parse_input(expr).unwrap_or(parse_input("0*x".to_string()).unwrap());
    let a = compute_point(ast, 0.0).1 * -1.0;
    a
}
