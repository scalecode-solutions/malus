use malus::*;

fn main() {
    App::new("Hello Malus")
        .window("Hello Malus", 480.0, 320.0,
            vstack![
                text("Welcome to Malus!").font_size(24.0),
                spacer(),
                hstack![
                    text("Name:"),
                    textfield("Enter your name...")
                        .on_change(|val| println!("typing: {val}")),
                ],
                spacer(),
                button("Say Hello")
                    .on_click(|| println!("Hello from Malus!")),
            ]
        )
        .run();
}
