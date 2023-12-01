# Time Travel in Conway's Game of Life

This repository roughly aims to identify instances of backwards time travel in Conway's Game of Life. To better describe the purpose of the code in this repository, I will first define some terms.

A state consists of a mapping $\omega \colon \mathbb{Z} \times \mathbb{Z} \to \{ 0, 1 \}$. Intuitively, we would say that a cell $(x, y)$ is *alive* if $\omega(x, y) = 1$.

Conway's Game of Life describes the evolution of such states of cells. The traditional rules of the game tell us how to go from one state $\omega_t$ to a new state $\omega_{t+1}$, namely:
- if a cell is alive at $t$, then it is alive in $t+1$ *iff* it has 2 or 3 living neighbors (adjacent cells, including diagonally)
- if a cell is dead at $t$, then it is only alive at $t+1$ if it has precisely 3 living neighbors.

In our augmented version, we explicitly allow for the possibility of "time travel", that is, some backwards causality. In particular, we introduce a new rule $R(h, d)$:
- if a cell has $h$ neighbors at step $t$, then it was alive at step $t-d$.


Of course, it is impossible to consecutively calculate games which include such rules, as it is not possible to know at step $t-d$ what the configuration will be at step $t$. However, it is in theory possible for there to exist entire sequences of states which are compatible with such an additional rule. The purpose of this repository is to explicitly find such sequences (for some values of $d$ and $h$).