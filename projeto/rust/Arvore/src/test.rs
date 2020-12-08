// typed_arena nos dá acesso a alocação rápida de memória, e rayon dá acesso a multitread
extern crate typed_arena;
extern crate rayon;

use typed_arena::Arena;
use rayon::prelude::*;
use std::env;
use std::time::*;
//std:env está aqui pq precisamos de args, para pegar o que é colocado na linha de comando.

/* 
    Option é usado para indicar que 'l' e 'r' podem ou não ter algum valor
caso exista algum valor -> Some, caso não exista -> None
' é usado para indicar um tempo de vida, todos elementos marcados com
o mesmo tempo de vida não são desalocados nem deletados até que o de "maior importancia" removido
& serve para "pegar emprestado" uma variavel (um ponteiro para a variavel)
isso é util pq Rust adora tomar posse das coisas, e vc pode acabar sem querer deletando algo que não é seu
*/
struct Arvore<'a> {
    l: Option<&'a Arvore<'a>>,
    r: Option<&'a Arvore<'a>>,
}

// não entendo pq vários códigos chamam isso de check sum, era mais facil chamar de contador de nós da arvore
// aparentemente rust tambem não precisa de "return" para retornar valores
fn check_sum (arvore: &Arvore) -> i32 {
    if let (Some(l), Some(r)) = (arvore.l, arvore.r) {
        1 + check_sum(l) + check_sum(r)
    } else { 1 }
}

/*
    Cria de forma iterativa a arvore, enquanto o tamanho for > 0 ele adiciona nós a arvore
esse código retorna uma referencia para a raiz da arvore
essa parte do código não é paralelizada, ele apenas cria nós de forma sequencial
para paralelizar podemos dar spawn em treads para criar mais de uma arvore ao memso tempo,
*/
fn make_tree<'b>(arena:&'b Arena<Arvore<'b>>, tamanho: i32 ) -> &'b Arvore<'b>{
    let arvore =  arena.alloc(Arvore {l: None, r :None});
    if tamanho > 0 {
        let l = make_tree(arena, tamanho-1);
        let r = make_tree(arena, tamanho-1);
        arvore.l = Some(l);
        arvore.r = Some(r);
    }
    return arvore
}

/* 
 vamos trocar o map() por um simples for para fazer a versão sequencial desse código.
*/

fn make_tree_iter (tamanho: i32, n_iter:i32) -> String {
    let t_temp = Instant::now();
    let par_check :i32 = (0..n_iter).std::into_iter().map(|_| {
        let arena_temp = Arena::new();
        let arvore_temp = make_tree(&arena_temp, tamanho);
        check_sum(arvore_temp)
        //nessa função implicita check_sum é a saida
    }).sum();
    // format https://doc.rust-lang.org/std/fmt/
    // format!() é bem parecido com o funcionamento do print!() do própiro rust
    // entretanto a saida dele é para uma variavel do tipo String em vez de ir para o terminal
    format!("{:>5} arvores, de tamanho {:>5}, check sum = {:>10}; tempo: {:?} ", n_iter, tamanho, par_check, t_temp.elapsed())
}
/* 
    O output de make_tree_par() poderia ser os valores retornados com seus tipos especificos,
 mas para facilitar o nosso entendimento vamos fazer igual ao exemplo e já dar o output como uma string
 isso facilita na final na hora de arrumar o display para o utilizador, e pode evir dor de cabeças
 com o rust e o uso de diversos tipos de variaveis,

*/


// o let para a quisição do valor da linha de comando foi copiado na cara dura de uma das implementações do benchmark
//args().nth(1), pega o primeiro argumento, .and_then() é uma função meio miraculosa comparada a outras linguagens,
//ela pega o resultado do que existe antes dela na chamada e aplica uma função nela, nesse caso é uma função implicita |n|,
//que da parse no valor recebido, e se o parse falhar ele usa unwrap_or para substituir por 10
//um equivalente sem linguagem epecifica seria :
// tamanho = args();
// try {tamanho_int = tamanho.to_int()};
// catch {tamanho_int = 10};

fn main() {
    let t = env::args().nth(1).and_then(|n| n.parse().ok()).unwrap_or(10);
    let t_min = 4;
    let t_max:i32 ;
    if (t_min + 2) > t {
        t_max = t_min + 2
    }  else { t_max = t}
    
    /*
        1º teste, linear de criação de arvore com tamanho "t_max + 1"
    Nome: arvore1
    Tamanho: t_max+1
        */
    // vamos usar std::time para tentar calcular a duração de cada parte do processo
    //ti = Instant::now(), para salvar o tempo inicial e depois usaremos ti.elapsed() para calcular a diferença
    let t1 = Instant::now();
    let arena1 = Arena::new();
    let tamanho = t_max + 1;
    let arvore1 = make_tree(&arena1, tamanho);
    println!("Arvore de tamanho {:>5} criada, check_sum = {:>5}, tempo:{:?} ", tamanho, check_sum(&arvore1), t1.elapsed());
    // 1º teste e tudo funciona até aqui

    let t2 = Instant::now();
    let arena2 = Arena::new();
    let arvore2 = make_tree(&arena2, t_max);

    let t_dividido = (t_min/2 .. t_max/2 +1).std::into_iter();
    for t_div in t_dividido { 
        let t_atual = t_dividido * 2;
        let iter = 1 << ((t_max + t_min - t_atual) as u32);
        make_tree_par(t_atual, iter)
    }).collect::<Vec<_>>();

    for msg in out_paralel{ println!("{}", msg)};

    /*
        eu quse esqueci a arvore2 que foi criada antes da parte paralela
        vamos dar o print dela agora após tudo isso 
    */

    println!("Arvore quase esqucida de tamanho {}, check:{}; tempo de vida: {:?} ", t_max, check_sum(&arvore2), t2.elapsed());
}