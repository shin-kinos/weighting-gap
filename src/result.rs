
use std::process;
use std::fs::File;
use std::io::Write;

pub fn show_result( arg_penalty : &Vec<f64>, arg_weight : &Vec<f64>, arg_title : &Vec<String> )
{
	println!( "\nResult:\n" );

	if ( *arg_weight ).len() != ( *arg_title ).len() {
		println!( "\nERROR !!!\n" );
		println!( "Length of ( *arg_weight ) != Length of ( *arg_title )" );
		println!( "\nProgram halted !!!\n" );

		process::exit( 1 );
	}

	let num_seq  : usize = ( *arg_weight ).len();
	let num_site : usize = ( *arg_penalty ).len();

	println!( "site\tgap_penalty" );
	for i in 0 .. num_site {
		println!( "{}\t{:.4}", i + 1, ( *arg_penalty)[ i ] );
	}
	print!( "\n" );

	println!( "num\tweight\ttitle" );
	for i in 0 .. num_seq {
		if ( *arg_title )[ i ].len() > 80 {
			print!( "{}\t{:.4}\t{}", i + 1, ( *arg_weight )[ i ], ( *arg_title )[ i ].chars().take( 80 ).collect::<String>() );
			println!( " ..." );
		} else {
			println!( "{}\t{:.4}\t{}", i + 1, ( *arg_weight )[ i ], ( *arg_title )[ i ] );
		}
	}

}

pub fn save_result( gap_pen_list : &Vec<f64>, weight_list : &Vec<f64>, title_list : &Vec<String>, arg_i : &String, arg_o : &String )
{
	let num_seq  : usize = ( *weight_list ).len();
	let num_site : usize = ( *gap_pen_list ).len();

	let cat_gap_pen : &str = "_gap_penalty.";
	let cat_weight  : &str = "_weighting.";

	let mut fout = File::create( ( *arg_o ).as_str() ).expect( "FAILED to open output file" );

	writeln!( fout, "{}", *arg_i ).expect( "FAILED to write" );
	writeln!( fout, "#" ).expect( "FAILED to write" );

	writeln!( fout, "loop_" ).expect( "FAILED to write" );
	writeln!( fout, "{}number_site", cat_gap_pen ).expect( "FAILED to write" );
	writeln!( fout, "{}gap_penalty_value", cat_gap_pen ).expect( "FAILED to write" );
	for i in 0 .. num_site {
		writeln!( fout, "{}\t{:.4}", i + 1, ( *gap_pen_list )[ i ] ).expect( "FAILED to write" );
	}
	writeln!( fout, "#" ).expect( "FAILED to write" );

	writeln!( fout, "loop_" ).expect( "FAILED to write" );
	writeln!( fout, "{}number_sequence", cat_weight ).expect( "FAILED to write" );
	writeln!( fout, "{}weighting_value", cat_weight ).expect( "FAILED to write" );
	writeln!( fout, "{}title_line", cat_weight ).expect( "FAILED to write" );
	for i in 0 .. num_seq {
		writeln!( fout, "{}\t{:.4}\t{}", i + 1, ( *weight_list )[ i ], ( *title_list )[ i ] ).expect( "FAILED to write" );
	}
	writeln!( fout, "#\n" ).expect( "FAILED to write" );

	println!( "\nThe output file was written correctly.\n" );
}
