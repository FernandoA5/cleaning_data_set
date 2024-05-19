use std::process::exit;

#[derive(Debug)]
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
    //PARA LA PRIMERA LINEA DE CADA TABLA (LA QUE CONTIENE " KORD") TOMAMOS LA FECHA ENTRE LA PALABRA GUIDANCE Y EL PRIMERO ESPACIO EN BLANCO. ENTRE LA FECHA Y LA PALABRA UTC, ESTÁ LA HORA.
    let mut dates: Vec<String> = Vec::new();
    let mut hours: Vec<String> = Vec::new();

    for line in kord.lines(){
        // println!("{}", line);
        let mut words = line.split_whitespace();
        let mut word = words.next();
        if word == Some("KORD"){
            // println!("{}", line);
            let mut date = String::new();
            let mut hour = String::new();
            while word != Some("UTC"){
                if word == Some("GUIDANCE"){
                    date = words.next().unwrap().to_string();
                    hour = words.next().unwrap().to_string();
                    break;
                }
                word = words.next();
            }
            dates.push(date);
            hours.push(hour);
        }
    }

    // println!("{:?}", dates);
    // println!("{:?}", hours);

    //AHORA EXTRAEMOS LOS HEADERS DE LA TABLA. EMPEZAMOS CON LA LINEA SIGUIENTE A LA QUE CONTIENE " KORD"
    //LA PRIMERA PALABRA DE CADA LINEA ES EL HEADER, SOLO HASTA LA QUE COMIENZA CON " OBV".
    let mut headers: Vec<String> = Vec::new();
    let mut header_found = false;

    for line in kord.lines(){
        if line.starts_with(" KORD"){
            header_found = true;
        }
        if header_found{
            let mut words = line.split_whitespace();
            let word = words.next();

            //SI LA LINEA ESTÁ VACÍA, TERMINAMOS EL PROCESO
            if word == None{
                break;
            }
            // println!("{}", word.unwrap());
            headers.push(word.unwrap().to_string());
        }
    }
    //ELIMINAMOS EL PRIMER ELEMENTO, QUE ES "KORD"
    headers.remove(0);

    //RECORREMOS TODAS LAS LINEAS DEL ARCHIVO. INMEDIATAMENTE DESPUES DEL HEADER, CADA 3 POSICIONES ES UN DATO.
    //THAT MEANS, NO PODEMOS USAR EL METODO SPLIT_WHITESPACE, SINO QUE DEBEMOS USAR UNA VARIABLE QUE NOS INDIQUE LA POSICION DE LA PALABRA QUE ESTAMOS LEYENDO.
    //SOLO DESPUES DE LA FILA QUE COMIENZA CON " KORD" HAY DATOS. (EXCLUIMOS LA QUE COMIENZA CON " KORD")

    let mut data_vector: Vec<Vec<String>> = Vec::new();
    let mut data_found = false;

    for line in kord.lines(){
        if line.starts_with(" KORD"){
            data_found = true;
            // println!("\n");
        }
        if data_found && !line.starts_with(" KORD"){
            //EL HEADER OCUPA LAS PRIMERAS 4 POSICIONES DE LA LINEA, OBTENEMOS EL RESTO DE LA LINEA
            //SI LA LINEA ESTÁ VACÍA, TERMINAMOS EL PROCESO
            let mut words = line.split_whitespace();
            let mut word = words.next();
            if word != None{
                let data_line = &line[5..];
                let mut data: Vec<String> = data_line.chars()
                    .collect::<Vec<char>>()
                    .chunks(3)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect();

                //ELIMINAMOS EL ULTIMO ELEMENTO, QUE ES UN ESPACIO EN BLANCO
                data.pop();
                // println!("{:?}", data);
                data_vector.push(data);
            }

        }
    }

    // println!("{:?}", data_vector);

    let final_vector: Vector = create_data_vector(headers, dates, hours, data_vector);
    
    //MOSTRAMOS LOS PRIMEROS 5 ELEMENTOS DE CADA VEC en VECTOR
    println!("{:?}", final_vector.headers);
    //ESCRIBIMOS EL VECTOR EN UN ARCHIVO CSV
    let mut writer = csv::Writer::from_path("KORD.csv").unwrap();
    
    writer.write_record(&final_vector.headers).unwrap();
    let mut contador = 0;
    for row in final_vector.data {
        writer.write_record(&row).unwrap();
    }

    writer.flush().unwrap();


}


fn create_data_vector(headers: Vec<String>, dates: Vec<String>, hours: Vec<String>, data_vector: Vec<Vec<String>>) -> Vector{
    //A LOS HEADERS, AL COMIENZO LES AGREGAMOS "DATE" Y "HOUR"
    let mut new_headers = headers.clone();
    new_headers.insert(0, "DATE".to_string());
    new_headers.insert(1, "HOUR".to_string());

    let total_rows = headers.len();
    println!("{:?}", total_rows);

    //EN LA PRIMER POSICION DE CADA FILA, AGREGAMOS LA FECHA Y EN LA SEGUNDA LA HORA
    let mut new_data: Vec<Vec<String>> = Vec::new();
    for i in 0..dates.len(){
        let mut new_row: Vec<String> = Vec::new();
        new_row.push(dates[i].to_string());
        new_row.push(hours[i].to_string());

        for j in 0..data_vector.len(){
            // println!("{:?}", data_vector[j][0]);
            new_row.push(data_vector[j][0].to_string());
        }
        
        new_data.push(new_row);
    }
    //TERMINAMOS EL PROGRAMA
    // exit(0);


    // println!("{:?}", new_data);

    let final_vector = Vector{
        headers: new_headers,
        data: new_data,
    };

    final_vector
}