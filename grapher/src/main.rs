use eframe::egui;
use egui::{Color32, FontId};
use gmath::{round_floor, PointsChache};

mod gmath;
fn main() -> eframe::Result {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = egui::ViewportBuilder::default()
        .with_maximized(true)
        .with_resizable(true)
        .with_maximize_button(true);
    
    eframe::run_native("fGrapher", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))))
}

struct MyEguiApp {
    sd : ScaleData,
    func_vec : Vec<GraphFeatures>,
    func_remove_queue : Vec<u32>
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_fonts(configure_font());
        Self::default()
    }
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            sd: ScaleData{
                scale : 100.0,
                x_scale: 1.0,
                y_scale : 1.0,
                min_grid_size: 32.0,
                max_grid_size: 128.0,
                screen_offset_x: 0.0,
                screen_offset_y: 0.0,
                resolution: 1.0,
                menu_scrolling: false,
            },
            func_vec: {
                let mut v = Vec::new();
                v.push(GraphFeatures::default());
                v
            },
            func_remove_queue : Vec::new(),
        }
    }
}

impl eframe::App for MyEguiApp 
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {
        //input
        ctx.input(|i|{
            if !self.sd.menu_scrolling{
                self.sd.screen_offset_x += 128.0*i.raw_scroll_delta.x/(300.0+self.sd.scale);
                self.sd.screen_offset_y += 128.0*i.raw_scroll_delta.y/(300.0+self.sd.scale);
            }

            if i.key_pressed(egui::Key::Enter){
                for f in self.func_vec.iter_mut(){
                    f.pc.clear();
                }
            }

            if (1.0..1000.0).contains(&(self.sd.scale.clone() * i.zoom_delta())){
                self.sd.scale *= i.zoom_delta()
            }    
        });

        //check if a change has been made to the function input
        for f in self.func_vec.iter_mut(){
            if f.pc.input != f.func_input {
                f.pc.clear();
                f.pc.input = f.func_input.clone();
            }
        }

        //dark mode
        ctx.set_visuals(egui::Visuals::dark());

        //draw graph
        egui::CentralPanel::default().show(ctx, |ui| {
            draw_graph(ui, &self.sd, &mut self.func_vec);
        });

        let frame_col1 = egui::Color32::from_rgb(35, 35, 35);
        let frame_col2 = egui::Color32::from_rgb(50, 50, 50);
        egui::Window::new("fGrapher")
            .movable(false)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .min_width(300.0)
            .min_height(1080.0)
            .collapsible(true)
            .show(ctx, |ui| {
                
                //setting up sizes
                ui.set_min_height(ui.available_height());
                ui.set_max_width(200.0);

                egui::ScrollArea::vertical().scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible).show(ui, |ui| {
                    egui::Frame::none()
                    .fill(frame_col1)
                    .inner_margin(5.0)
                    .outer_margin(5.0)
                    .rounding(1.0)
                    .show(ui, |ui| {
                        //function settings and input
                        ui.label("Functions");
                        let mut counter = 0;
                        for f in self.func_vec.iter_mut(){
                            egui::Frame::none()
                            .fill(frame_col2)
                            .inner_margin(5.0)
                            .outer_margin(5.0)
                            .rounding(1.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui |{
                                    ui.label("f(x) = ");
                                    ui.add_sized(egui::Vec2::new(150.0,20.0),egui::TextEdit::singleline(
                                    &mut f.func_input,
                                    ));
                                    if ui.button("Remove".to_string()).clicked(){
                                        self.func_remove_queue.push(counter.clone());
                                    }
                
                                });
                                ui.separator();
                                ui.vertical(|ui|{
                                    ui.label("Function Properties");
                                    ui.label(format!("Y-Intercept: {}",f.y_intercept));
                                });
                            });
                            counter += 1;
                        }
                        for f in 0..self.func_remove_queue.len(){
                            self.func_vec.remove(self.func_remove_queue.get(f).unwrap().clone() as usize);
                        }
                        self.func_remove_queue.clear();
                        ui.horizontal(|ui |{
                            egui::Frame::none()
                            .outer_margin(5.0)
                            .show(ui, |ui| {
                                if ui.add_sized(ui.available_size(), egui::Button::new("Add")
                                    .fill(egui::Color32::from_rgb(50, 50, 50))
                            ).clicked() {
                                    self.func_vec.push(GraphFeatures::default());
                                }
                                });
                        });
                    });
                    
                    egui::Frame::none()
                        .fill(frame_col1)
                        .inner_margin(5.0)
                        .outer_margin(5.0)
                        .rounding(1.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label("Graph Style");
                                ui.separator();
                                ui.add(egui::Slider::new(&mut self.sd.scale, 1.0..=1000.0)
                                    .text("Zoom")
                                    .logarithmic(true)
                                );
                                ui.add(egui::Slider::new(&mut self.sd.y_scale, 0.01..=10.0)
                                    .text("Y Stretch")
                                    .logarithmic(true)
                                );
                                ui.add(egui::Slider::new(&mut self.sd.x_scale, 0.01..=10.0)
                                    .text("X Stretch")
                                    .logarithmic(true)
                                );
                                if ui.add(egui::Slider::new(&mut self.sd.resolution, 0.1..=1.0)
                                    .text("Resolution")
                                    .logarithmic(true)
                                ).hovered(){
                                    
                                }
                                //counter for 
                                let mut count = 1;
                                for f in self.func_vec.iter_mut(){
                                    ui.horizontal(|ui | {
                                        ui.label(format!("Line {} colour: ",count));
                                        ui.color_edit_button_rgb(&mut f.line_colour);
                                    });
                                    
                                    count += 1;
                                }                              
                            });

                    });
                    ui.add_space(5.0);
                    ui.separator();
                });
            });
    }       
}

