mod ls;
mod rect;
mod threads;

fn main() {
    //threads::threads();
    //threads::thread_join();
    //threads::inter_thread();
    //threads::channels();
    threads::async_with_channels();
    // rect::test();
    // ls::ls();
}
