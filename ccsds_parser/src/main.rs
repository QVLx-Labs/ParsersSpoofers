// -Matzr3lla -$t@$h    QVLx Labs

/*
 * 1.Prompt user for path to input file..
 * 2.Parse file one packet at a time into library components
 * 3.With each parsed packet , write a nice representation to a new output file
 * (Demonstration of how this will look (purely theoretical)
 * Input file: blah.txt: (can have multiple packets encoded , use source sequence
 * count to see how many packets, parse out the header , 64 bits of header size to
 * parse, loop for examining packets , write to output file (ex: Packet: <generate iterator number> 
 * Version number: __ APID:_  Data:_ ) 
 * Data strucutre for whole file, have a pointer for selecting each selection of bytes
 */ 

use ccsds_primary_header::primary_header::*;
use std::io::{self};
use std::fs;
use hex::*;
use std::fs::File;
use std::env;
use std::io::Write;
fn main() {
    
    //retrieve the command line arguments to determine if user wants to write
    // to a file or print to terminal

    let args : Vec<String> = env::args().collect();
    let mut output: u8 = 0;
    let mut output_file = match File::create("pkt_output.txt") {
	Ok(input) => input,
	Err(err) => {
		      println!("Unable to create output file : pkt_output.txt. Error: {}", err);
		      return;
		    }
    };

    for input in args.iter() {
      if input.to_lowercase() == "y"
	{
	  output = 1;
	}
      else if input.to_lowercase() == "n" {
	     output = 0;
           }
    }
    
    println!("Please input your desired file path.");
    let mut ans = String::new();

    match io::stdin().read_line(&mut ans) {
        Ok(an) => an,
        Err(err) => {
            println!("Failed to read line. Error : {}", err);
            return;
        }
    };

   let packet  = match fs::read_to_string(ans.trim()) {
      Ok(input) => input,
      Err(err) => {
        println!("Failed to read the input file. Error : {}", err);
        return;
     }
   };
 
   let cleaned_pkt = packet.replace('\n', "").replace('\r', "").replace(' ', "");
   let packet_bytes: Vec<u8> = match Vec::from_hex(cleaned_pkt) {
      Ok(input) => input,
      Err(err) => {
        println!("Failed to read the input file. Error : {}", err);
        return;
     }
    };
    
    let mut header_counter = 0;
    let mut data_counter = 0;
    let mut packet_num = 1;
    let mut packet_data = Vec::new();
    let mut header = Vec::new();
    let mut primary_head;
    let mut data_length = 0;    
    
    let mut pkt_type;  
    // loop through whole file
    for i in 0..packet_bytes.len() {
      // header segment
      if header_counter < 6 {
        &header.push(packet_bytes[i]);
        header_counter = header_counter + 1;
      }
      // counter == 6 ie deserialize header
      else if header_counter == 6 {   
        let pheader_slice = extract_header(&header);
	primary_head = PrimaryHeader::new(pheader_slice);
	pkt_type = match primary_head.control.packet_type() {
		     PacketType::Command => "Command",
		     PacketType::Data => "Telemtry",
		     PacketType::Unknown => "Error , unknown packet type."
	};
        if output == 0 {
   	println!("Packet ID: {}", packet_num ); // packet ID 
    	println!("Version: {}", primary_head.control.version()); // version number
    	println!("APID: {}", primary_head.control.apid()); // apid
    	println!("Packet type: {}" , pkt_type);
	println!("Source sequence count: {}", primary_head.sequence.sequence_count());

	println!("Packet Length: {}", primary_head.packet_length()); // packet length
    	println!("Data Length: {}", primary_head.data_length()); // data length
	
  	}
	else {
	  match output_file.write_all(("Packet ID: ".to_owned() + &packet_num.to_string() + "\n" + "Version: " 
				+ &primary_head.control.version().to_string() + "\n" + "APID: " 
				+ &primary_head.control.apid().to_string() + "\n" +
				"Packet type: " + pkt_type + "\n" 
				+ "Source sequence count: " + &primary_head.sequence.sequence_count().to_string() 
				+ "\n" + "Packet Length: " +
				&primary_head.packet_length().to_string() + "\n" + "Data Length: " + 
				&primary_head.data_length().to_string() + "\n").as_bytes()) {
				Ok(input) => input,
				Err(err) => {
					       println!("Unable to write to output file. Error: {}",err);
					       return;
					    }
				};

	  }
        data_length = primary_head.data_length();
        packet_data.push(packet_bytes[i as usize]);
        data_counter = data_counter + 1;
        header_counter = header_counter + 1;	
        packet_num = packet_num + 1;
     }
     // counter > 6 ie process data segment
     else{
        if data_counter < (data_length - 1){
          packet_data.push(packet_bytes[i as usize]);
          data_counter = data_counter + 1;
        }
        // end of data
        else{
          packet_data.push(packet_bytes[i as usize]);
	  if output == 0 {
    	  	println!("Data: {:x?}", packet_data);
	  }
	  else {
	    
	    match output_file.write_all(("Data: ".to_owned() + &hex::encode(packet_data) + "\n").as_bytes()) {
	      Ok(input) => input,
	      Err(err) => {
			    println!("Unable to write to output file. Error: {}", err);
			    return;
			  }
	    };
	  } 
	  //reset values for the next packet in parser.
          packet_data = Vec::new();
          header = Vec::new();
          data_counter = 0;
          header_counter = 0;
          data_length = 0;
        }
     }
   }

}

fn extract_header(input: &Vec<u8>) -> [u8;6] {
  if input.len() > 6 {
    println!("extract_header: invalid input -- header too long.");
  }
  let mut output : [u8;6] = [0;6];
  for (i,item) in input.iter().enumerate(){
     //dereference &u8 variable and assign to output array.
     output[i] = *item; 
  }
  return output;
}
