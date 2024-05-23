#![allow(dead_code, unused_variables)]

//rendering
use eframe::egui;

//everything else
use rand::prelude::SliceRandom;
use std::thread;
use std::time::Duration;
use std::cmp::PartialEq;
use std::fmt;

/******************************************************************************/
/*                              COMPARISON SORTS                              */
/******************************************************************************/
fn bubble_sort(vec: &mut Vec<u32>, delay: Duration) {
    for i in (0..vec.len()).rev() {
        for j in 0..i {
            let uj= j;
            if vec[uj] > vec[uj+1] {
                vec[uj] ^= vec[uj+1];
                vec[uj+1] ^= vec[uj];
                vec[uj] ^= vec[uj+1];
            }
        }
        thread::sleep(delay);
        // println!("{:?}",vec);
    }
}

fn selection_sort(vec: &mut Vec<u32>, delay: Duration) {
    for i in 0..vec.len(){
        let ui = i;
        let mut min = ui;
        for j in i+1..vec.len(){
            let uj= j;
            if vec[uj] < vec[min]{
                min = uj;
            }
        }
        let t = vec[ui];
        vec[ui] = vec[min];
        vec[min] = t;

        thread::sleep(delay);
        // println!("{:?}",vec);
    }
}

fn insertion_sort(vec: &mut Vec<u32>, delay: Duration){
    for i in 1..vec.len(){
        let ui = i;
        let mut uj = ui;
        let cur = vec[ui];

        while uj > 0 && cur < vec[uj - 1] {
            vec[uj] = vec[uj - 1];
            uj -= 1;
        }

        vec[uj] = cur;
        thread::sleep(delay);
        // println!("{:?}",vec);
    }
}

fn gnome_sort(vec: &mut Vec<u32>, delay: Duration) {
    let mut pos:usize = 0;
    while pos < vec.len(){
        if pos == 0 || vec[pos] >= vec[pos-1]{
            pos += 1;
        }else{
            vec[pos] ^= vec[pos-1];
            vec[pos-1] ^= vec[pos];
            vec[pos] ^= vec[pos-1];
            pos-=1;
        }
        thread::sleep(delay);
        println!("{:?}",vec);
    }
}

fn heapify(vec: &mut Vec<u32>,sz: u32,i: u32, delay: Duration){
    let ui = i as usize;
    let mut max:usize = ui;
    let l:u32 = 2*i+1;
    let r:u32 = 2*i+2;
    if l < sz && vec[l as usize] > vec[max]{
        max = l as usize;
    }
    if r < sz && vec[r as usize] > vec[max]{
        max = r as usize;
    }
    if max != ui{
        vec[ui] ^= vec[max];
        vec[max] ^= vec[ui];
        vec[ui] ^= vec[max];
        heapify(vec,sz,max as u32,delay);
    }

}

fn heap_sort(vec: &mut Vec<u32>, delay: Duration) {
    for i in (0..(vec.len()/2)).rev(){
        heapify(vec,vec.len() as u32, i as u32,delay);

        thread::sleep(delay);
        // println!("{:?}",vec);
    }
    for i in (0..vec.len()).rev(){
        let ui= i;
        vec[0] ^= vec[ui];
        vec[ui] ^= vec[0];
        vec[0] ^= vec[ui];
        heapify(vec,i as u32,0,delay);

        thread::sleep(delay);
        // println!("{:?}",vec);
    }
}

