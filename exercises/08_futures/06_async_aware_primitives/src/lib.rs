/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
// use std::sync::mpsc;
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let msg = receiver.recv().await.unwrap() {
            println!("Pong received: {}", msg.payload);

            //Initialize a new channel for what?
            let (sender_3, new_receiver_3) = mpsc::channel(1);

            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender_3,
                })
                .await
                .unwrap();

            receiver = new_receiver_3;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    // use std::sync::mpsc;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        //Initializing query channel
        let (sender_1, receiver_1) = mpsc::channel(1);

        //Initializing response channel
        let (response_sender_2, mut response_receiver_2) = mpsc::channel(1);

        //Sending message in query channel
        sender_1
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender_2,
            })
            .await
            .unwrap();

        //Running pong fn as a new tokio task
        tokio::spawn(pong(receiver_1));

        //Fetching payload value from the response channel
        let answer = response_receiver_2.recv().await.unwrap().payload;

        //Assertion
        assert_eq!(answer, "pong");
    }
}
