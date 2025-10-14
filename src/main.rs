mod ls;
mod rect;
mod threads;

fn main() {
    threads::inter_thread();
    // threads::async_with_channels();
    // threads::thread_join();
    // rect::test();
    // ls::ls();
}
