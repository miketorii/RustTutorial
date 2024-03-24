fn main() {
    println!("---variables---");

    let mut x = 5;
    println!("num= {}", x);
    x = 6;
    println!("num= {}", x);

    println!("---const---");

    const MAX_POINTS: u32 = 100_000;
    println!("const val = {}", MAX_POINTS);

    // shadowing
    let y = 5;
    let y = y+1;
    println!("num={}", y);

    println!("---functions---");
   
    func1(23, 'm');

    let y1 = {
        let x = 3;
        x + 1
    };
    println!("func define={}", y1);

    let ret = func2(24);
    println!("func ret={}", ret);

    println!("---flow if---");
    let number = 100;
    if number < 5 {
        println!("number is less than 5.")
    } else if number == 100 {
        println!("number is 100")
    } 
    else {
        println!("number is more than 5.")
    }

    println!("---flow loop---");

    let mut cnt = 0;
    loop {
        if cnt > 3 {
            break;
        }
        println!("loop test");
        cnt+=1;
    }    

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    println!("---flow while---");

    let mut num2 = 3;

    while num2 != 0 {
        println!("num2={}", num2);
        num2 -= 1;
    }
    println!("End while");

    println!("---flow for---");
    let a = [10,20,30,40,50];
    for element in a {
        println!("array element={}", element);
    }

}

fn func1(value: i32, label: char){
    println!("arg={}, {}", value, label);
}

fn func2(val: i32) -> i32 {
    val + 100
}