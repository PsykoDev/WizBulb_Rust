
use crate::response_des::Root;
use crate::send_parameters::Params;
mod ip_from_mac;
mod scene_id;
mod response_des;
mod send_parameters;
mod error_parser;

use std::net::UdpSocket;
use std::time::{Duration, Instant};

fn send_data(data: & str, ip: &str, socket: & UdpSocket) -> String {
    let message = String::from(data);
    let msg_bytes = message.into_bytes();
    socket.send_to(&msg_bytes, ip).expect("couldn't send message");
    let s = String::from_utf8(msg_bytes).expect("Error parse message");
    return s;
}

async fn scan_light(socket: &UdpSocket) -> Vec<String> {
// retrieve all bulb light
    let mut ip_vec = Vec::new();
    let start_time = Instant::now();
    let desired_duration = Duration::from_millis(500);
    while start_time.elapsed() < desired_duration {
        send_data(r#"{"method":"getPilot","params":{}}"#,"255.255.255.255:38899", &socket);
       //send_data(r#"{"method":"getSystemConfig","params":{}}"#, "255.255.255.255:38899", &socket);
        let mut buf = [0; 4096];

        match socket.recv(&mut buf) {
            Ok(received) =>
                {
                    let resp = String::from_utf8_lossy(&buf[..received]);
                    let truc = serde_json::from_str::<Root>(&resp);
                    match truc {
                        Ok(resp) => {
                            let ip_address = ip_from_mac::ip_mac(resp.result.mac.as_ref());
                            if !ip_vec.contains(&ip_address){
                                ip_vec.push(ip_address);
                            }
                        }, // test
                        Err(e) => eprintln!("{}", e)
                    }
                },
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }
    return ip_vec;
}

fn send_arg(par: &Params, ip: &str, socket: &UdpSocket){
    let j = serde_json::to_string(&par).expect("Error Ser params");
    let format = format!(r#"{{"id":1,"method":"setState","params":{x}}}"#, x = j);
    send_data(&format, format!("{}:38899", ip).as_ref(), socket);
    println!("Send: {}", &format);
}

#[tokio::main]
async fn main() {

    let debug = false;

    // Connect
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Can't Bind Ip already used ?");
    socket.set_broadcast(true).expect("No Broadcast");
    println!("Broadcast: {:?}", socket.broadcast());

    let ip = scan_light(&socket).await;

    // build parameters to sending
    let par = Params{
        state: false, // on / off
        dimming: 50, // intensity
        sceneId: scene_id::Scene::Ocean as i32,
        r:0, // red
        g:0, // green
        b:0, // blue
        c:0, // cold
        w:0 // warm
    };

    // send request with parameters
    for ip_select in ip{
        send_arg(&par, ip_select.as_str(), &socket);
    }


        // get response or error
        println!("Awaiting responses...");
        loop {
            let mut buf = [0; 4096];
            match socket.recv(&mut buf) {
                Ok(received) =>
                    {
                        let resp = String::from_utf8_lossy(&buf[..received]);
                        let truc = serde_json::from_str::<Root>(&resp);
                        match truc {

                            Ok(resp) => if debug { println!("Result: {:?}", resp) }, // test
                            Err(_) => {
                                let error = serde_json::from_str::<error_parser::Root>(&resp);
                                match error {
                                    Ok(v) => eprintln!("{:?}", v.error.message),
                                    Err(e) => eprintln!("{:?}", e)
                                }
                            }
                        }
                    },
                Err(e) => println!("recv function failed: {e:?}"),
            }
        }
}

// wiz_df22da (192.168.1.118) at 6c2990df22da on en0 ifscope [ethernet] // chamber
// wiz_df40e6 (192.168.1.232) at 6c:29:90:df:40:e6 on en0 ifscope [ethernet] // kitchen