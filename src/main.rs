use std::io;
fn main() {
  let mut done = false;
    while done == false {
        let mut entrada = String::new();
        println!("Tempo do processo (30s a 90s):");
        io::stdin().read_line(&mut entrada).expect("Tempo Inválido");
      }
}
struct Processo {
    PID: i32,
    menSize: i32,
    timeExecution: i32,
}
struct Pilha {
    processos: Vec<Processo>,
    nextPid:i32
}