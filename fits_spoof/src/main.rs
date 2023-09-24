// -Matzr3lla -$t@$h    QVLx Labs

/*
	 This application will create fits files from given input files, input
	 has to follow the format "<keyword> , <value type>, <value> , <comment>".
	 Avoid putting any headers past data arrays, when data is detected
	 everything after the header will written to "out.fits" file as bytes.
 */
use fits_rs::types;
use std::fs::File;
use std::io::{Read,stdin};
use std::io::Write;
use std::str;
use std::str::FromStr;

fn main() {
	println!("Please input file path to primary header information:");
	println!("File must be in form \"<keyword> , <value type>, <value> , <comment>\".");

	let mut ph_path = String::new();
	match stdin().read_line(&mut ph_path) {
		Ok(input) => input,
			Err(err) => {
				println!("Unable to read primary header file path. Error: {}",err);
				return;
			}
	};
	let mut ph_file = match File::open(ph_path.trim()) {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to open specified primary header file path. Error: {}",err);
				return;
			}
	};

	let mut ph_data = Vec::new();
	match ph_file.read_to_end(&mut ph_data) {
		Ok(byte_num) => byte_num,
			Err(err) => {
				println!("Unable to read from specified file. Error: {}",err);
				return;
			}
	};


	let mut tup = get_kw(&mut ph_data);
	if (tup.0).len() == 0 {
		println!("Unable to extract key words from given file.");
		return;
	}
	let mut keyword_vec = tup.0;
	let mut r_data = tup.1;
	let mut out_file = match File::create("out.fits") {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create file \"out.fits\" for writing. Error: {}",err);
				return;
			}
	};
	let j = 0; 
	let tmp_kw_vec = keyword_vec.to_vec();
	let mut flag_tup = parse_kw(&mut out_file,tmp_kw_vec);

	while j < r_data.len() {
		if flag_tup.2 { 
			println!("Unable to parse keywords.");
			return;
		}
		if flag_tup.0 {
			//As soon as data is detected the rest of the data input
			//will be dumped to the file.
			write_data(&mut out_file, &mut (&r_data[j..]).to_vec());
			return;
		}
		if flag_tup.1 {
			tup = get_kw(&mut r_data[j..].to_vec());
			keyword_vec = tup.0;
			r_data = tup.1;
			let tmp_kw_vec = keyword_vec.to_vec();
			flag_tup = parse_kw(&mut out_file,tmp_kw_vec);
		}
	}
}

/*
	 This function will parse keywords and write parsed keywords to data file in
	 equivalent fits format.
 */
fn parse_kw(out_file:&mut File, keyword_vec: Vec<String>) -> (bool,bool,bool) {
	let mut i = 0;
	let mut data_flag :bool = false;
	let mut ext_flag :bool = false;

	while i < keyword_vec.len() {
		if (keyword_vec[i]) == "END" { 
			let pad = keyword_vec.len() * 80;
			let mut rem = pad % 2880;
			if rem != 0 {
				rem -= 80;
				rem = 2880 - rem;
			}
			else {rem = 80;}
			if !data_flag && !ext_flag {
				println!("LAST header no data ");
				match write!(out_file,"{value:width$}",value="END",width=rem-1) {
					Ok(val) => val,
						Err(err) => {
							println!("Unable to write to \"out.fits\". Error: {}",err);
							return (false,false,false);
						}
				};
				match write!(out_file,"{}","\n") {
					Ok(val) => val,
						Err(err) => {
							println!("Unable to write to \"out.fits\". Error: {}",err);
							return (false,false,false);
						}
				};
			}	
			else {
				match write!(out_file,"{value:width$}",value="END",width=rem) {
					Ok(val) => val,
						Err(err) => {
							println!("Unable to write to \"out.fits\". Error: {}",err);
							return (false,false,false);
						}
				};
			}
		}
		else {
			let typ_vec: Vec<&str> = (keyword_vec[i]).split(',').collect();
			if !fits_format_check(&typ_vec) {
				println!("{}",keyword_vec[i]);
				println!("Error unrecognized format");
				return (false,false,false);
			}

			let typ_2_0 = match (typ_vec[2].to_string()).chars().nth(0) {
				Some(pos) => pos,
					None => {
						println!("Unable to get element.");
						return (false,false,true);
					}
			};
			let typ_2_count = typ_vec[2].chars().count();
			let typ_3_count = typ_vec[3].chars().count();
			if typ_vec[0].contains("EXTEND") && typ_vec[2].contains("T") {ext_flag = true;}
			if typ_vec[0].contains("NAXIS0") {
				let naxis_val = match typ_vec[2].parse::<u32>() {
					Ok(val) => val,
						Err(err) => {
							println!("Unable to parse NAXIS value. Error: {}",err);
							return (false,false,true)
						}
				};

				if naxis_val != 0 {data_flag = true;}
			}
			if typ_vec[0].contains("BSCALE") || typ_vec[0].contains("BZER0") {
				match write!(out_file,"{:8}= {:12}{:<10}",typ_vec[0]," ",typ_vec[2]) {
					Ok(val) => val,
						Err(err) => {
							println!("Unable to write to \"out.fits\". Error: {}",err);
							return (false,false,false);
						}
				};
			}
			else if typ_vec[1] == "CharacterString" && typ_2_0 == '\'' && typ_2_count <= 70 {
				if typ_2_count <= 10 {
					match write!(out_file,"{:8}= {:<20} ",typ_vec[0],typ_vec[2]){
						Ok(val) => val,
							Err(err) => {
								println!("Unable to write to \"out.fits\". Error: {}",err);
								return (false,false,false);
							}
					};

				}
				else {
					match write!(out_file,"{:8}= {:<70}",typ_vec[0],typ_vec[2]){
						Ok(val) => val,
							Err(err) => {
								println!("Unable to write to \"out.fits\". Error: {}",err);
								return (false,false,false);
							}
					};

				}
			}
			else {
				match write!(out_file,"{:8}= {:>20} ",typ_vec[0],typ_vec[2]){
					Ok(val) => val,
						Err(err) => {
							println!("Unable to write to \"out.fits\". Error: {}",err);
							return (false,false,false);
						}
				};
			}

			if typ_3_count == 2 {
				match write!(out_file,"/ {:47}"," "){
					Ok(val) => val,
						Err(err) => {
							println!("Unable to write to \"out.fits\". Error: {}",err);
							return (false,false,false);
						}
				};

			}
			else if typ_3_count > 2 {
				match write!(out_file,"/ {:47}",typ_vec[3].replace("'","")){
					Ok(val) => val,
						Err(err) => {
							println!("Unable to write to \"out.fits\". Error: {}",err);
							return (false,false,false);
						}
				};

			}
		}
		i += 1;
	}
	(data_flag,ext_flag,false)

}