fn merge_sort(vec: &mut Vec<u32>, delay: Duration) {
    let len = vec.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left  = vec[0..mid].to_vec();
    let mut right = vec[mid..].to_vec();

    merge_sort(&mut left,delay);
    merge_sort(&mut right,delay);

    let mut i = 0; // Index for left half
    let mut j = 0; // Index for right half
    let mut k = 0; // Index for merged array
    
    //merge left & right
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            vec[k] = left[i];
            i += 1;
        } else {
            vec[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements from left half
    while i < left.len() {
        vec[k] = left[i];
        i += 1;
        k += 1;
    }

    // Copy remaining elements from right half
    while j < right.len() {
        vec[k] = right[j];
        j += 1;
        k += 1;
    }
    
    thread::sleep(delay);
    println!("{:?}",vec);
}

/******************************************************************************/
/*                            NON-COMPARISON SORTS                            */
/******************************************************************************/
fn pigeonhole_sort(vec: &mut Vec<u32>, delay: Duration) {
    let min = vec.iter().min().clone().expect("value");
    let range = vec.len() as u32 - min;
    let mut aux:Vec<u32> = vec![0; range as usize];
    for i in 0..range{
        let curr = vec[i as usize];
        let ind = curr - min;
        aux[ind as usize] = curr;
        thread::sleep(delay);
        // println!("{:?}",aux);
    }
    *vec = aux;
}

fn counting_sort(vec: &mut Vec<u32>, delay: Duration){
    let mut count_vec:Vec<u32> = vec![0; vec.len()+1];
    let mut aux:Vec<u32> = vec![0; vec.len()];
    for i in 0..vec.len(){
        let ind = vec[i];
        count_vec[ind as usize] +=1;
        thread::sleep(delay);
        // println!("{:?}",count_vec);
    }
    for i in 0..vec.len()-1{
        count_vec[i+1] += count_vec[i];
    }
    for i in (0..vec.len()).rev(){
        let a = vec[i];
        let b = count_vec[a as usize] - 1;
        count_vec[a as usize] -= 1;
        aux[b as usize] = a;
        thread::sleep(delay);
        // println!("{:?}",aux);
    }
    *vec = aux;
}

fn radix_helper(vec: &mut Vec<u32>,place: u32,radix: u32, delay: Duration){
    let mut count_vec:Vec<u32> = vec![0; vec.len()+1];
    let mut aux:Vec<u32> = vec![0; vec.len()];
    let digit_of = |x| x / place % radix;
    for i in 0..vec.len(){
        let ind = digit_of(vec[i]);
        count_vec[ind as usize] += 1;
    }

    for i in 1..vec.len(){
        let ui = i;
        count_vec[ui] += count_vec[ui-1];
    }

    for i in (0..vec.len()).rev(){
        let ui = i;
        let ind = digit_of(vec[ui]);
        let uind = ind as usize;
        count_vec[uind] -= 1;
        aux[count_vec[uind] as usize] = vec[ui];
        // println!("{:?}",aux);
    }
    *vec = aux;
}

fn radix_sort_lsd(vec: &mut Vec<u32>, delay: Duration, base: u32) {
    let mut mul = 1;
    let mut max = vec.len() as u32 - 1;
    let radix = base;
    while max > 0{
        radix_helper(vec,mul,radix,delay);
        mul*=radix;
        max/=radix;

        thread::sleep(delay);
    }
}
/******************************************************************************/
/*                             GRAPHICS HANDLING                              */
/******************************************************************************/
#[derive(PartialEq)]
enum Types {
    Bubble,
    Selection,
    Insertion,
    Heap,
    Merge,
    Pigeonhole,
    Counting,
    RadixLSD,
}

// string values for Types::
impl Types {
    fn as_str(&self) -> &'static str {
        match self {
            Types::Bubble => "Bubble Sort",
            Types::Selection => "Selection Sort",
            Types::Insertion => "Insertion Sort",
            Types::Heap => "Heap Sort",
            Types::Merge => "Merge Sort",
            Types::Pigeonhole => "Pigeonhole Sort",
            Types::Counting => "Counting Sort",
            Types::RadixLSD => "Radix Sort"
        }
    }
}

// formatting to allow Types:: to be printed
impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Types::Bubble => write!(f, "Bubble Sort"),
            Types::Selection => write!(f, "Selection Sort"),
            Types::Insertion => write!(f, "Insertion Sort"),
            Types::Heap => write!(f, "Heap Sort"),
            Types::Merge => write!(f, "Merge Sort"),
            Types::Pigeonhole => write!(f, "Pigeonhole Sort"),
            Types::Counting => write!(f, "Counting Sort"),
            Types::RadixLSD => write!(f, "Radix Sort")
        }
    }
}

struct MainWindow {
    sort_type: Types,
    vec_size: u32,
    delay_ms: u64,
    radix_base: u32,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            sort_type: Types::Bubble,
            vec_size: 10,
            delay_ms: 0,
            radix_base: 10,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Visual Sorting");

            // Dropdown menu for sorting types
            let selected_type:String = self.sort_type.to_string();
            ui.end_row();

            //initialize vec here
            //this means vec.len() can no longer be constant
            //going to have to replace all occurrences with vec.len()
            let mut vec: Vec<u32> = (0..self.vec_size).collect();
            let delay = Duration::from_millis(self.delay_ms);

            ui.horizontal(|ui| {
                ui.label("Sorting Type: ");

                //dropdown menu for all possible algorithms
                egui::ComboBox::from_label("")
                    .selected_text(format!("{}",self.sort_type))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(&mut self.sort_type, Types::Bubble, Types::Bubble.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Selection, Types::Selection.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Insertion, Types::Insertion.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Heap, Types::Heap.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Merge, Types::Merge.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Pigeonhole, Types::Pigeonhole.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Counting, Types::Counting.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::RadixLSD, Types::RadixLSD.to_string());
                    });
                ui.end_row();
            });

            //optional selector for the base used by radix sort
            if self.sort_type == Types::RadixLSD{
                //draggable value box for the delay between sorting iterations
                ui.horizontal(|ui| {
                    ui.label("Radix Base: ");
                    ui.add(egui::DragValue::new(&mut self.radix_base).speed(0.8));
                });
                ui.end_row();
                if self.radix_base < 2 {
                    self.radix_base = 2;
                }
                else if self.radix_base > 64 {
                    self.radix_base = 64;
                }
            }

            //draggable value box for # of elements in vec
            ui.horizontal(|ui| {
                ui.label("# of elements:");
                ui.add(egui::DragValue::new(&mut self.vec_size).speed(0.8));
                if self.vec_size < 1{
                    self.vec_size = 1;
                }
            });
            ui.end_row();

            //draggable value box for the delay between sorting iterations
            ui.horizontal(|ui| {
                ui.label("delay (ms): ");
                ui.add(egui::DragValue::new(&mut self.delay_ms).speed(0.8));
            });
            ui.end_row();

            //button to randomize the array
            if ui.button("Randomize Data").clicked() {
                vec.shuffle(&mut rand::thread_rng());
                // println!("{:?}",vec); // remove when no longer needed
            }
            ui.end_row();

            //figure out how to display the vec here, maybe use a plot :]
        });

    }
}

fn main() -> Result<(), eframe::Error>{
    //setup and display the window
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Visual Sorting",options,
        Box::new(|cc| {
            Box::<MainWindow>::default()
        }),
    )
}
