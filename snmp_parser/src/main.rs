// -Matzr3lla -$t@$h    QVLx Labs

/*
	Snmp parser parses snmp messages (no matter what version) and writes
	output to file "parsed_snmp.txt"
*/
use std::io::stdin;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use snmp_parser::parse_snmp_generic_message;
use snmp_parser::SnmpGenericMessage::V1;
use snmp_parser::SnmpGenericMessage::V2;
use snmp_parser::SnmpGenericMessage::V3;
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

    let mut out_file = match File::create("out/snmpparse/parsed_snmp.txt") {
		           	       Ok(input) => input,
			   			   Err(err) => {
				           			       println!("Unable to create out/snmpparse/parsed_tls.txt file. Error: {}",err);
					   					   return;
				       				   }
		       		   };

	let parse_snmp = match parse_snmp_generic_message(&buffer) {
					     Ok(parse) => parse,
						 Err(err) => {
									     println!("Unable to parse SNMP message. Error: {}",err);
										 return;
									 }
					 };

	let snmp_mes = match parse_snmp {
		               (_,V1(snmp_mes)) => snmp_mes,
					   (_,V2(snmp_mes)) => snmp_mes, 
					   (_,V3(snmpv3_mes)) => {
											 	match write!(out_file,"{:#?}",snmpv3_mes) {
													Ok(byte_num) => byte_num,
													Err(err) => {
																	println!("Unable to write to parsed_snmp.txt. Error: {}",err);
																	return;
																}
												};
												return;
											 }
				   };
					
										
	match write!(out_file,"{:#?}",snmp_mes) {
		Ok(file) => file,
		Err(err) => {
						println!("Unable to write to file parsed_snmp.txt. Error: {}",err);
						return;
					}
	};

}
