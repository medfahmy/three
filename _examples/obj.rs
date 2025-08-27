// use std::env;
// use three::Object;
//
// fn main() {
//     let mut args = env::args();
//     let obj_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/car.obj");
//     let path = args.nth(1).unwrap_or(obj_path.into());
//     let mut win = three::Window::new("Three-rs obj loading example");
//     let cam = win.factory.perspective_camera(60.0, 1.0 .. 1000.0);
//     let mut controls = three::controls::Orbit::builder(&cam).position([0.0, 2.0, -5.0]).target([0.0, 0.0, 0.0]).build();
//
//     let dir_light = win.factory.directional_light(0xffffff, 0.9);
//     dir_light.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);
//     win.scene.add(&dir_light);
//
//     let root = win.factory.group();
//     win.scene.add(&root);
//     let (mut group_map, _meshes) = win.factory.load_obj(&path);
//     for g in group_map.values_mut() {
//         root.add(g);
//     }
//
//     while win.update() && !win.input.hit(three::KEY_ESCAPE) {
//         win.render(&cam);
//     }
// }

fn main() {
}

struct Input {
}

enum Output {

}

trait App {
    fn update(&mut self) {
    }

    fn input(&mut self, input: Input) -> Output;
}

struct MyApp {
}

impl App for MyApp {
    fn input() -> {
    }

    fn update() -> {
    }
}

impl App {
    fn new() -> Self {
    }
}

impl Three for App {
    fn update() {
        self.controls.update(&win.input);
    }
}
