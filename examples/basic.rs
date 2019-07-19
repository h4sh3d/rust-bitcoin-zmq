use bitcoin_zmq::Subscriber;
use futures::{lazy, Future, Stream};

fn main() {
    // Declare subscriber
    let subscriber = Subscriber::new("tcp://127.0.0.1:28332", 1024);

    // Do something with stream of raw txs
    let print_txs = subscriber.subscribe_raw_tx().for_each(|raw_tx| {
        println!("raw tx: {:?}", raw_tx);
        Ok(())
    });

    // Connection future
    let connect = subscriber
        .connect()
        .map_err(|err| println!("err = {:?}", err));

    tokio::run(lazy(|| {
        tokio::spawn(connect); // Connect
        tokio::spawn(print_txs); // Do something
        Ok(())
    }))
}
