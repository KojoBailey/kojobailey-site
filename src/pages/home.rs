use leptos::prelude::*;
use leptos_icons::Icon;
use icondata;

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

#[component]
pub fn Home() -> impl IntoView {
    let (image_index, set_image_index) = signal(0);
    let (code_index, set_code_index) = signal(0);

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
                <div class="profile-image"
                    on:click=move |_| set_image_index.update(|n| *n = (*n + 1) % IMAGES.len())
                >
                    <img src=move || IMAGES[image_index.get()] />
                </div>
                <div class="name-section">
                    <span class="name-surround left">{move || NAME_CODE[code_index.get()].0}</span>
                    <span class="name"
                        on:click=move |_| set_code_index.update(|n| *n = (*n + 1) % NAME_CODE.len())
                    >"Kojo Bailey"</span>
                    <span class="name-surround right">{move || NAME_CODE[code_index.get()].1}</span>
                </div>
                <div class="links">
                    <LinkButton href="https://github.com/KojoBailey" icon=icondata::BsGithub />
                    <LinkButton href="https://www.linkedin.com/in/kojo-bailey/" icon=icondata::BsLinkedin />
                    <LinkButton href="https://www.youtube.com/@KojoBailey" icon=icondata::BsYoutube />
                    <LinkButton href="https://www.instagram.com/kojobailey/" icon=icondata::BsInstagram />
                    <LinkButton href="https://www.reddit.com/user/Spyromaniac666/" icon=icondata::BsReddit />
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
