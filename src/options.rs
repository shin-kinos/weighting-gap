
use std::env;
use std::process;

pub struct Options {
	pub input    : String,
	pub output   : String,
	pub method   : String,
	pub tolerate : String,
}

impl Options {
	pub fn new() -> Options
	{
		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_o : &String = &String::new();
		let mut arg_m : &String = &String::from( "hen" );
		let mut arg_t : &String = &String::from( "yes" );

		if argc < 5 { show_usage( &argv[ 0 ] ) };

		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"-i" => { i += 1; arg_i = &argv[ i ]; }
				"-o" => { i += 1; arg_o = &argv[ i ]; }
				"-m" => { i += 1; arg_m = &argv[ i ]; }
				"-t" => { i += 1; arg_t = &argv[ i ]; }
				"-h" => { show_usage( &argv[ 0 ] ); }
				_    => { show_usage( &argv[ 0 ] ); }
			}
			i += 1;
		}

		match ( *arg_m ).as_str() {
			"hen" | "va" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *arg_t ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		Options {
			input    : arg_i.to_string(),
			output   : arg_o.to_string(),
			method   : arg_m.to_string(),
			tolerate : arg_t.to_string(),
		}
	}

	pub fn show_parameter( &self )
	{
		println!( "\nParameter set :" );
		println!( "===========================================" );
		println!( "Input filename   : {}", self.input );
		println!( "Onput filename   : {}", self.output );
		println!( "Weighting method : {}", self.method );
		println!( "Tolerate BZXU    : {}", self.tolerate );
		println!( "===========================================" );
	}
}

fn show_usage( arg : &String )
{
	println!( "Usage: {} [Options] \n\nOptions\n\n", *arg );
	println!( "    -i    Input filename in aligned Multi-FASTA format, REQUIRED." );
	println!( "    -o    Onput filename, REQUIRED." );
	println!( "    -m    Method of sequence weighting ('hen' or 'va', default 'hen').\n              hen : Position-Based method by Henikoff and Henikoff\n              va  : Distance-Based method by Vingron and Argos" );
	println!( "    -t    Tolerate AA types 'B', 'Z', 'X' and 'U' in input file ('yes' or 'no', default 'yes').\n          If 'no', program halts when the input file includes B, Z, X, or U." );
	println!( "    -h    Print this help, ignore all other arguments." );
	println!( "\n" );

	process::exit( 1 );
}
