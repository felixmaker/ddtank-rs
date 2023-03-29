use clap::Parser;
use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

enum UserEvent {
    Navigation(String),
    ExistWithCookie(String),
}

#[derive(Parser, Debug, Default)]
#[command(version)]
struct Args {
    /// The target url to get cookie
    #[arg(short, long)]
    url: String,

    /// Javascript regex pattern. Print document.cookie and exit when it matches document.cookie.
    #[arg(short, long)]
    regex_pattern: String,

    /// Webview start page url, default to url.
    #[arg(short, long)]
    start_page: Option<String>,

    /// Webview window title, default to url.
    #[arg(short, long)]
    title: Option<String>,
}

fn main() {
    let args = Args::parse();
    let start_url = args.start_page.unwrap_or(args.url.clone());
    let title = args.title.unwrap_or(args.url.clone());
    let pattern = args.regex_pattern;

    let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
    let proxy_one = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title(&title)
        .build(&event_loop)
        .unwrap();

    let proxy = proxy_one.clone();
    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url(&start_url)
        .unwrap()
        .with_navigation_handler(move |uri| {
            let submittd = proxy.send_event(UserEvent::Navigation(uri.clone())).is_ok();
            submittd
        })
        .build()
        .unwrap();

    let proxy = proxy_one.clone();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                webview
                    .evaluate_script_with_callback("document.cookie", {
                        let proxy = proxy.clone();
                        let re = regex::Regex::new(&pattern).unwrap();
                        move |cookie: String| {
                            if re.is_match(&cookie) {
                                println!("#{}#", cookie);
                                let _ = proxy.send_event(UserEvent::ExistWithCookie(cookie));
                            }
                        }
                    })
                    .unwrap();
                *control_flow = ControlFlow::ExitWithCode(1);
            }
            Event::UserEvent(UserEvent::Navigation(uri)) => {
                webview
                    .evaluate_script_with_callback("document.cookie", {
                        let proxy = proxy.clone();
                        let re = regex::Regex::new(&pattern).unwrap();
                        move |cookie: String| {
                            if re.is_match(&cookie) {
                                let _ = proxy.send_event(UserEvent::ExistWithCookie(cookie));
                            }
                        }
                    })
                    .unwrap();
            }
            Event::UserEvent(UserEvent::ExistWithCookie(cookies)) => {
                println!("#{}#", cookies);
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    })
}
