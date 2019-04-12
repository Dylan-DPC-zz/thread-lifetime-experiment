#[macro_use]
extern crate rental;

use crossbeam_channel::{bounded, Receiver, Sender};
use crossbeam_utils::thread;
use simdjson::borrowed::Value;
use std::io::BufReader;
use std::io::{self, BufRead};

rental! {
    mod rentals {
        use super::*;
        #[rental_mut(covariant,debug)]
        pub struct Wrap {
            raw: Box<String>,
            parsed: Data<'raw>
        }
    }
}

#[derive(Debug)]
pub enum Data<'d> {
    Raw(Value<'d>),
    None,
}

fn main() {
    thread::scope(|t| {
        let (s1, r1): (Sender<rentals::Wrap>, Receiver<rentals::Wrap>) = bounded(5);
        let (s2, r2): (Sender<rentals::Wrap>, Receiver<rentals::Wrap>) = bounded(5);
        t.spawn(move |_| {
            for data in r2 {
                {
                    data.rent(|d| match d {
                        Data::Raw(v) => {
                            println!("{:?}", v);
                        }
                        _ => (),
                    });
                }
            }
        });

        t.spawn(move |_| {
            for mut data in r1 {
                data.rent_mut(|d| match d {
                    Data::Raw(Value::Object(ref mut m)) => {
                        m.insert("bla", Value::from("blubb"));
                    }
                    _ => (),
                });

                s2.send(data);
            }
        });

        t.spawn(move |_| {
            let buffer = BufReader::new(io::stdin());

            for line in buffer.lines() {
                unsafe {
                    let mut content = line.unwrap();
                    let value = rentals::Wrap::new(Box::new(content), |content| {
                        Data::Raw(simdjson::to_borrowed_value(content.as_bytes_mut()).unwrap())
                    });

                    s1.send(value);
                }
            }
        });
    });
}
