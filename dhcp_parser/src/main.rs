// -Matzr3lla -$t@$h    QVLx Labs

/*
	dhcp parser parses dhcp messages and writes output to seperate
	file "parsed_dhcp.txt"
*/
use std::io::stdin;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use dhcp_parser2::parse_dhcp_message;

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

    let mut buffer = Vec::new(); 
    match file.read_to_end(&mut buffer) {
        Ok(input) => input,
        Err(err) => {
    		     println!("Unable to read from specified file. Error: {}",err);
		     return;
 		    }	 
    };

    let mut out_file = match File::create("out/dhcpparse/parsed_dhcp.txt") {
		           	       Ok(input) => input,
			   			   Err(err) => {
				           			       println!("Unable to create parsed_tls.txt file. Error: {}",err);
					   					   return;
				       				   }
		       		   };
					
	match parse_dhcp_message(&buffer) {
		Ok(dhcp) => { 
					     match write!(out_file,"{:#?}",dhcp) {
						     Ok(file) => file,
							 Err(err) => {
										     println!("Unable to write to file out/dhcpparse/parsed_dhcp.txt. Error: {}",err);
											 return;
										 }
						 };
					 },
		Err(err) => {
						println!("Unable to parse DHCP message. Error: {}",err);
						return;
					}
	};

}
