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

            client.execute("BEGIN", &[]).unwrap();
            let query = format!("SET TRANSACTION ISOLATION LEVEL {}", iso_level);
            client.execute(&query, &[]).unwrap();

            let mut rng = rand::thread_rng();
            let user = (i % 10 + 1) as i32;
            let event = rng.gen_range(1..=3);
            let asiento = ((event - 1) * 10 + rng.gen_range(1..=10)) as i32;

            let insert_result = client.execute(
                "INSERT INTO reservas (usuario_id, evento_id, asiento_id, updated_at, created_at)
                 VALUES ($1, $2, $3, now(), now())",
                &[&user, &event, &asiento],
            );

            match insert_result {
                Ok(_) => {
                    println!("Hilo {} reservó exitosamente", i);
                    *success_clone.lock().unwrap() += 1;
                    client.execute("COMMIT", &[]).unwrap();
                }
                Err(e) => {
                    println!("Hilo {} falló al insertar reserva: {}", i, e);
                    client.execute("ROLLBACK", &[]).ok();
                    *fail_clone.lock().unwrap() += 1;
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
    let avg_duration = duration.as_millis() / n as u128;

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
