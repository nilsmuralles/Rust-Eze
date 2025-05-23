use postgres::{Client, NoTls};
use std::thread;
use std::sync::{Arc, Mutex};
use rand::Rng; 
use std::time::Instant;

fn transaction(n: usize, iso_level: &str) -> Vec<u128> {

    let mut handles = vec![];
    let start = Instant::now();

    let success = Arc::new(Mutex::new(0u128));
    let fail = Arc::new(Mutex::new(0u128));

    let mut client = Client::connect("host=localhost port=5433 user=admin password=admin dbname=reservations", NoTls)
        .expect("Error conectando a la base de datos");

    client.execute("TRUNCATE TABLE reservas RESTART IDENTITY;", &[]).unwrap();

    for i in 0..n {
        let iso_level = iso_level.to_string();
        let success_clone = Arc::clone(&success);
        let fail_clone = Arc::clone(&fail);

        let handle = thread::spawn(move || {
            let mut client = Client::connect("host=localhost port=5433 user=admin password=admin dbname=reservations", NoTls)
                .expect("Error conectando a la base de datos");

            let mut counted = false;

            if client.execute("BEGIN", &[]).is_err() {
                *fail_clone.lock().unwrap() += 1;
                return;
            }

            let query = format!("SET TRANSACTION ISOLATION LEVEL {}", iso_level);
            if client.execute(&query, &[]).is_err() {
                client.execute("ROLLBACK", &[]).ok();
                *fail_clone.lock().unwrap() += 1;
                return;
            }

            let mut rng = rand::thread_rng();
            let user = (i % 10 + 1) as i32;
            let event = 1;
            let asiento = rng.gen_range(1..=30) as i32;

            let check_query = "SELECT COUNT(*) > 0 FROM reservas WHERE evento_id = $1 AND asiento_id = $2";
            let asiento_ocupado: bool = match client.query_one(check_query, &[&event, &asiento]) {
                Ok(row) => row.get(0),
                Err(e) => {
                    println!("Error verificando asiento: {}", e);
                    client.execute("ROLLBACK", &[]).ok();
                    *fail_clone.lock().unwrap() += 1;
                    counted = true;
                    return;
                }
            };

            if asiento_ocupado {
                println!("Hilo {}: Asiento {} ya ocupado", i, asiento);
                client.execute("ROLLBACK", &[]).ok();
                if !counted {
                    *fail_clone.lock().unwrap() += 1;
                    counted = true;
                }
                return;
            }

            let insert_result = client.execute(
                "INSERT INTO reservas (usuario_id, evento_id, asiento_id, updated_at, created_at)
                 VALUES ($1, $2, $3, now(), now())",
                &[&user, &event, &asiento],
            );

            match insert_result {
                Ok(_) => {
                    println!("Hilo {} reservó exitosamente", i);
                    match client.execute("COMMIT", &[]) {
                        Ok(_) => {
                            *success_clone.lock().unwrap() += 1;
                            counted = true;
                        }
                        Err(e) => {
                            println!("Error haciendo COMMIT: {}", e);
                            client.execute("ROLLBACK", &[]).ok();
                            *fail_clone.lock().unwrap() += 1;
                            counted = true;
                        }
                    }
                }
                Err(_) => {
                    println!("Hilo {} falló al insertar reserva", i);
                    client.execute("ROLLBACK", &[]).ok();
                    if !counted {
                        *fail_clone.lock().unwrap() += 1;
                        counted = true;
                    }
                }
            }
            
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    client.execute("TRUNCATE TABLE reservas RESTART IDENTITY;", &[]).unwrap();

    let duration = start.elapsed();
    let avg_duration = if n > 0 { duration.as_millis() / n as u128 } else { 0 };

    vec![*success.lock().unwrap(), *fail.lock().unwrap(), avg_duration]
}


fn main() {
    // Transacciones READ COMMITTED
    let result1 = transaction(5, "READ COMMITTED");
    println!("Usuarios: 5 | Nivel Aislamiento: READ COMMITTED | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result1[0], result1[1], result1[2]);
    
    let result2 = transaction(10, "READ COMMITTED");
    println!("Usuarios: 10 | Nivel Aislamiento: READ COMMITTED | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result2[0], result2[1], result2[2]);
    
    let result3 = transaction(20, "READ COMMITTED");
    println!("Usuarios: 20 | Nivel Aislamiento: READ COMMITTED | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result3[0], result3[1], result3[2]);
    
    let result4 = transaction(30, "READ COMMITTED");
    println!("Usuarios: 30 | Nivel Aislamiento: READ COMMITTED | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result4[0], result4[1], result4[2]);
    
    // Transacciones REPEATABLE READ
    let result5 = transaction(5, "REPEATABLE READ");
    println!("Usuarios: 5 | Nivel Aislamiento: REPEATABLE READ | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result5[0], result5[1], result5[2]);
    
    let result6 = transaction(10, "REPEATABLE READ");
    println!("Usuarios: 10 | Nivel Aislamiento: REPEATABLE READ | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result6[0], result6[1], result6[2]);
    
    let result7 = transaction(20, "REPEATABLE READ");
    println!("Usuarios: 20 | Nivel Aislamiento: REPEATABLE READ | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result7[0], result7[1], result7[2]);
    
    let result8 = transaction(30, "REPEATABLE READ");
    println!("Usuarios: 30 | Nivel Aislamiento: REPEATABLE READ | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result8[0], result8[1], result8[2]);
    
    // Transacciones SERIALIZABLE
    let result9 = transaction(5, "SERIALIZABLE");
    println!("Usuarios: 5 | Nivel Aislamiento: SERIALIZABLE | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result9[0], result9[1], result9[2]);
    
    let result10 = transaction(10, "SERIALIZABLE");
    println!("Usuarios: 10 | Nivel Aislamiento: SERIALIZABLE | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result10[0], result10[1], result10[2]);
    
    let result11 = transaction(20, "SERIALIZABLE");
    println!("Usuarios: 20 | Nivel Aislamiento: SERIALIZABLE | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result11[0], result11[1], result11[2]);
    
    let result12 = transaction(30, "SERIALIZABLE");
    println!("Usuarios: 30 | Nivel Aislamiento: SERIALIZABLE | Exitos: {} | Fracasos: {} | Tiempo promedio: {} ms", result12[0], result12[1], result12[2]);
}
