use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc channel for sending input data from input thread to render thread
    let (input_tx, input_rx) = channel();

    // mpsc channel for sending gameover signal from render thread to input thread
    let (gameover_tx, gameover_rx) = channel();

    // the input thread will keep receiving direction input
    // and pass it to the render thread to update the snake's direction.
    let input_thread = thread::spawn(move || {
        loop {
            // exit loop when gameover signal is sent from the render thread
            if gameover_rx.try_recv().is_ok() {
                return;
            }

            // receive a line of string as input
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("input failed");

            // try to read and convert the first letter to a Direction
            let input = input
                .chars()
                .next()
                .map_or(None, |key| snake::direction_from_key(key));

            // notify the render thread if we have a valid input ('w', 'a', 's', or 'd')
            if let Some(direction) = input {
                input_tx
                    .send(direction)
                    .expect("sending input data to render thread failed");
            }
        }
    });

    // the render thread will receive input signal from the input thread
    // to update and render the game map every 0.5 seconds
    let render_thread = thread::spawn(move || {
        let mut game = snake::Game::new(20, 10, 5);

        loop {
            // receive input
            let input = input_rx
                .try_recv()
                .ok();
    
            // update and handle game over
            if let Err(reason) = game.update(input) {
                println!("Game over: {reason}");
                println!("Press enter to quit...");

                // notify input thread to stop loop
                gameover_tx
                    .send(())
                    .expect("sending gameover signal to input thread failed");

                return;
            }

            // render new state
            game.render();

            thread::sleep(Duration::from_millis(500));
        }
    });

    // cleanup threads
    input_thread
        .join()
        .expect("input thread failed to join");

    render_thread
        .join()
        .expect("render thread failed to join");
}
