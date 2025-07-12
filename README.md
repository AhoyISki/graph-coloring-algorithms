# graph-coloring-algorithms

Some graph coloring algorithms

In order to run these, just install [rust][__link0] and, from the cloned
repository’s path, call:

```bash
cargo run --release
```

This program will read any file with a name like “\*.col” in the
directory. These files must be formatted like this:

```text
c This is a comment line, there can be anything in here
c No line can be empty, apart from the last one
c
p edge {vertices} {edges}
e {vertex} {vertex}
```

The vertices should also be 1 indexed, for example:

```text
c This is a comment line, there can be anything in here
c No line can be empty, apart from the last one
c
p edge 3 2
e 1 3
e 1 2
```

## Benchmarks

Here are some benchmarks, from some [DIMACS graphs][__link1]:

first fit:

* took 39.80µs on paper-test.col with 3 colors
* took 1.04ms on dsjc250.5.col with 43 colors
* took 967.33µs on dsjc500.1.col with 20 colors
* took 3.25ms on dsjc500.5.col with 72 colors
* took 3.95ms on dsjc500.9.col with 175 colors
* took 3.69ms on dsjc1000.1.col with 31 colors
* took 7.48ms on dsjc1000.5.col with 98 colors
* took 7.03ms on dsjc1000.9.col with 225 colors
* took 865.23µs on r250.5.col with 79 colors
* took 9.96ms on r1000.1c.col with 138 colors
* took 13.85ms on r1000.5.col with 275 colors
* took 1.22ms on dsjr500.1c.col with 87 colors
* took 3.46ms on dsjr500.5.col with 143 colors
* took 1.18ms on le450_25c.col with 37 colors
* took 1.20ms on le450_25d.col with 35 colors
* took 1.13ms on flat300_28_0.col with 46 colors
* took 12.02ms on flat1000_50_0.col with 126 colors
* took 5.51ms on flat1000_60_0.col with 86 colors
* took 11.94ms on flat1000_76_0.col with 122 colors
* took 12.05ms on latin_square.col with 122 colors
* took 46.45ms on C2000.5.col with 226 colors
* took 180.33ms on C4000.5.col with 402 colors

welsh powell:

* took 2.02µs on paper-test.col with 3 colors
* took 391.56µs on dsjc250.5.col with 41 colors
* took 519.00µs on dsjc500.1.col with 19 colors
* took 1.36ms on dsjc500.5.col with 70 colors
* took 723.93µs on dsjc500.9.col with 173 colors
* took 1.99ms on dsjc1000.1.col with 29 colors
* took 3.35ms on dsjc1000.5.col with 97 colors
* took 1.59ms on dsjc1000.9.col with 217 colors
* took 337.92µs on r250.5.col with 70 colors
* took 1.61ms on r1000.1c.col with 112 colors
* took 4.71ms on r1000.5.col with 259 colors
* took 274.62µs on dsjr500.1c.col with 79 colors
* took 1.20ms on dsjr500.5.col with 135 colors
* took 581.35µs on le450_25c.col with 30 colors
* took 583.81µs on le450_25d.col with 30 colors
* took 498.21µs on flat300_28_0.col with 46 colors
* took 5.21ms on flat1000_50_0.col with 120 colors
* took 2.49ms on flat1000_60_0.col with 81 colors
* took 5.23ms on flat1000_76_0.col with 122 colors
* took 5.27ms on latin_square.col with 122 colors
* took 20.49ms on C2000.5.col with 218 colors
* took 79.76ms on C4000.5.col with 398 colors

largest degree ordering:

* took 2.50µs on paper-test.col with 3 colors
* took 965.37µs on dsjc250.5.col with 41 colors
* took 1.55ms on dsjc500.1.col with 19 colors
* took 3.82ms on dsjc500.5.col with 70 colors
* took 4.21ms on dsjc500.9.col with 173 colors
* took 5.92ms on dsjc1000.1.col with 29 colors
* took 8.64ms on dsjc1000.5.col with 97 colors
* took 8.12ms on dsjc1000.9.col with 217 colors
* took 1.08ms on r250.5.col with 70 colors
* took 9.97ms on r1000.1c.col with 112 colors
* took 16.25ms on r1000.5.col with 259 colors
* took 1.52ms on dsjr500.1c.col with 79 colors
* took 3.99ms on dsjr500.5.col with 135 colors
* took 1.63ms on le450_25c.col with 30 colors
* took 1.63ms on le450_25d.col with 30 colors
* took 1.36ms on flat300_28_0.col with 46 colors
* took 13.82ms on flat1000_50_0.col with 120 colors
* took 6.45ms on flat1000_60_0.col with 81 colors
* took 14.01ms on flat1000_76_0.col with 122 colors
* took 14.05ms on latin_square.col with 122 colors
* took 53.60ms on C2000.5.col with 218 colors
* took 213.38ms on C4000.5.col with 398 colors

