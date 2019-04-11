use crossbeam_channel::bounded;
use crossbeam_utils::thread;
use simdjson::borrowed::Value;
use std::io::BufReader;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Data<'e> {
    Raw(Value<'e>),
    None,
}

fn main() {
    thread::scope(|t| {
        let (s1, r1) = bounded(5);
        let (s2, r2) = bounded(5);
        t.spawn(move |_| {
            for data in r2 {
                println!("{:?}", data);
            }
        });

        t.spawn(move |_| {
            for data in r1 {
		let data = match data {
                  Data::Raw(d) => Data::Raw(Value::Array(vec![d])),
                  o => o
                };
                s2.send(data);
            }
        });

        t.spawn(move |_| {
            let buffer = BufReader::new(io::stdin());

            for line in buffer.lines() {
                unsafe {
                    let mut content = line.unwrap();
                    let value = simdjson::to_borrowed_value(content.as_bytes_mut()).unwrap();

                    s1.send(Data::Raw(value));
                }
            }
        });
    });
}