struct ScaleData{
    y_scale : f32,
    x_scale : f32,
    scale : f32,
    min_grid_size : f32,
    max_grid_size : f32,
    screen_offset_x : f32,
    screen_offset_y : f32,
    resolution: f32,
    menu_scrolling: bool
}


struct GraphFeatures{
    func_input : String,
    x_intercepts : Vec<f32>,    
    y_intercept : f32,
    pc: PointsChache,
    line_colour: [f32; 3]
}
impl Default for GraphFeatures{
    fn default() -> Self {
        GraphFeatures{
            func_input: "x".to_string(),
            y_intercept: 0.0,
            x_intercepts: Vec::new(),
            line_colour: [1.0,0.0,0.0],
            pc: {
                PointsChache{
                    points: Vec::new(),
                    d_points : Vec::new(),
                    input: "0".to_string(),
                    draw: false
                }
            },
        }
    }
}



fn draw_graph(ui: &mut egui::Ui, sd : &ScaleData, func_vec: &mut Vec<GraphFeatures>) {
    let available_rect  = ui.max_rect();
    let draw_rect = egui::Rect::from_min_max(
        egui::Pos2::new(available_rect .min.x + 0.0/*offset from left */, available_rect .min.y),
        available_rect.max,
    );
    ui.allocate_rect(draw_rect, egui::Sense::hover());
    let painter = ui.painter_at(draw_rect);

    draw_grid(&painter, &available_rect, &sd);

    for f in func_vec.iter_mut(){
        draw_graph_line(&painter, &available_rect, &sd, f);
    }
    

    
}

