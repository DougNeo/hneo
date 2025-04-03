use sysinfo::{System, SystemExt, ProcessExt};

fn main() {
    println!("Gerenciador de Processos!");

    // Inicializa o sistema e coleta informações
    let mut system = System::new_all();
    system.refresh_all();

    // Cabeçalho com alinhamento consistente
    println!("+-{:-<8}-+-{:-<15}-+-{:-<10}-+", "", "", "");
    println!("| {:^8} | {:^15} | {:^10} |", "PID", "NOME", "MEM (KB)");
    println!("+-{:-<8}-+-{:-<15}-+-{:-<10}-+", "", "", "");

    // Lista processos
    for (pid, process) in system.processes() {
        // Trunca nomes longos para manter o alinhamento
        let name = if process.name().len() > 15 {
            &process.name()[..15]
        } else {
            process.name()
        };

        // Formatação dos dados
        println!(
            "| {:^8} | {:^15} | {:^10} |", 
            pid.to_string(),          // PID alinhado à direita
            name,         // Nome alinhado à esquerda (com truncamento)
            process.memory() / 1024 // Memória alinhada à direita
        );
    }

    println!("+-{:-<8}-+-{:-<15}-+-{:-<10}-+", "", "", "");
}
