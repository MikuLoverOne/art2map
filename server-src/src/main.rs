use std::io::prelude::*;
use std::io;
use std::net::TcpListener;
use std::net::TcpStream;
mod colors;

const MESSAGE_BUFFER_SIZE: usize = 512; // Maximum message size
const LINK_BUFFER_SIZE: usize = 1024; // Maximum link buffer

fn main() {
    let mut output_port = String::new();
    println!("Type a custom 4-inch port or just pass:");
    match io::stdin().read_line(&mut output_port) {
        Ok(_) => {},
        Err(_) => {println!("Failed to read output. Try again."); return},
    }
    // We can set custom port, but if it doesnt work - we will just use standart one
    let port = {
        if output_port.len() >= 4 {
            output_port.replace("\n", "").replace("\r", "")
        } else {"5555".to_owned()}};

    let ip_adress = format!("127.0.0.1:{}", port);
    let kill_adress = format!("http://{}/kill", ip_adress);

    match ctrlc::set_handler(move || stop_server(&kill_adress)) {
        Ok(_) => {}, 
        Err(_) => {println!("Failed to setup CTRL-C handler. Try again."); return},
    };

    match TcpListener::bind(&ip_adress) // Please dont hate me, this part is just hard to read.
    {
        Ok(listener) => 
        {
            println!("Server has started on port {}. Press CTRL+C to stop it!", port);
            for stream in listener.incoming() 
            {
                match stream 
                {
                    Ok(stream) => if handle_connection(stream) == false { break },
                    Err(_) => break,
                }
            }
        },
        Err(e) => println!("Error while creating a listener: {}", e),
    }
    println!("Server has stopped.");
}

fn stop_server(kill_adress: &str) {
    match reqwest::blocking::get(kill_adress) {
        Ok(_) => println!("Trying to stop the server..."),
        Err(e) => println!("Attempt to stop the server has failed! {}", e),
    }
}

fn check_link(link: &str) -> bool {
    // Sends a request to the link. Checks if link works properly.
    match reqwest::blocking::get(link) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}

fn handle_connection(mut stream: TcpStream) -> bool {
    let mut buffer = [0; MESSAGE_BUFFER_SIZE];
    match stream.read(&mut buffer){
        Ok(_) => {},
        Err(_) => {return true},
    }
    // Try to read buffer, but if fails - just stop the connection.

    let mut response = String::from("HTTP/1.1 400 INCORRECT USAGE!");

    if buffer.starts_with(b"POST /art2map HTTP/1.1\r\n") {
        response = String::from("HTTP/1.1 200 OK \r\n\r\n");
        let content: String = String::from_utf8_lossy(&buffer[..]).into_owned();
        println!("New connection: \n{}", content);

        let link;
        if content.contains("application/x-www-form-urlencoded"){
            let mut link_buffer = [0; LINK_BUFFER_SIZE];
            // Getting link itself, which mod would try to send us
            match stream.read(&mut link_buffer){
                Ok(_) => {},
                Err(_) => {return true},
            }
            let raw_link = String::from_utf8_lossy(&link_buffer[..]).to_owned();
            link = decode_link(&raw_link);
            let link_safe = check_link(&link);
            if link != "" && link_safe == true {
                println!("Got request to transfrom link: {}", link);
                let json_array = convert_image(&link);
                if json_array != "" {
                    response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}", 
                    json_array.len(), json_array);
                }
                else {
                    response = String::from("HTTP/1.1 400 ERROR!");
                }
            }
            else {
                response = String::from("HTTP/1.1 400 ERROR!");
            }
        }
    } 
    else if buffer.starts_with(b"GET /kill HTTP/1.1\r\n") { 
        return false; 
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    return true;
}

fn convert_image(link: &str) -> String {
    /* Downloads image from the url, converts to minecraft map color-array, and then returns as json array string*/
    let img_bytes = match reqwest::blocking::get(link) {
        Ok(request) => match request.bytes() {
            Ok(bytes) => bytes,
            Err(_) => return String::from("")
        },
        Err(_) => return String::from("")
    };

    let raw_image = match image::load_from_memory(&img_bytes) {
        Ok(img) => img,
        Err(_) => return String::from("")
    };

    let resized_image = raw_image.resize_exact(128, 128, image::imageops::Triangle).to_rgb32f();
    // Downloading an image from internet

    let color_array: [u32; 16384] = colors::image_to_array(resized_image);
    // Converting image to minecraft map color-array

    let mut json_array: String = String::from("[");
    for (index, color_code) in color_array.iter().enumerate() {
        if index == 0 {
            json_array += &color_code.to_string();
        } else {
            json_array += &format!(", {}", &color_code.to_string());
        }
    }
    json_array += "]";
    // Converting vector to json array
    return json_array;
}

fn decode_link(link: &str) -> String {
    /*Converts ~encoded~ link to normal, that can be used to download images later*/

    let mut str_vec: Vec<String> = Vec::new();
    for char_number in link.split(".") {
        let num = match char_number.parse::<u32>() {
            Ok(num)  => num,
            Err(_) => return str_vec.join(""), 
            //In case a link is converted incorrectly
        };
        str_vec.push(char::from_u32(num).unwrap().to_string());
    }
    return str_vec.join("");
}