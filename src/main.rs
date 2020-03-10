mod modules;

use libbar::Bar;

fn configure(bar: &mut Bar) {
    bar.register(modules::Clock::new().boxed());
    bar.register(modules::Load::new().boxed());
    bar.register(modules::Battery::new().boxed());
}

fn main() {
    let mut bar = Bar::new();
    configure(&mut bar);
    bar.run();
}
