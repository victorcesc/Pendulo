//https://www.youtube.com/watch?v=U-X51GsTAzA&ab_channel=ManualdoC%C3%B3digo
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use vector::Vector; // tipo using namespace do Cpp
fn main() {
    //println!("Hello, world!");
    let window = Window::new_centered("Pendulum", (800, 600)).unwrap();
    let win: MyWindowHandler = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 400.0),
    };
    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        //graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        self.p.update();
        self.p.draw(graphics);
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    // If desired, on_mouse_move(), on_key_down(), etc...
}

struct Pendulum {
    origin: Vector, //aqui nao há necessidade de colocar vector::

    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_acc: f32,

    r: f32,
    m: f32,
    g: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acc: 0.0,
            r: r,
            m: 1.0, //massa da bola do pendulo
            g: 1.5, //gravidade do mundo do pendulo
        }
    }
    fn update(&mut self) {
        //equacao do pendulo para calcular a aceleracao angular
        self.angular_acc = -0.005 * self.g * self.angle.sin() / self.r;
        //aqui quanto maior o valor maior a aceleracao do pendulo
        //a velocidade angular mais a aceleração angular
        self.angular_velocity += self.angular_acc;

        //angulo sendo atualizado de acordo com a velocidade angular
        self.angle += self.angular_velocity;

        //a posicao das coordenadas polares traduzidas para cordenadas cartesianas
        //set vinda da struct Vector, sintaxe de metodo
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origin);
    }
    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            4.0,
            Color::RED,
        );
        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED)
    }
}

mod vector {

    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        //aqui nao manipula nada interno, tipo static methods
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y } // isso retorna o dado como se fosse um return
        }

        //self determinam metodos
        pub fn add(&mut self, other: &Vector) -> &Vector {
            //self é igual ao this, &mut para mudar as variaveis da struct
            self.x += other.x;
            self.y += other.y;
            self // isso retorna o dado como se fosse um return
        }
        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self // isso retorna o dado como se fosse um return
        }
    }
}
