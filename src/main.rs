use std::{
    env, fs,
    io::{stdout, Error, Write},
    panic::set_hook,
    path,
    process::exit,
    thread,
    time::Duration,
};

fn traverse(path: &path::Path, list: &mut Vec<u64>) -> Result<(), Error> {
    if path.is_dir() {
        let iter = fs::read_dir(path)?.map(|x| x.unwrap());
        for file in iter {
            if file.path().is_dir() {
                if let Err(err) = traverse(file.path().as_path(), list) {
                    print!("\n{err}: {}", file.path().to_string_lossy());
                }
            } else {
                let size = file.metadata();
                list.push(size.unwrap().len());
            }
        }
    }
    Ok(())
}

fn main() {
    // user friendly panic
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
        exit(0);
    }));

    let _exe = env::current_exe().unwrap();
    let exec_name = _exe.file_stem().unwrap().to_str().unwrap();

    let _a = env::args()
        .nth(1)
        .expect(format!("usage: {exec_name} <path> [bound=4096]").as_str());
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
    traverse(path, &mut list).unwrap();

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
    println!(
        "<{} {:.1}%",
        human_bytes::human_bytes(bound as f64),
        under4kb * 100.
    );
    println!(">{} {:.1}%", human_bytes::human_bytes(bound as f64), over4kb * 100.);
}
