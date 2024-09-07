use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::thread;
use std::sync::{mpsc, Arc};

fn get_arr(file: String) -> Vec<Vec<f64>> {
    let f = BufReader::new(File::open(file).unwrap());
    let arr: Vec<Vec<f64>> = f.lines()
        .map(|l| l.unwrap().split(char::is_whitespace)
             .map(|number| number.parse().unwrap())
             .collect())
        .collect();
    return arr
}

fn mat_mul(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a[0].len();
    let m = b.len();
    if n != m {
        println!("The number of columns in the first matrix must be equal to the number of rows in the second matrix!");
        process::exit(1);
    }

    let thread_count = 4;
    let mut threads: Vec<_> = Vec::new();
    let (tx, rx) = mpsc::channel();
    let amat1 = Arc::new(a.clone());
    let amat2 = Arc::new(b.clone());
    let mut c: Vec<Vec<f64>> = vec![];

    for th in 0..thread_count {
        let tx = tx.clone();
        let amat1 = Arc::clone(&amat1);
        let amat2 = Arc::clone(&amat2);
        threads.push( thread::spawn(move || {
            //println!("thread-{} started!", th);

            let start = (th*n)/thread_count;
            let end = (n*(th+1))/thread_count;
            let mut ans:Vec<Vec<f64>> = vec![vec![]; n];
            for i in start..end{
                for j in 0..n{
                    let mut cur: f64 = 0.0;
                    for k in 0..n{
                        cur += amat1[i][k]*amat2[k][j];
                    }
                    ans[i].push(cur);
                }
            }
            tx.send((start, end, ans)).unwrap();
        }));
    }

    let mut ans:Vec<Vec<f64>> = vec![vec![]; n];
    for v in rx.iter().take(thread_count){
        let (start, end, mat) = v;
        for i in start..end{
            ans[i].extend(&mat[i]);
        }
    }

    for handle in threads {
        handle.join().unwrap();
    }

    return ans; 
}

fn print_mat(a: &Vec<Vec<f64>>) {
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            print!("{} ", a[i][j]);
        }
        println!("");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Input:");
    let a = get_arr("m1.txt".to_owned());
    let b = get_arr("m2.txt".to_owned());
    print_mat(&a);
    println!("-----");
    print_mat(&b);
    println!("Result:");
    let c = mat_mul(a, b);
    print_mat(&c);

    Ok(())
}