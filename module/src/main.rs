mod libfunc;
mod libfolder;

fn main() {
    println!("---start---");

    libfunc::libfuncA();

    let c = libfunc::libfuncAdd(1, 2);
    println!("libfuncAdd={}", c);

    let d = libfunc::libfuncReturn10(100);
    println!("libfuncReturn10={}", d);

    libfolder::libmethods::libfuncB();

    let e = libfolder::libmethods::libfuncReturn1000(999);
    println!("libfolder::libmethods::libfuncReturn1000={}", e);

}
