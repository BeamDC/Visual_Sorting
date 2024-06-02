#![allow(dead_code, unused_variables)]

use std::cmp::PartialEq;
use std::fmt;
use std::thread;
use std::time::Duration;

//rendering
use eframe::egui;
use egui::*;
use egui::Rect;

//everything else
use rand::prelude::SliceRandom;

/******************************************************************************/
/*                              COMPARISON SORTS                              */
/******************************************************************************/
fn bubble_sort(vec: &mut Vec<u32>, delay: Duration, ui: &mut Ui) {
    for i in (0..vec.len()).rev() {
        for j in 0..i {
            let uj= j;
            if vec[uj] > vec[uj+1] {
                vec.swap(uj,uj+1)
            }
            // display_vec(&vec, ui);
            thread::sleep(delay);
        }
        display_vec(&vec, ui);
        // println!("{:?}",vec);
    }
    display_vec(&vec, ui);
}

fn cocktail_shaker_sort(vec: &mut Vec<u32>, delay: Duration) {
    let mut flip:bool = true;
    let mut start:usize = 0;
    let mut end:usize = vec.len() - 1;

    while flip {
        flip = false;

        for i in start..end{
            if vec[i] > vec[i+1]{
                vec.swap(i,i+1);
                flip = true;
            }
        }

        if !flip{
            break;
        }

        thread::sleep(delay);
        // println!("{:?}",vec);

        flip = false;
        end-=1;

        for i in (start..end).rev(){
            if vec[i] > vec[i+1]{
                vec.swap(i,i+1);
                flip = true;
            }
        }

        start+=1;
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
        // println!("{:?}",vec);
    }
}

fn shell_sort(vec: &mut Vec<u32>, delay: Duration) {
    let gaps:Vec<u32> = vec![701,301,132,57,23,10,4,1];

    for gap in gaps{
        for i in gap as usize..vec.len(){
            let temp = vec[i];
            let ugap = gap as usize;
            let mut j = i;
            while j >= ugap && vec[j-ugap] > temp{
                vec[j] = vec[j-ugap];
                j-=ugap;
            }
            vec[j] = temp;
            thread::sleep(delay);
            // println!("{:?}",vec);
        }
    }
}

fn comb_sort(vec: &mut Vec<u32>, delay: Duration) {
    let len = vec.len();
    let mut gap = len;
    let mut swapped = true;
    while gap > 1 || swapped {
        gap = (4 * gap) / 5;
        if gap < 1 {
            gap = 1;
        }
        let mut i = 0;
        swapped = false;
        while i + gap < len {
            if vec[i] > vec[i + gap] {
                vec.swap(i, i + gap);
                swapped = true;
            }
            i += 1;
        }
        thread::sleep(delay);
        // println!("{:?}",vec);
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

fn partition(vec: &mut Vec<u32>, left: isize, right: isize, delay: Duration) -> isize {
    let pivot = right;
    let mut i: isize = left - 1;

    for j in left..=right - 1 {
        if vec[j as usize] <= vec[pivot as usize] {
            i += 1;
            vec.swap(i as usize, j as usize);
        }
    }

    vec.swap((i + 1) as usize, pivot as usize);
    thread::sleep(delay);
    // println!("{:?}",vec);
    i + 1
}

fn quick_sort_helper(vec: &mut Vec<u32>, left: isize, right: isize, delay: Duration) {
    if left <= right {
        let partition_idx = partition(vec, left, right, delay);
        quick_sort_helper(vec, left, partition_idx - 1, delay);
        quick_sort_helper(vec, partition_idx + 1, right, delay);
    }
}

fn quick_sort(vec: &mut Vec<u32>, delay: Duration) {
    quick_sort_helper(vec, 0, (vec.len() - 1) as isize, delay);
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

fn radix_helper(vec: &mut Vec<u32>,place: u32,radix: u32, delay: Duration,ui: &mut Ui){
    let mut count_vec:Vec<u32> = vec![0; vec.len()+1];
    let mut aux:Vec<u32> = vec![0; vec.len()];
    let digit_of = |x| x / place % radix;
    for i in 0..vec.len(){
        let ind = digit_of(vec[i]);
        count_vec[ind as usize] += 1;
    }

    for i in 1..vec.len(){
        count_vec[i] += count_vec[i-1];
    }

    for i in (0..vec.len()).rev(){
        let ind = digit_of(vec[i]) as usize;
        count_vec[ind] -= 1;
        aux[count_vec[ind] as usize] = vec[i];
        // println!("{:?}",aux);
        display_vec(vec,ui);
    }
    *vec = aux;
}

fn radix_sort_lsd(vec: &mut Vec<u32>, delay: Duration, base: u32,ui: &mut Ui) {
    let mut mul = 1;
    let mut max = vec.len() as u32 - 1;
    let radix = base;
    while max > 0{
        radix_helper(vec,mul,radix,delay,ui);
        mul*=radix;
        max/=radix;

        thread::sleep(delay);
        // println!("{:?}",vec);
    }
}
/******************************************************************************/
/*                             GRAPHICS HANDLING                              */
/******************************************************************************/
#[derive(PartialEq)]
enum Types {
    Bubble,
    Cocktail,
    Selection,
    Insertion,
    Gnome,
    Shell,
    Comb,
    Heap,
    Merge,
    Quick,
    Pigeonhole,
    Counting,
    RadixLSD,
}

// string values for Types::
impl Types {
    fn as_str(&self) -> &'static str {
        match self {
            Types::Bubble     => "Bubble Sort",
            Types::Cocktail   => "Cocktail Shaker Sort",
            Types::Selection  => "Selection Sort",
            Types::Insertion  => "Insertion Sort",
            Types::Gnome      => "Gnome Sort",
            Types::Shell      => "Shell Sort",
            Types::Comb       => "Comb Sort",
            Types::Heap       => "Heap Sort",
            Types::Merge      => "Merge Sort",
            Types::Quick      => "Quick Sort",
            Types::Pigeonhole => "Pigeonhole Sort",
            Types::Counting   => "Counting Sort",
            Types::RadixLSD   => "Radix Sort"
        }
    }
}

// formatting to allow Types:: to be printed
impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Types::Bubble     => write!(f, "Bubble Sort"),
            Types::Cocktail   => write!(f, "Cocktail Shaker Sort"),
            Types::Selection  => write!(f, "Selection Sort"),
            Types::Insertion  => write!(f, "Insertion Sort"),
            Types::Gnome      => write!(f, "Gnome Sort"),
            Types::Shell      => write!(f, "Shell Sort"),
            Types::Comb       => write!(f, "Comb Sort"),
            Types::Heap       => write!(f, "Heap Sort"),
            Types::Merge      => write!(f, "Merge Sort"),
            Types::Quick      => write!(f, "Quick Sort"),
            Types::Pigeonhole => write!(f, "Pigeonhole Sort"),
            Types::Counting   => write!(f, "Counting Sort"),
            Types::RadixLSD   => write!(f, "Radix Sort")
        }
    }
}

