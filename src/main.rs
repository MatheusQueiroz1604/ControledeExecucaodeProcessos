use std::io;
use std::{thread, time::Duration};

struct Processo {
    pid: i32,
    memoria: i32,
    tempo_execucao: i32,
}
struct Pilha {
    processos: Vec<Processo>,
    next_pid: i32
}
impl Pilha {
  //cria uma nova pilha
    fn new() -> Pilha {
        Pilha {
            processos: Vec::new(),
            next_pid: 0,
        }
    }
  //adiciona um processo na pilha
    fn push(&mut self, memoria: i32, tempo_execucao: i32) {
        let processo = Processo {
            pid: self.next_pid,
            memoria,
            tempo_execucao,
        };
        self.processos.push(processo);
        self.next_pid += 1;
    }
  //remove o processo do topo da pilha
    fn remove_processo(&mut self, indice: i32) {
        self.processos.remove(indice.try_into().unwrap());
    }
  //imprime os ids dos processos
    fn print_pids(&self) {
        for processo in &self.processos {
            println!("{}", processo.pid);
        }
    }
}

fn main() {
    let mut pilha: Pilha = Pilha::new();
    let mut pilha_temporaria: Pilha = Pilha::new(); //guardar as informações dos processos mesmo depois de já removidos para o relatório final
    let mut entrada = String::new();
    let mut soma_tempo: i32 = 0;
    let mut soma_memoria: i32 = 0;
    let mut tempo_execucao: i32;
    let mut memoria: i32;
    loop{
        println!();
        println!("Digite a memória (MB) e o tempo de execução (segundos) do processo, ou 'sair' para terminar: ");
        io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");
      
        if entrada.trim() == "sair"{
          break;
        }
      
        let entrada_split: Vec<&str> = entrada.trim().split_whitespace().collect();
        if entrada_split.len() != 2 {
            println!("Entrada inválida. Por favor, forneça dois números.");
            continue;
        }
        memoria = entrada_split[0].parse().expect("Entrada inválida.");
        soma_memoria += memoria;
        tempo_execucao = entrada_split[1].parse().expect("Entrada inválida.");
        soma_tempo += tempo_execucao;
      
        if tempo_execucao < 30 || tempo_execucao > 90 {
          print!("\nTempo de execução inválido. O tempo deve estar entre 30 e 90 segundos.\n");
          continue;
        }
        pilha.push(memoria, tempo_execucao)
      }
    
    println!("PIDs dos processos a serem executados: ");
    pilha.print_pids();
    let tamanho: usize = pilha.processos.len();

      for i in 0..pilha.processos.len() {
          let processo = &pilha.processos[i];
          let mut tempo = processo.tempo_execucao;
          println!("===== Execução iniciada ================");
          println!("===== Processo sendo executado: {} ======",processo.pid);
          while tempo > 0 {
              print!(" {} ->",tempo);
              thread::sleep(Duration::from_secs(1));
              tempo -= 1;
          }
          println!("");
          println!("===== Execução do processo encerrada ===");
          if tamanho > 0 {
              pilha_temporaria.push(processo.memoria, processo.tempo_execucao);
              pilha.remove_processo(0);
          } else {
              println!("Processos Executados: ");
              for i in 0..pilha_temporaria.processos.len(){
                let processo = &pilha_temporaria.processos[i];
                println!("Processo {}: Memória: {} MB, Tempo de execução: {} segundos", processo.pid, processo.memoria, processo.tempo_execucao);
              }
              println!();
              println!("\nSoma da memória de todos os processos: {} MB",soma_memoria);
              println!("Tempo total de execução: {} segundos", soma_tempo);
              break;
          }
    }
}
