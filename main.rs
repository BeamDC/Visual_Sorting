use rand::prelude::SliceRandom;
use std::thread;
use std::time::Duration;

extern crate kiss3d;

use kiss3d::nalgebra::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

const SIZE:u32 = 10000;
const DELAY_MS:Duration = Duration::from_millis(0);

fn bubble_sort(vec: &mut Vec<u32>) {
    for i in (0..SIZE).rev() {
        for j in 0..i {
            let j=j as usize;
            if vec[j] > vec[j+1] {
                vec[j] ^= vec[j+1];
                vec[j+1] ^= vec[j];
                vec[j] ^= vec[j+1];
            }
        }
        //sleep for a given amount of time
        thread::sleep(DELAY_MS);
    }
}

fn set_data(){
    let mut vec: Vec<u32> = (0..SIZE).collect();
    vec.shuffle(&mut rand::thread_rng());
    println!("{:?}\n",vec);
    bubble_sort(&mut vec);
    println!("{:?}",vec);
}

fn main() {
    let mut window = Window::new("Sorting_Visualz");
    let mut c      = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}
