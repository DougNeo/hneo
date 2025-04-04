use sysinfo::{System, SystemExt, ProcessExt};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::error::Error;

fn main() {
    println!("Gerenciador de Processos!");
    
    loop {
        println!("\nEscolha uma opção:");
        println!("0. Para Listar Processos");
        println!("1. Pausar Processo");
        println!("2. Continuar Processo");
        println!("3. Matar processo");
        println!("4. Sair");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "0" => {
                list_process();
            }
            "1" => {
                let pid = get_pid_from_user();
                if let Err(e) = pause_process(pid) {
                    eprintln!("Erro ao pausar processo: {}", e);
                }
            }
            "2" => {
                let pid = get_pid_from_user();
                if let Err(e) = continue_process(pid) {
                    eprintln!("Erro ao continuar processo: {}", e);
                }
            }
            "3" => {
                let pid = get_pid_from_user();
                if let Err(e) = kill_process(pid) {
                    eprintln!("Erro ao finalizar processo: {}", e);
                }
            }
            "4" => break,
            _ => println!("Opção inválida!"),
        }
    }
    println!("Até logo!");
}

fn get_pid_from_user() -> i32 {
    println!("Digite o PID do processo:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap_or(-1)
}

fn pause_process(pid: i32) -> Result<(), Box<dyn Error>> {
    // Envia o sinal para pausar o processo
    kill(Pid::from_raw(pid), Signal::SIGSTOP)?;
    println!("Proceso {} pausado com suceso", pid);
    Ok(())
}

fn continue_process(pid: i32) -> Result<(), Box<dyn Error>> {
    // Envia sinal para continuar o processo
    kill(Pid::from_raw(pid), Signal::SIGCONT)?;
    println!("Processo {} retomado com sucesso", pid);
    Ok(())
}

fn kill_process(pid: i32) -> Result<(), Box<dyn Error>> {
    // Envia sinal para matar o processo
    kill(Pid::from_raw(pid), Signal::SIGKILL)?;
    println!("Processo {} encerrado com sucesso", pid);
    Ok(())
}

fn list_process() {
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