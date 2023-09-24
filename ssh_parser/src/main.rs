// -Matzr3lla -$t@$h    QVLx Labs

/*
	The ssh parser will parse ssh packets and will write the output
	to the file "parsed_ssh.txt".
*/
use ssh_parser::parse_ssh_packet;
use std::io::stdin;
use std::io::Write;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Input file name: ");
    let mut str_file = String::new();
    match stdin().read_line(&mut str_file) { 
        Ok(input) => input,
	Err(err) => {
		        println!("Unable to read file. Error: {}",err);
			return;
		    }	
    };
    let mut file = match File::open(str_file.trim()) {
		   Ok(input) => input,
		   Err(err) => {
			           println!("Unable to open specified file. Error: {}",err);
				   return;
			       }
		   };
    let mut buffer: Vec<u8> = Vec::new(); 
    match file.read_to_end(&mut buffer) {
        Ok(input) => input,
        Err(err) => {
    		     println!("Unable to read from specified file. Error: {}",err);
		     return;
 		    }	 
    };

    let mut out_file = match File::create("out/sshparse/parsed_ssh.txt") {
		           Ok(input) => input,
			   Err(err) => {
				           println!("Unable to create parsed_pkt.txt file. Error: {}",err);
					   return;
				       }
		       };

    let parse_pkt = match parse_ssh_packet(&buffer[..]) { 
			    Ok(input) => input,
			    Err(err) => {
					    println!("Unable to parse ssh packet. Error: {}",err);
					    return;
					}
			};

	match write!(out_file,"{:#?}",(parse_pkt.1).0) {
		Ok(num_bytes) => num_bytes,
		Err(err) => {
						println!("Unable to write to file out/sshparse/parsed_pkt.txt. Error: {}",err);
						return;
					}
	};
}