/*
	 get_kw returns keyword data from input file to be parsed later,
	 the remaining data will be stored as bytes in tuple position 1.
 */
fn get_kw(vec: &mut Vec<u8>) -> (Vec<String>,Vec<u8>) {
	let mut line: Vec<u8> = Vec::new();
	let mut t_vec = vec.to_vec();
	let mut kw: Vec<String> = Vec::new();
	for (pos,ele) in t_vec.iter().enumerate() {
		if kw.len() == 10 {break;}
		if *ele == 10 as u8 {
			line.push(*ele);
			let t_line = line.to_vec();
			let s = match String::from_utf8(t_line) {
				Ok(string) => string,
					Err(err) => {
						println!("Unable to convert from bytes to string. Error: {:?}",err);
						return (Vec::new(),Vec::new())
					}
			};
			line = Vec::new();
			let tmp_s = s.clone();
			kw.push(tmp_s.trim().to_string());
			if s.trim() == "END" {
				t_vec = (&vec[(pos+1)..]).to_vec();
				break;
			}
		}
		else {
			line.push(*ele);
		}

	}
	(kw,t_vec)
}

/*
	 fits_format_check determines if the given input format abides
	 the input parameters.
 */
fn fits_format_check(vec: &Vec<&str>) -> bool {
	let iden = match fits_rs::types::Keyword::from_str(vec[0]) {
		Ok(key) => key,
			_ => {	
				println!("Error unrecognized keyword.");
				return false;
			},		
	};
	let val = match vec[1] {
		"CharacterString" =>  0,
			"Logical" => 0,
			"Integer" => 0,
			"Real" => 0,
			"Complex" => 0,
			"Undefined" => 0,
			_ => 1,
	};
	if iden == types::Keyword::Unprocessed {return false;}
	else if val == 1 {return false;}
	true
}
/*
	 fn data_end(vec: &[u8]) -> usize {
	 let mut i = 0;
	 while i < vec.len() {
	 if vec[i] == 88 && vec[i+1] == 84 && vec[i+2] ==69 && 
	 vec[i+3] == 78 && vec[i+4] ==83 && vec[i+5] == 73 && vec[i+6] == 79 
	 && vec[i+7] == 78 {return i;}
	 i += 1;
	 }
	 return 0;
	 }
 */
/*
	 write_data writes the data array as bytes to the "out.fits" file
 */
fn write_data(out_file: &mut File, data_vec:&mut Vec<u8>) {
	if data_vec[data_vec.len() -1] == 10 {
		data_vec.pop();
		data_vec.push(32);
	}
	let mut rem = data_vec.len() % 2880;
	if rem != 0 {
		rem = 2880 - rem;
	}

	for _i_ in 0..rem {
		data_vec.push(32);
	}
	match out_file.write_all(&data_vec) {
		Ok(data) => data,
			Err(err) => {
				println!("Unable to write data to file. Error: {}",err);
				return;
			}
	};
}