fn draw_graph_line(painter : &egui::Painter, draw_rect : &egui::Rect, sd : &ScaleData, features: &mut GraphFeatures){
    //get vector of points
    let points = if !features.pc.points.is_empty(){
        features.pc.points.clone()
    }
    else{
        features.pc.points = gmath::get_points(-draw_rect.max.x, draw_rect.max.x, 1.1-sd.resolution, &features.func_input).points;
        features.y_intercept = gmath::get_y_intercept(features.func_input.clone());
        features.pc.points.clone()
    };
    
    let mut last_point = points[0];

    let origin_y = draw_rect.max.y / 2.0 - sd.scale/2.0;
    let origin_x = draw_rect.max.x / 2.0 - sd.scale/2.0;

    //draw main line
    for current_point in points.iter().skip(1) {
        if current_point.2{
            let start = egui::Pos2::new(last_point.0*sd.scale*sd.x_scale + origin_x + sd.screen_offset_x, last_point.1*sd.scale*sd.y_scale + origin_y + sd.screen_offset_y);
            let end = egui::Pos2::new(current_point.0*sd.scale*sd.x_scale + origin_x + sd.screen_offset_x, current_point.1*sd.scale*sd.y_scale + origin_y + sd.screen_offset_y);
            painter.line_segment([end, start], egui::Stroke::new(2.0, Color32::from_rgb((features.line_colour[0] * 255.0) as u8, (features.line_colour[1] * 255.0) as u8, (features.line_colour[2] * 255.0) as u8))); 
            last_point = current_point.clone();
        }
        else{
            last_point = current_point.clone();
        }
    }
}


