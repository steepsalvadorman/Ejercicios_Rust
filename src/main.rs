use std::io::{self, Write};


fn main() {
    //ejercicio1();
    //ejercicio2();
    ejercicio3();
}

fn ejercicio3(){
    
}





fn ejercicio2(){
    let mut parcial1 = String::new();
    let mut parcial2 = String::new();
    let mut parcial3 = String::new();

    print!("Ingrese la nota del primer parcial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut parcial1).expect("Error al leer la velocidad");
    let p1: f32 = parcial1.trim().parse().expect("Error al convertir la velocidad");
    println!("Ingrese la nota del segundo parcial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut parcial2).expect("Error al leer la velocidad");
    let p2: f32 = parcial2.trim().parse().expect("Error al convertir la velocidad");
    println!("Ingrese la nota del tercer parcial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut parcial3).expect("Error al leer la velocidad");
    let p3: f32 = parcial3.trim().parse().expect("Error al convertir la velocidad");

    let promedio: f32 = (p1 + p2 + p3) / 3.0;
    io::stdout().flush().unwrap();
    println!("El promedio es: {}", promedio);
}



 fn ejercicio1() {
     let mut velocidad = String::new();

     print!("Ingrese la velocidad: ");
     io::stdout().flush().unwrap(); // Limpia el buffer de salida para mostrar el mensaje

    io::stdin().read_line(&mut velocidad).expect("Error al leer la velocidad");

     let velocidad: f32 = velocidad.trim().parse().expect("Error al convertir la velocidad");

     let mut tiempo = String::new();

     print!("Ingrese el tiempo: ");
     io::stdout().flush().unwrap(); // Limpia el buffer de salida para mostrar el mensaje

     io::stdin().read_line(&mut tiempo).expect("Error al leer el tiempo");

     let tiempo: f32 = tiempo.trim().parse().expect("Error al convertir el tiempo");

     let distancia = velocidad * tiempo;

     println!("La distancia recorrida es: {}", distancia);
 }

