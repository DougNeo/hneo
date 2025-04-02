use sysinfo::{System, SystemExt, ProcessExt};

fn main() {
    println!("Gerenciador de Processos!");

    // Vamos inicializar o sistema e coletar informações.
    let mut system = System::new_all();
    system.refresh_all();

    println!("{:^8}|{:^15}|{:^10}", "PID", "NOME", "MEM (KB)");

    for (pid, process) in system.processes()  {
        println!("{:^8}|{:^15}|{:^10}", 
                 pid, 
                 process.name(), 
                 process.memory() / 1024);
    }
}

