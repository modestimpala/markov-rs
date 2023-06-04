use std::collections::HashMap;
use std::fs;
use std::io;
use std::option::Option;
use std::vec::Vec;

use rand::distributions::{Distribution, WeightedIndex};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio::task;

#[derive(Default)]
struct Markov<T>
    where
        T: Clone + Eq + std::hash::Hash,
{
    markov: HashMap<Vec<T>, (usize, HashMap<T, usize>)>,
}

impl<T> Markov<T>
    where
        T: Clone + Eq + std::hash::Hash,
{
    fn new() -> Self {
        Markov {
            markov: HashMap::new(),
        }
    }

    fn add_inscription(&mut self, inscription: Vec<T>) {
        if inscription.len() < 3 {
            return;
        }

        let mut source = self.markov.entry(inscription[..2].to_vec()).or_default();

        for i in 2..inscription.len() {
            let next = &mut source.1;

            *next.entry(inscription[i].clone()).or_insert(0) += 1;
            source.0 += 1;

            let key = inscription[i - 1..i + 1].to_vec();
            source = self.markov.entry(key).or_default();
        }
    }

    fn go(&self, n: usize) -> Vec<T> {
        if n * self.markov.len() == 0 {
            return Vec::new();
        }

        let e = rand::random::<usize>() % self.markov.len();
        let mut k = 0;

        for (key, _) in &self.markov {
            if k == e {
                let mut result = key.clone();
                let mut source = key.clone();
                for _ in 0..n {
                    if let Some(new_letter) = self.get_letter(&source) {
                        result.push(new_letter.clone());
                        source = result[result.len() - 2..].to_vec();
                    } else {
                        return result;
                    }
                }
                return result;
            }
            k += 1;
        }

        panic!("Invalid index");
    }

    fn get_letter(&self, source: &[T]) -> Option<T>
        where
            T: Clone,
    {
        if self.markov.is_empty() {
            return None;
        }

        if let Some(source_entry) = self.markov.get(source) {
            if !source_entry.1.is_empty() {
                let mut weights = Vec::new();
                let mut letters = Vec::new();

                for (letter, count) in &source_entry.1 {
                    letters.push(letter.clone());

                    weights.push(*count);
                }

                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = rand::thread_rng();
                let index = dist.sample(&mut rng);

                return Some(letters[index].clone());
            }
        }

        None
    }
}

#[tokio::main]
async fn main() {
    let mut markov: Markov<u8> = Markov::new();

    println!("Enter directory path: ");
    let mut dir_path = String::new();
    io::stdin()
        .read_line(&mut dir_path)
        .expect("Failed to read line");

    let dir_path = dir_path.trim();
    let entries = fs::read_dir(dir_path).expect("Failed to read directory");

    let (tx, mut rx) = mpsc::channel(100); // Specify the buffer size

    let mut tasks = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            if let Some(extension) = file_path.extension() {
                if extension == "txt" {
                    let tx = tx.clone();
                    let file_path = file_path.clone();
                    let task = task::spawn(async move {
                        let mut buffer = Vec::new();
                        let mut file = File::open(file_path).await.expect("Failed to open file");
                        let bytes_read = file
                            .read_to_end(&mut buffer)
                            .await
                            .expect("Failed to read from file");
                        if bytes_read > 0 {
                            tx.send(buffer).await.expect("Failed to send buffer");
                        }
                    });
                    tasks.push(task);
                }
            }
        }
    }

    drop(tx);

    for task in tasks {
        task.await.expect("Failed to wait for task");
    }

    while let Some(buffer) = rx.recv().await {

        markov.add_inscription(buffer);
    }

    println!("Enter number of letters: ");
    let mut number_of_letters = String::new();
    io::stdin()
        .read_line(&mut number_of_letters)
        .expect("Failed to read line");
    let number_of_letters: usize = number_of_letters.trim().parse().expect("Please type a number!");

    for i in markov.go(number_of_letters) {
        print!("{}", i as char);
    }

    println!();
}