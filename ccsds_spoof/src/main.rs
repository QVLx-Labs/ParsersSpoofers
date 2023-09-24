// -Matzr3lla -$t@$h    QVLx Labs

/*
 * This application will parse through a given ccsds packet file to obtain packet
 * data that will be written to the seperate file ccsds_pkt.txt
 */

use std::fs::File;
use std::fs;
use hex::*;
use std::io::*;
use pretty_hex::*;
use rustc_hex::ToHex;

fn main() {

// Prompt user for input file name
  println!("Please input file path: ");
  let mut file_name = String::new();
  match stdin().read_line(&mut file_name) {
    Ok(input) => input,
    Err(err) => {
      println!("Failed to open file. Error: {}",err);
      return;
    }
  };
  
  // Read in the input file
  let packet = match fs::read_to_string(file_name.trim()) {
    Ok(input) => input, 
    Err(err) => {
      println!("Failed to read the input file. Error: {}", err);
      return;
    }
  };
  
  // Convert input file to byte vector
  let trimmed_pkt = packet.replace('\n', "").replace('\r', "").replace(' ', "");
  let data_vector : Vec<u8> = match Vec::from_hex(trimmed_pkt) {
    Ok(input) => input, 
    Err(err) => {
      println!("Unable to convert packet string to vector of bytes. Error: {}",err);
      return;
    }
  };
  
  println!("Please give us CCSDS packet fields.");
   
  // Prompt user for version num
  print!("Version number: ");
  match stdout().flush() {
    Ok(input) => input,
    Err(err) => {
		  println!("There was an error flushing stdout. Error: {}", err);
		  return;
		}
  };

  let mut version_num = String::new();
  match stdin().read_line(&mut version_num) {
    Ok(input) => input,
    Err(err) => {
      println!("Failed to get version num. Error: {}",err);
      return;
    }
  };
 
  //TODO: Prompt user for packet type - Telemetry or Command
  print!("Please input packet type : Telemtry or Command.");
  match stdout().flush() {
    Ok(input) => input,
    Err(err) => {
		  println!("There was an error flushing stdout. Error: {}", err);
		  return;
		}
  };

  let mut packet_type = String::new();
  match stdin().read_line(&mut packet_type) {
    Ok(input) => input,
    Err(err) => {
		  println!("Failed to get packet type. Error : {}", err);
		  return;
		}
    };
  packet_type = packet_type.replace('\n', "");
  
  //TODO: Need to check all field values to make sure they don't exceed max.
  //example: Max version number can be is ((2 ^ 3) - 1) = 7 because 3 bits long. 
  //if input field is longer than max, print an error and exit for now. 
  
  //TODO: Convert packet type to 0 to 1 based on the Space Packet standard.
  let pkttype_bit : u16;
  if packet_type.eq("telemtry"){
    pkttype_bit = 0;
  }
  else {
    pkttype_bit = 1;
  }

  // Prompt user for APID
  print!("Application Process Identifier: ");
  match stdout().flush() {
    Ok(input) => input,
    Err(err) => {
		  println!("Failed to get packet type. Error : {}", err);
		  return;
		}
    };

  let mut apid = String::new();
  match stdin().read_line(&mut apid) {
    Ok(input) => input,
    Err(err) => {
      println!("Failed to get APID. Error: {}",err);
      return;
    }
  };
  
  // Prompt user for seq count
  print!("Source sequence count: ");    
  match stdout().flush() {
    Ok(input) => input,
    Err(err) => {
		  println!("Failed to get packet type. Error : {}", err);
		  return;
		}
    };

  let mut seq_count = String::new();
  match stdin().read_line(&mut seq_count) {
    Ok(input) => input,
    Err(err) => {
      println!("Failed to get seq count. Error: {}",err);
      return;
    }
  };
  
  // Convert fields to unsigned 8-bit integers
  let mut version_num_u16 = match version_num.trim().parse::<u16>() {
                                Ok(input) => input,
                                Err(err) => {
                                                println!("Unable to parse version_num. Error : {}",err);
                                                return;
                                            }
                            };
  let apid_u16 = match apid.trim().parse::<u16>() {
                     Ok(input) => input,
                     Err(err) => {
                                     println!("Unable to parse apid. Error: {}",err);
                                     return;
                                 }
                 };
  let seq_count_u16 = match seq_count.trim().parse::<u16>() {
                          Ok(input) => input,
                          Err(err) => {
                                          println!("Unable to seq_count. Error: {}",err);
                                          return;
                                      }
                      };

  
  // Initialize 3 2-octet wordss for CCSDS header
  let mut header_word1: u16 = 0; // Version num(3bits), Type(1bit), Sec hdr flag(1bit), APID(11bits)    
  let mut header_word2: u16 = 0; // Seq flags(2bits), Seq count(14bits)
  let header_word3: u16; // Data length(16bits)

  //NOTE: Sec hdr will be set to zero
  let secon_flag : u16 = 0;  
  //NOTE: Seq flags will be set to zero
  //NOTE: Data length is calculated from read input file string length after trimming whitepace
  let pktdata_len = (data_vector.len()/2+6) as u16 ;//.parse::<u16>().expect("need to make this a match");
  
  // Bitwise operations to pack fields into header words
  version_num_u16 = version_num_u16 << (13);
  header_word1 = header_word1 | version_num_u16;
  header_word1 = apid_u16 | header_word1;
  header_word1 = (secon_flag << 11) | header_word1;
  header_word1 = (pkttype_bit << 12) | header_word1;
  header_word2 = seq_count_u16 | header_word2;
  header_word2 = (secon_flag << 15) | header_word2;
  header_word3 = pktdata_len;
  
  //TODO: Finish packing the rest of the fields
  let primary_header_u64 : u64 ;
  primary_header_u64 = ((header_word1 as u64) << 32) | ((header_word2 as u64) << 16) | header_word3 as u64;
  
  let primary_hdr_bytes = primary_header_u64.to_be_bytes();
  let pri_to_hex: String = primary_hdr_bytes.to_hex();
  let mut pretty_hdr = string_to_octets(pri_to_hex);
  pretty_hdr.drain(0..2); // Remove first two elements

  // [header - 6 octets] [data - pktdata_len octets]
  // [you build this   ] [you literally just rewrite this to file as is after writing header]
 
  // read file into a string - this gets written as is to output after header
  // trim whitespace from said string
  // get length of string divided by two and thats your header pktdata_len field
  // add 6 to the pktdata_len field and that is your packet_len field. packet_len != pktdata_len
 
  // Create an output file to write packet to
  let mut _new_file = match File::create("ccsds_pkt.txt") {
    Ok(input) => input,
    Err(err) => {
      println!("Unable to write to a new file. Error : {}",err);
      return;
    }
  };

  //TODO: Write packet to output file
  //NOTE: Don't forget to print spaces in between octets in the file
   
  let mut _file_string_2 = &simple_hex(&data_vector).replace('\n', "").replace('\r', "").replace(' ', "");
  //let file_string_1 = &simple_hex(&primhdr_vec);
  let mut _file_string_1 = String::new();
  //let mut _file_vec_2 = string_to_octets(_file_string_2.to_string());
  for i in &pretty_hdr {
    _file_string_1.push_str(" ");
    _file_string_1.push_str(i);
  }
  let mut oct_string = String::new();
  for (i, item) in _file_string_2.chars().enumerate()  { 
    if i % 2 == 0 {
      oct_string.push_str(" ");
      oct_string.push(item);
    }
    else {
      oct_string.push(item);
    }
  }
  
  match _new_file.write_all((_file_string_1 + &oct_string).as_bytes()) {
    Ok(input) => input,
    Err(err) => {
		  println!("Unable to write to file. Error: {}", err);
		  return;
		}
  };
} 

/*
 * string_to_octets will take a String as input and will output
 * a primary header of type Vec<String>.
 */
fn string_to_octets(input: String) -> Vec<String> {
    // odd length -- bad
    if input.len() % 2 != 0 {
      return vec!["".to_string()];
    }
    let mut output = Vec::<String>::new();
    let mut octet = String::new(); 
    for (i,nibble) in input.chars().enumerate() {
      if i % 2 == 0 {
        octet.push(nibble);
      }
      else {
        octet.push(nibble);
        output.push(octet);
        octet = String::new();
      }
    }
    return output;
}
