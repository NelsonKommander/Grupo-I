fn func(x: f32) -> f32{
    return 10.0*x-x*x;
}

fn trapezio(a: f32, b:f32, n:i32) -> f32 {
    let mut sum = 0.0;
    let h = (b-a)/(n as f32);

    for i in 1..n {
        let x = a+(i as f32)*h;
        sum = sum + func(x as f32);
    }

    return h*(((func(a)+func(b))/2.0)+sum);
}

fn main() {
    let trabalho;
    trabalho = trapezio(0.0, 10.0, 100);
    println!("Valor:     {}", trabalho);
}