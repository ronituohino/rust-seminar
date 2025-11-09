mod channels;
mod threads;
mod tokio;

pub fn some_stuff() {
    //threads::threads();
    //threads::thread_join();
    //threads::inter_thread();
    //threads::channels();
    channels::async_with_channels();
    // rect::test();
    // ls::ls();
}
