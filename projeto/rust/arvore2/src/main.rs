// typed_arena nos dá acesso a alocação rápida de memória, e rayon dá acesso a multitread
extern crate typed_arena;
extern crate rayon;

use typed_arena::Arena;
use rayon::prelude::*;
use std::env;
use std::time::*;
//std:env está aqui pq precisamos de args, para pegar o que é colocado na linha de comando.


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
fn make_tree_par (tamanho: i32, n_iter:i32) -> String {
    let t_temp = Instant::now();
    let par_check :i32 = (0..n_iter).into_par_iter().map(|_| {
        let arena_temp = Arena::new();
        let arvore_temp = make_tree(&arena_temp, tamanho);
        check_sum(arvore_temp)
        //nessa função implicita check_sum é a saida
    }).sum();
    format!("{:>5} arvores, de tamanho {:>5}, check sum = {:>10}; tempo: {:?} ", n_iter, tamanho, par_check, t_temp.elapsed())
}
*/
///*
fn make_tree_par (tamanho: i32, n_iter:i32) -> String {
    let t_temp = Instant::now();
    let mut check :i32 = 0;
    
    for x in 0..n_iter {
        let arena_temp = Arena::new();
        let arvore_temp = make_tree(&arena_temp, tamanho);
        check += check_sum(&arvore_temp);
    }; 
    format!("{:>5} arvores, de tamanho {:>5}, check sum = {:>10}; tempo: {:?} ", n_iter, tamanho, check, t_temp.elapsed())

}
//*/

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

    /*
    let out_paralel = (t_min/2 .. t_max/2 +1).into_par_iter().map(|t_dividido| { 
        let t_atual = t_dividido * 2;
        let iter = 1 << ((t_max + t_min - t_atual) as u32);
        make_tree_par(t_atual, iter)
    }).collect::<Vec<_>>();
   */
    let a_iters = (t_min/2 .. t_max/2 +1).into_iter();
    let mut out_paralel = Vec::new();
    for a_iter in a_iters {
        let t_atual = a_iter * 2;
        let b_iters = 1 << ((t_max + t_min - t_atual) as u32);
        out_paralel.push(make_tree_par(t_atual, b_iters)); 
    }

    for msg in out_paralel{ println!("{}", msg)};

    println!("Arvore quase esqucida de tamanho {}, check:{}; tempo de vida: {:?} ", t_max, check_sum(&arvore2), t2.elapsed());
}
