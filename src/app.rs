use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use regex::bytes::Regex;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Code {
    code: &'static str,
    desc: &'static str,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rustexp_leptos.css"/>

        // sets the document title
        <Title text="Rustexp-leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|| view! {  <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (pattern, set_pattern) = create_signal(String::default());
    let (subject, set_subject) = create_signal(String::default());

    let result = create_memo(move |_| run_regex(&pattern.get(), &subject.get()));

    let reference = get_reference();
    let modifiers = get_modifiers();

    view! {
        <div class="bg-slate-800 container max-w-none m-0 px-4 pt-10 font-sans">
            <header class="container text-white text-center space-y-4 px-1">
                <p class="text-4xl">"Rustexp-leptos"</p>
                <p class="text-lg">"A Rust regular expression editor & tester"</p>
            </header>

            <div class="container resize-y flex max-w-5xl items-stretch font-mono leading-snug space-x-4 mt-8">
                <div class="container space-y-1 px-1">
                    <div>
                        <label for="pattern" class="block text-white">"Regex"</label>
                        <textarea
                            on:input=move |ev| {set_pattern.set(event_target_value(&ev));}
                            prop:value=pattern
                            class="bg-slate-600 text-slate-300 p-1 w-full"
                            name="pattern" rows="1"
                        />
                    </div>
                    <div>
                        <label for="subject" class="block text-white">"Subject"</label>
                        <textarea
                            on:input=move |ev| set_subject.set(event_target_value(&ev))
                            prop:value=subject
                            class="bg-slate-600 text-slate-300 p-1 w-full"
                            name="subject" rows="5"
                        />
                    </div>
                </div>
                <div class="container overflow-auto bg-slate-700 text-slate-300 p-2 mt-6 text-white">
                    <pre class="h-full">{ move || result.get() }</pre>
                </div>
            </div>
            <div class="container max-w-5xl text-slate-400 text-left font-mono mt-10 pb-8 px-1">
                <p class="mt-8 text-white">"Reference:"</p>
                <ul class="list-none mt-4 columns-1 md:columns-3 pl-2">
                    {
                        reference.into_iter()
                        .map(|r| view! {
                                <li>
                                    <code class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap">
                                        {move || r.code}
                                    </code>{move || r.desc}
                                </li>
                            })
                        .collect::<Vec<_>>()
                    }
                </ul>
                <p class="mt-8 text-white">"Modifiers (enable: "
                    <code class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap">"(?a)"</code>", disable: "
                    <code class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap">"(?-a)"</code>"):"
                </p>
                <ul class="list-none mt-4 columns-1 md:columns-3 pl-2">
                    {
                        modifiers.into_iter()
                        .map(|m| view! {
                                <li>
                                    <code class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap">
                                        {move || m.code}
                                    </code>{move || m.desc}
                                </li>
                            })
                        .collect::<Vec<_>>()
                    }
                </ul>
                <p class="mt-8 text-center">
                    "For more information see the "
                    <a href="https://docs.rs/regex/" class="text-white" target="_blank">"documentation for the regex crate"</a>"."
                </p>
            </div>
        </div>
        <footer class="container max-w-none m-0 py-8 px-1 bg-slate-700 text-slate-400 text-center">
            <p>
                "Rustexp-leptos is " <a href="http://en.wikipedia.org/wiki/Free_software" class="text-white" target="_blank">
                    "Free software"</a> ", available under the "
                <a href="https://gnu.org/licenses/agpl.html" class="text-white" target="_blank">"GNU
                    AGPL3"</a>" licence."
            </p>
            <p>
                "The source code is freely available on "
                <a href="https://github.com/wr8fdy/rustexp-leptos" class="text-white" target="_blank">"GitHub"</a>"."
            </p>
            <p>
                "Inspired by Louis Pilfold's excellent "
                <a href="https://rustexp.lpil.uk/" class="text-white" target="_blank">"Rustexp"</a>"."
            </p>
            <p>
                "Copyright © 2023 - Present "
                <a href="https://github.com/wr8fdy" class="text-white" target="_blank">"wr8fdy"</a>". All Rights
                Reserved."
            </p>
        </footer>
    }
}

fn get_modifiers() -> Vec<Code> {
    vec![
        Code {
            code: "u",
            desc: "unicode",
        },
        Code {
            code: "i",
            desc: "case insensitive",
        },
        Code {
            code: "s",
            desc: "dot matches newline",
        },
        Code {
            code: "m",
            desc: "multiline",
        },
        Code {
            code: "x",
            desc: "whitespace ignored",
        },
        Code {
            code: "f",
            desc: "start on first line",
        },
        Code {
            code: "r",
            desc: "inverts greediness",
        },
    ]
}

fn get_reference() -> Vec<Code> {
    vec![
        Code {
            code: ".",
            desc: "non-newline char",
        },
        Code {
            code: "^",
            desc: "start of line",
        },
        Code {
            code: "$",
            desc: "end of line",
        },
        Code {
            code: "\\b",
            desc: "word boundary",
        },
        Code {
            code: "\\B",
            desc: "non-word boundary",
        },
        Code {
            code: "\\A",
            desc: "start of subject",
        },
        Code {
            code: "\\z",
            desc: "end of subject",
        },
        Code {
            code: "\\d",
            desc: "decimal digit",
        },
        Code {
            code: "\\D",
            desc: "non-decimal digit",
        },
        Code {
            code: "\\s",
            desc: "whitespace",
        },
        Code {
            code: "\\S",
            desc: "non-whitespace",
        },
        Code {
            code: "\\w",
            desc: "word character",
        },
        Code {
            code: "\\W",
            desc: "non-word character",
        },
        Code {
            code: "(a|z)",
            desc: "a or z",
        },
        Code {
            code: "[az]",
            desc: "a or z",
        },
        Code {
            code: "[^az]",
            desc: "not a or z",
        },
        Code {
            code: "[a-z]",
            desc: "a through z",
        },
        Code {
            code: "(foo)",
            desc: "capture foo",
        },
        Code {
            code: "a?",
            desc: "0 or 1 a",
        },
        Code {
            code: "a*",
            desc: "0 or more a",
        },
        Code {
            code: "a+",
            desc: "1 or more a",
        },
        Code {
            code: "a{3}",
            desc: "3 of a",
        },
        Code {
            code: "a{3,}",
            desc: "3 or more a",
        },
        Code {
            code: "a{3,5}",
            desc: "3 through 5 a",
        },
    ]
}

fn run_regex(pattern: &str, subject: &str) -> String {
    if pattern.is_empty() {
        return String::new();
    }

    let regex = match Regex::new(&pattern) {
        Err(e) => {
            return e.to_string();
        }
        Ok(re) => re,
    };

    let formatted = format_captures(regex, &subject);
    formatted
}

fn format_captures(regex: regex::bytes::Regex, subject: &str) -> String {
    let mut buffer = String::new();

    for captures in regex.captures_iter(subject.as_bytes()) {
        write!(&mut buffer, "Some(Captures({{\n").unwrap();

        for (i, cap) in captures.iter().enumerate() {
            write!(
                &mut buffer,
                "    {}: Some({:#?}),\n",
                i,
                std::str::from_utf8(cap.unwrap().as_bytes()).unwrap()
            )
            .unwrap();
        }
        write!(&mut buffer, "}})),\n").unwrap();
    }

    if buffer == "" {
        String::from("None")
    } else {
        buffer
    }
}
