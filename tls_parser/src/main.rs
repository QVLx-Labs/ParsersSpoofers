// -Matzr3lla -$t@$h    QVLx Labs

/*
	The tls parser will parse either plaintext or encrypted packets
	and will write output to seperate files.
*/
use std::io::stdin;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use hex;
use tls_parser::parse_tls_plaintext; 
use tls_parser::parse_tls_encrypted;
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


    let mut exten = "";
    for (pos,ele) in (str_file.trim()).chars().enumerate() {
          if ele == '.' {
              exten = &str_file[pos..];

         }
    }

    let mut file = match File::open(str_file.trim()) {
		       Ok(input) => input,
		       Err(err) => {
			               println!("Unable to open specified file. Error: {}",err);
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

    let mut out_file = match File::create("out/tlsparse/parsed_tls.txt") {
		           	       Ok(input) => input,
			   			   Err(err) => {
				           			       println!("Unable to create out/tlsparse/parsed_tls.txt file. Error: {}",err);
					   					   return;
				       				   }
		       		   };

    if exten.trim() == ".enc" {
	    let parse_enc_pkt = match parse_tls_encrypted(&hex_vec[..]) {
			    				Ok(parsed) => parsed,
			    				Err(err) => {
					    						println!("Unable to parse encrypted TLS packet. Error: {}",err);
					    						return;
											}
							}; 
        match write!(out_file,"{:#?}",parse_enc_pkt.1) {
	    	Ok(output) => output,
	    	Err(err) => {
			    			println!("Unable to write to file parsed_tls.txt. Error: {}",err);
			    			return;
		        		}
    	};

    }
    else {
        let parse_pkt = match parse_tls_plaintext(&hex_vec[..]) { 
			    			Ok(input) => input,
			    			Err(err) => {
					    					println!("Unable to parse TLS packet. Error: {}",err);
					    					return;
										}
			
		    			};

        match write!(out_file,"{:#?}",parse_pkt.1) {
	    	Ok(output) => output,
	    	Err(err) => {
			    			println!("Unable to write to file parsed_tls.txt. Error: {}",err);
			    			return;
		        		}
        };
    }

}
