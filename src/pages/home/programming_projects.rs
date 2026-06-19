use leptos::prelude::*;

#[component]
pub fn SectionProgrammingProjects() -> impl IntoView {
    view! {
        <div id="programming-projects" class="programming-projects">
            <div class="title">"Programming Projects"</div>
            <div class="boxes">
                <ProgrammingProject
                    title="KojoBailey.me"
                    language="Rust + Leptos"
                    logo="Rust"
                    thumbnail="https://i.ebayimg.com/images/g/lc0AAOSwlNFmaIc4/s-l1200.jpg" 
                    link="https://github.com/KojoBailey/kojobailey-site"
                    desc="This website was created using Leptos, a full-stack framework for webdev in Rust."
                    motivation="With my love for low-level languages like C++, eagerness to learn more Rust, and dislike of JavaScript, I deemed it a fitting choice."
                />
                <ProgrammingProject
                    title="Tic-Tac-Toe"
                    language="C++"
                    logo="C++"
                    thumbnail="https://i.ebayimg.com/images/g/lc0AAOSwlNFmaIc4/s-l1200.jpg" 
                    link="https://github.com/KojoBailey/tic-tac-toe-cpp"
                    desc="A command-line implementation of the game Tic-Tac-Toe in modern C++."
                    motivation="With less than 400 lines of code, this was a good excuse for me to actually finish a project for once."
                />
                <ProgrammingProject
                    title="QuickShell Dots"
                    language="QML + QuickShell"
                    logo="QML"
                    thumbnail="https://i.ebayimg.com/images/g/lc0AAOSwlNFmaIc4/s-l1200.jpg" 
                    link="https://github.com/KojoBailey/quickshell-dotfiles"
                    desc="My configurations for QuickShell to rice my installation of Arch Linux Hyprland."
                    motivation="Plus, a good excuse to learn some QML, QT's DSL for UI."
                />
                <ProgrammingProject
                    title="Worlde Solver"
                    language="Haskell"
                    logo="Haskell"
                    thumbnail="https://i.ebayimg.com/images/g/lc0AAOSwlNFmaIc4/s-l1200.jpg" 
                    link="https://github.com/KojoBailey/wordle-solver-hs"
                    desc="A CLI tool to make solving Wordle puzzles stupidly easy."
                    motivation="Helped me to practise Haskell while studying it in university, and temporarily pacify my frustrations from solving Wordle daily on Discord."
                />
                <ProgrammingProject
                    title="Image to ASCII"
                    language="Haskell"
                    logo="Haskell"
                    thumbnail="https://i.ebayimg.com/images/g/lc0AAOSwlNFmaIc4/s-l1200.jpg" 
                    link="https://github.com/KojoBailey/image-to-ascii-hs"
                    desc="A CLI tool to convert generic images to ASCII art, with the help of the JuicyPixels module."
                    motivation="Allowed me to both practise Haskell and learn how ASCII art works as a follow up to a similar project of mine in C++."
                />
                <ProgrammingProject
                    title="CLI Calculator"
                    language="OCaml"
                    logo="OCaml"
                    thumbnail="https://i.ebayimg.com/images/g/lc0AAOSwlNFmaIc4/s-l1200.jpg" 
                    link="https://github.com/KojoBailey/cli-calculator-ocaml"
                    desc="A command-line calculator that evaluates mathematical expressions, with additional support for variables, constants, and functions."
                    motivation="Built to learn OCaml and the pipeline for evaluating expressions from character input."
                />
                <ProgrammingProject
                    title="XFBIN++"
                    language="C++"
                    logo="C++"
                    thumbnail="https://opengraph.githubassets.com/a29ca74f297390ee1b64552778d96008f9342589b47765161b23ea50bb5edd68/KojoBailey/XFBIN-Parser-PlusPlus" 
                    link="https://github.com/KojoBailey/nucc-cpp-library"
                    desc="A modern C++ library to allow easy interaction with CyberConnect2's XFBIN container file format."
                    motivation="Useful for modding many of their games."
                />
                <ProgrammingProject
                    title="Binary++"
                    language="C++"
                    logo="C++"
                    thumbnail="https://opengraph.githubassets.com/a29ca74f297390ee1b64552778d96008f9342589b47765161b23ea50bb5edd68/KojoBailey/XFBIN-Parser-PlusPlus" 
                    link="https://github.com/KojoBailey/binary-cpp-library"
                    desc="A modern C++ utility library to assist with reading from and writing to binary data, making use of my own experience as a reverse engineer."
                    motivation=""
                />
                <ProgrammingProject
                    title="JoJoAPI Wrapper"
                    language="Rust around C"
                    logo="Rust"
                    thumbnail="https://opengraph.githubassets.com/a29ca74f297390ee1b64552778d96008f9342589b47765161b23ea50bb5edd68/KojoBailey/XFBIN-Parser-PlusPlus" 
                    link="https://github.com/KojoBailey/japi-wrapper-rs"
                    desc="A Rust wrapper around the C-interface modding API for JoJo's Bizarre Adventure: All-Star Battle R, JoJoAPI."
                    motivation="This would make it easier for myself and others to create JoJoAPI plugins in Rust."
                />
                <ProgrammingProject
                    title="Snake"
                    language="Nim + Raylib"
                    logo="Nim"
                    thumbnail="https://raw.githubusercontent.com/KojoBailey/snake-nim/refs/heads/main/preview/gameplay.png" 
                    link="https://github.com/KojoBailey/snake-nim"
                    desc="An implementation of the game Snake in Nim, using Raylib."
                    motivation="Built as a playful introduction to Nim."
                />
                <ProgrammingProject
                    title="Pong"
                    language="Odin + Raylib"
                    logo="Odin"
                    thumbnail="https://raw.githubusercontent.com/KojoBailey/snake-nim/refs/heads/main/preview/gameplay.png" 
                    link="https://github.com/KojoBailey/pong-odin"
                    desc="An implementation of the classic game Pong in Odin, using Raylib."
                    motivation="Built as an interesting introduction to Odin given that the language is particularly known for game programming."
                />
                <ProgrammingProject
                    title="rsLox"
                    language="Rust"
                    logo="Rust"
                    thumbnail="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRd_iFdE-ld4B8x2W_R2g6Ls5Z4l5X0Fw9wQ4vx6q_US9RKV3XappXrs_Og&s=10" 
                    link="https://github.com/KojoBailey/rslox"
                    desc="An implementation of the Lox example programming language in Rust, following the Crafting Interpreters book despite its guidance being for Java."
                    motivation="My first proper attempt at creating a programming language compiler."
                />
                <ProgrammingProject
                    title="jojomodding.com"
                    language="React JavaScript"
                    logo="React"
                    thumbnail="https://static.wikitide.net/jojomoddingwiki/c/ce/Jojomodding_site.png" 
                    link="https://github.com/JoJosBizarreModdingCommunity/JoJosBizarreModdingCommunity.github.io"
                    desc="The official portal for JoJo's Bizarre Modding Community, powered by ReactJS."
                    motivation="Helps to promote the community's online presence."
                />
                <ProgrammingProject
                    title="SvZ2 Kaizen"
                    language="Unity C#"
                    logo="Unity"
                    thumbnail="https://raw.githubusercontent.com/KojoBailey/SvZ2-Kaizen/refs/heads/master/SplashScreen.png"
                    link="https://github.com/KojoBailey/SvZ2-Kaizen"
                    desc="A mod of the game Samurai vs Zombies Defense 2, which was decompiled for Unity 2017."
                    motivation="Combines programming with my passion for game design, redesigning the game to be more balanced, challenging, and fun!"
                />
                <ProgrammingProject
                    title="Shin SvZ"
                    language="Unity C#"
                    logo="Unity"
                    thumbnail="https://img.itch.zone/aW1hZ2UvMzYyNDU3OS8yMTU3MTI1MS5wbmc=/original/aT2zhk.png"
                    link="https://github.com/KojoBailey/Shin-Samurai-vs-Zombies-Defence"
                    desc="A remake of the Samurai vs Zombies Defense titles in modern Unity, combining and expanding them into the ultimate SvZ experience."
                    motivation="Rewriting the codebase from scratch allows me to make use of modern C# and Unity features."
                />
            </div>
        </div>
    }
}


#[component]
pub fn ProgrammingProject(
    title: &'static str,
    logo: &'static str,
    language: &'static str,
    thumbnail: &'static str,
    link: &'static str,
    desc: &'static str,
    motivation: &'static str,
) -> impl IntoView {
    view! {
        <a class="box" href={link} target="_blank">
            <div class="top">
                <div class="icon">
                    <img src=format!("/images/Logo_{}.svg", logo) />
                </div>
                <div class="text">
                    <div class="heading">{title}</div>
                    <div class="languages">{language}</div>
                </div>
            </div>
            <div class="thumbnail">
                <img src={thumbnail} />
            </div>
            <div class="desc"><p>{desc}</p><p>{motivation}</p></div>
        </a>
    }
}
