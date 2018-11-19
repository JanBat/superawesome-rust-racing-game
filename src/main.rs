extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

struct DrawGeometry{
radius:u32,
}



impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry{radius:1})
    }

   fn update(&mut self, _window: &mut Window) -> Result<()> {
	self.radius = self.radius+1;
	if (self.radius > 300){
		self.radius = 1;
	};
	
	Ok(())
   }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
        window.draw(&Circle::new((400, 300), 1+self.radius), Col(Color::GREEN));
        window.draw_ex(
            &Line::new((50, 80),(600, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
	//hallo git!
	

    		run::<DrawGeometry>("Draw Geometry", Vector::new(800, 600), Settings::default());
	
}
