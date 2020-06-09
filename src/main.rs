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

    	if departamento == String::from("0") {
    		break;
    	}

    	// if departamento == String::from("2") {
    	// 	empleados_lista(&database);
    	// }

    	loop {

    		println!("El departamento seleccionado es: {}", &departamento);
    		println!("Para seleccionar otro presione 0");
    		println!("Para listar a todos en {} presione 1", &departamento);
    		println!("Para listar a todos en la empresa presione 2");
    		println!("Introduzca el nombre del empleado...");

    		empleado = String::new();

    		io::stdin()
    			.read_line(&mut empleado)
    			.expect("No pude leer el nombre.");

    		empleado = empleado.trim().to_string();

    		match &empleado[..] {
    			"0" => break,
    			"1" => match database.get(&departamento) {
    				Some(vector) => {
    					let mut vector_copy = vector.clone();
    					vector_copy.sort();
    					println!("{:?}",vector_copy);
    				},
    				None => (),
    			},
    			"2" => empleados_lista(&database),
    			_ => {
    				let vector = database.entry(departamento.clone()).or_insert(vec![]);
    				vector.push(empleado);
    			},
    		}

    	}

    }

}

fn empleados_lista(mapa: &HashMap<String,Vec<String>>) {

	let mut deptos = vec![];

	for (key, _value) in mapa {
		deptos.push(key);
	}

	deptos.sort();

	for depto in &deptos {
		let mut vectemp = mapa.get(*depto).unwrap().clone();
		vectemp.sort();

		println!("Los emplados en {} son:", depto);

		for empleado in &vectemp {
			println!("{}",&empleado);
		}
	}
}
