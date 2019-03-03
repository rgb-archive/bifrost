use core::borrow::BorrowMut;
use std::convert::Into;
use std::io::Read;
use std::str::FromStr;

use bitcoin::network::serialize::Error;
use bitcoin::OutPoint;
use bitcoin::util::hash::Sha256dHash;
use hyper::method::Method;
use hyper::method::Method::{Get, Post};
use hyper::net::Fresh;
use hyper::NotFound;
use hyper::server::{Request, Response, Server};
use hyper::server::Handler;
use hyper::uri::RequestUri::AbsolutePath;
use rgb::proof::Proof;

use database::Database;

struct RGBServer {
    database: Database
}

impl RGBServer {
    pub fn new(database: Database) -> RGBServer {
        RGBServer {
            database
        }
    }

    fn get_outpoint(&self, path: &String) -> Option<OutPoint> {
        let parts: Vec<&str> = path.split("/").collect();

        if parts.len() != 2 {
            eprintln!("Invalid request");
            return None;
        }

        let parts: Vec<&str> = parts[1].split(":").collect();

        if let (Ok(vout), Ok(txid)) = (parts[1].parse(), Sha256dHash::from_hex(parts[0])) {
            Some(OutPoint {
                txid,
                vout
            })
        } else {
            None // Could not parse the vout or the txid
        }
    }
}

impl Handler for RGBServer {
    fn handle<'a, 'k>(&'a self, mut req: Request<'a, 'k>, mut res: Response<'a, Fresh>) {
        let mut buffer: Vec<u8> = Vec::new();
        req.borrow_mut().read_to_end(&mut buffer);

        match req.uri {
            AbsolutePath(ref path) => {
                if req.method == Get {
                    let outpoint = match self.get_outpoint(path) {
                        Some(val) => val,
                        None => {
                            eprintln!("Invalid outpoint in GET req for `{}`", path);
                            return;
                        }
                    };

                    let proofs = self.database.get_proofs_for(&outpoint);

                    use bitcoin::network::serialize::RawEncoder;
                    use bitcoin::network::encodable::ConsensusEncodable;

                    let mut encoded: Vec<u8> = Vec::new();
                    let mut enc = RawEncoder::new(encoded);
                    proofs.consensus_encode(&mut enc);

                    res.send(&enc.into_inner()).unwrap();

                    println!("Downloaded {} proofs for {}", proofs.len(), outpoint);

                    return;
                } else if req.method == Post {
                    let outpoint = match self.get_outpoint(path) {
                        Some(val) => val,
                        None => {
                            eprintln!("Invalid outpoint in POST req for `{}`", path);
                            return;
                        }
                    };

                    use bitcoin::network::serialize::deserialize;
                    let decoded: Result<Proof, Error> = deserialize(&mut buffer);

                    if let Err(e) = decoded {
                        eprintln!("Could not decode the uploaded proof for `{}`: {}", path, e);
                        return;
                    }

                    let decoded = decoded.unwrap();
                    println!("Uploaded proof for {}", outpoint);

                    self.database.save_proof(&decoded, &outpoint.txid);
                } else {
                    *res.status_mut() = NotFound;
                    return;
                }
            },
            _ => {
                return;
            }
        };
    }
}

pub fn start_server(port: String, database: Database) {
    let rgb_server = RGBServer::new(database);

    let _listening = Server::http(format!("0.0.0.0:{}", port)).unwrap()
        .handle(rgb_server);
    println!("Listening on http://0.0.0.0:{}", port);
}