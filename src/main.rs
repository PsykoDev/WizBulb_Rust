use std::io::{stdin, stdout, Write};
use std::net::UdpSocket;
use std::time::{Duration, Instant};

use crate::response_des::Root;
use crate::send_parameters::Params;

mod error_parser;
mod get_bulb_statut;
mod ip_from_mac;
mod response_des;
mod scene_id;
mod send_parameters;

fn send_data(data: &str, ip: &str, socket: &UdpSocket) {
    let msg_bytes = String::from(data).into_bytes();
    socket
        .send_to(&msg_bytes, ip)
        .expect("couldn't send message");
    let mut buf = [0; 4096];

    match socket.recv(&mut buf) {
        Ok(received) => println!("send_data: {}", String::from_utf8_lossy(&buf[..received])),
        Err(e) => println!("recv function failed: {e:?}"),
    }
}

fn get_status(data: &str, ip: &str, socket: &UdpSocket) {
    let msg_bytes = String::from(data).into_bytes();
    socket
        .send_to(&msg_bytes, format!("{}:38899", ip))
        .expect("couldn't send message");
    let mut buf = [0; 4096];

    match socket.recv(&mut buf) {
        Ok(received) => {
            let resp = String::from_utf8_lossy(&buf[..received]);
            let truc = serde_json::from_str::<get_bulb_statut::Root>(&resp);
            match truc {
                Ok(resp) => println!("get_status: {:?}", resp), // test
                Err(e) => eprintln!("{}", e),
            }
        }
        Err(e) => println!("recv function failed: {e:?}"),
    }
}

async fn scan_light(socket: &UdpSocket) -> Vec<String> {
    // retrieve all bulb light
    let mut ip_vec = Vec::new();
    let start_time = Instant::now();
    let desired_duration = Duration::from_millis(500);
    while start_time.elapsed() < desired_duration {
        send_data(
            r#"{"method":"getPilot","params":{}}"#,
            "255.255.255.255:38899",
            &socket,
        );
        //send_data(r#"{"method":"getSystemConfig","params":{}}"#, "255.255.255.255:38899", &socket);
        let mut buf = [0; 4096];

        match socket.recv(&mut buf) {
            Ok(received) => {
                let resp = String::from_utf8_lossy(&buf[..received]);
                let truc = serde_json::from_str::<Root>(&resp);
                match truc {
                    Ok(resp) => {
                        let ip_address = ip_from_mac::ip_mac(resp.result.mac.as_ref());
                        if !ip_vec.contains(&ip_address) {
                            ip_vec.push(ip_address);
                        }
                    } // test
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }
    return ip_vec;
}

fn send_arg(par: &Params, ip: &str, socket: &UdpSocket) {
    let j = serde_json::to_string(&par).expect("Error Ser params");
    let format = format!(r#"{{"id":1,"method":"setState","params":{x}}}"#, x = j);
    send_data(&format, format!("{}:38899", ip).as_ref(), socket);
    println!("send_arg: {}", &format);
}

#[tokio::main]
async fn main() {
    // Connect
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Can't Bind Ip already used ?");
    socket.set_broadcast(true).expect("No Broadcast");
    let ip = scan_light(&socket).await;

    let mut s = String::new();
    let mut bulb = String::new();

    loop {
        s.clear();
        bulb.clear();
        for i in 0..ip.len() {
            bulb += format!("{}: {}\n", i, ip[i]).as_str();
        }

        print!("{}", bulb);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string example: 0, off");

        let reply: Vec<&str> = s.split(",").collect();
        let ip_bulb = reply[0];

        let on_off = if reply.len() >= 2 { reply[1] } else { "on" };

        let on_off_value = match on_off.trim().to_lowercase().as_str() {
            "on" | "true" => true,
            "off" | "false" => false,
            _ => panic!("TA RACE"),
        };

        // get actual status of selected bulb
        let ip_selected = ip[ip_bulb.trim().parse::<usize>().unwrap()].as_str();
        get_status(r#"{"method":"getPilot","params":{}}"#, ip_selected, &socket);

        // build parameters to sending
        let par = Params {
            state: on_off_value, // on / off
            dimming: 50,         // intensity
            sceneId: scene_id::Scene::Romance as i32,
            r: 0, // red
            g: 0, // green
            b: 0, // blue
            c: 0, // cold
            w: 0, // warm
        };

        // send arg to bulb selected
        send_arg(&par, ip_selected, &socket);
    }
}

// wiz_df22da (192.168.1.118) at 6c:29:90:df:22:da on en0 ifscope [ethernet] // chamber
// wiz_df40e6 (192.168.1.232) at 6c:29:90:df:40:e6 on en0 ifscope [ethernet] // kitchen
