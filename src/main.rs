use std::io;

struct Processo {
    pid: i32,
    men_size: i32,
    time_execution: i32,
}
struct Pilha {
    processos: Vec<Processo>,
    next_pid:i32
}
impl Pilha {
    fn new() -> Pilha {
        Pilha {
            processos: Vec::new(),
            next_pid: 0,
        }
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
        println!("PIDs dos processos na pilha:");
        for processo in &self.processos {
            println!("{}", processo.pid);
        }
    }
}
fn main() {
}
