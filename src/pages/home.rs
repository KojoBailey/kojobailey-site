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
            <div class="container">
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
                </div>
                <div class="foreground">
                    <img src="/images/result_rank_deco_0.png" />
                </div>
            </div>
            <div class="container" style="background-color: red">
            <div class="site-header">
                <div class="sitename"><a href="https://kojobailey.me">"KojoBailey.me"</a></div>
            </div>
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
