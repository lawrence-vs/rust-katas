/// The length of every word, computed on spawned tasks. Order doesn't matter.
/// CLUE: `tokio::spawn` needs a `'static` future. Who owns `lengths` while the tasks run?
/// CLUE: A task can hand its result back without sharing anything. Which two ways does tokio give you?
pub async fn word_lengths(words: Vec<String>) -> Vec<usize> {
    let mut lengths = Vec::new();
    let mut handles = Vec::new();
    for w in words {
        handles.push(tokio::spawn(async {
            lengths.push(w.len());
        }));
    }
    for h in handles {
        h.await.unwrap();
    }
    lengths
}
