use std::{
    env, fs,
    io::{stdout, Write},
    panic::set_hook,
    path,
    process::exit,
    thread,
    time::Duration,
};

fn traverse(path: &path::Path, list: &mut Vec<u64>) {
    if path.is_dir() {
        let iter = fs::read_dir(path).unwrap().map(|x| x.unwrap());
        for file in iter {
            if file.path().is_dir() {
                traverse(file.path().as_path(), list)
            } else {
                let size = file.metadata();
                match size {
                    Ok(size) => list.push(size.len()),
                    Err(_) => println!("Permission denied: {}", file.path().to_string_lossy()),
                };
            }
        }
    }
}

fn main() {
    // user friendly panic
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
        exit(0);
    }));

    let _a = env::args()
        .nth(1)
        .expect("usage: traverse-dirs <path> [bound=4096]");
    let path: &path::Path = path::Path::new(&_a);

    let bound: u64 = env::args()
        .nth(2)
        .unwrap_or("4096".into())
        .parse()
        .expect("option: failed to parse number");

    print!("This may take a while...");
    thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(1));
        print!(".");
        stdout().flush().ok();
    });
    stdout().flush().ok();

    let mut list: Vec<u64> = Vec::with_capacity(50_000);
    traverse(path, &mut list);

    let size: u64 = list.iter().sum();
    println!("");
    println!("");
    println!("Total size:   {}", human_bytes::human_bytes(size as f64));
    println!(
        "Average size: {}",
        human_bytes::human_bytes((size / list.len() as u64) as f64)
    );
    println!("Distribution:");

    let under4kb = list.iter().filter(|&x| x < &bound).count() as f64 / list.len() as f64;
    let over4kb = list.iter().filter(|&x| x >= &bound).count() as f64 / list.len() as f64;
    println!("<4 KiB {:.1}%", under4kb * 100.);
    println!(">4 KiB {:.1}%", over4kb * 100.);
}
