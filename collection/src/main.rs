fn main() {
    println!("---vector---");

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    for i in v.iter() {
        println!("val={}", i);
    }

    let third: &i32 = &v[2];
    println!("third={}", third);

    let val = v.pop();
    println!("val={:?}", val);        

    let val2 = v.get(1);
    println!("val2={:?}", val2);        

    for j in v.iter_mut(){
        *j += 3;
        //println!("j={}", j);        
    }
    for k in v.iter() {
        println!("val={}", k);
    }
}
