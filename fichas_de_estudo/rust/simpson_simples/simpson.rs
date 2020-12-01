
fn f(x: f32) -> f32{
    let aux = 1.0 - (x*x);
    return aux.sqrt();
}

fn simps(a: f32, b: f32, n: i32) -> f32{
    let h;
    let y0;
    let yn;
    let sum;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    h = (b-a)/(n as f32);
    y0 = f(a+(0.0*h));
    yn = f(a+((n as f32)*h));

    for i in 1..n {
        if i%2 == 0 {
            sum1 = sum1 + f(a+((i as f32)*h));
        } else {
            sum2 = sum2 + f(a+((i as f32)*h));
        }
    }

    sum = (h/3.0)*(y0 + yn + (2.0*sum1) + (4.0*sum2));
    return sum;
}

fn main(){
    let mat_pi = std::f32::consts::PI;
    let pi;
    pi = 4.0*simps(0.0, 1.0, 1000);
    println!("Valor:     {}", pi);
    println!("Precisão: {}", (pi-mat_pi));
}