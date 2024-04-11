use std::io::Write;



fn main() {
    //ejercicio1();
    //ejercicio2();
    //ejercicio3();
    //ejercicio4();
    //ejercicio5();
    //ejercicio6();
    ejercicio7()
}

/*

Ejercicio 3
Elaborar un algoritmo que solicite el numero de respuestas correctas, incorrectas y en blanco,
correspondientes a postulantes, y muestre su puntaje final considerando, que por cada respuesta
correcta tendrá 4 puntos, respuestas incorrectas tendra -1 y respuestas en blanco tendra 0.

Nota: trim() elimina los espacios en blanco al principio y al final de 
la cadena incluyendo el caracter de salto de linea.

Ejercicio 4

Elaborar un algoritmo que permita ingresar el numero de partidos ganados, perdidos y empatados,
por algun equipo en el torneo apertura, se debe de mostrar su puntaje total, teniendo en cuenta
por cada partido ganado obtendra 3 puntos, empatado 1 punto y perdido 0 puntos.


Ejercicio 5

Se requiere el algoritmo para elaborar la planilla de un empleado. Para ello se dispone de sus horas 
laboradas en el mes, asi como de la tarifa por hora.

Ejercicio 6

Elabore un algoritmo que lea los 3 lados de un triangulo cualquiera y calcule su area, considerar:
Si A,B y C son los lados, y S el semiperimetro

Ejercicio 7

Para hacer una copia de seguridad, de la informacion almacenada en un disco cuya capacidad
se conoce. Considerar que el disco duro esta lleno de informacion, ademas expresado en
gigabyte. Un CD virgen tiene 700 Megabytes de capacidad y una Gigabyte es igual a 1,024 
megabyte.
*/ 

fn ejercicio7(){

    let mut gigabyte = String::new();

    println!("Ingresa la cantidad de Gigabyte del disco duro");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut gigabyte).expect("Error al leer la cantidad de gigas");
    let gigabyte: i32 = gigabyte.trim().parse().expect("Error al convertir gigabyte en un numero");
    let megabyte: i32 = gigabyte * 1024;
    println!("La cantidad de CDs que se necesita es {}", ((megabyte/700)+1));
}





/*
fn ejercicio6(){

    let mut lado_a = String::new();
    let mut lado_b = String::new();
    let mut lado_c = String::new();

    println!("Ingresa el valor del lado A: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut lado_a).expect("Error al leer el lado A");
    let lado_a: f32 = lado_a.trim().parse().expect("Error al convertir lado_a a número");

    println!("Ingresa el valor del lado B: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut lado_b).expect("Error al leer el lado B");
    let lado_b: f32 = lado_b.trim().parse().expect("Error al convertir lado_b a número");

    println!("Ingresa el valor del lado C: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut lado_c).expect("Error al leer el lado C");
    let lado_c: f32 = lado_c.trim().parse().expect("Error al convertir lado_c a número");

    let x: f32 = (lado_a + lado_b + lado_c) / 2.0;
    let solution = (x * (x - lado_a) * (x - lado_b) * (x - lado_c)).sqrt();

    println!("Su respuesta es {}", solution);


}


fn ejercicio5(){

    let mut horas_laborables = String::new();
    let mut tarifa_hora: String = String::new();

    print!("Ingrese el numero de horas laboradas en el mes: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut horas_laborables).expect("Erro al leer horas laboradas");
    let h: f32 = horas_laborables.trim().parse().expect("Error al convertir las horas laboradas");
    print!("Ingrese el precio de tarifa por hora: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tarifa_hora).expect("Error al leer el costo por hora");
    let t: f32 = tarifa_hora.trim().parse().expect("Error al convertir el costo a entero");

    let resultado: f32 = h*t;
    println!("El resultado para su planilla es: {}", resultado);
}

fn ejercicio4(){
    
    let mut ganados = String::new();
    let mut perdidos = String::new();
    let mut empatados = String::new();

    print!("Ingrese la cantidad de partidos ganados: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ganados).expect("Error al leer la cantidad de partidos ganados");
    let g: i32 = ganados.trim().parse().expect("Error al convertir ganados a entero");
    print!("Ingrese la cantidad de partidos perdidos: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut perdidos).expect("Error al leer partidos perdidos");
    let p: i32 = perdidos.trim().parse().expect("Error al convertir los partidos perdidos a entero");
    print!("Ingrese la cantidad de partidos empatados: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut empatados).expect("Error al leer los partidos empatados");
    let e: i32 = empatados.trim().parse().expect("Error al convertir los partidos empatados a entero");

    let resultado = (g*3)+ (e*1) + (p*0);
    io::stdout().flush().unwrap();
    println!("El putnaje total del equipo es: {}", resultado);


}

fn ejercicio3(){

    let mut correctas = String::new();
    let mut incorrectas = String::new();
    let mut en_blanco = String::new();

    print!("Ingrese cantidad de respuestas correctas: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut correctas).expect("Error al leer la cantidad de respuestas correctas");
    let c: i32 = correctas.trim().parse().expect("Error al convertir la cantidad de respuestas correctas");
    print!("Ingrese la cantidad de respuestas incorrectas: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut incorrectas).expect("Error al leer la cantidad de respuestas incorrectas");
    let i: i32 = incorrectas.trim().parse().expect("Error al convertir la cantidad de respuestas incorrectas");
    print!("Ingrese la cantidad de respuestas en blanco: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut en_blanco).expect("Error al leer la cantidad de respuestas en blanco");
    let e: i32 = en_blanco.trim().parse().expect("Error al convertir la cantidad de respuestas en blanco");

    let puntaje: i32 = (c * 4) + (i * -1) + (e * 0);

    io::stdout().flush().unwrap();
    println!("El puntaje final es: {}", puntaje);


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
     io::stdout().flush().unwrap(); 

    io::stdin().read_line(&mut velocidad).expect("Error al leer la velocidad");

     let velocidad: f32 = velocidad.trim().parse().expect("Error al convertir la velocidad");

     let mut tiempo = String::new();

     print!("Ingrese el tiempo: ");
     io::stdout().flush().unwrap(); 

     io::stdin().read_line(&mut tiempo).expect("Error al leer el tiempo");

     let tiempo: f32 = tiempo.trim().parse().expect("Error al convertir el tiempo");

     let distancia = velocidad * tiempo;

     println!("La distancia recorrida es: {}", distancia);
 }

*/