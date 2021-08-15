
mod options;
mod fasta;
mod weighting;
mod gap;
mod result;

fn main() {

	println!( "\nCalculate sequence weighting.\n" );

	// Set options.
	let opts = options::Options::new();
	opts.show_parameter();

	// Read an input file and get FASTA information.
	let data = fasta::Fasta::new( &( opts.input ) );
	data.check_fasta_info();

	/*
	println!( "Inputfile content :\n" );
	for i in 0 .. ( data.sequence ).len() {
		println!( "Title    {} : {}", i + 1, ( data.title )[ i ] );
		println!( "Sequence {} : {}", i + 1, ( data.sequence )[ i ] );
	}
	*/

	// Get site information to calculate Henikoff weighting factor..
	let site_list : Vec<String> = data.get_site_list();

	/*
	println!( "\nSite content :\n" );
	for i in 0 .. site_list.len() {
		println!( "Site {} : {}", i + 1, site_list[ i ] );
	}
	*/

	//println!( "fn main(), &seq_list  : {:p}", &( data.sequence ) );
	//println!( "fn main(), &site_list : {:p}", &site_list );

	let weight_list : Vec<f64> = weighting::seq_weight( &( data.sequence ), &site_list, &( opts.method ), &( opts.tolerate ) );

	/*
	println!( "\nSequence weighting :\n" );
	for i in 0 .. weight_list.len() {
		println!( "Weight of Sequence {} : {}", i + 1, weight_list[ i ] );
		sum_weight += weight_list[ i ];
	}
	*/

	// Calculate gap penalties taking acconut of sequence weighting.
	let gap_pen_list : Vec<f64> = gap::weight_gap_penalty( &site_list, &weight_list );

	/*
	for i in 0 .. gap_pen_list.len() {
		println!( "[ Gap penalty ] Site {} : {:.4}", i + 1, gap_pen_list[ i ] );
	}
	*/

	// Show result.
	result::show_result( &gap_pen_list, &weight_list, &( data.title ) );

	// Save result.
	result::save_result( &gap_pen_list, &weight_list, &( data.title ), &( opts.input ), &( opts.output ) );

	println!( "\nProgram completed !!!\n" );

}
