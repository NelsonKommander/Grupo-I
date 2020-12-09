# Ávores Binárias Rust/C

Foi designado ao nosso grupo criar algorítmos sequenciais e paralelos nas liguagem C e na liguagem Rust.

## O problema

Uma árvore binária é uma estrutura de dados útil quando precisam ser tomadas decisões bidirecionais em cada ponto de um processo. O nosso trabalho consiste em criar árvores binárias perfeitas usando no minimo o numero de alocações contidas na [implementação exemplo do Jeremy Zerfa.](https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/binarytrees-gcc-3.html)

## Desempenho do algorítmo feito em C

A implementação de memory pooling usada foi a encontrada na lib apr e, para paralelizar o algoritmo, usamos o a lib openmpi.

Foram tomadas medições em 7 diferentes execuções usando como parametro a profundidade de árvore de 21

### [Sequencial](https://raw.githubusercontent.com/NelsonKommander/Grupo-I/main/projeto/c/sequencial.c?token=ALL5TBBMQFZABLYDY2DH7GS73GMDM)

||
---------|------|
**Run 1**|2.939s|
**Run 2**|2.949s|
**Run 3**|3.113s|
**Run 4**|3.032s|
**Run 5**|3.018s|
**Run 6**|3.149s|
**Run 7**|2.842s|
**Média**|3.088s|


### [Paralelo](https://raw.githubusercontent.com/NelsonKommander/Grupo-I/main/projeto/c/parametrizado.c?token=ALL5TBDSZXXCBJAZCKS2IHK73GMFI)


|Numero de cores|    1  	|    2      |    3      |    4      |
|---------------| --------- | --------- | --------- | --------- |
|**Run 1**      | 3.005s 	| 1.800s	| 1.622s 	| 1.546s 	|
|**Run 2**      | 3.051s 	| 1.721s	| 1.678s 	| 1.556s 	|
|**Run 3**      | 3.056s 	| 3.056s 	| 1.733s 	| 1.463s 	|
|**Run 4**      | 3.162s 	| 1.807s	| 1.779s 	| 1.558s 	|
|**Run 5**      | 3.018s 	| 1.811s	| 1.646s 	| 1.580s 	|
|**Run 6**      | 3.158s 	| 1.880s 	| 1.557s 	| 1.556s 	|
|**Run 7**      | 3.067s 	| 1.790s	| 1.678s 	| 1.592s 	|
|**Média**      | 3.073s    | 1.796s    | 1.670s    | 1.550s    |


### Métricas

Dados apresentados foram calculados usando o tempo médio.


|||||
|--- |--- |--- |--- |
| --- |**Sequencial**| --- | --- |
||3.088s|||
| --- |**Paralelo**| --- | --- |
|**1 Core**|**2 Cores**|**3 Cores**|**4 Cores**|
|3.073s|1.796s |1.670s  |1.550s|
| --- |**Speedup**| --- | --- |
| 1   | 1.711  | 1.840  |1.982|
| --- |**Efficiency**| --- | --- |
|1.186| 1.079  | 0.947  |0.737|

## Desempenho do algorítmo feito em Rust

No quesito de gerenciamento de memória foi usada a crate [typed arena](https://crates.io/crates/typed-arena) e, para paralelizar o algoritmo, utilizamos a crate [rayon](https://github.com/rayon-rs/rayon).

Aqui também foram tomadas medições em 7 diferentes execuções usando como parametro a profundidade de árvore de 21

### [Sequencial](https://raw.githubusercontent.com/NelsonKommander/Grupo-I/main/projeto/rust/arvore2/src/main.rs?token=ALL5TBCQWRZFUTK64IMCZR273GMHS)


||
---------|------|
**Run 1**|4.408s|
**Run 2**|4.496s|
**Run 3**|4.548s|
**Run 4**|4.797s|
**Run 5**|4.693s|
**Run 6**|4.558s|
**Run 7**|4.853s|
**Média**|4.573s|

### [Paralelo](https://raw.githubusercontent.com/NelsonKommander/Grupo-I/main/projeto/rust/Arvore/src/main.rs?token=ALL5TBH5HA2FPDLZ7KC7PIC73GMJG)




|Numero de cores|    1  	|    2      |    3      |    4      |
|---------------| --------- | --------- | --------- | --------- |
|**Run 1**      | 3.770s 	| 2.209s	| 1.427s 	| 1.554s 	|
|**Run 2**      | 3.875s 	| 2.136s	| 1.703s 	| 1.521s 	|
|**Run 3**      | 3.897s 	| 1.999s 	| 1.486s 	| 1.439s 	|
|**Run 4**      | 3.763s 	| 2.121s	| 1.691s 	| 1.518s 	|
|**Run 5**      | 3.876s 	| 2.079s	| 1.692s 	| 1.450s 	|
|**Run 6**      | 3.941s 	| 2.169s 	| 1.655s 	| 1.324s 	|
|**Run 7**      | 3.024s 	| 2.157s	| 1.731s 	| 1.330s 	|
|**Média**      | 3.853s    | 2.119s    | 1.609s    | 1.448s    |


### Métricas

Dados apresentados foram calculados usando o tempo médio.

|||||
|--- |--- |--- |--- |
||**Sequencial**|||
||4.573s|||
| --- |**Paralelo**| --- | --- |
|**1 Core**|**2 Cores**|**3 Cores**|**4 Cores**|
|3.853s|2.119s|1.609s|1.448s|
| --- |**Speedup**| --- | --- |
|1,186|2,158|2,842|3,158|
| --- |**Efficiency**| --- | --- |
|1.186|1.079|0.947|0,789|



## Gráficos de tempo/Número de core

### C

![Alt](https://cdn.discordapp.com/attachments/725481134141472819/786035419321335838/WhatsApp_Image_2020-12-08_at_8.17.57_PM.jpeg)

### Rust

![Alt](https://cdn.discordapp.com/attachments/725481134141472819/786045015843274802/WhatsApp_Image_2020-12-08_at_10.38.16_PM.jpeg)
