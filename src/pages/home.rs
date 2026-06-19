use leptos::prelude::*;
use leptos_icons::Icon;
use icondata;

use syntect::html::{ClassedHTMLGenerator, ClassStyle};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

const IMAGES: [&str; 3] = [
    "https://avatars.githubusercontent.com/u/50509420",
    "https://scontent-man2-1.cdninstagram.com/v/t51.82787-19/711494615_18203457625346491_997510973910145355_n.jpg?_nc_cat=104&ccb=7-5&_nc_sid=bf7eb4&efg=eyJ2ZW5jb2RlX3RhZyI6InByb2ZpbGVfcGljLnd3dy4zMjAuQzMifQ%3D%3D&_nc_ohc=Y8CWJKpsEmoQ7kNvwFwTiSd&_nc_oc=Adrr-K6bR1UCfZzW6jKP9Tf4hCJ9WWxAozZ_TrBCjAHK2o5aNQE0m1F595nCmGOuAxM&_nc_zt=24&_nc_ht=scontent-man2-1.cdninstagram.com&_nc_gid=95Ova-4XZwo3NOluBvnZCg&_nc_ss=7b6a8&oh=00_Af9PAzUet9TgDAULj-tY-NNTKAmgApf8riCP4nEn4fl9KQ&oe=6A365670",
    "https://media.licdn.com/dms/image/v2/D4E03AQExn-yCtx_z9A/profile-displayphoto-scale_400_400/B4EZzvmpq0IUAg-/0/1773546414380?e=1782950400&v=beta&t=7Kv9tDIIpIwqucWMXSybLbZIq5jZPfCDjdMKT5LXXpU",
];

const NAME_CODE: [(&str, &str); 12] = [
    ("std::cout << \"", "!\\n\";"),
    ("std::println(\"", "!\");"),
    ("println!(\"", "!\");"),
    ("puts(\"", "!\");"),
    ("io::printn(\"", "!\");"),
    ("echo \"", "!\""),
    ("fmt.println(\"", "!\");"),
    ("putStrLn \"", "!\""),
    ("print_endline \"", "!\""),
    ("print(\"", "!\")"),
    ("System.out.println(\"", "!\");"),
    ("std.debug.print(\"", "!\\n\");"),
];

const BACKGROUND_CODE: &str = "#include <xfbin/detail/xfbin_reader.hpp>

using namespace kojo;
using namespace kojo::nucc;
using namespace kojo::type_abbreviations;

auto Xfbin::from(const std::filesystem::path& path, const std::array<u8, 8> crypt_key)
	-> std::expected<Xfbin, XfbinError>
{
        auto maybe_data = Binary::from(path);
        if (!maybe_data) {
		return std::unexpected{
			XfbinError::from(maybe_data.error().variant)
		};
        }
	return XfbinReader{*maybe_data, crypt_key}.parse();
}

auto Xfbin::from(std::span<const std::byte> span, const std::array<u8, 8> crypt_key)
	-> std::expected<Xfbin, XfbinError>
{
	return XfbinReader{span, crypt_key}.parse();
}

auto Xfbin::from(const std::byte* ptr, const std::array<u8, 8> crypt_key)
	-> std::expected<Xfbin, XfbinError>
{
	return XfbinReader{ptr, crypt_key}.parse();
}

auto Xfbin::fetch_type_from_map_index(std::uint32_t map_index) const noexcept
	-> std::expected<ChunkType, XfbinError>
{
	if (map_index >= map_indices.size()) {
		return std::unexpected{
			XfbinError::MapIndexOutOfBounds{map_index, map_indices.size()}
		};
	}

	const std::uint32_t true_map_index = map_indices[map_index];
		
	if (true_map_index >= maps.size()) {
		return std::unexpected{
			XfbinError::MapIndexOutOfBounds{true_map_index, maps.size()}
		};
	}
		
	const ChunkMap& chunk_map = maps[true_map_index];
	return types[chunk_map.type_index];
}";

