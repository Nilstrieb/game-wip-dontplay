sudo apt install libgdk-pixbuf-2.0

sudo apt-get install libsdl-pango-dev

sudo apt-get install libatk1.0-dev

sudo apt-get install libgtk-3-dev

sudo apt-get install libsfml-dev

```
cargo minimize --script-path=./repro.sh --script-path-lints=./minimize-lints.sh --ignore-file=./src/tiles.rs ./src
```