fn draw_grid(painter : &egui::Painter, draw_rect : &egui::Rect, sd : &ScaleData){
    let origin_y = draw_rect.max.y / 2.0 - sd.scale/2.0;
    let origin_x = draw_rect.max.x / 2.0 - sd.scale/2.0;

    //params
    let line_intensity = 50;
    let point_color = egui::Color32::YELLOW;
    let grid_offset : f32 = 0.0;
    let point_radius = 2.0;
    let text_col = egui::Color32::WHITE;
    let font_size = 12.0;
    let text_offset = 32.0;
    let label_freq : i32 = 1;
    let line_magnitude = 99000.0;

    //initial axis
    let mut x = origin_x;
    let start = egui::Pos2::new(x+sd.screen_offset_x, draw_rect.min.y+sd.screen_offset_y-line_magnitude);
    let end = egui::Pos2::new(x+sd.screen_offset_x, draw_rect.max.y+sd.screen_offset_y+line_magnitude);
    painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 255, 255))); 
    
    let mut grid_adjust_scale_x : f32 = 0.0;
    let mut x_add = sd.x_scale * sd.scale;

    //center
    let point_position = egui::Pos2::new(origin_x + sd.screen_offset_x, origin_y+sd.screen_offset_y);
    painter.circle_filled(point_position, 2.0, egui::Color32::RED);

    while x_add < sd.min_grid_size{
        x_add *= 2.0;
        grid_adjust_scale_x += 1.0;
    }

    while x_add > sd.max_grid_size {
        x_add /= 2.0;
        grid_adjust_scale_x -= 1.0;
    }
    x+=x_add;
    let two : f32 = 2.0;
    let mut counter : i32 = 0;
    while x < 1920.0{
        counter += 1;
        //line
        let start = egui::Pos2::new(x+sd.screen_offset_x, draw_rect.min.y+sd.screen_offset_y-line_magnitude);
        let end = egui::Pos2::new(x+sd.screen_offset_x, draw_rect.max.y+sd.screen_offset_y+line_magnitude);
        painter.line_segment([start, end], egui::Stroke::new(1.0, egui::Color32::from_rgb(line_intensity, line_intensity, line_intensity)));
        
        //point
        let point_position = egui::Pos2::new(x+sd.screen_offset_x, origin_y + grid_offset+sd.screen_offset_y);
        painter.circle_filled(point_position, point_radius, point_color);

        //text
        if counter % label_freq == 0{
            painter.text(egui::pos2(point_position.x, point_position.y + 20.0), egui::Align2::CENTER_CENTER, round_floor((counter as f32) * two.powf(grid_adjust_scale_x),3).to_string(), FontId::monospace(font_size), text_col);
        }
         
        x += x_add
    }
    counter = 0;
    x = origin_x - x_add;
    while x > -1920.0{
        counter += 1;
        //line
        let start = egui::Pos2::new(x+sd.screen_offset_x, draw_rect.min.y+sd.screen_offset_y - line_magnitude);
        let end = egui::Pos2::new(x+sd.screen_offset_x, draw_rect.max.y+sd.screen_offset_y + line_magnitude);
        painter.line_segment([start, end], egui::Stroke::new(1.0, egui::Color32::from_rgb(line_intensity, line_intensity, line_intensity))); // White line with 2.0 thickness
        
        //point
        let point_position = egui::Pos2::new(x+sd.screen_offset_x, origin_y + grid_offset+sd.screen_offset_y);
        painter.circle_filled(point_position, point_radius, point_color);

        //text
        if counter % label_freq == 0{
            painter.text(egui::pos2(point_position.x, point_position.y + 20.0), egui::Align2::CENTER_CENTER, round_floor((counter as f32) * two.powf(grid_adjust_scale_x),3).to_string(), FontId::monospace(font_size), text_col);
        }
        x -= x_add
    }

    let mut y = origin_y;
    let start = egui::Pos2::new(draw_rect.min.x+sd.screen_offset_x-line_magnitude, y+sd.screen_offset_y);
    let end = egui::Pos2::new(draw_rect.max.x+sd.screen_offset_x+line_magnitude, y + sd.screen_offset_y);
    painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 255, 255))); // White line with 2.0 thickness

    let mut grid_adjust_scale_y : f32 = 0.0;
    let mut y_add = sd.y_scale*sd.scale;
    while y_add < sd.min_grid_size{
        y_add *= 2.0;
        grid_adjust_scale_y += 1.0;
    }

    while y_add > sd.max_grid_size{
        y_add /= 2.0;
        grid_adjust_scale_y -= 1.0;
    }

    y += y_add;
    counter = 0;
    while y < 1920.0{
        counter += 1;
        //line
        let start = egui::Pos2::new(draw_rect.min.x+sd.screen_offset_x-line_magnitude, y+sd.screen_offset_y);
        let end = egui::Pos2::new(draw_rect.max.x+sd.screen_offset_x+line_magnitude, y+sd.screen_offset_y);
        painter.line_segment([start, end], egui::Stroke::new(1.0, egui::Color32::from_rgb(line_intensity, line_intensity, line_intensity))); // White line with 2.0 thickness
        
        //point
        let point_position = egui::Pos2::new(origin_x + grid_offset+sd.screen_offset_x, y+sd.screen_offset_y);
        painter.circle_filled(point_position, point_radius, point_color);
        
        //text
        if counter % label_freq == 0{
            painter.text(egui::pos2(point_position.x + text_offset, point_position.y), egui::Align2::CENTER_CENTER, round_floor((counter as f32) * two.powf(grid_adjust_scale_y), 3).to_string(), FontId::monospace(font_size), text_col);
        }
        y += y_add
    }
    y = origin_y - y_add;
    counter = 0;
    while y > -1920.0{
        counter += 1;
        //line
        let start = egui::Pos2::new(draw_rect.min.x+sd.screen_offset_x-line_magnitude, y+sd.screen_offset_y);
        let end = egui::Pos2::new(draw_rect.max.x+sd.screen_offset_x+line_magnitude, y+sd.screen_offset_y);
        painter.line_segment([start, end], egui::Stroke::new(1.0, egui::Color32::from_rgb(line_intensity, line_intensity, line_intensity))); // White line with 2.0 thickness

        //point
        let point_position = egui::Pos2::new(origin_x + grid_offset+sd.screen_offset_x, y+sd.screen_offset_y);
        painter.circle_filled(point_position, point_radius, point_color);

        //text
        if counter % label_freq == 0{
            painter.text(egui::pos2(point_position.x + text_offset, point_position.y), egui::Align2::CENTER_CENTER, round_floor((counter as f32) * two.powf(grid_adjust_scale_y),3).to_string(), FontId::monospace(font_size), text_col);
        }

        y -= y_add
    }
}


fn configure_font() -> egui::FontDefinitions{
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/IBMPlexMono-Regular.ttf")));
    
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());

    fonts
}