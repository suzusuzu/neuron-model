# neuron model

## Integrate-and-fire
[if.rs](https://github.com/suzusuzu/neuron-model/blob/master/src/bin/if.rs)

```sh
cargo run --bin if
gnuplot -p < if.plt
```

![if](https://raw.githubusercontent.com/suzusuzu/neuron-model/master/if.png)

## Hodgkin–Huxley

[hh.rs](https://github.com/suzusuzu/neuron-model/blob/master/src/bin/hh.rs)

```sh
cargo run --bin hh
gnuplot -p < hh.plt
```

![hh](https://raw.githubusercontent.com/suzusuzu/neuron-model/master/hh.png)

## Izhikevich

[iz.rs](https://github.com/suzusuzu/neuron-model/blob/master/src/bin/iz.rs)

```sh
cargo run --bin iz
gnuplot -p < iz.plt
```

![iz](https://raw.githubusercontent.com/suzusuzu/neuron-model/master/iz.png)

## FitzHugh-Nagumo

[fn.rs](https://github.com/suzusuzu/neuron-model/blob/master/src/bin/fn.rs)

```sh
cargo run --bin fn
gnuplot -p < fn.plt
```

![fn](https://raw.githubusercontent.com/suzusuzu/neuron-model/master/fn.png)

