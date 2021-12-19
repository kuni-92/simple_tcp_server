mod listener;
use listener::SimpleListener;

fn main() {
    SimpleListener::new("127.0.0.1:60080").run();

}
