use std::process::exit;

#[derive(Debug)]
struct Vector{
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

const NAMESTATION: &str = " KEWR";
const FILENAME: &str = "KEWR";

fn main() {
    //COMENCEMOS CON EL KORD.txt
    //Cargamos el archivo
    let path = format!("{}.txt", FILENAME);
    let kord = std::fs::read_to_string(path).unwrap();
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
        if word == Some("KEWR"){
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

    //AHORA EXTRAEMOS LOS HEADERS DE LA TABLA. EMPEZAMOS CON LA LINEA SIGUIENTE A LA QUE CONTIENE " KORD"
    //LA PRIMERA PALABRA DE CADA LINEA ES EL HEADER, SOLO HASTA LA QUE COMIENZA CON " OBV".
    let mut headers: Vec<String> = Vec::new();
    let mut header_found = false;

    for line in kord.lines(){
        if line.starts_with(NAMESTATION){
            header_found = true;
        }
        if header_found{
            let mut words = line.split_whitespace();
            let word = words.next();

            //SI LA LINEA ESTÁ VACÍA, TERMINAMOS EL PROCESO
            if word == None{
                break;
            }
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
        if line.starts_with(NAMESTATION){
            data_found = true;
        }
        if data_found && !line.starts_with(NAMESTATION){
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
                data_vector.push(data);
            }

        }
    }

    // println!("{:?}", data_vector);

    let final_vector: Vector = create_data_vector(headers, dates, hours, data_vector.clone());
    
    //MOSTRAMOS LOS PRIMEROS 5 ELEMENTOS DE CADA VEC en VECTOR
    // println!("{:?}", data_vector);

    //ESCRIBIMOS EL VECTOR EN UN ARCHIVO CSV
    let path = format!("{}.csv", FILENAME);
    let mut writer = csv::Writer::from_path(path).unwrap();
    
    writer.write_record(&final_vector.headers).unwrap();
    for row in final_vector.data {
        match writer.write_record(&row){
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
                exit(1);
            }
        }
    }

    writer.flush().unwrap();


}


fn create_data_vector(headers: Vec<String>, dates: Vec<String>, hours: Vec<String>, data_vector: Vec<Vec<String>>) -> Vector{

    //A LOS HEADERS, AL COMIENZO LES AGREGAMOS "DATE" Y "HOUR"
    let mut new_headers = headers.clone();
    new_headers.insert(0, "DATE".to_string());
    new_headers.insert(1, "HOUR".to_string());

    let total_rows = headers.len();
    // println!("{:?}", total_rows);

    // println!("{:?}", data_vector.len());
    // println!("{:?}", dates.len());
    // println!("{:?}", hours.len());

    //EN LA PRIMER POSICION DE CADA FILA, AGREGAMOS LA FECHA Y EN LA SEGUNDA LA HORA
    let mut new_data: Vec<Vec<String>> = Vec::new();
    for i in 0..dates.len(){
        let mut new_row: Vec<String> = Vec::new();
        new_row.push(dates[i].to_string());
        new_row.push(hours[i].to_string());
        
        for j in total_rows*i..(total_rows*(i) + total_rows ){
            if j >= data_vector.len(){
                break;
            }
            // println!("Index: {} vs {}", j+1, data_vector.len());
            new_row.push(data_vector[j][0].to_string());
            
        }
        // println!("{:?}", new_row);
        new_data.push(new_row);
    }

    let final_vector = Vector{
        headers: new_headers,
        data: new_data,
    };

    final_vector
}