struct MainWindow {
    sort_type: Types,
    vec_size: u32,
    delay_ms: u64,
    radix_base: u32,
    vec: Vec<u32>,
    // chart: BarChart,
    allow_zoom: Vec2b,
    allow_drag: Vec2b,
    allow_scroll: Vec2b,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            sort_type: Types::Bubble,
            vec_size: 10,
            delay_ms: 0,
            radix_base: 4,
            vec: (0..30).collect(),
            // chart: BarChart::new(vec![
            //     Bar::new(1.0,1.0),
            //     Bar::new(2.0,2.0),
            //     Bar::new(3.0,3.0),
            //     Bar::new(4.0,4.0),
            //     Bar::new(5.0,5.0)
            // ])
            //     .color(Color32::WHITE)
            //     .width(1.0),
            allow_zoom: false.into(),
            allow_drag: false.into(),
            allow_scroll: false.into(),
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // store the current vec size;
            let prev_size = self.vec_size;

            // Dropdown menu for sorting types
            let selected_type:String = self.sort_type.to_string();
            ui.end_row();

            //initialize vec and delay
            let delay = Duration::from_millis(self.delay_ms);

            //dropdown menu for selecting sorts
            ui.horizontal(|ui| {
                ui.label("Sorting Type: ");

                //dropdown menu for all possible algorithms
                ComboBox::from_label("")
                    .selected_text(format!("{}",self.sort_type))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(&mut self.sort_type, Types::Bubble, Types::Bubble.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Cocktail, Types::Cocktail.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Selection, Types::Selection.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Insertion, Types::Insertion.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Gnome, Types::Gnome.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Shell, Types::Shell.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Comb, Types::Comb.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Heap, Types::Heap.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Merge, Types::Merge.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Quick, Types::Quick.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Pigeonhole, Types::Pigeonhole.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::Counting, Types::Counting.to_string());
                        ui.selectable_value(&mut self.sort_type, Types::RadixLSD, Types::RadixLSD.to_string());
                    });
                ui.end_row();
            });

            //optional selector for the base used by radix sort
            if self.sort_type == Types::RadixLSD{
                //draggable value box for the base used in radix sort
                ui.horizontal(|ui| {
                    ui.label("Radix Base: ");
                    ui.add(DragValue::new(&mut self.radix_base).speed(0.8));
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
                ui.add(DragValue::new(&mut self.vec_size).speed(0.8));
                if self.vec_size < 2{
                    self.vec_size = 2;
                }
                else if self.vec_size > 10000{
                    self.vec_size = 10000;
                }
            });
            ui.end_row();

            //if the vector has changed in size, update it
            if self.vec_size != prev_size{
                self.vec = (0..self.vec_size).collect();
            }

            //draggable value box for the delay between sorting iterations
            ui.horizontal(|ui| {
                ui.label("delay (ms): ");
                ui.add(DragValue::new(&mut self.delay_ms).speed(0.8));
            });
            ui.end_row();

            //button to randomize the vector
            if ui.button("Randomize Data").clicked() {
                self.vec.shuffle(&mut rand::thread_rng());
                // println!("{:?}",vec); // remove when no longer needed
            }
            ui.end_row();

            //button to sort the vector
            if ui.button("Sort Data").clicked() {
                //execute the sorting
                bubble_sort(&mut self.vec,delay,ui);
            }
            ui.end_row();

            //try using rects instead
            ui.painter_at(Rect::EVERYTHING);

            //loop through vec, and draw bars;
            display_vec(&self.vec, ui);
        });
    }
}

fn display_vec(vec: &Vec<u32>, ui: &mut Ui){
    let x_offset = 10.0;
    let y_offset = 400.0;
    for (x,y) in vec.iter().enumerate() {
        let x:f32 = x_offset + x as f32;
        let y:f32 = (*y as f32 / (vec.len() as f32 - 1.0))* 200.0;

        let min = Pos2::new(x,(y_offset - y) + 1.0);
        let max = Pos2::new(x+1.0,y_offset);
        // println!("Min: {},{}\nMax: {},{}",min.x,min.y,max.x,max.y);
        let bar = Rect::from_min_max(min,max);
        ui.painter().rect_filled(bar,Rounding::ZERO,Color32::WHITE);
    }
}

fn main() -> Result<(), eframe::Error>{
    //setup and display the window
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([640.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native("Visual Sorting",options,
                       Box::new(|cc| {
                           Box::<MainWindow>::default()
                       }),
    )
}
