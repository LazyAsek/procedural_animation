use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, video::Window, Sdl, VideoSubsystem};

fn main() {
    
    let sld_context : Sdl = sdl2::init().unwrap();
    let video_subsystem : VideoSubsystem = sld_context.video().unwrap();

    const WIDTH : u32 = 1080;
    const HEIGHT: u32 = 720;
    const LEN : i32 = 5;

    let window : Window = video_subsystem.window("Animatioon", WIDTH, HEIGHT)
    .borderless()
    .build()
    .unwrap();

    let mut event_pump = sld_context.event_pump().unwrap();

    let mut canvas : Canvas<Window> = window.into_canvas().build().unwrap();

    let r : f64 = 50.0;
    let mut x :i32 = 100;
    let mut y:i32 =100;
    let mut points:Vec<Point> = init_circles(x, y, r, LEN);
    'run : loop{
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. } =>{
                    break 'run;
                },
                Event::KeyDown { keycode : Some(Keycode::Escape), ..  } =>{
                    break 'run;
                },
                Event::KeyDown { keycode : Some(Keycode::A) , .. } => {
                    points[0] = points[0].offset(-1, 0) ;
                },
                Event::KeyDown { keycode : Some(Keycode::D) , .. } => {
                    points[0] = points[0].offset(1, 0) ;
                },
                Event::KeyDown { keycode : Some(Keycode::W) , .. } => {
                    points[0] = points[0].offset(0, -1) ;
                },
                Event::KeyDown { keycode : Some(Keycode::S) , .. } => {
                    points[0] = points[0].offset(0, 1) ;
                },


                _ =>{}
            }
        }
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(40,40,40));

        for i in 0 .. points.len(){
            canvas = draw_circle(points[i].x, points[i].y, r, canvas);
        }
        points = update_circles(points, r);


        canvas.present();
    }
}

fn draw_circle(x:i32,y:i32,r:f64,mut canvas:Canvas<Window>) -> Canvas<Window>{

    let middle : Point = Point::new(x, y);
    canvas.draw_point(middle).expect("drawing middle failed");

    for i in 0..360{
        let angle: f64 = i.try_into().unwrap();

        let point_x: i32 = x+(r*angle.sin()) as i32;
        let point_y: i32 = y+(r*angle.cos()) as i32;
        canvas.draw_point(Point::new(point_x,point_y)).expect("drawing point failed");
    }

    return canvas;
}

fn get_point_on_circle(x:i32,y:i32,r:f64,angle:f64) -> [i32;2]{
    let point_x: i32 = x+(r*angle.cos()) as i32;
    let point_y: i32 = y+(r*angle.sin()) as i32;
    return [point_x,point_y];
}

fn get_angle(orginal_x:i32,orginal_y:i32,x:i32,y:i32) -> f64{
    let x_len:f64 = (x - orginal_x).try_into().unwrap();
    let y_len:f64 = (y - orginal_y).try_into().unwrap();
    return 360.0-(y_len.atan2(x_len).to_degrees()) ;
}

fn init_circles(x:i32,y:i32,r:f64,len:i32) -> Vec<Point>{
    let mut points: Vec<Point> = vec![];
    points.push(Point::new(x, y));
    let mut prev:Point;
    let mut coords;

    for i in 0..len as usize{
        prev = points[i];
        coords = get_point_on_circle(prev.x, prev.y, r, 0.0);
        points.push(Point::new(coords[0],coords[1]));
    }

    return points;
}

fn update_circles(mut points:Vec<Point>,r:f64) -> Vec<Point>{
    let mut prev:Point = points[0];
    let mut coords:[i32;2];
    let mut angle: f64;
    let mut pangle: f64 = get_angle(points[1].x, points[1].y,prev.x, prev.y);


    for i in 1..points.len() as usize{
        angle = get_angle(points[i].x, points[i].y,prev.x, prev.y);
        
        let distance = (prev.x-points[i].x).pow(2) + (prev.y-points[i].y).pow(2);
        if distance > r.powf(2.0) as i32{  
            //coords = get_point_on_circle(prev.x,prev.y, r, pangle - angle);
            //points[i] = Point::new(coords[0],coords[1]);

        }
        prev = points[i];
        pangle = angle;
    }



    return  points;
}