//rendering
// extern crate kiss3d;
// use kiss3d::nalgebra::{Vector3, UnitQuaternion};
// use kiss3d::window::Window;
// use kiss3d::light::Light;

//everything else
use rand::prelude::SliceRandom;
use std::thread;
use std::time::Duration;

const SIZE:u32 = 10;
const DELAY_MS:Duration = Duration::from_millis(200);

/******************************************************************************/
/*                             COMPARISON SORTS                               */
/******************************************************************************/
fn bubble_sort(vec: &mut Vec<u32>) {
    for i in (0..SIZE).rev() {
        for j in 0..i {
            let uj=j as usize;
            if vec[uj] > vec[uj+1] {
                vec[uj] ^= vec[uj+1];
                vec[uj+1] ^= vec[uj];
                vec[uj] ^= vec[uj+1];
            }
        }
        thread::sleep(DELAY_MS);
        println!("{:?}",vec);
    }
}

fn heapify(vec: &mut Vec<u32>,sz: u32,i: u32){
    let ui = i as usize;
    let mut max:usize = ui;
    let l:u32 = 2*i+1;
    let r:u32 = 2*i+2;
    if l < sz && vec[l as usize] > vec[max as usize]{
        max = l as usize;   
    }
    if r < sz && vec[r as usize] > vec[max as usize]{
        max = r as usize;   
    }
    if max != ui{
        vec[ui] ^= vec[max];
        vec[max] ^= vec[ui];
        vec[ui] ^= vec[max];
        heapify(vec,sz,max as u32);
    }
    
}

fn heap_sort(vec: &mut Vec<u32>) {
    for i in (0..(SIZE/2)).rev(){
        heapify(vec,SIZE,i);
        
        thread::sleep(DELAY_MS);
        println!("{:?}",vec); 
    }
    for i in (0..SIZE).rev(){
        let ui=i as usize;
        vec[0] ^= vec[ui];
        vec[ui] ^= vec[0];
        vec[0] ^= vec[ui];
        heapify(vec,i,0);
        
        thread::sleep(DELAY_MS);
        println!("{:?}",vec); 
    }
}

/******************************************************************************/
/*                           NON-COMPARISON SORTS                             */
/******************************************************************************/
fn pigeonhole_sort(vec: &mut Vec<u32>) {
    let min = vec.iter().min().clone().expect("value");
    let range = SIZE - min;
    let mut aux:Vec<u32> = vec![0; range as usize];
    for i in 0..range{
        let curr = vec[i as usize];
        let ind = curr - min;
        aux[ind as usize] = curr;
        thread::sleep(DELAY_MS);
        println!("{:?}",aux);
    }
    *vec = aux;
}

fn counting_sort(vec: &mut Vec<u32>){
    let mut count_vec:Vec<u32> = vec![0; (SIZE+1) as usize];
    let mut aux:Vec<u32> = vec![0;SIZE as usize];
    for i in 0..SIZE{
        let ind = vec[i as usize];
        count_vec[ind as usize] +=1;
        thread::sleep(DELAY_MS);
        println!("{:?}",count_arr);
    }
    for i in 0..SIZE-1{
        count_vec[(i+1) as usize] += count_vec[i as usize];
        thread::sleep(DELAY_MS);
        println!("{:?}",count_vec);
    }
    for i in (0..SIZE).rev(){
        let a = vec[i as usize];
        let b = count_vec[a as usize] - 1;
        count_vec[a as usize] -= 1;
        aux[b as usize] = a;
        thread::sleep(DELAY_MS);
        println!("{:?}",aux);
    }
    *vec = aux;
}

fn set_data(){
    let mut vec: Vec<u32> = (0..SIZE).collect();
    vec.shuffle(&mut rand::thread_rng());
    println!("{:?}\n",vec);
    counting_sort(&mut vec);
    println!("\n{:?}",vec);
}

fn main() {
    set_data()
}
