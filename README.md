# 设计大师

像大师一样设计，下一代原型设计软件

* 原型即代码 -- 直接导出Web/桌面/小程序/手机组件代码
* 高复用 -- 直接使用他人写好的组件/自定义组件
* 高性能 -- 修改立即生效，无需保存、等待
* 低存储 -- 导出文件通常只有几十K而不是几个G
* 多人协同设计 -- 多人同时设计同一作品
* 多平台 -- Web、Mac、Windows、Linux都是一等公民

## Getting started

Start by clicking "Use this template" at https://github.com/emilk/eframe_template/ or follow [these instructions](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template).

Change the name of the crate: Chose a good name for your project, and change the name to it in:
* `Cargo.toml`
    * Change the `package.name` from `eframe_template` to `your_crate`
    * Change the `package.authors`
    * Change the `package.default-run` from `eframe_template_bin` to `your_crate_bin` (note the `_bin`!)
    * Change the `bin.name` from `eframe_template_bin` to `your_crate_bin` (note the `_bin`!)
* `main.rs`
    * Change the `let app =…` line from `eframe_template::TemplateApp` to `your_crate::TemplateApp`
* `docs/index.html`
    * Change the `<title>`
    * Change the `<script src=…` line from `eframe_template.js` to `your_crate.js`
    * Change the `wasm_bindgen(…` line from `eframe_template_bg.wasm` to `your_crate_bg.wasm` (note the `_bg`!)
* `docs/sw.js`
    * Change the `'./eframe_template.js'` to `./your_crate.js` (in `filesToCache` array)
    * Change the `'./eframe_template_bg.wasm'` to `./your_crate_bg.wasm` (in `filesToCache` array)
* Remove the web build of the old name: `rm docs/eframe_template*`

### Learning about egui

`src/app.rs` contains a simple example app. This is just to give some inspiration - most of it can be removed if you like.

The official egui docs are at <https://docs.rs/egui>. If you prefer watching a video introduction, check out <https://www.youtube.com/watch?v=NtUkr_z7l84>. For inspiration, check out the [the egui web demo](https://emilk.github.io/egui/index.html) and follow the links in it to its source code.

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel`

For running the `build_web.sh` script you also need to install `jq` and `binaryen` with your packet manager of choice.

### Compiling for the web

Make sure you are using the latest version of stable rust by running `rustup update`.

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page. For this you need to set up some tools. There are a few simple scripts that help you with this:

```sh
./setup_web.sh
./build_web.sh
./start_server.sh
open http://127.0.0.1:8080/
```

* `setup_web.sh` installs the tools required to build for web
* `build_web.sh` compiles your code to wasm and puts it in the `docs/` folder (see below)
* `start_server.sh` starts a local HTTP server so you can test before you publish
* Open http://127.0.0.1:8080/ in a web browser to view

The finished web app is found in the `docs/` folder (this is so that you can easily share it with [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)). It consists of three files:

* `index.html`: A few lines of HTML, CSS and JS that loads your app. **You need to edit this** (once) to replace `eframe_template` with the name of your crate!
* `your_crate_bg.wasm`: What the Rust code compiles to.
* `your_crate.js`: Auto-generated binding between Rust and JS.

You can test the template app at <https://emilk.github.io/eframe_template/>.

## Updating egui

As of 2022, egui is in active development with frequent releases with breaking changes. [eframe_template](https://github.com/emilk/eframe_template/) will be updated in lock-step to always use the latest version of egui.

When updating `egui` and `eframe` it is recommended you do so one version at the time, and read about the changes in [the egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and [eframe changelog](https://github.com/emilk/egui/blob/master/eframe/CHANGELOG.md).
