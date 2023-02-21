use std::f64::consts::PI;
use itertools_num::linspace;
use std::fs::File;
use csv;

const T: usize = 10000;
const K: f64 = 0.5;
const Y0_PRECISION: usize = 100;

fn standard_map(x0: f64, y0: f64, k: f64) -> [Vec<f64>; 2]{
    let mut x = vec![0.0; T+1];
    let mut y = vec![0.0; T+1];
    x[0] = x0;
    y[0] = y0;
    // evolve
    for i in 0..T{
        y[i+1] = y[i] - (k/(2.0*PI))*(2.0*PI*x[i]).sin();
        x[i+1] = x[i] + y[i+1];
    }
    // modulo 1
    let x = x.iter().map(|x| x - x.floor() - 0.5).collect::<Vec<_>>();
    let y = y.iter().map(|y| y - 0.0).collect::<Vec<_>>();
    
    [x, y]
}


fn main(){
    let time = std::time::Instant::now();
    let x0: f64 = -0.5;
    let y0_all = linspace(-0.45, 0.45, Y0_PRECISION);
        
    // save to file part after part
    let file = File::create("standard_map.csv").unwrap();
    for y0 in y0_all{
        let [x,y] = standard_map(x0, y0, K);
        
        let mut wtr = csv::Writer::from_writer(&file);
        for i in 0..T+1{
            wtr.write_record(&[x[i].to_string(), y[i].to_string()]).unwrap();
        }
        
    }
    println!("Time elapsed: {} ms", time.elapsed().as_millis());
    // precalculate all values
    // let standard_map_values = y0.map(|y0| {
    //     standard_map(x0, y0, 0.5)
    // }).collect::<Vec<_>>();
    
} 

