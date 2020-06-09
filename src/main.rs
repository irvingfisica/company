use std::io;
use std::collections::HashMap;

fn main() {
    println!("Bienvenido a la compañía");

    let mut departamento: String;
    let mut empleado: String;

    let mut database: HashMap<String,Vec<String>> = HashMap::new();

    loop {

    	println!("Para salir presione 0");
    	println!("Introduzca el departamento...");

    	departamento = String::new();

    	io::stdin()
    		.read_line(&mut departamento)
    		.expect("No pude leer el departamento.");

    	departamento = departamento.trim().to_string();

    	println!("{:?}",departamento);

    	if departamento == String::from("0") {
    		break;
    	}

    	loop {

    		println!("El departamento seleccionado es: {}", &departamento);
    		println!("Para seleccionar otro presione 0");
    		println!("Introduzca el nombre del empleado...");

    		empleado = String::new();

    		io::stdin()
    			.read_line(&mut empleado)
    			.expect("No pude leer el nombre.");

    		empleado = empleado.trim().to_string();

    		println!("{:?}",empleado);

    		if empleado == "0" {
    			break;
    		}

    		let vector = database.entry(departamento.clone()).or_insert(vec![]);
    		vector.push(empleado);
    	}

    }

    println!("{:?}",database);

}