#[component]
pub fn Home() -> impl IntoView {
    let (image_index, set_image_index) = signal(0);
    let (code_index, set_code_index) = signal(0);

    let advance_image = move |_| {
        set_image_index.update(|n| *n += 1)
    };

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_name("C++").unwrap();
    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(BACKGROUND_CODE) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }
    let output_html = html_generator.finalize();

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="site-header">
                <div class="left">
                    <div class="sitename"><a href="/">"KojoBailey.me"</a></div>
                </div>
                <div class="right">
                    <div class="blog"><a href="https://kojobailey.me" target="_blank">blog</a></div>
                </div>
            </div>
            <div class="section-top">
                <pre class="background-text"><div inner_html=output_html.clone() /></pre>
                <pre class="background-text clone"><div inner_html=output_html /></pre>
                <div class="background-earth">
                    <img src="https://images-assets.nasa.gov/image/GSFC_20171208_Archive_e002131/GSFC_20171208_Archive_e002131~large.jpg" />
                </div>
                <div class="profile-image"
                    on:click=advance_image>
                    <img src=move || IMAGES[image_index.get() % IMAGES.len()] class="fade-img" />
                </div>
                <div class="name-section">
                    <span class="name-surround left">{move || NAME_CODE[code_index.get()].0}</span>
                    <span on:click=move |_| set_code_index.update(|n| *n = (*n + 1) % NAME_CODE.len()) >
                        <svg class="name" viewBox="0 0 410 80" width="410" height="80"><text x="10" y="60" font-size="60" font-weight="bold"
                            stroke-linejoin="round"
                            paint-order="stroke fill"
                        >"Kojo Bailey"</text></svg>
                    </span>
                    <span class="name-surround right">{move || NAME_CODE[code_index.get()].1}</span>
                </div>
                <div class="subtitle hover-swap">
                    <HoverSwap content=("(*", "/*") />
                    " Systems Programmer "
                    <HoverSwap content=("*)", "*/") />
                </div>
                <div class="links">
                    <div class="name-background">
                        <img src="/images/result_rank_plate_dropping_0.png" />
                    </div>
                    <LinkButton href="https://github.com/KojoBailey" icon=icondata::BsGithub />
                    <LinkButton href="https://www.linkedin.com/in/kojo-bailey/" icon=icondata::BsLinkedin />
                    <LinkButton href="https://www.youtube.com/@KojoBailey" icon=icondata::BsYoutube />
                    <LinkButton href="https://www.instagram.com/kojobailey/" icon=icondata::BsInstagram />
                    <LinkButton href="https://www.reddit.com/user/Spyromaniac666/" icon=icondata::BsReddit />
                    <LinkButton href="https://x.com/KojoBailey" icon=icondata::BsTwitter />
                </div>
                <div class="foreground">
                    <img src="/images/result_rank_deco_0.png" />
                </div>
                <div class="scroll-notice">
                    <a href="#top-quote">"↓ scroll down to learn more ↓"</a>
                </div>
            </div>
            <div id="top-quote" class="top-quote">
                <div class="quote">"Jack of all trades, master of none,"<br/>"but oftentimes better than a master of one."</div>
            </div>
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
            <div id="footer" class="site-footer">
                <div class="copyright">"© Kojo Bailey 2026"</div>
            </div>
        </ErrorBoundary>
    }
}

#[component]
pub fn LinkButton(href: &'static str, icon: icondata::Icon) -> impl IntoView {
    view! {
        <a class="link" href=href target="_blank"><Icon icon={icon} /></a>
    }
}

#[component]
pub fn HoverSwap(
    content: (&'static str, &'static str),
) -> impl IntoView {
    view! {
        <span class="hover-swap">
            <span class="show-by-default">{content.0}</span>
            <span class="show-on-hover">{content.1}</span>
        </span>
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
