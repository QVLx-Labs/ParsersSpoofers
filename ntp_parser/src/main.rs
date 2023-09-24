// -Matzr3lla -$t@$h    QVLx Labs

/*
	Ntp parser parses ntp packets and writes output to
	file "parse_ntp.txt"
*/
use std::io::stdin;
use ntp_parser::parse_ntp;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use ntp_parser::NtpPacket::V3;
use ntp_parser::NtpPacket::V4;
fn main() {
    println!("Input file path: ");
	let mut str_file = String::new();
	match stdin().read_line(&mut str_file) {
		Ok(num_byte) => num_byte,
		Err(err) => {
						println!("Unable to read input. Error: {}",err);
						return;
					}
	};

	let mut file = match File::open(str_file.trim()) {
				       Ok(file) => file,
					   Err(err) => {
								       println!("Unable to create specified file. Error: {}",err);
									   return;
								   }
				   };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(input) => input,
        Err(err) => {
                	    println!("Unable to read from specified file. Error: {}",err);
              			return;
             		}
    };

    buffer = (buffer.trim()).replace("\n"," ");
    let vec: Vec<&str> = buffer.split(" ").collect();
    let mut hex_vec: Vec<u8> = Vec::new();
    let mut hex_dec;

    for ele in vec.iter() {
        hex_dec = match hex::decode(ele) {
                      Ok(hex) => hex,
                      Err(err) => {
                       		          println!("Unable to decode &str to raw bytes. Error: {}",err);
                       			      return;
                   			      }
                  };
    	hex_vec.push(match hex_dec.pop() {
             	     	 Some(raw_byte) => raw_byte,
                     	 None => {
                                     println!("Unable to pop element.");
                     			 	 return;
                 			 	 }
              		});
    }
	
	let mut out_file = match File::create("out/ntpparse/parse_ntp.txt") {
					       Ok(file) => file,
						   Err(err) => {
									       println!("Unable to create file out/ntpparse/parse_ntp.txt. Error: {}",err);
										   return;
									   }
					   };	

	let ntp = match parse_ntp(&hex_vec) {
			      Ok((_,ntp)) => ntp,
		          //Ok((_,V4(ntpv4))) => ntpv4,
		          Err(err) => {
				      		      println!("Unable to parse NTP packet. Error: {}",err);
					 			  return;
				  			  }
			  };
	match ntp {
		V3(ntpv3) => {
					     write!(out_file,"{:#?}",ntpv3).expect("TKJEK");
					 },
		V4(ntpv4) => {
					 	
					     write!(out_file,"{:#?}",ntpv4).expect("#$");
					 }
	};

}
