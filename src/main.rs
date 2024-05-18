struct Vector{
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

fn main() {
    //COMENCEMOS CON EL KORD.txt
    //Cargamos el archivo
    let kord = std::fs::read_to_string("KORD.txt").unwrap();
    //Imprimimos el contenido
    // println!("{}", kord);

    //RECORREMOS EL ARCHIVO
    //PARA LA PRIMERA LINEA DE CADA TABLA (LA QUE CONTIENE " KORD") TOMAMOS LA FECHA ENTRE LA PALABRA GUIDANCE Y EL PRIMERO ESPACIO EN BLANCO
    let mut dates: Vec<String> = Vec::new();

    for line in kord.lines(){
        let mut date = String::new();
        if line.contains(" KORD"){
            let mut words = line.split_whitespace();
            let mut word = words.next().unwrap();
            while word != "GUIDANCE"{
                word = words.next().unwrap();
            }
            date = words.next().unwrap().to_string();
            dates.push(date);
        }
    }
    println!("{:?}", dates);
}
