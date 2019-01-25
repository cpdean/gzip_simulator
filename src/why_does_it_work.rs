use cursive::Cursive;
use cursive::views::TextView;

fn main() {
	let mut siv = Cursive::termion();

	siv.add_global_callback('q', |s| s.quit());

	siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));

	siv.run();
}
