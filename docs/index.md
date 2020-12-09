# Ávores Binárias Rust/C

Foi designado ao nosso grupo criar algorítmos sequenciais e paralelos nas liguagem C e na liguagem Rust.

## O problema

Uma árvore binária é uma estrutura de dados útil quando precisam ser tomadas decisões bidirecionais em cada ponto de um processo. Adicionamos a versão paralelizada para que a criação de alguns desses caminhos possam ser feitas de forma simuntâneas e agilizar o processo.

## Desempenho do Algorítmo feito em c

Para paralelizar o algoritmo, usamos o a lib do mpi.


### Sequencial

|Sequencial|
|-		|
|2.939s		|
|2.949s		|
|3.113s		|
|3.032s		|
|3.018s		|
|3.149s		|
|2.842s		|

Média: 3.088

### Paralelo


| 1 núcleo 	|
|-	|
| 3.005s 	|
| 3.051s 	|
| 3.056s 	|
| 3.162s 	|
| 3.018s 	|
| 3.158s 	|
| 3.067s 	|


| 2 núcleos 	|
|-	|
| 1.800s	|
| 1.721s	|
| 3.056s 	|
| 1.807s	|
| 1.811s	|
| 1.880s 	|
| 1.790s	|


| 3 núcleos 	|
|-	|
| 1.622ss 	|
| 1.678s 	|
| 1.733s 	|
| 1.779s 	|
| 1.646ss 	|
| 1.557s 	|
| 1.678s 	|


| 4 núcleos 	|
|-	|
| 1.546s 	|
| 1.556s 	|
| 1.463s 	|
| 1.558s 	|
| 1.580s 	|
| 1.556s 	|
| 1.592s 	|


### Média de tempo por número de core

| 1      | 2      | 3      | 4      |
|--------|--------|--------|--------|
| 3.073s | 1.796s | 1.670s | 1.550s |

### Speedup
1 CORE - 1
2 CORES - 1,711
3 CORES - 1,840
4 CORES - 1,982

### Efficiency
1 core - 1,186
2 cores - 1,079
3 cores - 0,947
4 cores - 0,737


## Desempenho do Algorítmo feito em Rust

Para paralelizar o algoritmo, utilizamos uma lib chamada rayon. [Link para intalação da lib.](https://github.com/rayon-rs/rayon)

### Sequencial

|Sequencial|
|-		|
|4.408s		|
|4.496s		|
|4.548s		|
|4.797s		|
|4.693s		|
|4.558s		|
|4.853s		|

Média: 4.573


### Paralelo


| 1 núcleo 	|
|-	|
| 3.770s 	|
| 3.875s 	|
| 3.897s 	|
| 3.763s 	|
| 3.876s 	|
| 3.941s 	|
| 3.024s 	|


| 2 núcleos 	|
|-	|
| 2.209s	|
| 2.136s	|
| 1.999s 	|
| 2.121s	|
| 2.079s	|
| 2.169s 	|
| 2.157s	|


| 3 núcleos 	|
|-	|
| 1.427s 	|
| 1.703s 	|
| 1.486s 	|
| 1.691s 	|
| 1.692s 	|
| 1.655s 	|
| 1.731s 	|


| 4 núcleos 	|
|-	|
| 1.554s 	|
| 1.521s 	|
| 1.439s 	|
| 1.518s 	|
| 1.450s 	|
| 1.324s 	|
| 1.330s 	|


### Média de tempo por número de core

| 1      | 2      | 3      | 4      |
|--------|--------|--------|--------|
| 3.853s | 2.119s | 1.609s | 1.550s |

### Speedup
1 core - 1,186
2 cores - 2,158
3 cores - 2,842
4 cores - 2,950

### Efficiency
1 core - 1,186
2 cores - 1,079
3 cores - 0,947
4 cores - 0,737

## Gráficos de tempo/Número de core

### C

![Alt](https://cdn.discordapp.com/attachments/725481134141472819/786035419321335838/WhatsApp_Image_2020-12-08_at_8.17.57_PM.jpeg)

### Rust

![Alt](https://cdn.discordapp.com/attachments/725481134141472819/786045015843274802/WhatsApp_Image_2020-12-08_at_10.38.16_PM.jpeg)

