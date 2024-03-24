use std::io;
use std::thread;
use std::time::Duration;

struct Processo {
    pid: i32,
    men_size: i32,
    time_execution: i32,
}
struct Pilha {
    processos: Vec<Processo>,
    next_pid: i32,
}
impl Pilha {
    fn new() -> Pilha {
        Pilha {
            processos: Vec::new(),
            next_pid: 0,
        }
    }
    fn remove_processo(&mut self) {
        self.processos.remove(0);
    }
    fn push(&mut self, men_size: i32, time_execution: i32) {
        let processo = Processo {
            pid: self.next_pid,
            men_size,
            time_execution,
        };
        self.processos.push(processo);
        self.next_pid += 1;
    }

    fn print_pids(&self) {
        for processo in &self.processos {
            println!("{}", processo.pid);
        }
    }
}

fn main() {
    let mut soma_tempo: i32 = 0; //somatório final
    let mut soma_memoria: i32 = 0;
    let mut pilha: Pilha = Pilha::new();
    let mut pilha_temporaria: Pilha = Pilha::new(); //guardar as informações dos processos mesmo depois de já removidos para o relatório final
    let mut input_line = String::new(); //Serve para converter os valores que o usuário digitar de String para inteiro

    //obtem os inputs do usuário
    println!();
    let mut parar: bool = false;
    while parar != true {
        let mut tempo_execucao: i32 = 0;
        let mut memoria: i32 = 0;
        loop {
            println!("Quantidade de memória (MB) a ser alocada para o processo, ou 'sair para terminar': ");
            input_line.clear();
            io::stdin().read_line(&mut input_line).expect("Falha ao ler a entrada");
            if input_line.trim() == "sair"{
                parar = true;
                break;
            } else {
            memoria = input_line.trim().parse().expect("Valor inválido");
            soma_memoria += memoria;
            if memoria > 0 {
              break;
            } else {
            print!("O valor é muito baixo.");}
          }
        }
      println!();
        loop {
          if parar == true{
            break;
          }
            println!("Digite o valor do tempo de execução (S) para o processo (mínimo 30s; máximo 90s): ");
            input_line.clear();
            io::stdin().read_line(&mut input_line).expect("Falha ao ler a entrada");
            tempo_execucao = input_line.trim().parse().expect("Valor inválido");
            soma_tempo += tempo_execucao;

            if tempo_execucao >= 30 && tempo_execucao <= 90 {
              break;
            } else {
              println!("O TEMPO DE EXECUÇÃO É INVÁLIDO")
            }
    }
      println!();
        //adiciona o processo na pilha
        if parar == true{
          break;
        } else { 
        pilha.push(memoria, tempo_execucao)
        }
    }
    //exibe o id dos processos a serem executados
  if pilha.processos.len() == 0 {
    println!("A pilha está vazia, não há nada para ser executado.")
  } else {
    println!("PIDs dos processos a serem executados:");
    pilha.print_pids();
    //execução da pilha
    println!();
    let i = 0;
      while i != pilha.processos.len(){
      let processo = &pilha.processos[i];
      let mut tempo = processo.time_execution;

      println!("=============== Execução iniciada ================");
      println!("========== Processo sendo executado: {} ==========",processo.pid);
        
        while tempo > 0 {
          println!("=> {}", tempo);
          tempo -= 1;
          thread::sleep(Duration::from_secs(1));
        }
      println!();
      println!("========= Execução do processo encerrada =========");
      println!();
      println!();
      pilha_temporaria.push(processo.men_size,processo.time_execution);
      pilha.remove_processo();
  }
          println!("Processos Executados: ");
          for i in 0..pilha_temporaria.processos.len(){
            let processo = &pilha_temporaria.processos[i];
            println!("Processo {}: Memória: {} MB, Tempo de execução: {} segundos", processo.pid, processo.men_size, processo.time_execution);
          }
          println!();
          println!("\nSoma da memória de todos os processos: {} MB",soma_memoria);
          println!("Tempo total de execução: {} segundos", soma_tempo);
    }
}
