#![allow(dead_code, unused_variables)]

//rendering
use eframe::egui;

//everything else
use rand::prelude::SliceRandom;
use std::thread;
use std::time::Duration;

const SIZE:u32 = 10;
const DELAY_MS:Duration = Duration::from_millis(0);

/******************************************************************************/
/*                              COMPARISON SORTS                              */
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
        // println!("{:?}",vec);
    }
}

fn selection_sort(vec: &mut Vec<u32>) {
    for i in 0..SIZE{
        let ui = i as usize;
        let mut min = ui;
        for j in i+1..SIZE{
            let uj=j as usize;
            if vec[uj] < vec[min]{
                min = uj;
            }
        }
        let t = vec[ui];
        vec[ui] = vec[min];
        vec[min] = t;

        thread::sleep(DELAY_MS);
        // println!("{:?}",vec);
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
        // println!("{:?}",vec);
    }
    for i in (0..SIZE).rev(){
        let ui=i as usize;
        vec[0] ^= vec[ui];
        vec[ui] ^= vec[0];
        vec[0] ^= vec[ui];
        heapify(vec,i,0);

        thread::sleep(DELAY_MS);
        // println!("{:?}",vec);
    }
}

//modify this to work in place
fn merge(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32>{
    let mut i = 0;
    let mut j = 0;
    let mut merged:Vec<u32> = Vec::new();
    while i<left.len() && j<right.len(){
        if left[i] < right[j]{
            merged.push(left[i]);
            i+=1;
        }else{
            merged.push(right[j]);
            j+=1;
        }
    }
    
    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }
    thread::sleep(DELAY_MS);
    println!("{:?}",merged);
    merged
}

fn merge_sort(vec: &Vec<u32>) -> Vec<u32> {
    if vec.len() < 2{
        vec.to_vec()
    }else{
        let mid = vec.len()/2;
        let left = merge_sort(&vec[0..mid].to_vec());
        let right = merge_sort(&vec[mid..].to_vec());
        let merged = merge(&left,&right);

        thread::sleep(DELAY_MS);
        //println!("{:?}",vec);
        merged   
    }
}

/******************************************************************************/
/*                            NON-COMPARISON SORTS                            */
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
        // println!("{:?}",aux);
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
        // println!("{:?}",count_vec);
    }
    for i in 0..SIZE-1{
        count_vec[(i+1) as usize] += count_vec[i as usize];
    }
    for i in (0..SIZE).rev(){
        let a = vec[i as usize];
        let b = count_vec[a as usize] - 1;
        count_vec[a as usize] -= 1;
        aux[b as usize] = a;
        thread::sleep(DELAY_MS);
        // println!("{:?}",aux);
    }
    *vec = aux;
}

fn radix_helper(vec: &mut Vec<u32>,place: u32,radix: u32){
    let mut count_vec:Vec<u32> = vec![0; (SIZE+1) as usize];
    let mut aux:Vec<u32> = vec![0;SIZE as usize];
    let digit_of = |x| x / place % radix;
    for i in 0..SIZE{
        let ind = digit_of(vec[i as usize]);
        count_vec[ind as usize] += 1;
    }
    
    for i in 1..SIZE{
        let ui = i as usize;
        count_vec[ui] += count_vec[ui-1];
    }
    
    for i in (0..SIZE).rev(){
        let ui = i as usize;
        let ind = digit_of(vec[ui]);
        let uind = ind as usize;
        count_vec[uind] -= 1;
        aux[count_vec[uind] as usize] = vec[ui];
        // println!("{:?}",aux);
    }
    *vec = aux;
}

fn radix_sort_lsd(vec: &mut Vec<u32>) {
    let mut mul = 1;
    let mut max = SIZE-1;
    let radix = 10;
    while max > 0{
        radix_helper(vec,mul,radix);
        mul*=radix;
        max/=radix;
        
        thread::sleep(DELAY_MS);
        // println!("{:?}",vec);
    }
}

fn set_data(){
    let mut vec: Vec<u32> = (0..SIZE).collect();
    vec.shuffle(&mut rand::thread_rng());
    println!("{:?}\n",vec);
    counting_sort(&mut vec);
    println!("\n{:?}",vec);
}

/******************************************************************************/
/*                             GRAPHICS HANDLING                              */
/******************************************************************************/
struct MainWindow {
    sort_type: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            sort_type: "Bubble Sort".to_owned(),
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Visual Sorting");

            // Dropdown menu for sorting types
            ui.label(format!("Selected Sorting Algorithm: '{}'", self.sort_type));

            ui.horizontal(|ui| {
                let label = ui.label("Sorting Type:");

                let algorithms =
                    ["Bubble Sort", "Selection Sort", "Insertion Sort", "Heap Sort",
                        "Pigeonhole Sort", "Counting Sort"];

                //dropdown menu for all possible algorithms
            });
        });

    }
}

fn main() -> Result<(), eframe::Error>{
    
    //setup and display the window
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Visual Sorting",
        options,
        Box::new(|cc| {
            Box::<MainWindow>::default()
        }),
    )
}
