# yew-investigation

This repository accompanies the medium article <INSERT>. The code is a 
comparison of 3 approaches that calculate a nth term of the Fibonacci
sequence in VueJS, VueJS/WebAssembly and Yew (WebAssembly). This 
comparison is used to evalue Yew.

The directories are as follow:
* `www` - corresponds to the home page allowing one to select one of the three
approaches;
* `vuejs` - corresponds to the VueJS implementation;
* `vuejs_wasm` - corresponds to the VueJS section of the VueJS/WebAssembly;
* `vuejs_wasm_rust` - is the Rust section of the VueJS/WebAssembly that gets 
compiled to WebAssembly;
* `yew` - corresponds to the Yew implementation.

# Usage

The directories `www`, `vuejs`, `vuejs_wasm` and `yew` are each serving one 
webpage. I agree that this is inefficient, but prevents any crossing over
between project - in other words, it is a deliberate choice.

To get it all working, following the instruction to start the webserver in 
each of the above mentioned directories (instruction listed below), then 
open your webbrowser and load the url `http://localhost:8080/yew-investigation/index.html`.

# `www`
## Pre-requisite:
* `npm` from <https://nodejs.org/en/>

## Usage
```
cd www
npm install
npm run serve
```

## Unit testing
No unit testing - it is just a static page.

## Build distribution
```
cd www
npm run build
```

# `vuejs`
## Pre-requisite:
* `npm` from <https://nodejs.org/en/>

## Usage
```
cd vuejs
npm install
npm run serve
```

## Unit testing
```
cd vuejs
npm run test:unit
```

## Build distribution
```
cd vuejs
npm run build
```

# `vuejs_wasm`
## Pre-requisite:
* `npm` from <https://nodejs.org/en/>
* `rust` from <https://www.rust-lang.org/tools/install>
* WebAssembly target: `rustup target add wasm32-unknown-unknown`
* `wasm-pack` from <https://rustwasm.github.io/wasm-pack/installer/>

## Usage
```
cd vuejs_wasm
npm install
npm run serve
```

## Unit testing
Unit testing is undertaken for the VueJS implementation:
```
cd vuejs_wasm
npm run test:unit
```

And the Rust implementation:
```
cd vuejs_wasm_rust
cargo test
```

## Build distribution
```
cd vuejs_wasm
npm run build
```

# `yew`
## Pre-requisites:
* `rust` from <https://www.rust-lang.org/tools/install>
* WebAssembly target: `rustup target add wasm32-unknown-unknown`
* `trunk` utility: `cargo install trunk`

## Usage
```
cd yew
trunk serve
```

## Unit testing
```
cd yew
cargo test
```

## Build distribution
```
cd yew
trunk build --release
```