incidence degree ordering:

* took 1.07ms on dsjc250.5.col with 41 colors
* took 1.99ms on dsjc500.1.col with 18 colors
* took 4.19ms on dsjc500.5.col with 69 colors
* took 4.69ms on dsjc500.9.col with 177 colors
* took 7.25ms on dsjc1000.1.col with 30 colors
* took 10.07ms on dsjc1000.5.col with 96 colors
* took 9.49ms on dsjc1000.9.col with 230 colors
* took 1.12ms on r250.5.col with 69 colors
* took 12.18ms on r1000.1c.col with 124 colors
* took 17.70ms on r1000.5.col with 250 colors
* took 1.80ms on dsjr500.1c.col with 81 colors
* took 4.50ms on dsjr500.5.col with 130 colors
* took 1.94ms on le450_25c.col with 30 colors
* took 1.96ms on le450_25d.col with 31 colors
* took 1.46ms on flat300_28_0.col with 46 colors
* took 15.54ms on flat1000_50_0.col with 125 colors
* took 7.84ms on flat1000_60_0.col with 85 colors
* took 15.56ms on flat1000_76_0.col with 126 colors
* took 15.57ms on latin_square.col with 126 colors
* took 60.01ms on C2000.5.col with 223 colors
* took 232.74ms on C4000.5.col with 397 colors

degree of saturation:

* took 2.00ms on dsjc250.5.col with 35 colors
* took 2.81ms on dsjc500.1.col with 16 colors
* took 8.83ms on dsjc500.5.col with 65 colors
* took 13.98ms on dsjc500.9.col with 166 colors
* took 11.21ms on dsjc1000.1.col with 26 colors
* took 20.38ms on dsjc1000.5.col with 88 colors
* took 29.08ms on dsjc1000.9.col with 208 colors
* took 2.07ms on r250.5.col with 68 colors
* took 37.34ms on r1000.1c.col with 105 colors
* took 46.02ms on r1000.5.col with 250 colors
* took 3.63ms on dsjr500.1c.col with 76 colors
* took 9.45ms on dsjr500.5.col with 129 colors
* took 3.02ms on le450_25c.col with 29 colors
* took 3.06ms on le450_25d.col with 29 colors
* took 2.83ms on flat300_28_0.col with 43 colors
* took 35.30ms on flat1000_50_0.col with 113 colors
* took 14.94ms on flat1000_60_0.col with 76 colors
* took 35.38ms on flat1000_76_0.col with 115 colors
* took 35.06ms on latin_square.col with 115 colors
* took 164.26ms on C2000.5.col with 208 colors
* took 1.20s on C4000.5.col with 377 colors

recursive largest first:

* took 1.24ms on dsjc250.5.col with 41 colors
* took 2.62ms on dsjc500.1.col with 19 colors
* took 7.02ms on dsjc500.5.col with 71 colors
* took 12.75ms on dsjc500.9.col with 170 colors
* took 16.07ms on dsjc1000.1.col with 31 colors
* took 23.63ms on dsjc1000.5.col with 97 colors
* took 34.46ms on dsjc1000.9.col with 219 colors
* took 1.49ms on r250.5.col with 72 colors
* took 32.76ms on r1000.1c.col with 113 colors
* took 66.21ms on r1000.5.col with 260 colors
* took 3.71ms on dsjr500.1c.col with 78 colors
* took 9.72ms on dsjr500.5.col with 138 colors
* took 2.74ms on le450_25c.col with 32 colors
* took 2.74ms on le450_25d.col with 32 colors
* took 1.82ms on flat300_28_0.col with 46 colors
* took 40.56ms on flat1000_50_0.col with 122 colors
* took 21.44ms on flat1000_60_0.col with 83 colors
* took 39.90ms on flat1000_76_0.col with 122 colors
* took 40.06ms on latin_square.col with 122 colors
* took 269.73ms on C2000.5.col with 222 colors
* took 1.98s on C4000.5.col with 394 colors


 [__link0]: https://www.rust-lang.org/tools/install
 [__link1]: https://cedric.cnam.fr/~porumbed/graphs/
